"""Readable presentation must remain honest, complete and evidence-preserving."""

import copy
from decimal import Decimal
from pathlib import Path
import tempfile
import unittest

import archive
import github_charts as g
import presentation as p
from test_presentation import sample


class GithubChartsTests(unittest.TestCase):
    def test_order_and_reading_show_losses_without_changing_values(self):
        values = {"kernix": sample(2000), "numpy": sample(1000), "faer": sample(1000)}
        original = copy.deepcopy(values)
        self.assertEqual(g.ranked(values), ["faer", "numpy", "kernix"])
        self.assertIn("100.0% more time than numpy", g.observation(values, str))
        self.assertEqual(values, original)
        self.assertIn(
            "50.0% less",
            g.observation({"kernix": sample(1000), "numpy": sample(2000)}, str),
        )

    def test_reading_never_invents_peers_ratios_or_precision(self):
        self.assertIn("No external", g.observation({"kernix": sample(1)}, str))
        self.assertIn(
            "No external",
            g.observation({"kernix": sample(1), "kernix-accelerate": sample(0)}, str),
        )
        self.assertIn(
            "undefined", g.observation({"kernix": sample(1), "numpy": sample(0)}, str)
        )
        self.assertIn(
            "same recorded",
            g.observation({"kernix": sample(0), "numpy": sample(0)}, str),
        )
        self.assertIn(
            "< 0.1% less",
            g.observation(
                {"kernix": sample(Decimal("999.999")), "numpy": sample(1000)}, str
            ),
        )

    def test_filenames_cannot_escape_the_chart_directory(self):
        self.assertEqual(g.filename("matmul_128_f64"), "charts/matmul_128_f64.png")
        for w in ["../../raw.json", "a/b", "<script>", "[label]"]:
            path = Path(g.filename(w))
            self.assertEqual(path.parent, Path("charts"))
            self.assertTrue(path.name.startswith("workload-"))
            self.assertNotEqual(g.filename(w), g.filename(w + "x"))

    def test_every_workload_gets_a_github_chart_link_and_group(self):
        records = archive.entries(Path(__file__).with_name("results"))
        for record in records:
            if record["paired"]:
                shown = [
                    w for _, _, items in g.pair_sheets(record["data"]) for w, _ in items
                ]
                self.assertCountEqual(shown, record["data"]["results"])
            else:
                data = p.normalize(record["data"])
                import report

                groups = report.rows(data)
                md = "\n".join(g.sections(groups, report.label))
                for w, _ in groups:
                    self.assertEqual(md.count("](" + g.filename(w) + ")"), 1)

    def test_only_generated_charts_are_excluded_from_evidence_hashes(self):
        with tempfile.TemporaryDirectory() as d:
            root = Path(d)
            run = root / "run"
            (run / "charts").mkdir(parents=True)
            (run / "charts" / "matmul.png").write_bytes(b"generated")
            (run / "raw.json").write_bytes(b"immutable")
            self.assertEqual(set(archive.protected_files(root)), {"run/raw.json"})

    @unittest.skipUnless(
        __import__("importlib").util.find_spec("reportlab"),
        "chart export dependencies not installed",
    )
    def test_all_retained_chart_labels_fit_the_image(self):
        import report
        from reportlab.pdfbase.pdfmetrics import stringWidth

        records = archive.entries(Path(__file__).with_name("results"))
        for record in records:
            h = dict(
                label=report.label,
                names=lambda k: report.NAMES.get(k, k),
                cpu=record["cpu"],
            )
            if record["paired"]:
                scenes = (
                    g.paired_scene(title, items, record["data"], h)
                    for _, title, items in g.pair_sheets(record["data"])
                )
            else:
                data = p.normalize(record["data"])
                scenes = (
                    g.latency_scene(w, values, data["metadata"], h)
                    for w, values in report.rows(data)
                )
            for scene in scenes:
                for kind, *args in scene.commands:
                    if kind != "text":
                        continue
                    x, y, value, size, _, bold, anchor = args
                    width = stringWidth(value, "PlexBold" if bold else "Plex", size)
                    left = (
                        x - width
                        if anchor == "end"
                        else x - width / 2
                        if anchor == "middle"
                        else x
                    )
                    self.assertGreaterEqual(left, -1, (record["directory"], value))
                    self.assertLessEqual(
                        left + width, scene.width + 1, (record["directory"], value)
                    )
                    self.assertGreaterEqual(y - size, 0)
                    self.assertLessEqual(y, scene.height)
