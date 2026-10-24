"""Precise, accessible views of immutable CPU benchmark evidence."""

import base64
from decimal import Decimal, localcontext
import hashlib
import html
import json
import math
from pathlib import Path
import re
import subprocess

FAMILIES = {
    "matmul": "Matrix multiplication",
    "parallel": "Explicit parallel execution",
    "algebra": "Decompositions & solvers",
    "training": "Training & autodiff",
    "contraction": "Tensor contractions",
    "statistics": "Statistics & activations",
    "construction": "Tensor construction",
    "vision": "Vision workloads",
    "other": "Other workloads",
}


def family(w):
    if w.startswith(("parallel_", "matmul_reuse_")):
        return "parallel"
    if w.startswith(("matmul_sum_backward_", "training_", "dense_sgd_")):
        return "training"
    if w.startswith("matmul_"):
        return "matmul"
    if w.startswith(("cholesky_", "qr_", "pivoted_qr_", "min_norm_", "solve_")):
        return "algebra"
    if w.startswith("tensordot_"):
        return "contraction"
    if w.startswith("create_"):
        return "construction"
    if w.startswith(("softmax_", "add_", "sum_")):
        return "statistics"
    if w.startswith(("rgb", "conv_")):
        return "vision"
    return "other"


def contract(w):
    if w.startswith("parallel_"):
        return "2 workers · reused output · pool preparation excluded"
    if w.startswith("matmul_reuse_"):
        return "1 worker · reused output"
    if w.startswith(("training_shared_backward_", "training_backward_")):
        return "1 worker · prepared graph · fresh gradients"
    if w.startswith("dense_sgd_"):
        return "Graph-based Kernix vs hand-derived comparator updates; API costs differ"
    if w.startswith("min_norm_"):
        return "1 worker · 4 RHS · factorization included · algorithms differ"
    if w.startswith("matmul_batched_"):
        return "1 worker · allocated output · broadcast batches"
    if w.startswith("matmul_ab_"):
        return "1 worker · allocated output · independent operands"
    if w.startswith("matmul_"):
        return "1 worker · allocated output · self product"
    if w.startswith("sum_"):
        return "1 worker · scalar output"
    if w.startswith(("rgb", "conv_")):
        return "1 worker · allocated output · comparator implementations differ"
    return (
        "1 worker · allocated output"
        if not w.startswith("training_")
        else "1 worker · preparation as documented"
    )


def number(value):
    """Decimal display preserves every source digit; never use binary /1000 first."""
    if value is None:
        return "unavailable"
    d = value if isinstance(value, Decimal) else Decimal(str(value))
    if not d.is_finite():
        raise ValueError("non-finite measured value")
    text = format(d, "f")
    return text.rstrip("0").rstrip(".") if "." in text else text


def us(value):
    if value is None:
        return "unavailable"
    with localcontext() as ctx:
        ctx.prec = max(80, len(str(value)) + 10)
        return number(Decimal(str(value)) / Decimal(1000))


def compact(value):
    """Readable chart label; approximation is explicit, exact tables remain lossless."""
    d = Decimal(str(value))
    rounded = Decimal(format(d, ".6g"))
    return ("≈ " if rounded != d else "") + number(rounded)


def chart_us(value):
    with localcontext() as ctx:
        ctx.prec = max(80, len(str(value)) + 10)
        return compact(Decimal(str(value)) / 1000)


def load(path):
    return json.loads(Path(path).read_text(encoding="utf-8"), parse_float=Decimal)


def normalize(data):
    """Adapt the seven historical batch schemas without modifying input objects."""
    meta = dict(data["metadata"])
    legacy = meta.get(
        "_batch_only", any("timing" not in v for v in data["results"].values())
    )
    meta["_batch_only"] = legacy
    meta.setdefault("run_id", meta.get("version", "unrecorded"))
    meta.setdefault("revision", "unrecorded")
    if "environment" not in meta:
        meta["environment"] = {
            k: meta[k] for k in ["affinity_cpu", "rustc"] if k in meta
        }
        meta["environment"].update(
            cpu_model=meta.get("cpu", "CPU unrecorded"),
            platform=meta.get("platform", ""),
        )
    if not meta.get("library_packages"):
        meta["library_packages"] = [
            dict(name="kernix", version=meta.get("version", "historical"))
        ]
    results = {}
    for key, v in data["results"].items():
        if "timing" in v:
            results[key] = v
        else:
            results[key] = dict(timing=dict(batch_means=v), resources={})
    return dict(metadata=meta, results=results)


def median(v):
    t = v["timing"]
    return t.get("per_call", t.get("batch_means", {}))["p50_ns"]


def metric(meta):
    return "Batch-mean median" if meta.get("_batch_only") else "Per-call median"


def font_css():
    base = Path(__file__).with_name("assets")
    return "".join(
        "@font-face{font-family:Plex;src:url(data:font/woff2;base64,"
        + base64.b64encode((base / f"IBMPlexSans-{style}.woff2").read_bytes()).decode()
        + ') format("woff2");font-weight:'
        + weight
        + ";font-style:normal;font-display:swap}"
        for style, weight in [("Regular", "400"), ("SemiBold", "600")]
    )


def font_license():
    return "\n".join(
        line.rstrip()
        for line in (Path(__file__).with_name("assets") / "OFL.txt")
        .read_text(encoding="utf-8")
        .splitlines()
    ).replace("--", "—")


def paired_results(data, baseline, paired, warnings):
    if paired is None:
        return {}
    for key in ["run_id", "revision", "binary_sha256"]:
        if paired["current"].get(key) != data["metadata"].get(key):
            raise ValueError(
                "paired evidence does not belong to the selected current run"
            )
        if baseline and paired["baseline"].get(key) != baseline["metadata"].get(key):
            raise ValueError(
                "paired evidence does not belong to the selected baseline run"
            )
    if warnings:
        raise ValueError("paired evidence has incompatible environments")
    for key, p in paired["results"].items():
        r, lo, hi = (p[k] for k in ["median_ratio", "ci95_low", "ci95_high"])
        if (
            not all(math.isfinite(x) and x > 0 for x in [r, lo, hi])
            or not lo <= r <= hi
        ):
            raise ValueError("invalid paired ratio or interval")
        if key + "/kernix" not in data["results"]:
            raise ValueError("paired workload absent from current run")
    return paired["results"]


def verdict(p):
    if p is None:
        return "No paired comparison", "neutral"
    if p["ci95_high"] < 1:
        return "Observed time reduction", "improved"
    if p["ci95_low"] > 1:
        return "Observed time increase", "regressed"
    return "Timing direction unclear", "neutral"


def ratio_text(p):
    return (
        "—"
        if p is None
        else f"{number(p['median_ratio'])}× [{number(p['ci95_low'])}, {number(p['ci95_high'])}]"
    )


def _attrs(w, caption):
    return f'data-workload="{html.escape(w, quote=True)}" data-family="{family(w)}" data-search="{html.escape((w + " " + caption).lower(), quote=True)}"'


def _chart(w, title, shape, dtype, values, names, legacy=False):
    esc = html.escape
    maximum = max((median(v) for v in values.values()), default=1) or 1
    rows = []
    ordered = [
        x
        for x in [
            "kernix",
            "kernix-accelerate",
            "numpy",
            "candle",
            "faer",
            "ndarray",
            "nalgebra",
        ]
        if x in values
    ] + sorted(
        set(values)
        - {
            "kernix",
            "kernix-accelerate",
            "numpy",
            "candle",
            "faer",
            "ndarray",
            "nalgebra",
        }
    )
    for lib in ordered:
        ns = median(values[lib])
        kind = (
            "native"
            if lib == "kernix-accelerate"
            else "kernix"
            if lib == "kernix"
            else "peer"
        )
        width = float(Decimal(str(ns)) / Decimal(str(maximum)) * 100)
        rows.append(
            f'<div class="measure-row {kind}"><span class="measure-label">{esc(names(lib))}</span><span class="measure-value" data-ns="{esc(number(ns))}" title="Exact: {us(ns)} µs">{chart_us(ns)} <small>µs</small></span><div class="track" aria-hidden="true"><i style="width:{width:.8f}%"></i></div></div>'
        )
    caption = " · ".join(filter(None, [title, shape, dtype]))
    badge = (
        "No peer comparison measured"
        if len(values) == 1
        else "Absolute latency · lower is faster"
    )
    return (
        f'<article class="plot" {_attrs(w, caption)}><div class="chart-top"><span class="eyebrow">{esc(FAMILIES[family(w)])}</span><span class="chart-unit">{metric({"_batch_only": legacy})} / µs</span></div><h3>{esc(title)}</h3><p class="shape">{esc(shape)} <span>{esc(dtype)}</span></p><p class="contract">{esc(contract(w))}</p><div class="bar-chart">'
        + "".join(rows)
        + f'</div><div class="chart-foot"><span>{badge}</span><span>0 → {us(maximum)} µs</span></div></article>'
    )


def _interval(w, p, label):
    title, shape, dtype = label(w)
    caption = " · ".join(filter(None, [title, shape, dtype]))
    status, kind = verdict(p)
    if p is None:
        return ""
    r, lo, hi = (Decimal(str(p[k])) for k in ["median_ratio", "ci95_low", "ci95_high"])
    maximum = max(Decimal("1.25"), hi * Decimal("1.12"))
    position = lambda v: number(v / maximum * 100)
    return f'<article class="interval-card {kind}" {_attrs(w, caption)}><div><span class="eyebrow">PAIRED PROCESS COMPARISON</span><h3>{html.escape(title)}</h3><p class="shape">{html.escape(shape)} <span>{html.escape(dtype)}</span></p><p class="contract">{html.escape(contract(w))}</p></div><p class="interpretation">{status}</p><div class="interval-plot" role="img" aria-label="{html.escape(status + "; ratio " + number(r) + "; 95% interval " + number(lo) + " to " + number(hi))}"><i class="same-line" style="left:{position(Decimal(1))}%"></i><i class="range-line" style="left:{position(lo)}%;width:{position(hi - lo)}%"></i><b style="left:{position(r)}%"></b><span class="axis-zero">0</span><span class="axis-same" style="left:{position(Decimal(1))}%">1 = same time</span></div><dl class="interval-numbers"><div><dt>Current / baseline</dt><dd>{compact(r)}×</dd></div><div><dt>95% process interval</dt><dd>{compact(lo)} – {compact(hi)}</dd></div></dl></article>'


def _page(dest, tokens):
    base = Path(__file__).parent
    tokens["CSS"] = font_css() + (base / "report_style.css").read_text(encoding="utf-8")
    tokens["ARCHIVE_HREF"] = (
        "../index.html"
        if (dest.parent / "index.html").exists()
        or dest.parent.resolve() == (base / "results").resolve()
        else "#explore"
    )
    tokens["ARCHIVE_LABEL"] = (
        "All reports" if tokens["ARCHIVE_HREF"].startswith("..") else "Workloads"
    )
    tokens.setdefault(
        "SOURCE_STATUS", "Source cleanliness is recorded in the original metadata."
    )
    tokens["FONT_LICENSE"] = "<!-- Typeface: " + font_license() + " -->"
    template = (base / "report_frontend.html").read_text(encoding="utf-8")
    doc = re.sub(r"\{\{([A-Z_]+)\}\}", lambda m: tokens[m[1]], template)
    (dest / "report.html").write_text(doc, encoding="utf-8")


def receipt(dest, data, baseline=None, paired=None):
    base = Path(__file__).parent
    try:
        revision = subprocess.check_output(
            ["git", "rev-parse", "HEAD"], cwd=base, text=True, stderr=subprocess.DEVNULL
        ).strip()
    except (OSError, subprocess.CalledProcessError):
        revision = None
    files = [
        "report.py",
        "presentation.py",
        "visuals.py",
        "report_frontend.html",
        "report_style.css",
        "archive.py",
        "assets/fonts.json",
        "assets/OFL.txt",
        "assets/IBMPlexSans-Regular.ttf",
        "assets/IBMPlexSans-SemiBold.ttf",
        "assets/IBMPlexSans-Regular.woff2",
        "assets/IBMPlexSans-SemiBold.woff2",
    ]
    sources = {
        name: hashlib.sha256((dest / name).read_bytes()).hexdigest()
        for name in ["raw.json", "paired.json", "original-report.txt", "DETAILS.md"]
        if (dest / name).exists()
    }
    record = dict(
        design_version=2,
        measured_revision=data.get("metadata", data.get("current", {})).get("revision"),
        renderer_revision=revision,
        renderer_sha256={
            f: hashlib.sha256((base / f).read_bytes()).hexdigest() for f in files
        },
        source_files_sha256=sources,
        paired_evidence_attached=paired is not None or "current" in data,
        raw_modified=False,
        number_policy="Decimal source digits; exact decimal nanoseconds to microseconds; chart geometry only uses floats",
    )
    (dest / "presentation.json").write_text(
        json.dumps(record, indent=2) + "\n", encoding="utf-8"
    )


def render(dest, data, baseline, paired, h):
    esc = html.escape
    dest = Path(dest)
    data = normalize(data)
    baseline = normalize(baseline) if baseline else None
    meta = data["metadata"]
    groups = h["rows"](data)
    legacy = meta["_batch_only"]
    label = h["label"]
    names = h["names"]
    warnings = h["baseline_warnings"](
        data, baseline or ({"metadata": paired["baseline"]} if paired else None)
    )
    if baseline and legacy != baseline["metadata"]["_batch_only"]:
        warnings.append(
            "Different measurement statistics: batch means versus individual calls."
        )
    ratios = paired_results(data, baseline, paired, warnings)
    version = next(
        (
            p["version"]
            for p in meta.get("library_packages", [])
            if p["name"] == "kernix"
        ),
        "historical",
    )
    env = meta.get("environment", {})
    cpu = h["cpu"]
    aff = env.get("affinity_cpu")
    host = (
        f"CPU affinity pinned to logical CPU {aff}. "
        if aff is not None
        else "CPU affinity was not pinned. "
    )
    host += "Frequency and background load are uncontrolled."
    platform = env.get("platform", "")
    os_name = (
        "Windows"
        if "Windows" in platform
        else "macOS"
        if "macOS" in platform
        else "Linux"
        if "Linux" in platform
        else "OS unrecorded"
    )
    base_meta = (baseline or {}).get("metadata", {}) or (paired or {}).get(
        "baseline", {}
    )
    baseline_id = base_meta.get("run_id")
    note = (
        f"Baseline: {baseline_id}. "
        if baseline_id
        else "No previous-version baseline attached. "
    )
    note += "Sequential run changes are descriptive; paired process evidence is shown separately."
    if warnings:
        note += " Version deltas are disabled because recorded conditions differ."
    solver = meta.get("minimum_norm_algorithm", "pivoted QR plus scaled right QR")
    numerical = (
        f"Minimum-norm rows use {solver} in Kernix; NumPy native lstsq; nalgebra native SVD/solve. Algorithm costs and rank policies differ. See original method for thresholds."
        if any(w.startswith("min_norm_") for w, _ in groups)
        else ""
    )
    libs = [lib for lib in h["libraries"] if any(lib in values for _, values in groups)]
    libs += sorted({lib for _, values in groups for lib in values} - set(libs))
    columns = "".join(f'<th scope="col">{esc(names(lib))}</th>' for lib in libs)
    peer_rows = []
    version_rows = []
    details = []
    plots = []
    losses = []
    md = [
        f"# Kernix {version} · performance brief",
        "",
        f"{cpu} / {os_name}",
        "",
        f"**{metric(meta)}, microseconds per operation. Lower is faster. All recorded digits retained.**",
        "",
        f"{len(groups)} workloads · {len(data['results'])} observations",
        "",
        note,
        "",
        "[Open the visual report](report.html) · [Vector PDF](latency.pdf) · [Raw measurements](raw.json)",
        "",
        "| Workload | "
        + " | ".join(names(x) for x in libs)
        + (" | Sequential run change" if baseline else "")
        + " |",
        "|---|" + "---:|" * (len(libs) + (1 if baseline else 0)),
    ]
    for w, values in groups:
        title, shape, dtype = label(w)
        caption = " · ".join(filter(None, [title, shape, dtype]))
        attrs = _attrs(w, caption)
        latencies = [Decimal(str(median(v))) for v in values.values()]
        if not all(x.is_finite() and x >= 0 for x in latencies):
            raise ValueError("invalid measured latency")
        best = min(latencies)
        cells = []
        md_cells = []
        for lib in libs:
            if lib not in values:
                cells.append('<td class="missing">—</td>')
                md_cells.append("—")
                continue
            ns = median(values[lib])
            winner = len(values) > 1 and Decimal(str(ns)) == best
            style = "best" if winner else ""
            cells.append(f'<td class="{style}" data-ns="{number(ns)}">{us(ns)}</td>')
            md_cells.append(f"**{us(ns)}**" if winner else us(ns))
        if baseline:
            previous = baseline["results"].get(w + "/kernix")
            current = values.get("kernix")
            delta = (
                (Decimal(str(median(current))) / Decimal(str(median(previous))) - 1)
                * 100
                if current and previous and median(previous) > 0 and not warnings
                else None
            )
            md_cells.append(f"{delta:+.1f}%" if delta is not None else "—")
        md.append("| " + caption + " | " + " | ".join(md_cells) + " |")
        heading = f'<th scope="row">{esc(title)}<small>{esc(shape)} · {esc(dtype)}</small><small>{esc(contract(w))}</small></th>'
        badge = (
            "No peer comparison measured"
            if len(values) == 1
            else "Observed median · not a statistical peer ranking"
        )
        peer_rows.append(
            f"<tr {attrs}>{heading}"
            + "".join(cells)
            + f'<td class="status">{badge}</td></tr>'
        )
        p = ratios.get(w)
        status, kind = verdict(p)
        version_rows.append(
            f'<tr {attrs}>{heading}<td>{ratio_text(p)}</td><td class="status {kind}">{status}</td></tr>'
        )
        plots.append(_chart(w, title, shape, dtype, values, names, legacy))
        k = values.get("kernix")
        others = {lib: v for lib, v in values.items() if not lib.startswith("kernix")}
        if k and others:
            lib, v = min(others.items(), key=lambda x: median(x[1]))
            if median(v) < median(k):
                losses.append((caption, names(lib), us(median(k)), us(median(v))))
        resource = []
        for lib, v in values.items():
            t = v["timing"]
            q = t.get("per_call", {})
            b = t.get("batch_means", {})
            a = v.get("resources", {}).get("rust_allocator")
            vals = [
                us(q.get("p50_ns")),
                us(q.get("p95_ns")),
                us(q.get("p99_ns")),
                us(b.get("p50_ns")),
                us(b.get("p95_ns")),
                us(t.get("first_call_return_ns")),
                number(a.get("allocation_calls")) if a else "unavailable",
                number(a.get("peak_live_requested_bytes")) if a else "unavailable",
            ]
            resource.append(
                f'<tr><th scope="row">{esc(names(lib))}</th>'
                + "".join("<td>" + x + "</td>" for x in vals)
                + "</tr>"
            )
        headers = [
            "Library",
            "Call median / µs",
            "Call p95 / µs",
            "Call p99 / µs",
            "Batch median / µs",
            "Batch max (historical) / p95 (modern) / µs",
            "First API call / µs",
            "Allocations",
            "Peak requested bytes",
        ]
        details.append(
            f'<details {_attrs(w, caption)}><summary>{esc(caption)}</summary><p>{esc(contract(w))}. First API call is not a cold-cache measurement.</p><div class="scroll"><table><caption>Exact timing and warmed Rust allocation probes; unavailable stays unavailable</caption><thead><tr>'
            + "".join(f'<th scope="col">{x}</th>' for x in headers)
            + "</tr></thead><tbody>"
            + "".join(resource)
            + "</tbody></table></div></details>"
        )
    intervals = "".join(
        _interval(w, ratios[w], label) for w, _ in groups if w in ratios
    )
    if not intervals:
        intervals = '<div class="empty-state"><h3>No paired process evidence</h3><p>Absolute observations are available. Repeated calls within one process do not supply a process-level confidence interval.</p></div>'
    count = f"{len(ratios)} of {len(groups)} workloads have paired evidence."
    md += [
        "",
        "## Version comparison",
        "",
        count
        + " Exploratory 95% intervals resample process pairs, not calls. No multiple-comparison adjustment.",
        "",
        "| Workload | Current / baseline [95% interval] | Interpretation |",
        "|---|---:|---|",
    ]
    for w, _ in groups:
        md.append(
            "| "
            + " · ".join(filter(None, label(w)))
            + " | "
            + ratio_text(ratios.get(w))
            + " | "
            + verdict(ratios.get(w))[0]
            + " |"
        )
    if legacy:
        md += [
            "",
            "## Historical statistic",
            "",
            "These are medians of 15 warmed batch means, not per-call medians. The original p95 field is the maximum of those 15 batch means. No per-call p95/p99, accuracy or allocation probes are inferred.",
        ]
    md += [
        "",
        "## Method and provenance",
        "",
        host,
        "",
        numerical,
        "",
        "Numbers retain source precision; recorded digits do not imply equivalent physical measurement accuracy. Lowest observed medians are bold only when peers were measured. No aggregate winner or universal ranking is inferred.",
        "",
        "[Original report and commentary](original-report.txt) · [Raw measurements](raw.json)",
        "",
        f"Measured source `{meta['revision']}`. Run `{meta['run_id']}`. Rendering does not change measurements.",
        "",
    ]
    (dest / "README.md").write_text("\n".join(md), encoding="utf-8")
    facts = f'<div><span class="eyebrow">WORKLOADS</span><strong>{len(groups):02d}</strong><small>All measured operations</small></div><div><span class="eyebrow">OBSERVATIONS</span><strong>{len(data["results"]):,}</strong><small>Library × workload pairs</small></div><div><span class="eyebrow">PAIRED COVERAGE</span><strong>{len(ratios):02d}<i> / {len(groups)}</i></strong><small>Fresh-process comparisons</small></div>'
    tokens = dict(
        VERSION=esc(str(version)),
        RUN_ID=esc(meta["run_id"]),
        CPU=esc(cpu),
        PLATFORM=esc(os_name),
        DATE=esc(
            str(meta.get("timestamp_utc", meta.get("timestamp", "Date unrecorded")))[
                :10
            ]
        ),
        METRIC=metric(meta),
        HOST=esc(host),
        FACTS=facts,
        BASELINE=esc(note),
        PAIRED_COUNT=count,
        FAMILIES='<option value="">Every workload family</option>'
        + "".join(
            f'<option value="{k}">{v}</option>'
            for k, v in FAMILIES.items()
            if any(family(w) == k for w, _ in groups)
        ),
        COLUMNS=columns,
        ROWS="".join(peer_rows),
        VERSIONS="".join(version_rows),
        PLOTS="".join(plots),
        DETAILS="".join(details),
        INTERVALS=intervals,
        NUMERICAL=esc(numerical),
        WARNINGS=esc(" ".join(warnings)),
        HAS_PAIRED=str(bool(ratios)).lower(),
        LIMITS=(
            "Historical batch measurements. The recorded p95 is the maximum of 15 batch means, not call-level tail latency."
            if legacy
            else "Ordinary rows include output allocation. Explicit reused-output rows exclude pool/output setup. Peer medians are observations, not paired confidence intervals."
        ),
        LOSSES="".join(
            f"<li><strong>{esc(c)}</strong><span>Kernix {kt} µs · {esc(lib)} {pt} µs</span></li>"
            for c, lib, kt, pt in losses[:4]
        )
        or "<li>No lower external median among measured peers. Unmeasured libraries remain unmeasured.</li>",
        SOURCE_STATUS=(
            "The measurement records uncommitted source changes."
            if meta.get("dirty") or meta.get("dirty_before") or meta.get("dirty_after")
            else "The measurement records clean source."
            if "dirty" in meta or "dirty_before" in meta
            else "Source cleanliness was not recorded."
        ),
        PROVENANCE=esc(
            f"Measured source {meta['revision']} · run {meta['run_id']}. Original evidence and recorded precision are unchanged."
        ),
        RAW_LINK="raw.json",
        RAW_LABEL="Raw measurements",
    )
    _page(dest, tokens)
    receipt(dest, data, baseline, paired)


def render_pair(dest, paired, h):
    """Standalone paired reports retain ratios and all process blocks unchanged."""
    dest = Path(dest)
    meta = paired["current"]
    old = paired["baseline"]
    ratios = paired["results"]
    esc = html.escape
    for p in ratios.values():
        r, lo, hi = (p[k] for k in ["median_ratio", "ci95_low", "ci95_high"])
        if (
            not all(math.isfinite(x) and x > 0 for x in [r, lo, hi])
            or not lo <= r <= hi
        ):
            raise ValueError("invalid paired ratio or interval")
    version = next(
        (
            p["version"]
            for p in meta.get("library_packages", [])
            if p["name"] == "kernix"
        ),
        "historical",
    )
    label = h["label"]
    cpu = h["cpu"]
    rows = []
    md = [
        f"# Kernix {version} · paired process comparison",
        "",
        f"{paired['rounds']} fresh-process pairs per workload. Ratios below 1 mean less time.",
        "",
        "[Open visual comparison](report.html) · [Vector PDF](latency.pdf) · [Original process evidence](paired.json)",
        "",
        "| Workload | Current / baseline [95% interval] | Interpretation |",
        "|---|---:|---|",
    ]
    for w, p in ratios.items():
        caption = " · ".join(filter(None, label(w)))
        status, kind = verdict(p)
        rows.append(
            f'<tr {_attrs(w, caption)}><th scope="row">{esc(caption)}</th><td>{ratio_text(p)}</td><td class="{kind}">{status}</td></tr>'
        )
        md.append("| " + caption + " | " + ratio_text(p) + " | " + status + " |")
    md += [
        "",
        "## Method",
        "",
        paired.get("interval", "95% process-paired bootstrap intervals."),
        "",
        f"Current `{meta.get('run_id')}`; baseline `{old.get('run_id')}`. All source ratio digits, paired observations and process ordering are preserved. No absolute peer latencies are inferred.",
        "",
    ]
    (dest / "README.md").write_text("\n".join(md), encoding="utf-8")
    tokens = dict(
        VERSION=esc(str(version)),
        RUN_ID=esc(meta.get("run_id", "unrecorded")),
        CPU=esc(cpu),
        PLATFORM="Paired processes",
        DATE="",
        METRIC="Current / baseline latency",
        HOST=f"{paired['rounds']} fresh-process pairs per workload. Exploratory 95% bootstrap intervals; no multiple-comparison adjustment.",
        FACTS=f'<div><span class="eyebrow">WORKLOADS</span><strong>{len(ratios)}</strong><small>With paired evidence</small></div><div><span class="eyebrow">PROCESS PAIRS / WORKLOAD</span><strong>{paired["rounds"]}</strong><small>Not individual timed calls</small></div><div><span class="eyebrow">REFERENCE RATIO</span><strong>1.0</strong><small>Same latency as baseline</small></div>',
        BASELINE=esc("Baseline: " + old.get("run_id", "unrecorded")),
        PAIRED_COUNT=f"{len(ratios)} paired workloads.",
        FAMILIES='<option value="">Every workload family</option>'
        + "".join(
            f'<option value="{k}">{v}</option>'
            for k, v in FAMILIES.items()
            if any(family(w) == k for w in ratios)
        ),
        COLUMNS="",
        ROWS="",
        VERSIONS="".join(rows),
        PLOTS='<div class="empty-state">This report contains paired ratios. Open the matching absolute-latency report for peer observations.</div>',
        DETAILS="",
        INTERVALS="".join(_interval(w, p, label) for w, p in ratios.items()),
        NUMERICAL="",
        WARNINGS="",
        HAS_PAIRED="true",
        LIMITS=esc(paired.get("interval", "Exploratory process intervals.")),
        LOSSES="<li>Intervals crossing 1 leave timing direction unclear. No process-paired peer ranking is measured here.</li>",
        PROVENANCE=esc(
            f"Current source {meta.get('revision')} · baseline source {old.get('revision')}. Raw process blocks unchanged."
        ),
        RAW_LINK="paired.json",
        RAW_LABEL="Raw paired process evidence",
    )
    _page(dest, tokens)
    receipt(dest, paired)


def static_report(dest, data, h, paired=None):
    from visuals import Scene, heading, latency_card, export

    data = normalize(data)
    meta = data["metadata"]
    groups = h["rows"](data)
    label = h["label"]
    names = h["names"]
    cpu = h["cpu"]
    specs = []
    featured = next(
        (x for x in groups if x[0] == "matmul_128_f64"), groups[0] if groups else None
    )
    if featured:
        specs.append(("Performance brief", [featured]))
    for fam, title in FAMILIES.items():
        items = [x for x in groups if family(x[0]) == fam and x != featured]
        for offset in range(0, len(items), 2):
            specs.append((title, items[offset : offset + 2]))
    scenes = []
    pair_count = (len(paired["results"]) + 1) // 2 if paired else 0
    appendix = [(w, k, v) for w, values in groups for k, v in values.items()]
    appendix_count = (len(appendix) + 9) // 10
    for index, (family_title, items) in enumerate(specs):
        scene = Scene()
        version = next(
            (
                p["version"]
                for p in meta.get("library_packages", [])
                if p["name"] == "kernix"
            ),
            "historical",
        )
        heading(
            scene,
            f"Kernix {version} / {family_title}",
            cpu + " · " + metric(meta) + " · all values in µs · lower is faster",
            index + 1,
            len(specs) + pair_count + appendix_count,
            str(meta.get("run_id")) + " · source " + str(meta.get("revision"))[:12],
        )
        if index == 0:
            scene.text(64, 218, f"{len(groups):02d}", 44, bold=True)
            scene.text(160, 214, "workloads", 17)
            scene.text(420, 218, f"{len(data['results']):,}", 44, bold=True)
            scene.text(548, 214, "observations", 17)
            scene.text(800, 214, "Source precision retained", 17)
        for j, (w, values) in enumerate(items):
            title, shape, dtype = label(w)
            ordered = [
                k
                for k in [
                    "kernix",
                    "kernix-accelerate",
                    "numpy",
                    "candle",
                    "faer",
                    "ndarray",
                    "nalgebra",
                ]
                if k in values
            ] + sorted(
                set(values)
                - {
                    "kernix",
                    "kernix-accelerate",
                    "numpy",
                    "candle",
                    "faer",
                    "ndarray",
                    "nalgebra",
                }
            )
            v = [
                dict(
                    key=k,
                    name=names(k),
                    ns=median(values[k]),
                    us=chart_us(median(values[k])),
                )
                for k in ordered
            ]
            latency_card(
                scene,
                280 if index == 0 else 196 + j * 300,
                title,
                " · ".join(filter(None, [shape, dtype, contract(w)])),
                v,
                metric(meta),
                height=max(260, 110 + 27 * len(v)),
            )
        if index == 0:
            scene.text(64, 702, "How to read this report", 18, bold=True)
            note = (
                "Historical medians of 15 batch means. Recorded p95 is the batch maximum."
                if meta["_batch_only"]
                else "Allocation-inclusive API latency unless marked reused output. Peer medians are observations."
            )
            scene.text(64, 730, note, 14)
            scene.text(
                64,
                754,
                "Charts round to 6 significant digits (≈). The exact-value appendix preserves all recorded digits.",
                14,
            )
        scenes.append(scene)
    if paired:
        scenes.extend(
            static_pair(
                dest,
                paired,
                h,
                offset=len(specs),
                total=len(specs) + pair_count + appendix_count,
                return_scenes=True,
            )
        )
    for offset in range(0, len(appendix), 10):
        scene = Scene()
        heading(
            scene,
            "Recorded precision / exact latency table",
            cpu + " · " + metric(meta) + " · no rounding applied",
            len(scenes) + 1,
            len(specs) + pair_count + appendix_count,
            "Original samples remain in raw.json · source "
            + str(meta.get("revision"))[:12],
        )
        scene.text(64, 196, "WORKLOAD / LIBRARY", 11, bold=True)
        scene.text(820, 196, "NANOSECONDS", 11, bold=True, anchor="end")
        scene.text(1136, 196, "MICROSECONDS", 11, bold=True, anchor="end")
        for i, (w, lib, v) in enumerate(appendix[offset : offset + 10]):
            yy = 233 + i * 52
            title, shape, dtype = label(w)
            scene.text(64, yy, title + " / " + names(lib), 13, bold=True)
            scene.text(64, yy + 18, shape + " · " + dtype, 11, color="#566779")
            scene.text(820, yy + 5, number(median(v)), 12, anchor="end")
            scene.text(1136, yy + 5, us(median(v)), 12, anchor="end")
            scene.line(64, yy + 31, 1136, yy + 31)
        scenes.append(scene)
    export(scenes, Path(dest), "latency")


def static_pair(dest, paired, h, offset=0, total=None, return_scenes=False):
    from visuals import Scene, heading, interval_card, export

    entries = list(paired["results"].items())
    pages = [entries[i : i + 2] for i in range(0, len(entries), 2)]
    scenes = []
    for index, entries in enumerate(pages):
        scene = Scene()
        heading(
            scene,
            "Version comparison / paired processes",
            h["cpu"]
            + f" · {paired['rounds']} process pairs per workload · ratio below 1 means less time",
            index + 1 + offset,
            total or len(pages),
            "Current source "
            + str(paired["current"].get("revision"))[:12]
            + " · 95% exploratory process intervals",
        )
        for i, (w, p) in enumerate(entries):
            title, shape, dtype = h["label"](w)
            interval_card(
                scene,
                196 + i * 294,
                title,
                " · ".join(filter(None, [shape, dtype])),
                number(p["median_ratio"]),
                number(p["ci95_low"]),
                number(p["ci95_high"]),
                verdict(p)[0],
            )
        scenes.append(scene)
    if return_scenes:
        return scenes
    export(scenes, Path(dest), "latency")
