"""Directly readable GitHub charts and captions over immutable observations."""

from decimal import Decimal
from pathlib import Path
import re
import hashlib
import html

import presentation as p
from visuals import BLUE, GREY, INK, LINE, MUTED, PAPER, TEAL, Scene, raster, wrap


def ranked(values):
    """Stable point ordering; this is not a statistical ranking."""
    return sorted(values, key=lambda key: (p.median(values[key]), key))


def observation(values, names):
    """Explain the default Kernix median against the lowest external peer median."""
    peers = [key for key in values if not key.startswith("kernix")]
    if "kernix" not in values or not peers:
        return "No external comparison with Kernix was measured for this workload."
    peer = min(peers, key=lambda key: p.median(values[key]))
    current, other = (Decimal(str(p.median(values[key]))) for key in ["kernix", peer])
    if current == other:
        return f"Kernix and {names(peer)} have the same recorded median."
    if other == 0:
        return f"{names(peer)} has a zero recorded median; a relative difference is undefined."
    percent = abs(current / other - 1) * 100
    display = f"≈ {percent:.1f}%" if percent >= Decimal("0.1") else "< 0.1%"
    return f"Kernix took {display} {'less' if current < other else 'more'} time than {names(peer)} in this run."


def filename(workload):
    if not re.fullmatch(r"[A-Za-z0-9_.-]+", workload):
        workload = "workload-" + hashlib.sha256(workload.encode("utf-8")).hexdigest()
    return f"charts/{workload}.png"


def sections(groups, label):
    lines = [
        "## Charts by workload",
        "",
        "**Shorter bars mean less time.** Blue is Kernix; grey is another library; teal is an explicitly selected Kernix backend. Entries are sorted by the recorded median, with a separate zero-based scale per chart. These point observations do not establish a statistical ranking.",
        "",
        "Expand a group to see every chart here on GitHub. Click a chart to enlarge it. The exact table below keeps every source digit.",
        "",
    ]
    for fam, title in p.FAMILIES.items():
        items = [(w, values) for w, values in groups if p.family(w) == fam]
        if not items:
            continue
        lines += [
            "<details>",
            f"<summary>{title} · {len(items)} workloads</summary>",
            "",
        ]
        for w, _ in items:
            caption = (
                html.escape(" · ".join(filter(None, label(w))), quote=False)
                .replace("[", r"\[")
                .replace("]", r"\]")
            )
            lines += [
                f"**{caption}**",
                "",
                f"![{caption}. Shorter bars mean less time.]({filename(w)})",
                "",
            ]
        lines += ["</details>", ""]
    return lines


def pair_sheets(paired):
    for fam, title in p.FAMILIES.items():
        items = [(w, row) for w, row in paired["results"].items() if p.family(w) == fam]
        for offset in range(0, len(items), 6):
            yield (
                f"charts/paired-{fam}-{offset // 6 + 1}.png",
                title,
                items[offset : offset + 6],
            )


def pair_sections(paired):
    lines = [
        "",
        "### Version changes at a glance",
        "",
        "The vertical line marks unchanged time. Left is faster; right is slower. Ranges are exploratory 95% intervals over fresh-process pairs. An interval crossing the line leaves direction unclear.",
        "",
    ]
    for path, title, _ in pair_sheets(paired):
        lines += [
            f"![{title}. Current divided by baseline; below 1 is faster.]({path})",
            "",
        ]
    return lines


def version(meta):
    return next(
        (
            str(lib["version"])
            for lib in meta.get("library_packages", [])
            if lib["name"] == "kernix"
        ),
        str(meta.get("version", "unrecorded")),
    )


def header(scene, title, cpu, metric):
    scene.rect(0, 0, scene.width, scene.height, PAPER)
    scene.rect(0, 0, 9, scene.height, BLUE)
    scene.text(48, 42, "KERNIX / " + title.upper(), 19, bold=True)
    scene.text(1152, 42, "LESS TIME IS BETTER", 17, MUTED, anchor="end")
    scene.text(48, 81, cpu, 22, bold=True)
    scene.text(48, 113, metric, 18, MUTED)
    scene.line(48, 136, 1152, 136)


def latency_scene(w, values, meta, h):
    title, shape, dtype = h["label"](w)
    names = h["names"]
    reading = observation(values, names)
    lines = wrap(reading, 1104, 22, True)
    scene = Scene(height=420 + len(values) * 46 + max(0, len(lines) - 1) * 29)
    header(
        scene,
        "workload comparison · " + version(meta),
        h["cpu"],
        p.metric(meta) + " · microseconds (µs) · lower is faster",
    )
    scene.text(48, 184, title, 31, bold=True)
    scene.text(48, 220, " · ".join(filter(None, [shape, dtype])), 23)
    scene.text(48, 251, p.contract(w), 18, MUTED)
    maximum = max((p.median(v) for v in values.values()), default=1) or 1
    x0, x1 = 330, 920
    for i, key in enumerate(ranked(values)):
        yy = 300 + i * 46
        value = p.median(values[key])
        color = BLUE if key == "kernix" else TEAL if key.startswith("kernix") else GREY
        scene.text(48, yy + 5, names(key), 23, INK, key.startswith("kernix"))
        scene.rect(x0, yy - 14, x1 - x0, 22, "#E5EAF1", 3)
        scene.rect(
            x0,
            yy - 14,
            float(Decimal(str(value)) / Decimal(str(maximum))) * (x1 - x0),
            22,
            color,
            3,
        )
        scene.text(
            1152,
            yy + 5,
            p.chart_us(value) + " µs",
            23,
            INK,
            key.startswith("kernix"),
            "end",
        )
    yy = 300 + len(values) * 46
    scene.text(x0, yy - 9, "0", 17, MUTED)
    scene.text(x1, yy - 9, p.chart_us(maximum) + " µs", 17, MUTED, anchor="end")
    for i, line in enumerate(lines):
        scene.text(48, yy + 40 + i * 29, line, 22, bold=True)
    scene.text(
        48,
        scene.height - 38,
        "Single-run medians · ≈ marks rounding · exact values and conditions in the adjacent report",
        17,
        MUTED,
    )
    scene.text(
        48,
        scene.height - 13,
        "Run "
        + str(meta.get("run_id", "unrecorded"))
        + " · source "
        + str(meta.get("revision", "unrecorded"))[:12],
        14,
        MUTED,
    )
    return scene


def paired_scene(title, items, paired, h):
    scene = Scene(height=230 + 90 * len(items))
    header(
        scene,
        "Kernix " + version(paired["current"]) + " vs " + version(paired["baseline"]),
        h["cpu"],
        f"{paired['rounds']} fresh-process pairs · current / baseline · below 1 is faster",
    )
    lo = min(
        Decimal("0.8"),
        min(Decimal(str(v["ci95_low"])) for _, v in items) * Decimal("0.95"),
    )
    hi = max(
        Decimal("1.2"),
        max(Decimal(str(v["ci95_high"])) for _, v in items) * Decimal("1.05"),
    )
    x0, x1 = 580, 900

    def pos(v):
        return x0 + float((Decimal(str(v)) - lo) / (hi - lo)) * (x1 - x0)

    scene.text(48, 174, title, 24, bold=True)
    scene.text(x0, 174, "FASTER ←", 17, TEAL)
    scene.text(x1, 174, "→ SLOWER", 17, MUTED, anchor="end")
    scene.line(pos(1), 190, pos(1), scene.height - 63, MUTED, 2)
    for i, (w, row) in enumerate(items):
        yy = 216 + i * 90
        name, shape, dtype = h["label"](w)
        title_lines = wrap(name, 480, 21, True)
        for line_index, line in enumerate(title_lines):
            scene.text(48, yy - 3 + line_index * 24, line, 21, bold=True)
        scene.text(48, yy + len(title_lines) * 24 + 4, shape + " · " + dtype, 17, MUTED)
        status = (
            "Faster"
            if row["ci95_high"] < 1
            else "Slower"
            if row["ci95_low"] > 1
            else "Unclear"
        )
        color = (
            TEAL if status == "Faster" else "#A63C40" if status == "Slower" else MUTED
        )
        scene.line(x0, yy + 4, x1, yy + 4, LINE)
        scene.line(
            pos(row["ci95_low"]), yy + 4, pos(row["ci95_high"]), yy + 4, color, 6
        )
        scene.circle(pos(row["median_ratio"]), yy + 4, 6, INK)
        scene.text(1152, yy, p.compact(row["median_ratio"]) + "×", 22, INK, True, "end")
        scene.text(1152, yy + 29, status, 19, color, True, "end")
    scene.text(x0, scene.height - 39, p.compact(lo), 16, MUTED)
    scene.text(pos(1), scene.height - 39, "1 = same time", 16, MUTED, anchor="middle")
    scene.text(x1, scene.height - 39, p.compact(hi), 16, MUTED, anchor="end")
    scene.text(
        48,
        scene.height - 14,
        "Exploratory 95% process intervals · no multiple-comparison adjustment · exact values in report",
        15,
        MUTED,
    )
    return scene


def generate(dest, data, h, paired=None):
    dest = Path(dest)
    (dest / "charts").mkdir(exist_ok=True)
    data = p.normalize(data)
    groups = h["rows"](data)
    featured = next(
        (w for w, _ in groups if w == "matmul_128_f64"),
        groups[0][0] if groups else None,
    )
    for w, values in groups:
        scene = latency_scene(w, values, data["metadata"], h)
        raster(scene, dest / filename(w), scale=1.25)
        if w == featured:
            raster(scene, dest / "latency.png", scale=1.25)
            (dest / "latency.svg").write_text(scene.svg(), encoding="utf-8")
    if paired:
        generate_pairs(dest, paired, h)


def generate_pairs(dest, paired, h, standalone=False):
    dest = Path(dest)
    (dest / "charts").mkdir(exist_ok=True)
    for index, (path, title, items) in enumerate(pair_sheets(paired)):
        scene = paired_scene(title, items, paired, h)
        raster(scene, dest / path, scale=1.25)
        if standalone and index == 0:
            raster(scene, dest / "latency.png", scale=1.25)
            (dest / "latency.svg").write_text(scene.svg(), encoding="utf-8")
