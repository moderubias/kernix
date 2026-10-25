#!/usr/bin/env python3
"""Measure an immutable clean revision; artifacts default to ignored local storage."""
import argparse
import hashlib
import io
import json
import math
import os
from pathlib import Path
import platform
import random
import shutil
import subprocess
import sys
import tarfile
import time

ROOT = Path(__file__).resolve().parents[1]
ACCELERATE_CASES = ['matmul_128_f64','matmul_256_f64','matmul_128_f32','matmul_256_f32']
CASES = {
    'solve_cholesky_128x1_f64':['kernix','nalgebra','faer'],
    'solve_cholesky_256x64_f64':['kernix','nalgebra','faer'],
    'solve_cholesky_512x128_f64':['kernix','nalgebra','faer'],
    'solve_lu_256x64_f64':['kernix','nalgebra','faer'],
    'cholesky_rank_update_256_f64':['kernix','refactor','nalgebra'],
    'cholesky_rank_downdate_256_f64':['kernix','refactor','nalgebra'],
    'cholesky_rank_update_512_f64':['kernix','refactor','nalgebra'],
    'matmul_transposed_128_f64':['kernix','numpy'],
    'matmul_transposed_128_f32':['kernix','numpy'],
    'qr_512x128_f64':['kernix','nalgebra','faer','numpy'],
    'qr_1024x256_f64':['kernix','nalgebra','faer','numpy'],
    'cholesky_factor_512_f64':['kernix','nalgebra','faer','numpy'],
    'matmul_ab_128_f64':['kernix','numpy'],
    'matmul_ab_128_f32':['kernix','numpy'],
    'parallel_matmul_into_64_f64':['kernix'],
    'parallel_matmul_into_128_f64':['kernix'],
    'parallel_matmul_into_256_f64':['kernix'],
    'parallel_matmul_into_128_f32':['kernix'],
    'tensordot_16x32x8_32x8x16_f64':['kernix','numpy'],
    'training_shared_backward_64x32_f64':['kernix','candle'],
    'matmul_batched_4x16x32x16_f64':['kernix','candle','numpy'],
    'matmul_batched_4x16x32x16_f32':['kernix','candle','numpy'],
    'training_forward_2x4x8x4_f64':['kernix','candle'],
    'training_record_2x4x8x4_f64':['kernix','candle'],
    'training_backward_2x4x8x4_f64':['kernix','candle'],
    'training_sgd_2x4x8x4_f64':['kernix'],
    'create_zeros_64x64_f32':['kernix','ndarray','faer','candle','numpy'],
    'create_ones_64x64_f32':['kernix','ndarray','faer','candle','numpy'],
    'matmul_32_f64': ['kernix','ndarray','nalgebra','faer','candle','numpy'],
    'matmul_64_f64': ['kernix','ndarray','nalgebra','faer','candle','numpy'],
    'matmul_128_f64': ['kernix','ndarray','nalgebra','faer','candle','numpy'],
    'matmul_256_f64': ['kernix','ndarray','nalgebra','faer','candle','numpy'],
    'matmul_64_f32': ['kernix','ndarray','nalgebra','faer','candle','numpy'],
    'matmul_128_f32': ['kernix','ndarray','nalgebra','faer','candle','numpy'],
    'matmul_256_f32': ['kernix','ndarray','nalgebra','faer','candle','numpy'],
    'pivoted_qr_96x32_f64':['kernix','nalgebra','faer'],
    'pivoted_qr_256x64_f64':['kernix','nalgebra','faer'],
    'min_norm_64x32_rank16_f64':['kernix','nalgebra','numpy'],
    'min_norm_32x64_rank32_f64':['kernix','nalgebra','numpy'],
    'qr_96x32_f64': ['kernix','nalgebra','faer','numpy'],
    'qr_256x64_f64': ['kernix','nalgebra','faer','numpy'],
    'cholesky_factor_64_f64': ['kernix','nalgebra','faer','numpy'],
    'cholesky_factor_128_f64': ['kernix','nalgebra','faer','numpy'],
    'cholesky_factor_256_f64': ['kernix','nalgebra','faer','numpy'],
    'softmax_256x64_f64': ['kernix','ndarray','candle','numpy'],
    'matmul_sum_backward_32x16x8_f64': ['kernix','candle'],
}


def command(*args, cwd=ROOT):
    return subprocess.check_output(args,cwd=cwd,text=True).strip()


def clean_revision(root=ROOT):
    if command('git','status','--porcelain','--untracked-files=normal',cwd=root):
        raise ValueError('benchmark requires a clean tracked and untracked working tree; commit changes first')
    return command('git','rev-parse','HEAD',cwd=root)


def measurement_schedule(seed, legacy_autograd=False, legacy_pivoted=False, legacy_rank_updates=False):
    schedule=[(workload,library) for workload,libraries in CASES.items()
              if not (legacy_autograd and workload.startswith('training_'))
              and not (legacy_pivoted and workload.startswith(('pivoted_qr_','min_norm_')))
              for library in libraries if not (legacy_rank_updates and workload.startswith('cholesky_rank_') and library=='kernix')]
    random.Random(seed).shuffle(schedule)
    return schedule


def distribution(samples):
    values=sorted(samples)
    if not values:
        raise ValueError('empty sample set')
    return dict(p50_ns=values[math.ceil(len(values)*0.50)-1],
                p95_ns=values[math.ceil(len(values)*0.95)-1],p99_ns=values[math.ceil(len(values)*0.99)-1],max_ns=values[-1],samples_ns=samples)


def parallel_affinity(allowed):
    """Prefer two distinct reported physical cores; never infer P/E class."""
    if not allowed:
        return None
    cores={}
    for cpu in sorted(allowed):
        topology=Path('/sys/devices/system/cpu')/f'cpu{cpu}'/'topology'
        try:
            key=tuple((topology/name).read_text().strip() for name in ['physical_package_id','core_id'])
        except OSError:
            key=('unknown',cpu)
        cores.setdefault(key,cpu)
    candidates=list(cores.values())
    if len(candidates)<2:
        candidates=sorted(allowed)
    return candidates[:2]


def peak_rss():
    try:
        for line in Path('/proc/self/status').read_text(encoding='utf-8').splitlines():
            if line.startswith('VmHWM:'):
                return int(line.split()[1])*1024
    except OSError:
        pass
    return None


def configure_numpy_threads(configuration, binaries):
    blas=configuration.get('Build Dependencies',{}).get('blas',{}).get('name','')
    if 'accelerate' not in blas.lower():
        return dict(method='backend environment variables',requested_threads=1)
    if sys.platform!='darwin':raise ValueError('Accelerate controller requires macOS')
    controller=binaries/'accelerate-threads.dylib'
    command('clang','-dynamiclib','-O2','-framework','Accelerate',
            str(ROOT/'benchmarks/accelerate_threads.c'),'-o',str(controller))
    os.environ['KERNIX_BENCH_ACCELERATE_CONTROLLER']=str(controller)
    apply_accelerate_policy()
    return dict(method='BLASSetThreading/BLASGetThreading',verified_threads=1,
                controller_source_sha256=hashlib.sha256((ROOT/'benchmarks/accelerate_threads.c').read_bytes()).hexdigest(),
                controller_sha256=hashlib.sha256(controller.read_bytes()).hexdigest())


def apply_accelerate_policy():
    if sys.platform!='darwin':return
    import ctypes
    controller=os.environ.get('KERNIX_BENCH_ACCELERATE_CONTROLLER')
    if not controller:raise ValueError('macOS NumPy worker requires an explicit Accelerate controller')
    library=ctypes.CDLL(controller)
    function=library.kernix_bench_single_thread
    function.argtypes=[];function.restype=ctypes.c_int
    if function()!=0:raise ValueError('Accelerate single-thread policy could not be verified')


def numpy_case(workload,mode):
    import numpy as np
    if sys.platform=='darwin' and 'accelerate' in np.show_config(mode='dicts')['Build Dependencies']['blas']['name'].lower():
        apply_accelerate_policy()
    if workload.startswith('create_'):
        one=workload.startswith('create_ones_');value=1.0 if one else 0.0
        operation=(lambda:np.ones((64,64),dtype=np.float32)) if one else (lambda:np.zeros((64,64),dtype=np.float32))
        def check(out):
            assert out.shape==(64,64) and out.dtype==np.float32 and np.all(out==value)
            return dict(max_absolute_error=0.0,budget=0.0)
    elif workload=='tensordot_16x32x8_32x8x16_f64':
        a=((np.arange(16*32*8)%23).astype(np.float64)/16-.5).reshape(16,32,8)
        b=((np.arange(32*8*16)%13).astype(np.float64)/8-.5).reshape(32,8,16)
        operation=lambda:np.tensordot(a,b,axes=([1,2],[0,1]))
        def check(out):
            av=a.reshape(16,256);bv=b.reshape(256,16)
            expected=np.array([[sum(float(av[i,k])*float(bv[k,j]) for k in range(256)) for j in range(16)] for i in range(16)])
            error=float(np.max(np.abs(out-expected)));assert error==0. and out.shape==(16,16)
            return dict(coordinate_max_absolute_error=error,budget=0.)
    elif workload in ['matmul_batched_4x16x32x16_f64','matmul_batched_4x16x32x16_f32']:
        dtype=np.float32 if workload.endswith('f32') else np.float64
        budget=1e-5 if dtype==np.float32 else 1e-11
        a=((np.arange(4*16*32)%13).astype(dtype)/13).reshape(4,16,32)
        b=((np.arange(32*16)%17).astype(dtype)/17).reshape(1,32,16)
        operation=lambda:a@b
        def check(out):
            expected=np.array([[[sum(float(a[t,i,k])*float(b[0,k,j]) for k in range(32)) for j in range(16)] for i in range(16)] for t in range(4)])
            error=np.abs(out-expected);assert out.dtype==dtype and out.shape==(4,16,16)
            assert np.all(error<=budget*(1+np.abs(expected)))
            return dict(max_absolute_error=float(error.max()),relative_plus_absolute_budget=budget)
    elif workload.startswith('matmul_transposed_'):
        dtype=np.float32 if workload.endswith('f32') else np.float64
        a=((np.arange(128*128)%23).astype(dtype)/16-.5).reshape(128,128)
        b=((np.arange(128*128)%13).astype(dtype)/8-.5).reshape(128,128)
        operation=lambda:a.T@b
        def check(out):
            expected=np.array([[sum(float(a[k,i])*float(b[k,j]) for k in range(128)) for j in range(128)] for i in range(128)])
            error=float(np.max(np.abs(out-expected)));assert error==0.0 and out.dtype==dtype
            return dict(coordinate_max_absolute_error=error,budget=0.0,borrowed_transpose=True)
    elif workload.startswith('matmul_ab_'):
        dtype=np.float32 if workload.endswith('f32') else np.float64
        a=((np.arange(128*128)%23).astype(dtype)/16-.5).reshape(128,128)
        b=((np.arange(128*128)%13).astype(dtype)/8-.5).reshape(128,128)
        operation=lambda:a@b
        def check(out):
            expected=np.array([[sum(float(a[i,k])*float(b[k,j]) for k in range(128)) for j in range(128)] for i in range(128)])
            error=float(np.max(np.abs(out-expected)));assert error==0. and out.dtype==dtype
            return dict(coordinate_max_absolute_error=error,budget=0.)
    elif workload.startswith('matmul_'):
        n=int(workload.split('_')[1]);dtype=np.float32 if workload.endswith('_f32') else np.float64
        a=((np.arange(n*n)%13).astype(dtype)/13).reshape(n,n)
        operation=lambda:a@a
        def check(out):
            # Ordered scalar oracle; no BLAS call before the first measured call.
            expected=np.array([[sum(float(a[i,k])*float(a[k,j]) for k in range(n)) for j in range(n)] for i in range(n)])
            error=np.abs(out-expected)
            budget=1e-5 if dtype==np.float32 else 1e-11
            assert np.all(error<=budget*(1+np.abs(expected)))
            assert out.dtype==dtype
            return dict(max_absolute_error=float(error.max()),relative_plus_absolute_budget=budget)
    elif workload.startswith('min_norm_'):
        m,n=map(int,workload.split('_')[2].split('x'));rank=int(workload.split('_')[3][4:])
        i,k=np.indices((m,rank));u=(i==k)*3+((i*7+k*3)%11).astype(np.float64)/16-5/16
        a=np.empty((m,n),dtype=np.float64);a[:,:rank]=u
        for j in range(rank,n):
            k=(j-rank)%rank;a[:,j]=u[:,k]+.5*u[:,(k+1)%rank]
        b=((np.arange(m*4)*11%29).astype(np.float64)-14).reshape(m,4)/8
        operation=lambda:np.linalg.lstsq(a,b,rcond=1e-12)[0]
        def check(out):
            normal=float(np.max(np.abs(a.T@(a@out-b))))
            null=max(float(np.max(np.abs(out[j]-out[(j-rank)%rank]-.5*out[((j-rank)%rank+1)%rank]))) for j in range(rank,n))
            assert normal<2e-10 and null<2e-10
            return dict(normal_residual_max_absolute_error=normal,null_space_orthogonality_max_absolute_error=null,budget=2e-10)
    elif workload.startswith('qr_'):
        m,n=map(int,workload.split('_')[1].split('x'));i=np.arange(m*n).reshape(m,n)
        a=((i*13%47).astype(np.float64)-23)/47+(i//n==i%n)*3
        operation=lambda:np.linalg.qr(a,mode='reduced')
        def check(out):
            q,r=out;reconstruction=np.max(np.abs(q@r-a));orthogonality=np.max(np.abs(q.T@q-np.eye(n)))
            assert np.all(np.abs(q@r-a)<2e-12*(1+np.abs(a))) and orthogonality<2e-12
            return dict(reconstruction_max_absolute_error=float(reconstruction),orthogonality_max_absolute_error=float(orthogonality),budget=2e-12)
    elif workload=='softmax_256x64_f64':
        a=((np.arange(256*64)%41).astype(np.float64)/4).reshape(256,64)
        def operation():
            e=np.exp(a-a.max(axis=1,keepdims=True))
            return e/e.sum(axis=1,keepdims=True)
        def check(out):
            error=0.0
            for row in range(256):
                values=[float(x) for x in a[row]];top=max(values)
                weights=[math.exp(x-top) for x in values];total=sum(weights)
                error=max(error,max(abs(float(out[row,col])-weights[col]/total) for col in range(64)))
            assert error<2e-14
            return dict(max_absolute_error=error,budget=2e-14)
    elif workload.startswith('cholesky_factor_'):
        n=int(workload.split('_')[2])
        row,col=np.indices((n,n));a=.01/(1+np.abs(row-col))+np.eye(n)*4
        operation=lambda:np.linalg.cholesky(a)
        def check(out):
            error=np.abs(out@out.T-a);assert np.all(error<2e-12*(1+np.abs(a)))
            return dict(reconstruction_max_absolute_error=float(error.max()),budget=2e-12)
    else:
        raise ValueError(workload)
    start=time.perf_counter_ns();first=operation();first_ns=time.perf_counter_ns()-start
    accuracy=check(first);del first
    if mode=='first':
        return dict(first_call_return_ns=first_ns,accuracy=accuracy)
    warmup_start=time.perf_counter_ns();warmup_calls=0
    while warmup_calls<8 or time.perf_counter_ns()-warmup_start<10_000_000:
        operation();warmup_calls+=1
    warmup_ns=time.perf_counter_ns()-warmup_start
    if mode=='resources':
        operation()
        return dict(warmup_calls=warmup_calls,warmup_ns=warmup_ns,rust_allocator=None,process_peak_rss_bytes=peak_rss(),accuracy=accuracy)
    calls=[]
    for _ in range(256):
        start=time.perf_counter_ns();operation();calls.append(time.perf_counter_ns()-start)
    batches=[];counts=[]
    for _ in range(15):
        start=time.perf_counter_ns();count=0
        while time.perf_counter_ns()-start<10_000_000:
            operation();count+=1
        batches.append((time.perf_counter_ns()-start)/count);counts.append(count)
    timers=[]
    for _ in range(256):
        start=time.perf_counter_ns();timers.append(time.perf_counter_ns()-start)
    return dict(first_call_return_ns=first_ns,warmup_calls=warmup_calls,warmup_ns=warmup_ns,per_call=distribution(calls),batch_means=distribution(batches),
                batch_call_counts=counts,timer_pair=distribution(timers),process_peak_rss_bytes=peak_rss(),accuracy=accuracy)


def snapshot(revision,dest):
    # Archive committed bytes, never scratch files or a potentially modified tree.
    paths=command('git','ls-tree','-r','--name-only',revision).splitlines()
    with tarfile.open(dest,'w:gz') as archive:
        for name in paths:
            if name.startswith(('benchmarks/results/','web/')):
                continue
            content=subprocess.check_output(['git','show',f'{revision}:{name}'],cwd=ROOT)
            info=tarfile.TarInfo(name);info.size=len(content);info.mode=0o644;info.mtime=0
            archive.addfile(info,io.BytesIO(content))
    return hashlib.sha256(dest.read_bytes()).hexdigest()


def environment(cpu):
    def read(path):
        try:return Path(path).read_text(encoding='utf-8').strip()
        except OSError:return None
    def capture(*args):
        try:return subprocess.check_output(args,text=True,stderr=subprocess.DEVNULL).strip()
        except (OSError,subprocess.CalledProcessError):return None
    hardware=(capture('lscpu','--json') if sys.platform.startswith('linux') else
              capture('sysctl','-n','machdep.cpu.brand_string') if sys.platform=='darwin' else
              capture('powershell','-NoProfile','-Command','(Get-CimInstance Win32_Processor).Name'))
    try:ram_bytes=os.sysconf('SC_PHYS_PAGES')*os.sysconf('SC_PAGE_SIZE')
    except (AttributeError,OSError,ValueError):ram_bytes=None
    cpu_info=read('/proc/cpuinfo') or ''
    model=next((line.split(':',1)[1].strip() for line in cpu_info.splitlines() if line.startswith('model name')),platform.processor())
    return dict(platform=platform.platform(),cpu_model=model,hardware_details=hardware,logical_cpus=os.cpu_count(),
                ram_bytes=ram_bytes,
                affinity_cpu=cpu,governor=read(f'/sys/devices/system/cpu/cpu{cpu}/cpufreq/scaling_governor'),
                scaling_driver=read(f'/sys/devices/system/cpu/cpu{cpu}/cpufreq/scaling_driver'),
                rustc=command('rustc','-Vv'),cargo=command('cargo','-V'),python=sys.version,
                rustflags=os.environ.get('RUSTFLAGS'),encoded_rustflags=os.environ.get('CARGO_ENCODED_RUSTFLAGS'),
                rustc_wrapper=os.environ.get('RUSTC_WRAPPER',''),
                thread_environment={key:os.environ[key] for key in ('RAYON_NUM_THREADS','OPENBLAS_NUM_THREADS','OMP_NUM_THREADS','MKL_NUM_THREADS')})


def render_details(dest,data):
    meta=data['metadata'];lines=['# Clean CPU evidence: '+meta['run_id'],'',
        f"Measured source `{meta['revision']}`, tree `{meta['tree']}`; dirty before/after: false.",
        f"Library source: {meta['library_source']}. CPU affinity: {meta['environment'].get('affinity_cpu') if meta['environment'].get('affinity_cpu') is not None else 'not pinned'}. Dtype explicitly named per workload; row-major inputs.",
        '', '| Workload / library | Per-call p50 / p95 (us) | Batch-mean p50 / p95 (us) | Rust allocations | Peak additional requested bytes | Process peak RSS (MiB) |',
        '|---|---:|---:|---:|---:|---:|']
    if meta.get('unavailable_workloads'):
        lines[5:5]=['Unavailable historical Kernix workloads: '+', '.join(meta['unavailable_workloads'])+'. '+meta['unavailable_reason']+'.', '']
    for key,value in sorted(data['results'].items()):
        call=value['timing']['per_call'];batch=value['timing']['batch_means'];resources=value['resources'];alloc=resources['rust_allocator']
        rss=resources['process_peak_rss_bytes'];rss=f'{rss/1048576:.2f}' if rss is not None else 'unavailable'
        lines.append(f"| {key} | {call['p50_ns']/1000:.3f} / {call['p95_ns']/1000:.3f} | {batch['p50_ns']/1000:.3f} / {batch['p95_ns']/1000:.3f} | {alloc['allocation_calls'] if alloc else 'unavailable'} | {alloc['peak_live_requested_bytes'] if alloc else 'unavailable'} | {rss} |")
    lines += ['', '## Method and limits', '',
        '- Rows explicitly marked two workers use a private Kernix executor with two workers and reusable output allocated before the first call. RefCell borrowing is included; pool construction is excluded. These rows compare Kernix versions, with no single-thread peer score. Linux uses up to two allowed CPUs, preferring distinct reported physical cores; Windows/macOS affinity is not pinned. Exact per-workload CPU sets are in explicit_execution metadata. Ordinary cases retain one BLAS/Rayon thread and the original single-CPU Linux policy.',
        '- The optional Kernix Accelerate variant is measured only on macOS 15+ when compiled with the explicit feature. SDK calls request one BLAS thread and restore the caller policy. Rust probes exclude native vendor workspace. Registry versions before 0.11 lack this variant; no substitute is measured as historical Kernix.',
        '- Coalesced tensordot contracts [16,32,8] with [32,8,16] over two adjacent axes using the actual Kernix/NumPy tensor APIs. Shared-variable backward uses a prepared real graph of sum(X*X), 64x32 f64, and fresh gradient stores; recording and input construction are excluded in both frameworks.',
        '- Per-call quantiles use nearest ranks over 256 individual warmed calls; batch quantiles use 15 means over separate 10-ms batches. They are separate distributions. Raw samples remain in raw.json.',
        '- Eight warmup calls follow validation. Ordinary timing includes allocation, dispatch, computation and output destruction; explicitly marked reusable-output cases reuse their fixture buffer. The first call field excludes output destruction. Inputs and library/runtime imports are prepared outside timing.',
        '- Three extra processes record their first operation calls. These are first-use observations, not cache-flushed or device-startup latency. Timer-pair overhead is reported without subtraction; tiny calls are clock-sensitive.',
        '- ndarray and Candle softmax are explicit stable compositions, not claims about a native softmax API.',
        '- Constructor cases allocate owned 64x64 f32 tensors. Pre-0.6 registry baselines use full(value), explicitly marked in metadata; current workspace runs use zeros/ones. No nested literal reinterpretation is involved.',
        '- All GEMM inputs are C-order and match elementwise. Native output layout differs by library. QR materializes thin Q and R in every implementation. Cholesky returns reusable factors (or NumPy L); no extra factor materialization is timed.',
        f"- Pivoted QR materializes thin Q/R and an owned permutation in Kernix, nalgebra and faer. Minimum-norm cases include factorization and four RHS: Kernix uses {meta.get('minimum_norm_algorithm', 'pivoted QR plus scaled right QR')} with relative cutoff 1e-12, NumPy uses native lstsq with rcond=1e-12, and nalgebra uses native SVD plus solve with absolute cutoff 1e-10. These algorithms have different costs and truncation policies. Fixtures have a wide rank gap; independent null-space orthogonality and normal-residual checks must both pass 2e-10 before timing. Results do not establish equivalence near a rank threshold or general SVD performance.",
        '- Kernix checks finite values/symmetry. Some competitors validate a narrower contract. No finite-data precondition check was removed to improve timing. Accuracy results and budgets are retained per worker.',
        '- The backward case uses an actual reverse-mode graph in Kernix and Candle for sum(A @ W), with A constant and W trainable. Both copy input/weight data and returned gradients; no manual derivative is used as a framework ranking.',
        '- Allocations are measured in a separately compiled instrumented binary after warmup, outside all timing loops. Counts cover Rust global-allocator requests, not native allocator internals, stack memory or all NumPy allocations. NumPy allocation counts are unavailable, not zero.',
        '- Native engines may retain thread-local packing buffers. Warmed allocation probes exclude first-use allocations and retained scratch; first-use timing and process RSS are separate observations. See the measured library numerical contract for that version.',
        '- Requested-byte peak is additional live Rust allocation during one call, including output; it is not scratch-only memory or peak RSS. RSS is the process high-water mark including fixtures, validation and runtime, read from Linux VmHWM. Other platforms may report unavailable.',
        '- The published cases are allocation-inclusive CPU APIs and graph workloads. Pure internal kernel-only and end-to-end application/model latency are not measured. No workload result implies a complete application frame rate.',
        '- faer is explicitly serial; Rayon and BLAS thread counts are one. No GPU comparison. Python/NumPy timings include Python dispatch. Backend and dependency versions/configuration, governor and RAM are recorded.',
        '- Training stages use batched matmul + bias + layer_norm + tanh + step-two feature slice + MSE (Candle index_select). Forward excludes graph recording; record includes parameter/input/target snapshots and eager forward/loss; backward reuses a prepared graph and returns fresh derivatives. Allocation probes measure the corresponding stage, including graph values/scratch, not graph-container-only bytes. SGD uses fixed precomputed real gradients and includes transactional validation/parameter updates; it is not an end-to-end converging training loop. Candle normalization is an explicit population-statistics composition; its checks/rounding differ. Historical registry baselines without these Tape operations omit the stages, recorded in metadata, rather than substitute manual derivatives.',
        '- Host load/frequency/cache effects are uncontrolled. Report comparator wins and losses; no universal ranking follows. The source archive comes from committed bytes, with result directories excluded. Library dependencies are pinned by the saved driver lockfile.', '']
    (dest/'DETAILS.md').write_text('\n'.join(lines), encoding='utf-8')


def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('run_id',nargs='?')
    parser.add_argument('--seed',type=int,default=20261022,help='seed for shuffled workload/library process order')
    parser.add_argument('--registry-baseline',help='pin all Kernix crates to this published version')
    parser.add_argument('--require-openblas',action='store_true')
    parser.add_argument('--worker',nargs=2,metavar=('WORKLOAD','MODE'),help=argparse.SUPPRESS)
    args=parser.parse_args()
    if args.worker:
        print(json.dumps(numpy_case(*args.worker)));return
    if not args.run_id or Path(args.run_id).name!=args.run_id or args.run_id in {'.','..'}:
        parser.error('run_id must be one path component')
    revision=clean_revision()
    accelerate = sys.platform=='darwin' and (not args.registry_baseline or tuple(map(int,args.registry_baseline.split('.'))) >= (0,11,0))
    if accelerate:
        for w in ACCELERATE_CASES: CASES[w].append('kernix-accelerate')
    os.environ.update(RAYON_NUM_THREADS='1',OPENBLAS_NUM_THREADS='1',OMP_NUM_THREADS='1',MKL_NUM_THREADS='1')
    import numpy as np
    configuration=np.show_config(mode='dicts')
    if args.require_openblas and 'openblas' not in str(configuration.get('Build Dependencies',{}).get('blas',{})).lower():
        raise ValueError('this run requires a NumPy OpenBLAS build')
    dest=ROOT/'web/benchmarks'/args.run_id;dest.mkdir(parents=True,exist_ok=False)
    manifest=ROOT/'benchmarks/harness/Cargo.toml'
    if args.registry_baseline:
        import re
        if not re.fullmatch(r'\d+\.\d+\.\d+',args.registry_baseline):raise ValueError('invalid baseline version')
        driver=ROOT/'web/drivers'/args.run_id;shutil.copytree(ROOT/'benchmarks/harness/src',driver/'src')
        text=manifest.read_text(encoding='utf-8');text=re.sub(r'kernix = \{[^\n]+',f'kernix = {{ version = "={args.registry_baseline}", default-features = false, features = ["std", "parallel"] }}',text)
        if tuple(map(int,args.registry_baseline.split('.'))) < (0,11,0):
            text=text.replace('accelerate = ["kernix/accelerate"]','accelerate = []')
        text=text.replace('[profile.release]',f'kernix-core = "={args.registry_baseline}"\nkernix-ops = "={args.registry_baseline}"\n\n[profile.release]')
        manifest=driver/'Cargo.toml';manifest.write_text(text, encoding='utf-8')
        shutil.copyfile(ROOT/'benchmarks/harness/Cargo.lock',manifest.with_name('Cargo.lock'))
        command('cargo','metadata','--format-version','1','--manifest-path',str(manifest))
    builds={}
    binaries=ROOT/'web/binaries'/args.run_id
    binaries.mkdir(parents=True,exist_ok=False)
    numpy_threads=configure_numpy_threads(configuration,binaries)
    legacy=bool(args.registry_baseline and tuple(map(int,args.registry_baseline.split('.'))) < (0,6,0))
    legacy_autograd=bool(args.registry_baseline and tuple(map(int,args.registry_baseline.split('.'))) < (0,7,0))
    legacy_pivoted=bool(args.registry_baseline and tuple(map(int,args.registry_baseline.split('.'))) < (0,9,0))
    legacy_rank_updates=bool(args.registry_baseline and tuple(map(int,args.registry_baseline.split('.'))) < (0,12,0))
    for mode,features in [('time',[]),('resources',['--features','allocation-counting'])]:
        enabled=(['accelerate'] if accelerate else [])+(['allocation-counting'] if mode=='resources' else [])+(['legacy-constructors'] if legacy else [])+(['legacy-autograd'] if legacy_autograd else [])+(['legacy-pivoted'] if legacy_pivoted else [])+(['legacy-rank-updates'] if legacy_rank_updates else [])
        features=['--features',','.join(enabled)] if enabled else []
        target=ROOT/'benchmarks/harness/target'
        command('cargo','build','--release','--locked','--manifest-path',str(manifest),'--target-dir',str(target),*features)
        built=target/'release'/('kernix-benchmark-harness'+('.exe' if os.name=='nt' else ''))
        builds[mode]=binaries/(mode+('.exe' if os.name=='nt' else ''))
        shutil.copy2(built,builds[mode])
    original_affinity=os.sched_getaffinity(0) if hasattr(os,'sched_getaffinity') else None
    cpu=min(original_affinity) if original_affinity else None
    parallel_cpus=parallel_affinity(original_affinity)
    if cpu is not None:os.sched_setaffinity(0,{cpu})
    fingerprints=[]
    for file in (ROOT/'benchmarks/harness/target/release/.fingerprint').glob('kernix-benchmark-harness-*/bin-kernix-benchmark-harness.json'):
        flags=json.loads(file.read_text(encoding='utf-8')).get('rustflags',[])
        if flags not in fingerprints:fingerprints.append(flags)
    packages=json.loads(command('cargo','metadata','--format-version','1','--locked','--manifest-path',str(manifest)))['packages']
    library_packages=[]
    for package in packages:
        if package['name'] in {'kernix','kernix-core','kernix-ops'}:
            vcs_file=Path(package['manifest_path']).with_name('.cargo_vcs_info.json')
            library_packages.append(dict(name=package['name'],version=package['version'],source=package['source'],
                vcs=json.loads(vcs_file.read_text(encoding='utf-8')) if vcs_file.exists() else None))
    kernix_version=next(p['version'] for p in library_packages if p['name']=='kernix')
    minimum_norm_algorithm=('pivoted QR plus scaled implicit RZ' if tuple(map(int,kernix_version.split('.'))) >= (0,10,0) else 'pivoted QR plus scaled right QR')
    metadata=dict(schema_version=2,unavailable_rank_modification_api=legacy_rank_updates,unavailable_observations=[w+"/kernix" for w in CASES if legacy_rank_updates and w.startswith("cholesky_rank_")],rank_modification_protocol="Kernix/nalgebra: prepared base factor, owned modified factor; refactor: same Kernix version, full factorization of preformed modified matrix; base setup and matrix formation excluded",minimum_norm_algorithm=minimum_norm_algorithm,accelerate_variant=accelerate,run_id=args.run_id,revision=revision,tree=command('git','rev-parse','HEAD^{tree}'),
        crates_tree=command('git','rev-parse',revision+':crates'),library_packages=library_packages,
        kernix_constructor_api=('full (legacy baseline)' if legacy else 'zeros/ones'),
        unavailable_workloads=[w for w in CASES if (legacy_autograd and w.startswith('training_')) or (legacy_pivoted and w.startswith(('pivoted_qr_','min_norm_')))],
        unavailable_reason=('registry lacks the corresponding Tape or pivoted/minimum-norm APIs; no substitute algorithms used as historical Kernix' if legacy_autograd or legacy_pivoted else None),
        effective_harness_rustflags=fingerprints,dirty_before=False,dirty_after=False,timestamp_utc=time.strftime('%Y-%m-%dT%H:%M:%SZ',time.gmtime()),
        library_source=('registry '+args.registry_baseline if args.registry_baseline else 'workspace '+revision),
        environment=environment(cpu),numpy_version=np.__version__,numpy_configuration=configuration,numpy_thread_policy=numpy_threads,
        explicit_execution={w:dict(workers=2,affinity_cpus=parallel_cpus,output='reused; allocated before first call',wrapper='RefCell borrow included; pool construction excluded') for w in CASES if w.startswith('parallel_matmul_into_')},
        dependency_tree=command('cargo','tree','--locked','--manifest-path',str(manifest)),
        binary_sha256={m:hashlib.sha256(p.read_bytes()).hexdigest() for m,p in builds.items()})
    schedule=measurement_schedule(args.seed,legacy_autograd,legacy_pivoted,legacy_rank_updates)
    metadata['measurement_order_seed']=args.seed
    metadata['measurement_order']=[w+'/'+lib for w,lib in schedule]
    results={}
    try:
        for workload,library in schedule:
            if original_affinity is not None:
                os.sched_setaffinity(0,set(parallel_cpus) if workload.startswith('parallel_matmul_into_') else {cpu})
            def worker(mode):
                if library=='numpy':return json.loads(command(sys.executable,str(Path(__file__).resolve()),'--worker',workload,mode))
                binary=builds['resources' if mode=='resources' else 'time']
                return json.loads(command(str(binary),workload,library,mode))
            timing=worker('time');resources=worker('resources')
            first=[worker('first')['first_call_return_ns'] for _ in range(3)]
            results[workload+'/'+library]=dict(timing=timing,resources=resources,fresh_process_first_calls=distribution(first))
            (dest/'partial.json').write_text(json.dumps(dict(metadata=metadata,results=results),indent=2)+'\n', encoding='utf-8')
    finally:
        if original_affinity is not None:os.sched_setaffinity(0,original_affinity)
    if clean_revision()!=revision:raise ValueError('source revision changed during measurement')
    metadata['source_sha256']=snapshot(revision,dest/'source.tar.gz')
    shutil.copyfile(manifest,dest/'driver.Cargo.toml');shutil.copyfile(manifest.with_name('Cargo.lock'),dest/'driver.Cargo.lock')
    data=dict(metadata=metadata,results=results)
    (dest/'raw.json').write_text(json.dumps(data,indent=2)+'\n', encoding='utf-8');(dest/'partial.json').unlink()
    render_details(dest,data)
    from report import render_report
    render_report(dest,data)
    print(dest/'README.md')


if __name__=='__main__':
    main()
