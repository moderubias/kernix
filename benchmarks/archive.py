#!/usr/bin/env python3
"""Rebuild presentation for every retained run; refuse any evidence mutation."""

import argparse
import hashlib
import html
import json
from pathlib import Path
import re
import shutil

import presentation
import report

GENERATED = {
    "README.md",
    "report.html",
    "presentation.json",
    "latency.pdf",
    "latency.svg",
    "latency.png",
    "comparison.pdf",
    "comparison.png",
    "comparison.svg",
    "report.pdf",
}


def protected_files(root):
    return {
        p.relative_to(root).as_posix(): hashlib.sha256(p.read_bytes()).hexdigest()
        for p in root.rglob("*")
        if p.is_file()
        and p.parent != root
        and "charts" not in p.relative_to(root).parts
        and p.name not in GENERATED
        and p.name != "original-report.txt"
    }


def entries(root):
    records = []
    for directory in sorted(root.iterdir()):
        if not directory.is_dir():
            continue
        for name in ["raw.json", "paired.json"]:
            if (directory / name).exists():
                data = presentation.load(directory / name)
                meta = data["metadata"] if name == "raw.json" else data["current"]
                env = meta.get("environment", {})
                records.append(
                    dict(
                        directory=directory,
                        data=data,
                        paired=name == "paired.json",
                        meta=meta,
                        run=meta.get("run_id", meta.get("version", directory.name)),
                        cpu=report.cpu_name(env)
                        if env
                        else meta.get("cpu", "CPU unrecorded"),
                        version=next(
                            (
                                p["version"]
                                for p in meta.get("library_packages", [])
                                if p["name"] == "kernix"
                            ),
                            meta.get("version", "historical"),
                        ),
                    )
                )
                break
    return records


def make_index(root, records):
    esc = html.escape
    ordered = sorted(
        records,
        key=lambda x: (
            tuple(
                int(n) for n in re.findall(r"\d+", str(x["version"]).split("-")[0])[:3]
            ),
            x["directory"].name,
        ),
        reverse=True,
    )
    rows = []
    md = [
        "# Kernix performance archive",
        "",
        "Read the reports below directly on GitHub: every workload has a chart and a plain-language caption. Download the [complete offline archive](https://github.com/moderubias/kernix/releases/latest/download/kernix-benchmark-archive.zip) for the interactive edition, or open [index.html](index.html) locally.",
        "",
        "All raw measurements, process pairs, source archives, locks and original methods are preserved. Chart labels use six significant digits with ≈ for rounding. Exact tables preserve all source digits through Decimal unit conversion.",
        "",
        "| Report | Hardware | Measure | Workloads |",
        "|---|---|---|---:|",
    ]
    for r in ordered:
        data = r["data"]
        name = r["directory"].name
        meta = r["meta"]
        env = meta.get("environment", {})
        platform = env.get("platform", meta.get("platform", ""))
        os = (
            "Windows"
            if "Windows" in platform
            else "macOS"
            if "macOS" in platform
            else "Linux"
            if "Linux" in platform
            else "OS unrecorded"
        )
        legacy = not r["paired"] and any(
            "timing" not in v for v in data["results"].values()
        )
        measure = (
            "Paired process ratios"
            if r["paired"]
            else "Batch-mean medians"
            if legacy
            else "Individual-call medians"
        )
        count = (
            len(data["results"])
            if r["paired"]
            else len({k.rsplit("/", 1)[0] for k in data["results"]})
        )
        search = esc(
            (
                str(r["version"])
                + " "
                + r["cpu"]
                + " "
                + os
                + " "
                + name
                + " "
                + measure
            ).lower(),
            quote=True,
        )
        rows.append(
            f'<tr data-search="{search}" data-kind="{"paired" if r["paired"] else "batch" if legacy else "calls"}"><th scope="row"><a href="{esc(name)}/report.html">Kernix {esc(str(r["version"]))}<span aria-hidden="true"> ↗</span></a><small>{esc(name)}</small></th><td>{esc(r["cpu"])}<small>{os}</small></td><td><span class="type-tag">{measure}</span></td><td>{count}</td><td class="archive-actions"><a href="{esc(name)}/latency.pdf">PDF</a><a href="{esc(name)}/README.md">Exact data</a></td></tr>'
        )
        md.append(
            f"| [{name}]({name}/README.md) | {r['cpu']} / {os} | {measure} | {count} |"
        )
    base = Path(__file__).parent
    css = (
        presentation.font_css()
        + (base / "report_style.css").read_text(encoding="utf-8")
        + """
    .archive-hero{padding:65px 0 35px;max-width:850px}.archive-hero h1{font-size:64px}.archive-hero>p:last-child{max-width:650px;color:var(--muted)}.archive-controls{display:flex;justify-content:space-between;gap:15px;flex-wrap:wrap;margin:35px 0 18px}.archive-controls input{width:380px;max-width:100%}.archive-table{margin-bottom:60px}.archive-table th:first-child{min-width:280px}.archive-table td{text-align:left;white-space:normal}.archive-table th a{font-size:15px;text-decoration:none;font-weight:600}.archive-table th small{font-size:10px;color:var(--muted);font-weight:400;overflow-wrap:anywhere}.archive-table td small{display:block;font-size:11px;color:var(--muted);margin-top:5px}.archive-table th:last-child,.archive-table td:last-child{white-space:nowrap}.archive-actions a{margin-right:18px;font-size:12px}.type-tag{display:inline-block;font-size:10px;padding:4px 8px;border:1px solid var(--line);border-radius:4px;color:var(--muted)}.archive-count{font-size:12px;color:var(--muted);margin:16px 0}.archive-note{font-size:13px;color:var(--muted);max-width:760px;margin:30px 0}@media(max-width:700px){.archive-hero{padding:35px 0 20px}.archive-hero h1{font-size:43px}.archive-controls input{width:100%}.archive-controls{margin-top:25px}.archive-controls select{width:100%}}"""
    )
    raw_count = sum(not r["paired"] for r in records)
    pairs = len(records) - raw_count
    doc = (
        """<!doctype html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Kernix / Performance archive</title><style>"""
        + css
        + """</style></head><body><nav class="masthead"><span class="wordmark"><b class="monogram">K</b>KERNIX<span>PERFORMANCE ARCHIVE</span></span><a href="https://github.com/moderubias/kernix">Project overview ↗</a></nav><main><header class="archive-hero"><p class="eyebrow">RESEARCH & ENGINEERING / ALL RETAINED EVIDENCE</p><h1>Every run.<br><span>One clear view.</span></h1><p>Explore the complete CPU benchmark record. Choose a release, inspect a machine, understand the measurement. Original numbers, rebuilt presentation.</p></header><section class="score-strip"><div><span class="eyebrow">REPORTS</span><strong>"""
        + str(len(records))
        + """</strong><small>Complete historical archive</small></div><div><span class="eyebrow">FULL RUNS</span><strong>"""
        + str(raw_count)
        + """</strong><small>Raw measurements retained</small></div><div><span class="eyebrow">PAIRED STUDIES</span><strong>"""
        + str(pairs)
        + """</strong><small>Fresh-process uncertainty</small></div></section><div class="archive-controls"><input id="archive-search" type="search" aria-label="Search archive" placeholder="Search a version, CPU, operating system…"><select id="archive-kind" aria-label="Measurement type"><option value="">Every measurement type</option><option value="calls">Individual-call medians</option><option value="batch">Historical batch means</option><option value="paired">Paired process ratios</option></select></div><p id="archive-count" class="archive-count" role="status" aria-live="polite"></p><div class="scroll archive-table"><table><caption>All reports / select a run to explore its full measurements</caption><thead><tr><th scope="col">Report & source run</th><th scope="col">Hardware</th><th scope="col">Measurement</th><th scope="col">Workloads</th><th scope="col">Downloads</th></tr></thead><tbody>"""
        + "".join(rows)
        + """</tbody></table></div><p class="archive-note">Different hosts, backends and statistics are kept separate. This archive does not combine them into a historical speedup curve or an aggregate winner. Historical batch-mean runs are labeled separately from individual-call tail-latency measurements.</p><footer>Evidence integrity: <a href="presentation-integrity.json">SHA-256 manifest</a>. Original samples, process order, build snapshots and commentary remain adjacent to each report. <a href="original-index.txt">Original archive commentary</a>.</footer></main><script>(()=>{const search=document.getElementById('archive-search'),kind=document.getElementById('archive-kind');function apply(){const q=search.value.toLowerCase().trim();const rows=[...document.querySelectorAll('tbody tr')];rows.forEach(r=>r.hidden=!(r.dataset.search.includes(q)&&(!kind.value||r.dataset.kind===kind.value)));document.getElementById('archive-count').textContent=rows.filter(r=>!r.hidden).length+' of '+rows.length+' reports';}search.addEventListener('input',apply);kind.addEventListener('change',apply);apply()})();</script></body></html>"""
    )
    license = presentation.font_license()
    (root / "index.html").write_text(
        doc.replace("</head>", "<!--" + license + "--></head>"), encoding="utf-8"
    )
    (root / "README.md").write_text("\n".join(md) + "\n", encoding="utf-8")


def rebuild(root):
    if not (root / "original-index.txt").exists():
        if (root / "README.md").exists():
            shutil.copyfile(root / "README.md", root / "original-index.txt")
        else:
            (root / "original-index.txt").write_text(
                "No archive commentary existed before the first render.\n",
                encoding="utf-8",
            )
    before = protected_files(root)
    records = entries(root)
    lookup = {r["run"]: r for r in records if not r["paired"]}
    pairs = {r["data"]["current"]["run_id"]: r["data"] for r in records if r["paired"]}
    # Capture original commentary once; subsequent renders are idempotent.
    originals = {}
    for r in records:
        d = r["directory"]
        original = d / "original-report.txt"
        if not original.exists():
            original.write_bytes((d / "README.md").read_bytes())
        originals[original.relative_to(root).as_posix()] = hashlib.sha256(
            original.read_bytes()
        ).hexdigest()
    for r in records:
        d = r["directory"]
        data = r["data"]
        if r["paired"]:
            h = dict(label=report.label, cpu=r["cpu"])
            presentation.render_pair(d, data, h)
            presentation.static_pair(d, data, h)
        else:
            paired = pairs.get(r["run"])
            old = lookup.get(paired["baseline"]["run_id"]) if paired else None
            report.render_report(d, data, old["data"] if old else None, paired)
        # Retain every old public URL while replacing presentation, never evidence.
        for name in ["comparison.pdf", "report.pdf"]:
            if (d / name).exists():
                shutil.copyfile(d / "latency.pdf", d / name)
        if (d / "comparison.png").exists():
            shutil.copyfile(d / "latency.png", d / "comparison.png")
        if (d / "comparison.svg").exists():
            shutil.copyfile(d / "latency.svg", d / "comparison.svg")
        print(d.name, flush=True)
    if any(
        hashlib.sha256((root / name).read_bytes()).hexdigest() != digest
        for name, digest in originals.items()
    ):
        raise RuntimeError("original commentary mutation refused")
    after = protected_files(root)
    if any(after.get(name) != digest for name, digest in before.items()):
        raise RuntimeError("evidence mutation refused")
    record = dict(
        design_version=3,
        reports=len(records),
        raw_runs=sum(not x["paired"] for x in records),
        paired_runs=sum(x["paired"] for x in records),
        protected_original_files=before,
        original_commentary_sha256=originals,
        original_archive_commentary_sha256=hashlib.sha256(
            (root / "original-index.txt").read_bytes()
        ).hexdigest()
        if (root / "original-index.txt").exists()
        else None,
        measurement_bytes_unchanged=True,
        chart_precision="6 significant digits; ≈ marks rounding",
        exact_table_precision="all source digits, Decimal ns/1000",
        new_measurements=False,
    )
    (root / "presentation-integrity.json").write_text(
        json.dumps(record, indent=2) + "\n", encoding="utf-8"
    )
    make_index(root, records)
    return record


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--root", type=Path, default=Path(__file__).with_name("results")
    )
    args = parser.parse_args()
    result = rebuild(args.root)
    print(
        "Verified:",
        result["reports"],
        "reports;",
        len(result["protected_original_files"]),
        "original evidence files unchanged",
    )


if __name__ == "__main__":
    main()
