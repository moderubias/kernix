#!/usr/bin/env python3
"""Audit runtime and development lockfiles without hiding upstream findings."""
import argparse
import datetime
import json
from pathlib import Path
import subprocess

ROOT=Path(__file__).resolve().parents[1]
# Candle 0.11 pins zip 8.6, whose extraction APIs are not used by the fixed,
# synthetic-data benchmark workers. Do not modify a measured comparator's source.
# Remove once https://github.com/huggingface/candle/issues/4043 is released.
BENCHMARK_EXCEPTIONS={'RUSTSEC-2026-0367','RUSTSEC-2026-0368','RUSTSEC-2026-0369'}
EXPIRY=datetime.date(2026,11,22)


def unexpected(report, benchmark=False, today=None):
    today=today or datetime.datetime.now(datetime.timezone.utc).date()
    return [finding for finding in report['vulnerabilities']['list']
            if not (benchmark and today<=EXPIRY
                    and finding['advisory']['id'] in BENCHMARK_EXCEPTIONS
                    and finding['package']['name']=='zip'
                    and finding['package']['version']=='8.6.0')]


def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--output',type=Path,default=ROOT/'web/advisories')
    args=parser.parse_args();args.output.mkdir(parents=True,exist_ok=True)
    failed=False
    for name,lockfile,benchmark in [('library','Cargo.lock',False),('benchmark','benchmarks/harness/Cargo.lock',True)]:
        result=subprocess.run(['cargo','audit','--file',lockfile,'--json'],cwd=ROOT,stdout=subprocess.PIPE,text=True)
        try:report=json.loads(result.stdout)
        except json.JSONDecodeError:raise SystemExit('cargo-audit did not return JSON; audit failed')
        if result.returncode not in (0,1):raise SystemExit('cargo-audit failed with '+str(result.returncode))
        (args.output/(name+'.json')).write_text(result.stdout,encoding='utf-8')
        findings=unexpected(report,benchmark)
        found=report['vulnerabilities']['count']
        print(f'{name}: {found} vulnerabilities; {len(findings)} outside reviewed exceptions; warnings={list(report["warnings"])}')
        if found and not findings:print('Benchmark-only zip 8.6 extraction exceptions expire '+EXPIRY.isoformat())
        failed |= bool(findings)
    raise SystemExit(1 if failed else 0)


if __name__=='__main__':main()
