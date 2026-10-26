#!/usr/bin/env python3
"""Compile and run the public README and user-guide examples as downstream code."""
import json
import os
from pathlib import Path
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[1]
with tempfile.TemporaryDirectory(prefix="kernix-doc-examples-") as directory:
    package = Path(directory)
    (package / "src").mkdir()
    manifest = ('[package]\nname="kernix-doc-examples"\nversion="0.0.0"\n'
                'edition="2024"\n[dependencies]\nkernix={path='
                + json.dumps((ROOT / "crates/kernix").as_posix()) + '}\n')
    (package / "Cargo.toml").write_text(manifest, encoding="utf-8")
    documents = (ROOT / "README.md", ROOT / "docs/USER_GUIDE.md", ROOT / "docs/API-ERGONOMICS.md", ROOT / "docs/SPARSE.md")
    source = "\n".join('#![doc=include_str!(' + json.dumps(p.as_posix()) + ')]'
                       for p in documents)
    (package / "src/lib.rs").write_text(source, encoding="utf-8")
    environment = dict(os.environ)
    environment.setdefault("CARGO_TARGET_DIR", str(ROOT / "target/doc-examples"))
    subprocess.run(["cargo", "test", "--doc", "--manifest-path",
                    str(package / "Cargo.toml")], check=True, env=environment, cwd=ROOT)
