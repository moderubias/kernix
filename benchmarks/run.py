#!/usr/bin/env python3
"""Save immutable comparative measurements and printable reports (dev tools only)."""
import argparse, hashlib, io, json, os, platform, statistics, subprocess, tarfile, time
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
    # Build before pinning to keep compilation out of timing and retain parallel builds.
    examples=[('kernix-ops','compare'),('kernix-ops','vision_compare'),
              ('kernix-ops','linalg_compare'),('kernix','training_compare'),
              ('kernix','construction_compare')]
    examples=[(package,name) for package,name in examples
              if (ROOT/f'crates/{package}/examples/{name}.rs').exists()]
    for package,name in examples:
        command('cargo','build','--release','-q','-p',package,'--example',name)
    affinity=None
    if hasattr(os,'sched_getaffinity'):
        affinity=min(os.sched_getaffinity(0))
        os.sched_setaffinity(0,{affinity})
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
    if (ROOT/'crates/kernix-ops/examples/vision_compare.rs').exists():
        results.update(json.loads(command('cargo','run','--release','-q','-p','kernix-ops','--example','vision_compare')))
        rgb=(np.arange(1080*1920*3,dtype=np.uint32)%251).astype(np.uint8).reshape(1080,1920,3)
        def resize():
            y=np.clip((np.arange(224)+0.5)*1080/224-0.5,0,1079)
            x=np.clip((np.arange(224)+0.5)*1920/224-0.5,0,1919)
            y0=y.astype(np.intp);x0=x.astype(np.intp);y1=np.minimum(y0+1,1079);x1=np.minimum(x0+1,1919)
            fy=(y-y0)[:,None,None];fx=(x-x0)[None,:,None]
            top=rgb[y0[:,None],x0]*(1-fx)+rgb[y0[:,None],x1]*fx
            bottom=rgb[y1[:,None],x0]*(1-fx)+rgb[y1[:,None],x1]*fx
            return ((top*(1-fy)+bottom*fy)/255).astype(np.float32).transpose(2,0,1)[None].copy()
        results['rgb1080_to224_f32/numpy']=measure(resize)
        av=((np.arange(3*32*32)%31).astype(np.float32)-15)/31
        wv=((np.arange(8*3*3*3)%13).astype(np.float32)-6)/13
        image=av.reshape(1,3,32,32);weight=wv.reshape(8,3,3,3)
        def convolution():
            windows=np.lib.stride_tricks.sliding_window_view(image,(3,3),axis=(2,3))
            return np.einsum('ncyxkl,ockl->noyx',windows,weight,optimize=True)
        results['conv_1x3x32x32_8x3x3x3_f32/numpy']=measure(convolution)
    if ('kernix-ops','linalg_compare') in examples:
        results.update(json.loads(command('cargo','run','--release','-q','-p','kernix-ops','--example','linalg_compare')))
        n=64; row,col=np.indices((n,n))
        a=0.01/(1+np.abs(row-col))+np.eye(n)*4
        b=np.arange(n,dtype=np.float64)/n
        results['solve_64_f64/numpy']=measure(lambda:np.linalg.solve(a,b))
        results['cholesky_64_f64/numpy']=measure(lambda:np.linalg.cholesky(a))
        for m,n in ((8,4),(32,16),(96,32),(256,64)):
            index=np.arange(m*n).reshape(m,n)
            a=((index*13%47).astype(np.float64)-23)/47+(index//n==index%n)*3
            results[f'qr_{m}x{n}_f64/numpy']=measure(lambda:np.linalg.qr(a,mode='reduced'))
    if ('kernix','training_compare') in examples:
        results.update(json.loads(command('cargo','run','--release','-q','-p','kernix','--example','training_compare')))
    if ('kernix','construction_compare') in examples:
        results.update(json.loads(command('cargo','run','--release','-q','-p','kernix','--example','construction_compare')))
        results['create_full_64x64_f64/numpy']=measure(lambda:np.full((64,64),1.0,dtype=np.float64))
        results['create_matrix_3x3_f64/numpy']=measure(lambda:np.array([[1.,2.,3.],[4.,5.,6.],[7.,8.,9.]],dtype=np.float64))
    source=dest/'source.tar.gz'
    with tarfile.open(source,'w:gz') as archive:
        paths=command('git','ls-files','-co','--exclude-standard').splitlines()
        for path in sorted(set(paths)):
            file=ROOT/path
            if file.is_file() and not path.startswith('benchmarks/results/'):
                archive.add(file,arcname=path)
    source_digest=hashlib.sha256(source.read_bytes()).hexdigest()
    cpu=platform.processor() or platform.machine()
    if platform.system()=='Linux':
        for line in command('lscpu').splitlines():
            if line.startswith('Model name:'): cpu=line.split(':',1)[1].strip()
    metadata = dict(version=args.version, timestamp=time.strftime('%Y-%m-%dT%H:%M:%SZ',time.gmtime()),
        platform=platform.platform(), cpu=cpu, affinity_cpu=affinity, source_sha256=source_digest,
        rustc=command('rustc','-V'), revision=command('git','rev-parse','HEAD'),
        dirty=bool(command('git','status','--porcelain')), numpy=np.__version__,
        threads=1, dtype='f64 except explicitly named f32 workloads', timing='allocation-inclusive; 15 warmed 25ms batches; p95=max batch mean',
        dependency_tree=command('cargo','tree','-p','kernix-ops'),
        numpy_configuration=str(np.show_config(mode='dicts')))
    (dest/'raw.json').write_text(json.dumps(dict(metadata=metadata,results=results),indent=2)+'\n')
    import matplotlib
    matplotlib.use('Agg')
    import matplotlib.pyplot as plt
    workloads=sorted({key.split('/')[0] for key in results})
    figure,axis=plt.subplots(figsize=(11,max(6.6,0.32*len(workloads)+2)),layout='constrained')
    colors={'ndarray':'#2166ac','nalgebra':'#dd751c','numpy':'#319873'}
    for library,color in colors.items():
        rows=[i for i,w in enumerate(workloads) if f'{w}/{library}' in results]
        ratios=[results[f'{workloads[i]}/{library}']['p50_ns']/results[f'{workloads[i]}/kernix']['p50_ns'] for i in rows]
        axis.scatter(ratios,rows,label=library,color=color,s=50,zorder=3)
    axis.axvline(1,color='#333333',linewidth=1,label='Kernix = 1')
    axis.set_xscale('log');axis.set_yticks(range(len(workloads)),[w.replace('_',' ') for w in workloads])
    axis.invert_yaxis();axis.grid(axis='x',alpha=0.18)
    axis.set_xlabel('Competitor median time / Kernix median time (higher = Kernix faster)')
    axis.set_title(f'Kernix {args.version} | single CPU | warmed batch medians',loc='left',fontweight='bold',pad=16)
    axis.spines[['top','right','left']].set_visible(False)
    axis.legend(loc='upper left',bbox_to_anchor=(0,-0.12),ncol=4,frameon=False)
    figure.savefig(dest/'comparison.png',dpi=160)
    figure.savefig(dest/'comparison.pdf')
    plt.close(figure)
    lines = [f'# Kernix benchmark report: {args.version}', '',
        f"Measured {metadata['timestamp']} on {metadata['cpu']}.", '',
        f"Rust: {metadata['rustc']}; NumPy: {np.__version__}; one CPU thread; dtype in workload name.", '',
        f"Source: `{metadata['revision']}`; working tree dirty: {metadata['dirty']}.", '',
        '## Comparative CPU results', '', '![Median comparison; API and measurement limits apply.](comparison.png)', '',
        '| Workload | Kernix p50 / p95 (us) | ndarray p50 / p95 (us) | nalgebra p50 / p95 (us) | NumPy p50 / p95 (us) |',
        '|---|---:|---:|---:|---:|']
    for workload in sorted({key.split('/')[0] for key in results}):
        cells=[]
        for lib in ('kernix','ndarray','nalgebra','numpy'):
            v=results.get(f'{workload}/{lib}')
            cells.append(f"{v['p50_ns']/1000:.3f} / {v['p95_ns']/1000:.3f}" if v else 'not measured')
        lines.append('| '+workload+' | '+' | '.join(cells)+' |')
    lines += ['', '## Method and limits', '',
        'Operations allocate outputs except scalar sums and explicitly named matmul_reuse cases.',
        'Reuse cases time preallocated output writes in all libraries. Inputs are preallocated and',
        'contiguous. Kernix and ndarray share the matrixmultiply GEMM engine (no external BLAS); NumPy',
        'uses the build recorded in raw.json. BLAS/OMP thread counts are fixed to one.',
        f'CPU affinity: {affinity}. Warmed sequential samples on a shared laptop; CPU frequency',
        'and other system load are uncontrolled. p95 is the maximum of 15 batch means,',
        'not a per-request tail-latency guarantee. Different sum orders may round differently.',
        'The ndarray softmax is an explicitly coded stable rowwise comparison, not a native',
        'ndarray softmax API. NumPy timings include Python call overhead. No GPU comparison.',
        'Vision workloads, when present: ndarray uses explicit bilinear/conv loops, while',
        'NumPy uses vectorized bilinear interpolation and sliding-window einsum. No native',
        'convolution/resize API is implied for ndarray. RGB input is 1920x1080 packed u8;',
        'output is 1x3x224x224 f32 (602112 bytes), with no full-resolution float tensor.',
        'No claim that a complete Full HD model reaches 120 FPS follows from these measurements.',
        'The dense SGD case compares a validated reverse-mode Kernix graph against explicit',
        'analytical ndarray derivatives, including weight reset/copy in both. Graph lifetime,',
        'finite validation and parameter safety have costs; these are different API levels.',
        'LU solve includes factorization; QR materializes thin Q and R in all measured libraries.',
        'Construction cases produce owned dynamically shaped tensors/arrays in every library;',
        'they include allocation, filling/moving values and metadata validation. No fixed-size',
        'stack-only matrix is timed against a heap-owned tensor. Python overhead dominates small arrays.',
        'Historical reports were not pinned and lack source archives; their trends are indicative.',
        'source.tar.gz preserves the exact current source; SHA-256 is recorded in raw.json.',
        'raw.json contains all measurements, dependencies and environment metadata.', '']
    report=dest/'README.md'; report.write_text('\n'.join(lines))
    subprocess.run(['pandoc',str(report),'-o',str(dest/'report.pdf'),'--pdf-engine=xelatex',
                    '-V','geometry:margin=12mm,landscape','-V','fontsize=9pt','-V','colorlinks=true','--resource-path',str(dest)],check=True,cwd=ROOT)
    print(report.relative_to(ROOT))
if __name__ == '__main__': main()
