import copy
import importlib.util
from html.parser import HTMLParser
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

spec = importlib.util.spec_from_file_location('report', Path(__file__).with_name('report.py'))
report = importlib.util.module_from_spec(spec)
spec.loader.exec_module(report)


def result(ns):
    return dict(timing=dict(per_call=dict(p50_ns=ns, p95_ns=ns*2), batch_means=dict(p50_ns=ns/2, p95_ns=ns)),
                resources=dict(rust_allocator=None, process_peak_rss_bytes=None))


class ReportTests(unittest.TestCase):
    def test_hardware_names_are_readable_without_mutating_provenance(self):
        self.assertEqual(report.cpu_name(dict(cpu_model='arm',hardware_details='Apple M1 (Virtual)')),'Apple M1 (Virtual)')
        self.assertEqual(report.cpu_name(dict(cpu_model='aarch64',hardware_details='{"lscpu":[{"field":"Model name:","data":"Neoverse-N2"}]}')),'Neoverse-N2')
        self.assertEqual(report.cpu_name({}),'CPU unspecified')

    def test_nominal_throughput_uses_call_latency_and_only_dense_gemm(self):
        self.assertEqual(report.throughput('matmul_128_f64',result(1000)),2*128**3/1000)
        self.assertIsNone(report.throughput('qr_96x32_f64',result(10)))
        self.assertIsNone(report.throughput('matmul_128_f64',result(0)))

    def test_latency_change_keeps_sign_and_missing_is_not_zero(self):
        self.assertAlmostEqual(report.change(result(80), result(100)), -20)
        self.assertAlmostEqual(report.change(result(120), result(100)), 20)
        self.assertIsNone(report.change(result(120), None))
        self.assertIsNone(report.change(None, result(100)))
        self.assertIsNone(report.change(result(10), result(0)))

    def test_render_retains_losses_unknown_cases_and_escapes_metadata(self):
        data = dict(metadata=dict(revision='abc<unsafe>', run_id='<script>ROWS</script>',
                                  library_packages=[dict(name='kernix', version='0.0.0')]),
                    results={'matmul_128_f64/kernix': result(120), 'matmul_128_f64/numpy': result(80),
                             'future_<case>/kernix': result(40)})
        original = copy.deepcopy(data)
        baseline = copy.deepcopy(data)
        baseline['results'] = {'matmul_128_f64/kernix': result(100)}
        with tempfile.TemporaryDirectory() as temporary, patch.object(report, 'render_chart'):
            root = Path(temporary)
            report.render_report(root, data, baseline)
            md = (root/'README.md').read_text()
            html = (root/'report.html').read_text()
        self.assertIn('0.120 | **0.080** | +20.0%', md)
        self.assertIn('future <case>', md)
        self.assertIn('future &lt;case&gt;', html)
        self.assertIn('unavailable', html)
        self.assertIn('&lt;script&gt;ROWS&lt;/script&gt;', html)
        self.assertNotIn('<script>ROWS</script>', html)
        self.assertEqual(data, original)

    def test_baseline_environment_mismatch_is_visible(self):
        a = dict(metadata=dict(environment=dict(cpu_model='first', governor='powersave')))
        b = dict(metadata=dict(environment=dict(cpu_model='second', governor='performance')))
        warnings = report.baseline_warnings(a, b)
        self.assertIn('Different cpu model.', warnings)
        self.assertIn('Different governor.', warnings)
        self.assertEqual(report.baseline_warnings(a, None), [])
        a['metadata']['explicit_execution']={'work':{'workers':2,'affinity_cpus':[0,2]}}
        self.assertIn('Different explicit executor or affinity policies.',report.baseline_warnings(a,b))

    def test_html_links_are_valid_and_literal_template_text_is_preserved(self):
        class Links(HTMLParser):
            def __init__(self):
                super().__init__()
                self.hrefs = []
                self.details = 0
                self.workloads = []

            def handle_starttag(self, tag, attrs):
                if tag == 'a':
                    self.hrefs.append(dict(attrs).get('href'))
                if tag == 'details':
                    self.details += 1
                if tag == 'tr' and 'data-workload' in dict(attrs):
                    self.workloads.append(dict(attrs)['data-workload'])

        data = dict(metadata=dict(revision='abc', run_id='DETAILS.md {{VERSION}}'),
                    results={'matmul_128_f64/kernix': result(100)})
        with tempfile.TemporaryDirectory() as temporary, patch.object(report, 'render_chart'):
            root = Path(temporary)
            report.render_report(root, data)
            text = (root/'report.html').read_text()
        links = Links()
        links.feed(text)
        self.assertEqual(links.hrefs, ['DETAILS.md', 'raw.json', 'latency.pdf'])
        self.assertEqual(links.details, 1)
        self.assertEqual(links.workloads, ['matmul_128_f64'])
        self.assertIn('run DETAILS.md {{VERSION}}.', text)

    def test_affinity_and_solver_contract_follow_actual_observations(self):
        data = dict(metadata=dict(revision='abc', run_id='host', environment=dict(cpu_model='AMD', affinity_cpu=None)),
                    results={'min_norm_32x64_rank32_f64/kernix': result(100)})
        original = copy.deepcopy(data)
        with tempfile.TemporaryDirectory() as temporary, patch.object(report, 'render_chart'):
            root = Path(temporary)
            report.render_report(root, data)
            for name in ['README.md', 'report.html']:
                text = (root/name).read_text()
                self.assertIn('CPU affinity was not pinned.', text)
                self.assertNotIn('shared laptop', text)
                self.assertIn('native SVD/solve', text)
                self.assertIn('rank policies differ', text)
            data['metadata']['environment']['affinity_cpu'] = 0
            report.render_report(root, data)
            self.assertIn('CPU affinity pinned to logical CPU 0.', (root/'README.md').read_text())
            data['metadata']['minimum_norm_algorithm'] = 'pivoted QR plus scaled implicit RZ'
            report.render_report(root, data)
            for name in ['README.md', 'report.html']:
                self.assertIn('scaled implicit RZ', (root/name).read_text())
            del data['metadata']['minimum_norm_algorithm']
        data['metadata']['environment']['affinity_cpu'] = None
        self.assertEqual(data, original)


if __name__ == '__main__':
    unittest.main()
