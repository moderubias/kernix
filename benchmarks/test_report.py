import copy
import importlib.util
from html.parser import HTMLParser
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

spec = importlib.util.spec_from_file_location(
    "report", Path(__file__).with_name("report.py")
)
report = importlib.util.module_from_spec(spec)
spec.loader.exec_module(report)


def result(ns):
    return dict(
        timing=dict(
            per_call=dict(p50_ns=ns, p95_ns=ns * 2),
            batch_means=dict(p50_ns=ns / 2, p95_ns=ns),
        ),
        resources=dict(rust_allocator=None, process_peak_rss_bytes=None),
    )


class ReportTests(unittest.TestCase):
    def test_hardware_names_are_readable_without_mutating_provenance(self):
        self.assertEqual(
            report.cpu_name(
                dict(cpu_model="arm", hardware_details="Apple M1 (Virtual)")
            ),
            "Apple M1 (Virtual)",
        )
        self.assertEqual(
            report.cpu_name(
                dict(
                    cpu_model="aarch64",
                    hardware_details='{"lscpu":[{"field":"Model name:","data":"Neoverse-N2"}]}',
                )
            ),
            "Neoverse-N2",
        )
        self.assertEqual(report.cpu_name({}), "CPU unspecified")

    def test_nominal_throughput_uses_call_latency_and_only_dense_gemm(self):
        self.assertEqual(
            report.throughput("matmul_128_f64", result(1000)), 2 * 128**3 / 1000
        )
        self.assertIsNone(report.throughput("qr_96x32_f64", result(10)))
        self.assertIsNone(report.throughput("matmul_128_f64", result(0)))

    def test_latency_change_keeps_sign_and_missing_is_not_zero(self):
        self.assertAlmostEqual(report.change(result(80), result(100)), -20)
        self.assertAlmostEqual(report.change(result(120), result(100)), 20)
        self.assertIsNone(report.change(result(120), None))
        self.assertIsNone(report.change(None, result(100)))
        self.assertIsNone(report.change(result(10), result(0)))

    def test_render_retains_losses_unknown_cases_and_escapes_metadata(self):
        data = dict(
            metadata=dict(
                revision="abc<unsafe>",
                run_id="<script>ROWS</script>",
                library_packages=[dict(name="kernix", version="0.0.0")],
            ),
            results={
                "matmul_128_f64/kernix": result(120),
                "matmul_128_f64/numpy": result(80),
                "future_<case>/kernix": result(40),
            },
        )
        original = copy.deepcopy(data)
        baseline = copy.deepcopy(data)
        baseline["results"] = {"matmul_128_f64/kernix": result(100)}
        with (
            tempfile.TemporaryDirectory() as temporary,
            patch.object(report, "render_chart"),
        ):
            root = Path(temporary)
            report.render_report(root, data, baseline)
            md = (root / "README.md").read_text(encoding="utf-8")
            html = (root / "report.html").read_text(encoding="utf-8")
        self.assertIn("0.12 | **0.08** | +20.0%", md)
        self.assertIn("future <case>", md)
        self.assertIn("future &lt;case&gt;", html)
        self.assertIn("unavailable", html)
        self.assertIn("&lt;script&gt;ROWS&lt;/script&gt;", html)
        self.assertNotIn("<script>ROWS</script>", html)
        self.assertEqual(data, original)

    def test_baseline_environment_mismatch_is_visible(self):
        a = dict(
            metadata=dict(environment=dict(cpu_model="first", governor="powersave"))
        )
        b = dict(
            metadata=dict(environment=dict(cpu_model="second", governor="performance"))
        )
        warnings = report.baseline_warnings(a, b)
        self.assertIn("Different cpu model.", warnings)
        self.assertIn("Different governor.", warnings)
        self.assertEqual(report.baseline_warnings(a, None), [])
        a["metadata"]["explicit_execution"] = {
            "work": {"workers": 2, "affinity_cpus": [0, 2]}
        }
        self.assertIn(
            "Different explicit executor or affinity policies.",
            report.baseline_warnings(a, b),
        )

    def test_controller_identity_requires_matching_source_and_verified_policy(self):
        a = dict(
            metadata=dict(
                numpy_thread_policy=dict(
                    method="BLASSetThreading/BLASGetThreading",
                    verified_threads=1,
                    controller_source_sha256="same-source",
                    controller_sha256="binary-a",
                )
            )
        )
        b = copy.deepcopy(a)
        b["metadata"]["numpy_thread_policy"]["controller_sha256"] = "binary-b"
        self.assertEqual(report.baseline_warnings(a, b), [])
        for field, value in [
            ("verified_threads", 2),
            ("method", "environment"),
            ("controller_source_sha256", "different-source"),
        ]:
            changed = copy.deepcopy(b)
            changed["metadata"]["numpy_thread_policy"][field] = value
            self.assertIn(
                "Different NumPy thread-policy verification.",
                report.baseline_warnings(a, changed),
            )
        for metadata in (a["metadata"], b["metadata"]):
            del metadata["numpy_thread_policy"]["controller_source_sha256"]
        self.assertIn(
            "Different NumPy thread-policy verification.",
            report.baseline_warnings(a, b),
        )

    def test_html_links_are_valid_and_literal_template_text_is_preserved(self):
        class Links(HTMLParser):
            def __init__(self):
                super().__init__()
                self.hrefs = []
                self.details = 0
                self.workloads = []

            def handle_starttag(self, tag, attrs):
                if tag == "a":
                    self.hrefs.append(dict(attrs).get("href"))
                if tag == "details":
                    self.details += 1
                if tag == "tr" and "data-workload" in dict(attrs):
                    self.workloads.append(dict(attrs)["data-workload"])

        data = dict(
            metadata=dict(revision="abc", run_id="DETAILS.md {{VERSION}}"),
            results={"matmul_128_f64/kernix": result(100)},
        )
        with (
            tempfile.TemporaryDirectory() as temporary,
            patch.object(report, "render_chart"),
        ):
            root = Path(temporary)
            report.render_report(root, data)
            text = (root / "report.html").read_text(encoding="utf-8")
        links = Links()
        links.feed(text)
        self.assertIn("raw.json", links.hrefs)
        self.assertIn("latency.pdf", links.hrefs)
        self.assertIn("original-report.txt", links.hrefs)
        self.assertIn("latency.svg", links.hrefs)
        self.assertEqual(links.details, 3)
        self.assertEqual(links.workloads, ["matmul_128_f64", "matmul_128_f64"])
        self.assertIn("run DETAILS.md {{VERSION}}.", text)

    def test_affinity_and_solver_contract_follow_actual_observations(self):
        data = dict(
            metadata=dict(
                revision="abc",
                run_id="host",
                environment=dict(cpu_model="AMD", affinity_cpu=None),
            ),
            results={"min_norm_32x64_rank32_f64/kernix": result(100)},
        )
        original = copy.deepcopy(data)
        with (
            tempfile.TemporaryDirectory() as temporary,
            patch.object(report, "render_chart"),
        ):
            root = Path(temporary)
            report.render_report(root, data)
            for name in ["README.md", "report.html"]:
                text = (root / name).read_text(encoding="utf-8")
                self.assertIn("CPU affinity was not pinned.", text)
                self.assertNotIn("shared laptop", text)
                self.assertIn("native SVD/solve", text)
                self.assertIn("rank policies differ", text)
            data["metadata"]["environment"]["affinity_cpu"] = 0
            report.render_report(root, data)
            self.assertIn(
                "CPU affinity pinned to logical CPU 0.",
                (root / "README.md").read_text(encoding="utf-8"),
            )
            data["metadata"]["minimum_norm_algorithm"] = (
                "pivoted QR plus scaled implicit RZ"
            )
            report.render_report(root, data)
            for name in ["README.md", "report.html"]:
                self.assertIn(
                    "scaled implicit RZ", (root / name).read_text(encoding="utf-8")
                )
            del data["metadata"]["minimum_norm_algorithm"]
        data["metadata"]["environment"]["affinity_cpu"] = None
        self.assertEqual(data, original)


class ComparisonMeaningTests(unittest.TestCase):
    def data(self, run="new", cpu="same"):
        return dict(
            metadata=dict(
                revision=run,
                run_id=run,
                binary_sha256={"time": run},
                environment={"cpu_model": cpu},
            ),
            results={
                "matmul_128_f64/kernix": result(100),
                "matmul_128_f64/numpy": result(80),
            },
        )

    def render(self, data, baseline=None, paired=None):
        with tempfile.TemporaryDirectory() as d, patch.object(report, "render_chart"):
            p = Path(d)
            report.render_report(p, data, baseline, paired)
            return (p / "README.md").read_text(encoding="utf-8"), (
                p / "report.html"
            ).read_text(encoding="utf-8")

    def test_single_measurement_is_not_a_peer_winner(self):
        d = self.data()
        del d["results"]["matmul_128_f64/numpy"]
        md, html = self.render(d)
        self.assertNotIn('class="best"', html)
        self.assertIn("No peer comparison measured", html)
        self.assertNotIn("**0.100**", md)

    def test_incompatible_environments_disable_directional_delta(self):
        current = self.data()
        old = self.data("old", "other")
        old["results"]["matmul_128_f64/kernix"] = result(50)
        md, html = self.render(current, old)
        self.assertIn("deltas are disabled", html)
        self.assertNotIn("+100.0%", md)
        self.assertIn("0.1", md)

    def test_paired_uncertainty_is_visible_and_wrong_runs_are_rejected(self):
        current = self.data()
        old = self.data("old")
        p = dict(
            current=current["metadata"],
            baseline=old["metadata"],
            results={
                "matmul_128_f64": dict(median_ratio=0.99, ci95_low=0.96, ci95_high=1.03)
            },
        )
        original = copy.deepcopy(current)
        md, html = self.render(current, old, p)
        self.assertIn("Timing direction unclear", html)
        self.assertIn("0.99× [0.96, 1.03]", md)
        self.assertEqual(current, original)
        bad = copy.deepcopy(p)
        bad["current"]["run_id"] = "another"
        with self.assertRaisesRegex(ValueError, "current run"):
            self.render(current, old, bad)
        bad = copy.deepcopy(p)
        bad["results"]["matmul_128_f64"]["ci95_low"] = float("nan")
        with self.assertRaisesRegex(ValueError, "invalid paired"):
            self.render(current, old, bad)

    def test_all_views_keep_searchable_names_and_explicit_contracts(self):
        d = self.data()
        d["results"]["parallel_matmul_into_128_f64/kernix"] = result(40)
        md, html = self.render(d)
        self.assertIn("2 workers · reused output", html)
        self.assertIn('data-search="parallel_matmul_into_128_f64', html)
        self.assertIn('scope="col"', html)
        self.assertIn('scope="row"', html)
        self.assertIn("<caption>", html)
        self.assertIn("No paired comparison", md)


if __name__ == "__main__":
    unittest.main()
