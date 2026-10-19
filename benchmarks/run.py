#!/usr/bin/env python3
"""Save immutable comparative measurements and printable reports (dev tools only)."""
import argparse, json, os, platform, statistics, subprocess, time
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]

def command(*args):
    return subprocess.check_output(args, cwd=ROOT, text=True).strip()

def measure(operation):
    for _ in range(8): operation()
    samples = []
    for _ in range(15):
        start = time.perf_counter_ns(); count = 0
        while time.perf_counter_ns() - start < 25_000_000:
            operation(); count += 1
        samples.append((time.perf_counter_ns() - start) / count)
    return dict(p50_ns=statistics.median(samples), p95_ns=max(samples), samples_ns=samples)

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('version', help='unique version/run directory; never overwritten')
    args = parser.parse_args()
    dest = ROOT / 'benchmarks' / 'results' / args.version
    dest.mkdir(parents=True, exist_ok=False)
    os.environ.update(OPENBLAS_NUM_THREADS='1', OMP_NUM_THREADS='1', MKL_NUM_THREADS='1')
    import numpy as np
    results = json.loads(command('cargo', 'run', '--release', '-q', '-p', 'kernix-ops', '--example', 'compare'))
    a = (np.arange(65536) % 31).astype(np.float64) / 31
    results['add_65536_f64/numpy'] = measure(lambda: a+a)
    results['sum_65536_f64/numpy'] = measure(lambda: a.sum())
    for size in (32,128):
        a = ((np.arange(size*size) % 13).astype(np.float64)/13).reshape(size,size)
        results[f'matmul_{size}_f64/numpy'] = measure(lambda: a@a)
        out=np.empty_like(a)
        results[f'matmul_reuse_{size}_f64/numpy'] = measure(lambda: np.matmul(a,a,out=out))
    a = ((np.arange(256*64)%41).astype(np.float64)/4).reshape(256,64)
    def softmax():
        e = np.exp(a - a.max(axis=-1, keepdims=True))
        return e / e.sum(axis=-1, keepdims=True)
    results['softmax_256x64_f64/numpy'] = measure(softmax)
    metadata = dict(version=args.version, timestamp=time.strftime('%Y-%m-%dT%H:%M:%SZ',time.gmtime()),
        platform=platform.platform(), cpu=command('sh','-c',"lscpu | sed -n 's/^Model name: *//p'"),
        rustc=command('rustc','-V'), revision=command('git','rev-parse','HEAD'),
        dirty=bool(command('git','status','--porcelain')), numpy=np.__version__,
        threads=1, dtype='f64', timing='allocation-inclusive; 15 warmed 25ms batches; p95=max batch mean',
        dependency_tree=command('cargo','tree','-p','kernix-ops'),
        numpy_configuration=str(np.show_config(mode='dicts')))
    (dest/'raw.json').write_text(json.dumps(dict(metadata=metadata,results=results),indent=2)+'\n')
    lines = [f'# Kernix benchmark report: {args.version}', '',
        f"Measured {metadata['timestamp']} on {metadata['cpu']}.", '',
        f"Rust: {metadata['rustc']}; NumPy: {np.__version__}; one CPU thread; float64.", '',
        f"Source: `{metadata['revision']}`; working tree dirty: {metadata['dirty']}.", '',
        '## Comparative CPU results', '',
        '| Workload | Kernix p50 / p95 (us) | ndarray p50 / p95 (us) | NumPy p50 / p95 (us) |',
        '|---|---:|---:|---:|']
    for workload in sorted({key.split('/')[0] for key in results}):
        cells=[]
        for lib in ('kernix','ndarray','numpy'):
            v=results.get(f'{workload}/{lib}')
            cells.append(f"{v['p50_ns']/1000:.3f} / {v['p95_ns']/1000:.3f}" if v else 'not measured')
        lines.append('| '+workload+' | '+' | '.join(cells)+' |')
    lines += ['', '## Method and limits', '',
        'Operations allocate outputs except scalar sums and explicitly named matmul_reuse cases.',
        'Reuse cases time preallocated output writes in all libraries. Inputs are preallocated and',
        'contiguous. ndarray uses its default matrixmultiply engine (no BLAS feature); NumPy',
        'uses the build recorded in raw.json. BLAS/OMP thread counts are fixed to one.',
        'These are warmed, sequential wall-clock samples on a shared laptop; CPU frequency',
        'and other system load are uncontrolled. p95 is the maximum of 15 batch means,',
        'not a per-request tail-latency guarantee. Different sum orders may round differently.',
        'The ndarray softmax is an explicitly coded stable rowwise comparison, not a native',
        'ndarray softmax API. NumPy timings include Python call overhead. No GPU comparison.',
        'No claim that a complete Full HD model reaches 120 FPS follows from these measurements.',
        'raw.json contains all measurements, dependencies and environment metadata.', '']
    report=dest/'README.md'; report.write_text('\n'.join(lines))
    subprocess.run(['pandoc',str(report),'-o',str(dest/'report.pdf'),'--pdf-engine=xelatex',
                    '-V','geometry:margin=18mm','-V','fontsize=10pt'],check=True,cwd=ROOT)
    print(report.relative_to(ROOT))
if __name__ == '__main__': main()
