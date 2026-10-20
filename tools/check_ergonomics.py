#!/usr/bin/env python3
"""Compile the real no_std workflow outside the workspace with default features off."""
import argparse
import json
import os
from pathlib import Path
import subprocess
import tempfile

ROOT=Path(__file__).resolve().parents[1]

def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--toolchain')
    parser.add_argument('--target')
    args=parser.parse_args()
    with tempfile.TemporaryDirectory(prefix='kernix-ergonomics-') as directory:
        package=Path(directory);(package/'src').mkdir()
        manifest='[package]\nname="kernix-ergonomics-consumer"\nversion="0.0.0"\nedition="2024"\nrust-version="1.86"\n[dependencies]\nkernix={path='+json.dumps((ROOT/'crates/kernix').as_posix())+',default-features=false}\n'
        (package/'Cargo.toml').write_text(manifest,encoding='utf-8')
        (package/'src/lib.rs').write_bytes((ROOT/'crates/kernix/tests/fixtures/no_std_workflow.rs').read_bytes())
        command=['cargo']+(['+'+args.toolchain] if args.toolchain else [])+[('check' if args.target else 'test'),'--manifest-path',str(package/'Cargo.toml')]
        if args.target:command+=['--target',args.target]
        environment=dict(os.environ);environment.setdefault('CARGO_TARGET_DIR',str(ROOT/'target/ergonomics-consumer'))
        subprocess.run(command,cwd=ROOT,env=environment,check=True)

if __name__=='__main__':main()
