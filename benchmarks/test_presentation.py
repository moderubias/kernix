"""Source precision, historical statistic semantics and evidence preservation."""

import copy
from decimal import Decimal
import importlib.util
import json
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

import archive
import presentation as p
import report


def sample(ns):
    return dict(
        timing=dict(
            per_call=dict(p50_ns=ns, p95_ns=ns, p99_ns=ns),
            batch_means=dict(p50_ns=ns, p95_ns=ns),
        ),
        resources=dict(rust_allocator=None),
    )


class PrecisionTests(unittest.TestCase):
    def test_sparse_payloads_and_index_widths_are_readable_without_changing_data(self):
        a=sample(Decimal("12500.00000000001"))
        a["timing"]["storage"]=dict(left_nnz=2560,left_payload_bytes=45064,dense_left_bytes=2097152,index_bytes=8)
        b=sample(Decimal("15000.00000000001"))
        b["timing"]["storage"]=dict(left_nnz=2560,left_payload_bytes=32772,dense_left_bytes=2097152,index_bytes=4)
        data=dict(metadata=dict(revision="abc",run_id="sparse",sparse_protocol="Finite checks and prepared five-diagonal operands"),results={"sparse_matmul_512x32_f64/kernix":a,"sparse_matmul_512x32_f64/scipy":b})
        original=copy.deepcopy(data)
        with tempfile.TemporaryDirectory() as d,patch.object(report,"render_chart"):
            root=Path(d);report.render_report(root,data)
            for name in ["README.md","report.html"]:
                text=(root/name).read_text()
                for value in ["45064","32772","2097152","SciPy", "CSR"]:self.assertIn(value,text)
            self.assertIn("12.50000000000001",(root/"README.md").read_text())
        self.assertEqual(p.family("sparse_matmul_512x32_f64"),"sparse")
        self.assertEqual(data,original)

    def test_decimal_unit_conversion_keeps_original_digits_and_large_integers(self):
        for ns, expected in [
            ("126333.00000000001", "126.33300000000001"),
            ("9007199254740993", "9007199254740.993"),
            ("0.000000000001", "0.000000000000001"),
            ("1000", "1"),
            ("0", "0"),
        ]:
            self.assertEqual(p.us(Decimal(ns)), expected)
        with self.assertRaises(ValueError):
            p.us(Decimal("NaN"))

    def test_numpy_backend_uses_recorded_configuration_in_each_schema(self):
        configuration = {"Build Dependencies": {"blas": {"name": "scipy-openblas"}}}
        self.assertEqual(
            p.backend({"numpy_configuration": configuration}), "scipy-openblas"
        )
        self.assertEqual(
            p.backend({"numpy_configuration": repr(configuration)}), "scipy-openblas"
        )
        self.assertEqual(
            p.backend({"numpy_configuration": "not a configuration"}), "unrecorded"
        )

    def test_loading_never_rounds_json_numbers_through_a_binary_float(self):
        with tempfile.TemporaryDirectory() as d:
            path = Path(d) / "data.json"
            path.write_text(
                '{"time":126333.00000000001,"count":9007199254740993}', encoding="utf-8"
            )
            x = p.load(path)
            self.assertEqual(x["time"], Decimal("126333.00000000001"))
            self.assertEqual(x["count"], 9007199254740993)

    def test_rounded_chart_values_are_explicit_and_not_used_in_exact_tables(self):
        self.assertEqual(p.chart_us(Decimal("126333.00000000001")), "≈ 126.333")
        self.assertEqual(p.chart_us(Decimal("81483")), "81.483")
        data = dict(
            metadata=dict(revision="abc", run_id="new"),
            results={"matmul_128_f64/kernix": sample(Decimal("126333.00000000001"))},
        )
        original = copy.deepcopy(data)
        with tempfile.TemporaryDirectory() as d, patch.object(report, "render_chart"):
            path = Path(d)
            report.render_report(path, data)
            doc = (path / "report.html").read_text(encoding="utf-8")
            md = (path / "README.md").read_text(encoding="utf-8")
            self.assertIn('data-ns="126333.00000000001"', doc)
            self.assertIn(">126.33300000000001</td>", doc)
            self.assertIn("126.33300000000001", md)
            self.assertIn("≈ 126.333", doc)
        self.assertEqual(data, original)

    def test_refactor_reference_stays_in_exact_table_and_out_of_external_ranking(self):
        import github_charts

        values = {"kernix": sample(1000), "refactor": sample(100), "nalgebra": sample(2000)}
        data = dict(metadata=dict(run_id="rank", environment={}, library_source="workspace new"),
                    results={"cholesky_rank_update_256_f64/" + lib: row for lib, row in values.items()})
        with tempfile.TemporaryDirectory() as d, patch.object(report, "render_chart"):
            path = Path(d)
            report.render_report(path, data)
            doc = (path / "report.html").read_text(encoding="utf-8")
            md = (path / "README.md").read_text(encoding="utf-8")
            self.assertIn('data-ns="100"', doc)
            self.assertIn("Kernix refactor", md)
            self.assertIn("same-version refactor reference", doc)
        summary = github_charts.observation(values, lambda lib: report.NAMES[lib])
        self.assertIn("less time than nalgebra", summary)
        self.assertNotIn("refactor", summary)

    def test_historical_batch_means_do_not_become_call_tails_or_allocations(self):
        data = dict(
            metadata=dict(version="0.1.1", cpu="old CPU"),
            results={
                "sum_65536_f64/kernix": dict(
                    p50_ns=Decimal("123.456789"), p95_ns=200, samples_ns=[100, 200]
                )
            },
        )
        original = copy.deepcopy(data)
        adapted = p.normalize(data)
        self.assertTrue(adapted["metadata"]["_batch_only"])
        self.assertTrue(p.normalize(adapted)["metadata"]["_batch_only"])
        self.assertNotIn(
            "per_call", adapted["results"]["sum_65536_f64/kernix"]["timing"]
        )
        with tempfile.TemporaryDirectory() as d, patch.object(report, "render_chart"):
            path = Path(d)
            report.render_report(path, data)
            md = (path / "README.md").read_text(encoding="utf-8")
            doc = (path / "report.html").read_text(encoding="utf-8")
            self.assertIn("Batch-mean median", md)
            self.assertIn("0.123456789", md)
            self.assertIn("not per-call", md)
            self.assertIn("unavailable", doc)
        self.assertEqual(data, original)

    def test_unlike_measurement_statistics_cannot_supply_a_version_delta(self):
        current = dict(
            metadata=dict(revision="abc", run_id="a"),
            results={"matmul_128_f64/kernix": sample(100)},
        )
        old = dict(
            metadata=dict(revision="old", version="old"),
            results={
                "matmul_128_f64/kernix": dict(
                    p50_ns=50, p95_ns=100, samples_ns=[50, 100]
                )
            },
        )
        with tempfile.TemporaryDirectory() as d, patch.object(report, "render_chart"):
            path = Path(d)
            report.render_report(path, current, old)
            doc = (path / "report.html").read_text(encoding="utf-8")
            self.assertIn("Different measurement statistics", doc)
            self.assertIn("deltas are disabled", doc)


class ArchiveTests(unittest.TestCase):
    def test_rebuild_preserves_bytes_original_commentary_and_repeated_render(self):
        with tempfile.TemporaryDirectory() as d:
            root = Path(d)
            run = root / "original"
            run.mkdir()
            raw = b'{"metadata":{"version":"0.1.1","cpu":"CPU","revision":"abc"},"results":{"sum_65536_f64/kernix":{"p50_ns":1.123,"p95_ns":2,"samples_ns":[1,2]}}}\n'
            (run / "raw.json").write_bytes(raw)
            (run / "README.md").write_text(
                "Original method & 0.001123\n", encoding="utf-8"
            )
            (run / "source.tar.gz").write_bytes(b"opaque-source-snapshot")
            with patch.object(report, "render_chart"):
                a = archive.rebuild(root)
                first = (run / "README.md").read_bytes()
                b = archive.rebuild(root)
            self.assertEqual(a, b)
            self.assertEqual(first, (run / "README.md").read_bytes())
            self.assertEqual((run / "raw.json").read_bytes(), raw)
            self.assertEqual(
                (run / "source.tar.gz").read_bytes(), b"opaque-source-snapshot"
            )
            self.assertEqual(
                (run / "original-report.txt").read_text(encoding="utf-8"),
                "Original method & 0.001123\n",
            )
            self.assertTrue((root / "index.html").exists())

    def test_full_archive_exact_cells_match_every_original_median(self):
        # Uses the retained corpus only; tests real schema and dtype diversity.
        root = Path(__file__).with_name("results")
        from html.parser import HTMLParser

        class Cells(HTMLParser):
            def __init__(self):
                super().__init__()
                self.values = []

            def handle_starttag(self, tag, attrs):
                if tag == "td" and "data-ns" in dict(attrs):
                    self.values.append(Decimal(dict(attrs)["data-ns"]))

        for path in root.glob("*/raw.json"):
            if not (path.parent / "presentation.json").exists():
                continue
            receipt = json.loads(
                (path.parent / "presentation.json").read_text(encoding="utf-8")
            )
            if receipt.get("design_version") != 2:
                continue
            data = p.normalize(p.load(path))
            parser = Cells()
            parser.feed((path.parent / "report.html").read_text(encoding="utf-8"))
            expected = sorted(
                Decimal(str(p.median(v))) for v in data["results"].values()
            )
            self.assertEqual(sorted(parser.values), expected, str(path))

    @unittest.skipUnless(
        importlib.util.find_spec("reportlab") and importlib.util.find_spec("PIL"),
        "export dependencies not installed",
    )
    def test_export_is_vector_pdf_and_preview_svg_contains_font_license(self):
        from visuals import Scene, export

        with tempfile.TemporaryDirectory() as d:
            scene = Scene()
            scene.text(40, 50, "Recorded value: 126.33300000000001", 18)
            export([scene], d, "test")
            root = Path(d)
            self.assertTrue((root / "test.pdf").read_bytes().startswith(b"%PDF-"))
            svg = (root / "test.svg").read_text(encoding="utf-8")
            self.assertIn("126.33300000000001", svg)
            self.assertIn("OPEN FONT LICENSE", svg)
            self.assertNotIn("<image", svg)
            self.assertTrue((root / "test.png").exists())
