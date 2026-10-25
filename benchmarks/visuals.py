"""Shared vector scene for publication PDFs, SVG previews and raster thumbnails."""

from pathlib import Path
import html

ASSETS = Path(__file__).with_name("assets")
INK = "#142437"
MUTED = "#566779"
LINE = "#DFE5EC"
BLUE = "#2255D9"
TEAL = "#147D81"
GREY = "#A7B4C5"
PAPER = "#F5F7FA"
W, H = 1200, 848.5


class Scene:
    """One coordinate system; exporters do not recalculate chart data."""

    def __init__(self, width=W, height=H):
        self.width, self.height = width, height
        self.commands = []

    def rect(self, x, y, w, h, color, radius=0):
        self.commands.append(("rect", x, y, w, h, color, radius))

    def line(self, x, y, xx, yy, color=LINE, width=1):
        self.commands.append(("line", x, y, xx, yy, color, width))

    def circle(self, x, y, radius, color):
        self.commands.append(("circle", x, y, radius, color))

    def text(self, x, y, value, size=16, color=INK, bold=False, anchor="start"):
        self.commands.append(("text", x, y, str(value), size, color, bold, anchor))

    def svg(self):
        from presentation import font_css, font_license

        elements = [
            f'<svg xmlns="http://www.w3.org/2000/svg" width="{self.width}" height="{self.height}" viewBox="0 0 {self.width} {self.height}">',
            "<title>Kernix performance report preview</title>",
            "<style>"
            + font_css()
            + "text{font-family:Plex,Arial,sans-serif;font-variant-numeric:tabular-nums}</style>",
        ]
        for cmd in self.commands:
            kind, *v = cmd
            if kind == "rect":
                x, y, w, h, c, r = v
                elements.append(
                    f'<rect x="{x}" y="{y}" width="{w}" height="{h}" rx="{r}" fill="{c}"/>'
                )
            elif kind == "line":
                x, y, xx, yy, c, width = v
                elements.append(
                    f'<line x1="{x}" y1="{y}" x2="{xx}" y2="{yy}" stroke="{c}" stroke-width="{width}"/>'
                )
            elif kind == "circle":
                x, y, r, c = v
                elements.append(f'<circle cx="{x}" cy="{y}" r="{r}" fill="{c}"/>')
            else:
                x, y, t, size, c, bold, anchor = v
                elements.append(
                    f'<text x="{x}" y="{y}" font-size="{size}" fill="{c}" font-weight="{600 if bold else 400}" text-anchor="{anchor}">{html.escape(t)}</text>'
                )
        return "".join(elements) + "<!--" + font_license() + "--></svg>"


def fonts():
    from reportlab.pdfbase import pdfmetrics
    from reportlab.pdfbase.ttfonts import TTFont

    for name, style in [("Plex", "Regular"), ("PlexBold", "SemiBold")]:
        if name not in pdfmetrics.getRegisteredFontNames():
            pdfmetrics.registerFont(
                TTFont(name, str(ASSETS / f"IBMPlexSans-{style}.ttf"))
            )


def wrap(value, width, size=16, bold=False):
    from reportlab.pdfbase.pdfmetrics import stringWidth

    fonts()
    lines = []
    line = ""
    for word in str(value).split():
        candidate = (line + " " + word).strip()
        if (
            line
            and stringWidth(candidate, "PlexBold" if bold else "Plex", size) > width
        ):
            lines.append(line)
            line = word
        else:
            line = candidate
    return lines + [line] if line else lines


def heading(scene, title, subtitle, page, total, source):
    scene.rect(0, 0, W, H, PAPER)
    scene.rect(40, 38, 22, 22, BLUE, 5)
    scene.line(47, 44, 47, 54, "#FFFFFF", 2)
    scene.line(49, 49, 56, 44, "#FFFFFF", 2)
    scene.line(49, 49, 56, 55, "#FFFFFF", 2)
    scene.text(74, 55, "KERNIX", 16, bold=True)
    scene.text(W - 40, 55, "PERFORMANCE / RESEARCH EDITION", 12, MUTED, anchor="end")
    scene.line(40, 78, W - 40, 78)
    scene.text(40, 124, title, 30, bold=True)
    for j, line in enumerate(wrap(subtitle, 1090, 14)):
        scene.text(40, 151 + j * 19, line, 14, MUTED)
    scene.line(40, H - 48, W - 40, H - 48)
    scene.text(40, H - 25, source, 11, MUTED)
    scene.text(W - 40, H - 25, f"{page:02d} / {total:02d}", 11, MUTED, anchor="end")


def latency_card(scene, y, title, subtitle, values, metric, height=280):
    """Absolute latency: zero baseline, direct labels, exact values from the model."""
    scene.rect(40, y, W - 80, height, "#FFFFFF", 12)
    scene.text(64, y + 31, title, 20, bold=True)
    scene.text(64, y + 55, subtitle, 13, MUTED)
    scene.text(W - 64, y + 31, metric, 12, MUTED, anchor="end")
    start, end = 280, 875
    maximum = max(float(v["ns"]) for v in values) if values else 1
    maximum = maximum or 1
    for i, v in enumerate(values):
        yy = y + 86 + i * 27
        color = (
            BLUE
            if v["key"] == "kernix"
            else TEAL
            if v["key"] == "kernix-accelerate"
            else GREY
        )
        scene.text(64, yy + 4, v["name"], 14, INK, v["key"].startswith("kernix"))
        scene.rect(start, yy - 8, end - start, 12, PAPER, 3)
        scene.rect(
            start, yy - 8, (end - start) * float(v["ns"]) / maximum, 12, color, 3
        )
        scene.text(W - 64, yy + 4, v["us"], 13, INK, False, "end")
    yy = y + height - 23
    scene.text(start, yy, "0", 11, MUTED)
    scene.text(
        end,
        yy,
        values[max(range(len(values)), key=lambda i: float(values[i]["ns"]))]["us"]
        + " µs",
        11,
        MUTED,
        anchor="end",
    )
    scene.text(64, yy, "Lower is faster", 11, MUTED)


def interval_card(scene, y, title, subtitle, ratio, lo, hi, status, height=270):
    scene.rect(40, y, W - 80, height, "#FFFFFF", 12)
    scene.text(64, y + 32, title, 20, bold=True)
    scene.text(64, y + 56, subtitle, 13, MUTED)
    scene.text(64, y + 90, status, 14, INK, True)
    x0, x1 = 200, 1040
    maximum = max(1.25, float(hi) * 1.12)
    pos = lambda v: x0 + float(v) / maximum * (x1 - x0)
    yy = y + 142
    scene.line(x0, yy, x1, yy, LINE, 2)
    scene.line(pos(1), yy - 25, pos(1), yy + 25, MUTED, 1)
    scene.line(pos(lo), yy, pos(hi), yy, TEAL if float(hi) < 1 else BLUE, 6)
    scene.circle(pos(ratio), yy, 7, INK)
    scene.text(x0, yy + 40, "0", 12, MUTED)
    scene.text(pos(1), yy + 40, "1 = same latency", 12, MUTED, anchor="middle")
    scene.text(64, y + height - 32, "CURRENT / BASELINE", 11, MUTED, True)
    scene.text(262, y + height - 32, str(ratio), 14, INK, True)
    scene.text(555, y + height - 32, "95% INTERVAL", 11, MUTED, True)
    scene.text(700, y + height - 32, str(lo) + " – " + str(hi), 13, INK)


def export(scenes, dest, stem):
    """PDF stays vector; PNG is a deterministic preview of the same first scene."""
    from reportlab.pdfgen.canvas import Canvas
    from reportlab.lib.colors import HexColor

    fonts()
    dest = Path(dest)
    (dest / (stem + ".svg")).write_text(scenes[0].svg(), encoding="utf-8")
    canvas = Canvas(str(dest / (stem + ".pdf")), pagesize=(841.89, 595.28), invariant=1)
    canvas.setTitle("Kernix · CPU performance")
    canvas.setAuthor("Kernix")
    for scene in scenes:
        canvas.saveState()
        canvas.scale(841.89 / W, 595.28 / H)
        for cmd in scene.commands:
            kind, *v = cmd
            if kind == "rect":
                x, y, w, h, c, r = v
                canvas.setFillColor(HexColor(c))
                canvas.roundRect(x, H - y - h, w, h, r, stroke=0, fill=1)
            elif kind == "line":
                x, y, xx, yy, c, width = v
                canvas.setStrokeColor(HexColor(c))
                canvas.setLineWidth(width)
                canvas.line(x, H - y, xx, H - yy)
            elif kind == "circle":
                x, y, r, c = v
                canvas.setFillColor(HexColor(c))
                canvas.circle(x, H - y, r, stroke=0, fill=1)
            else:
                x, y, t, size, c, bold, anchor = v
                canvas.setFillColor(HexColor(c))
                canvas.setFont("PlexBold" if bold else "Plex", size)
                {"end": canvas.drawRightString, "middle": canvas.drawCentredString}.get(
                    anchor, canvas.drawString
                )(x, H - y, t)
        canvas.restoreState()
        canvas.showPage()
    canvas.save()
    raster(scenes[0], dest / (stem + ".png"))


def raster(scene, path, scale=1.5):
    """Offline PNG for GitHub, with readable type and lossless PNG storage."""
    from PIL import Image, ImageDraw, ImageFont

    image = Image.new(
        "RGB", (round(scene.width * scale), round(scene.height * scale)), "white"
    )
    draw = ImageDraw.Draw(image)

    def xy(*v):
        return tuple(round(x * scale) for x in v)

    for cmd in scene.commands:
        kind, *v = cmd
        if kind == "rect":
            x, y, w, h, c, r = v
            draw.rounded_rectangle(
                xy(x, y, x + w, y + h), radius=round(r * scale), fill=c
            )
        elif kind == "line":
            x, y, xx, yy, c, width = v
            draw.line(xy(x, y, xx, yy), fill=c, width=max(1, round(width * scale)))
        elif kind == "circle":
            x, y, r, c = v
            draw.ellipse(xy(x - r, y - r, x + r, y + r), fill=c)
        else:
            x, y, t, size, c, bold, anchor = v
            font = ImageFont.truetype(
                str(ASSETS / f"IBMPlexSans-{'SemiBold' if bold else 'Regular'}.ttf"),
                round(size * scale),
            )
            draw.text(
                xy(x, y),
                t,
                font=font,
                fill=c,
                anchor={"end": "rs", "middle": "ms"}.get(anchor, "ls"),
            )
    # No numeric or geometric data is altered; palette storage reduces archive size.
    image.save(path, optimize=True)
