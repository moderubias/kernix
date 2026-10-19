#!/usr/bin/env python3
"""Apply Kernix numerics overlay 04a corrective patch.

Only fixes four known mistakes from overlay 04. It preserves unrelated local
changes and fails before writing if any expected source anchor is absent.
Compatible with Python 3.9+; no third-party dependencies.
"""
from __future__ import annotations

import argparse
import os
from pathlib import Path
import re
import stat
import sys
import tempfile

ROOT = Path(__file__).resolve().parents[2]
REDUCE = "crates/kernix-ops/src/ops/reduce.rs"
TEST = "crates/kernix-ops/tests/numeric_dispatch_04.rs"


def fix_reduce(src: str) -> str:
    # Leave other existing imports untouched; rustfmt will normalize spacing.
    # Anchor the import deletion to the actual `use super` list so we never
    # touch unrelated occurrences in comments or function bodies.
    use_match = re.search(r"(?m)^use super::\{[^;]+\};", src)
    if use_match is None:
        raise ValueError(f"{REDUCE}: cannot find use super import")
    raw = use_match.group(0)
    if "elementwise::map" in raw:
        modified = re.sub(r",\s*elementwise::map\b", "", raw)
        if modified == raw:
            raise ValueError(f"{REDUCE}: unexpected import layout")
        src = src[:use_match.start()] + modified + src[use_match.end():]
    elif "elementwise::map" in src:
        raise ValueError(f"{REDUCE}: import token moved unexpectedly")

    src = replace_unique(
        src,
        old="mean += value.clone() / divisor;",
        new="mean += *value / divisor;",
        context="mean_all contiguous clone_on_copy",
    )
    src = replace_unique(
        src,
        old="mean += source.get(&coords)?.clone() / divisor;",
        new="mean += *source.get(&coords)? / divisor;",
        context="mean_all strided clone_on_copy",
    )
    return src


def replace_unique(src: str, *, old: str, new: str, context: str) -> str:
    if src.count(old) == 1 and src.count(new) == 0:
        return src.replace(old, new, 1)
    if src.count(new) == 1 and src.count(old) == 0:
        return src  # already fixed
    raise ValueError(f"{context}: expected exactly one old or new variant; patch refused")


def fix_test(src: str) -> str:
    # For reversed a, rows are [3,2,1], [6,5,4]. Matmul against b gives
    # [50,56,131,146]; the original test oracle was arithmetic error.
    return replace_unique(
        src,
        old="assert_eq!(out.storage_slice(), &[46.0, 52.0, 127.0, 142.0]);",
        new="assert_eq!(out.storage_slice(), &[50.0, 56.0, 131.0, 146.0]);",
        context="noncontiguous matmul oracle",
    )


def write_atomically(path: Path, contents: str) -> None:
    mode = stat.S_IMODE(path.stat().st_mode)
    tmp_path = None
    try:
        with tempfile.NamedTemporaryFile(
            mode="w", encoding="utf-8", newline="", dir=path.parent,
            prefix="." + path.name + ".", suffix=".tmp", delete=False,
        ) as tmp:
            tmp.write(contents)
            tmp_path = Path(tmp.name)
        os.chmod(tmp_path, mode)
        os.replace(tmp_path, path)
    finally:
        if tmp_path is not None and tmp_path.exists():
            tmp_path.unlink()


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true", help="preflight only; do not write")
    args = parser.parse_args()
    if not (ROOT / "Cargo.toml").is_file():
        parser.error(f"expected Kernix repository root at {ROOT}")
    if not (ROOT / "tools/kernix-update-04/apply.py").is_file():
        parser.error("install kernix-numerics-softmax-04.zip before hotfix 04a")

    changes: list[tuple[Path, str]] = []
    try:
        for rel, fn in [(REDUCE, fix_reduce), (TEST, fix_test)]:
            path = ROOT / rel
            original = path.read_text(encoding="utf-8")
            updated = fn(original)
            if updated != original:
                changes.append((path, updated))
                print(f"READY: {rel}")
            else:
                print(f"ALREADY FIXED: {rel}")
    except (ValueError, OSError) as exc:
        print(f"Preflight failed: {exc}", file=sys.stderr)
        return 1

    if args.check:
        print(f"Preflight successful: {len(changes)} files need modification; no files changed.")
        return 0
    for path, contents in changes:
        write_atomically(path, contents)
        print(f"Fixed: {path.relative_to(ROOT)}")
    print("Hotfix 04a complete. Run cargo fmt/test/clippy.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
