#!/usr/bin/env python3
"""Gate source compatibility against a fixed release, retaining JSON run evidence."""
import argparse
import json
import os
from pathlib import Path
import subprocess

ROOT=Path(__file__).resolve().parents[1]


def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--baseline',default='v0.9.0')
    parser.add_argument('--output',type=Path,default=ROOT/'web/api-compatibility.json')
    args=parser.parse_args()
    revision=subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip()
    baseline=subprocess.check_output(['git','rev-parse',args.baseline+'^{commit}'],cwd=ROOT,text=True).strip()
    version=subprocess.check_output(['cargo','semver-checks','--version'],cwd=ROOT,text=True).strip()
    runs=[]
    # Explicit minor permits additive APIs but rejects removals/signature changes,
    # independent of the next crate version (including Rust's 0.x version rules).
    for profile,features in [('all-features',['--all-features']),('no-default-features',['--only-explicit-features'])]:
        command=['cargo','semver-checks','--workspace','--baseline-rev',baseline,
                 '--release-type','minor','--color','never',*features]
        result=subprocess.run(command,cwd=ROOT,text=True,stdout=subprocess.PIPE,stderr=subprocess.STDOUT,env=os.environ)
        runs.append(dict(profile=profile,command=command,exit_code=result.returncode,diagnostics=result.stdout))
        print(profile+': '+('passed' if result.returncode==0 else 'FAILED'),flush=True)
        if result.returncode:print(result.stdout,flush=True)
    # The tool has no stable structured findings mode. Retain full diagnostics
    # without parsing human text into invented structured API findings.
    report=dict(schema_version=1,tool=version,revision=revision,baseline=baseline,
                working_tree_dirty=bool(subprocess.check_output(['git','status','--porcelain'],cwd=ROOT,text=True).strip()),runs=runs,
                passed=all(run['exit_code']==0 for run in runs))
    args.output.parent.mkdir(parents=True,exist_ok=True)
    args.output.write_text(json.dumps(report,indent=2)+'\n',encoding='utf-8')
    raise SystemExit(0 if report['passed'] else 1)


if __name__=='__main__':main()
