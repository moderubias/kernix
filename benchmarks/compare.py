#!/usr/bin/env python3
"""Interleave fresh frozen workers; bootstrap paired process medians, not calls."""
import argparse
import hashlib
import json
import math
import os
from pathlib import Path
import random
import statistics
import subprocess

from evidence import ROOT, environment
from report import baseline_warnings, label


def paired_interval(pairs, seed=0, resamples=10000):
    """Percentile bootstrap CI for median of paired process latency ratios."""
    if len(pairs) < 5 or resamples < 100:
        raise ValueError('need at least five process pairs and 100 resamples')
    if any(not math.isfinite(x) or x <= 0 for pair in pairs for x in pair):
        raise ValueError('latencies must be positive and finite')
    ratios=[current/baseline for current,baseline in pairs]
    rng=random.Random(seed)
    samples=sorted(statistics.median(rng.choices(ratios,k=len(ratios))) for _ in range(resamples))
    return dict(median_ratio=statistics.median(ratios),
                ci95_low=samples[math.ceil(resamples*.025)-1],
                ci95_high=samples[math.ceil(resamples*.975)-1])


def frozen_worker(directory):
    data=json.loads((directory/'raw.json').read_text(encoding='utf-8'))
    meta=data['metadata']
    binary=ROOT/'web/binaries'/meta['run_id']/('time.exe' if os.name=='nt' else 'time')
    if hashlib.sha256(binary.read_bytes()).hexdigest()!=meta['binary_sha256']['time']:
        raise ValueError('frozen worker hash mismatch: '+str(binary))
    return data,binary


def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('current',type=Path)
    parser.add_argument('baseline',type=Path)
    parser.add_argument('output',type=Path)
    parser.add_argument('--rounds',type=int,default=9)
    parser.add_argument('--seed',type=int,default=20261022)
    parser.add_argument('--workloads',nargs='+',default=['cholesky_factor_64_f64','cholesky_factor_128_f64',
        'cholesky_factor_256_f64','qr_96x32_f64','qr_256x64_f64','matmul_128_f64','matmul_128_f32'])
    args=parser.parse_args()
    if args.rounds<5:parser.error('at least five process pairs required')
    if len(args.workloads)!=len(set(args.workloads)):parser.error('duplicate workloads')
    current,current_binary=frozen_worker(args.current)
    baseline,baseline_binary=frozen_worker(args.baseline)
    warnings=baseline_warnings(current,baseline)
    if warnings:raise ValueError('incompatible recorded environments: '+' '.join(warnings))
    for workload in args.workloads:
        for data in [current,baseline]:
            if workload+'/kernix' not in data['results']:raise ValueError('missing workload: '+workload)
    os.environ.update(RAYON_NUM_THREADS='1',OPENBLAS_NUM_THREADS='1',OMP_NUM_THREADS='1',MKL_NUM_THREADS='1')
    env=dict(os.environ)
    affinity=os.sched_getaffinity(0) if hasattr(os,'sched_getaffinity') else None
    cpu=current['metadata']['environment'].get('affinity_cpu')
    if affinity is not None:
        if cpu not in affinity:raise ValueError('recorded CPU unavailable')
        os.sched_setaffinity(0,{cpu})
    rng=random.Random(args.seed)
    blocks=[]
    try:
        for _ in range(args.rounds):
            schedule=[(workload,version) for workload in args.workloads for version in ['current','baseline']]
            rng.shuffle(schedule)
            measurements={}
            for workload,version in schedule:
                if affinity is not None:
                    policy=current['metadata'].get('explicit_execution',{}).get(workload,{})
                    cpus=set(policy.get('affinity_cpus') or [cpu])
                    if not cpus<=affinity:raise ValueError('recorded workload CPUs unavailable')
                    os.sched_setaffinity(0,cpus)
                binary=current_binary if version=='current' else baseline_binary
                value=json.loads(subprocess.check_output([str(binary),workload,'kernix','time'],text=True,env=env))
                measurements[workload+'/'+version]=value
            blocks.append(dict(order=schedule,measurements=measurements))
    finally:
        if affinity is not None:os.sched_setaffinity(0,affinity)
    results={}
    for i,workload in enumerate(args.workloads):
        pairs=[tuple(block['measurements'][workload+'/'+v]['per_call']['p50_ns'] for v in ['current','baseline']) for block in blocks]
        results[workload]=dict(pairs_ns=pairs,**paired_interval(pairs,args.seed+i))
    args.output.mkdir(parents=True,exist_ok=False)
    data=dict(schema_version=1,seed=args.seed,rounds=args.rounds,environment=environment(cpu),
        current=current['metadata'],baseline=baseline['metadata'],blocks=blocks,results=results,
        statistic='median of paired fresh-process per-call p50 latency ratios, current/baseline',
        interval='95% percentile bootstrap over paired process blocks; 10000 resamples; exploratory, no multiple-comparison adjustment')
    (args.output/'paired.json').write_text(json.dumps(data,indent=2)+'\n',encoding='utf-8')
    current_version=next(p['version'] for p in current['metadata']['library_packages'] if p['name']=='kernix')
    baseline_version=next(p['version'] for p in baseline['metadata']['library_packages'] if p['name']=='kernix')
    lines=[f'# Kernix {current_version} vs {baseline_version} · paired processes','',
        f"Current: `{current['metadata']['run_id']}`. Baseline: `{baseline['metadata']['run_id']}`.",'',f'{args.rounds} fresh-process pairs per workload. Lower ratio is faster.',
        '95% bootstrap intervals resample process pairs, not individual calls. Exploratory intervals do not correct for multiple comparisons.',
        'Shared-machine drift and small sample counts remain limits; these are not per-call p95 intervals.','',
        '| Workload | Current / baseline latency | 95% interval |','|---|---:|---:|']
    for w,value in results.items():
        name,shape,dtype=label(w)
        lines.append(f"| {name} {shape} {dtype} | {value['median_ratio']:.3f}× | [{value['ci95_low']:.3f}, {value['ci95_high']:.3f}] |")
    lines+=['','Full worker samples, accuracy checks, randomized order, executable hashes and source metadata: [paired.json](paired.json).','']
    (args.output/'README.md').write_text('\n'.join(lines),encoding='utf-8')
    print(args.output/'README.md')


if __name__=='__main__':main()
