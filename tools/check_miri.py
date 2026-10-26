#!/usr/bin/env python3
"""Exercise checked native wrappers with bounded independent coordinate oracles."""
import argparse
import json
import os
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[1]


def write_changed(path, data):
    if not path.exists() or path.read_bytes() != data:
        path.write_bytes(data)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--native', action='store_true', help='run the same fixture without Miri')
    parser.add_argument('--root', type=Path, default=ROOT)
    args = parser.parse_args()
    # Miri launchers retain their manifest cwd. A deleted TemporaryDirectory can
    # leave a cached launcher pointing at a nonexistent directory on the next run.
    package = args.root / 'target/miri-wrappers-fixture'
    (package / 'src').mkdir(parents=True, exist_ok=True)
    manifest = '[package]\nname="kernix-miri-wrappers"\nversion="0.0.0"\nedition="2024"\nrust-version="1.86"\n[workspace]\n[dev-dependencies]\nkernix-ops={path=' + json.dumps((args.root / 'crates/kernix-ops').as_posix()) + '}\n'
    write_changed(package / 'Cargo.toml', manifest.encode())
    write_changed(package / 'src/lib.rs', (Path(__file__).parent / 'fixtures/miri_native.rs').read_bytes())
    command = ['cargo'] + ([] if args.native else ['+nightly', 'miri']) + ['test', '--manifest-path', str(package / 'Cargo.toml')]
    environment = dict(os.environ)
    environment.setdefault('CARGO_TARGET_DIR', str(args.root / 'target/miri-wrappers-bounded'))
    subprocess.run(command, cwd=args.root, env=environment, check=True)


if __name__ == '__main__':
    main()
