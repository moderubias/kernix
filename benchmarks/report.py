#!/usr/bin/env python3
"""Readable views of retained evidence; rendering never changes the measurements."""
import argparse
import html
import json
import re
from pathlib import Path

LIBRARIES = ('kernix', 'ndarray', 'nalgebra', 'faer', 'candle', 'numpy')
NAMES = dict(zip(LIBRARIES, ('Kernix', 'ndarray', 'nalgebra', 'faer', 'Candle', 'NumPy')))
COLORS = dict(zip(LIBRARIES, ('#5b47d6', '#2c8c99', '#bf8742', '#60884a', '#ca627f', '#577da8')))
WORKLOADS = {
    'parallel_matmul_into_64_f64': ('Matmul · 2 workers · reused output', '64 × 64 · 64 × 64', 'f64'),
    'parallel_matmul_into_128_f64': ('Matmul · 2 workers · reused output', '128 × 128 · 128 × 128', 'f64'),
    'parallel_matmul_into_256_f64': ('Matmul · 2 workers · reused output', '256 × 256 · 256 × 256', 'f64'),
    'parallel_matmul_into_128_f32': ('Matmul · 2 workers · reused output', '128 × 128 · 128 × 128', 'f32'),
    'tensordot_16x32x8_32x8x16_f64': ('Coalesced tensor contraction', '16 × 32 × 8 · 32 × 8 × 16', 'f64'),
    'training_shared_backward_64x32_f64': ('Shared-variable backward', 'sum(X * X), 64 × 32; prepared graph', 'f64'),
    'matmul_32_f64': ('Matrix multiplication', '32 × 32 · 32 × 32', 'f64'),
    'matmul_64_f64': ('Matrix multiplication', '64 × 64 · 64 × 64', 'f64'),
    'matmul_128_f64': ('Matrix multiplication', '128 × 128 · 128 × 128', 'f64'),
    'matmul_256_f64': ('Matrix multiplication', '256 × 256 · 256 × 256', 'f64'),
    'matmul_64_f32': ('Matrix multiplication', '64 × 64 · 64 × 64', 'f32'),
    'matmul_128_f32': ('Matrix multiplication', '128 × 128 · 128 × 128', 'f32'),
    'matmul_256_f32': ('Matrix multiplication', '256 × 256 · 256 × 256', 'f32'),
    'matmul_batched_4x16x32x16_f64': ('Batched multiplication', '4 × 16 × 32 · 1 × 32 × 16', 'f64'),
    'pivoted_qr_96x32_f64': ('Column-pivoted QR', '96 × 32', 'f64'),
    'pivoted_qr_256x64_f64': ('Column-pivoted QR', '256 × 64', 'f64'),
    'min_norm_64x32_rank16_f64': ('Minimum-norm least squares', '64 × 32, rank 16, 4 RHS', 'f64'),
    'min_norm_32x64_rank32_f64': ('Minimum-norm least squares', '32 × 64, rank 32, 4 RHS', 'f64'),
    'qr_96x32_f64': ('Thin QR', '96 × 32', 'f64'),
    'qr_256x64_f64': ('Thin QR', '256 × 64', 'f64'),
    'cholesky_factor_64_f64': ('Cholesky factorization', '64 × 64', 'f64'),
    'cholesky_factor_128_f64': ('Cholesky factorization', '128 × 128', 'f64'),
    'cholesky_factor_256_f64': ('Cholesky factorization', '256 × 256', 'f64'),
    'softmax_256x64_f64': ('Row softmax', '256 × 64', 'f64'),
    'create_zeros_64x64_f32': ('Create zeros', '64 × 64', 'f32'),
    'create_ones_64x64_f32': ('Create ones', '64 × 64', 'f32'),
    'matmul_sum_backward_32x16x8_f64': ('Record + backward', 'sum(A · W), 32 × 16 · 16 × 8', 'f64'),
    'training_forward_2x4x8x4_f64': ('Model inference', 'batched normalized/sliced MLP', 'f64'),
    'training_record_2x4x8x4_f64': ('Model recording + forward', 'batched normalized/sliced MLP', 'f64'),
    'training_backward_2x4x8x4_f64': ('Model backward', 'prepared graph → fresh gradients', 'f64'),
    'training_sgd_2x4x8x4_f64': ('Parameter update', 'transactional SGD, fixed gradients', 'f64'),
}


def cpu_name(environment):
    details=environment.get('hardware_details')
    if details:
        try:
            for row in json.loads(details).get('lscpu',[]):
                if row.get('field')=='Model name:':return row['data']
        except (ValueError,AttributeError):
            return details.strip()
    return environment.get('cpu_model') or 'CPU unspecified'


def label(workload):
    # Future workloads stay visible even before a presentation label is added.
    return WORKLOADS.get(workload, (workload.replace('_', ' '), '', ''))


def rows(data):
    grouped = {}
    for key, value in data['results'].items():
        workload, library = key.rsplit('/', 1)
        grouped.setdefault(workload, {})[library] = value
    order = list(WORKLOADS)
    return sorted(grouped.items(), key=lambda item: (order.index(item[0]) if item[0] in order else len(order), item[0]))


def micros(result, metric='p50_ns'):
    return result['timing']['per_call'][metric] / 1000


def throughput(workload, value):
    """Nominal 2*n^3 operations per allocation-inclusive GEMM latency, GFLOP/s."""
    match = re.fullmatch(r'matmul_(\d+)_f(?:32|64)', workload)
    if not match:
        return None
    ns = value['timing']['per_call']['p50_ns']
    return 2 * int(match[1])**3 / ns if ns > 0 else None


def change(current, previous):
    """Signed latency change: negative is faster. Never infer missing baselines."""
    if current is None or previous is None:
        return None
    old = micros(previous)
    return (micros(current) / old - 1) * 100 if old > 0 else None


def baseline_warnings(data, baseline):
    if baseline is None:
        return []
    a, b = data['metadata'], baseline['metadata']
    warnings = []
    for key in ('cpu_model', 'affinity_cpu', 'governor', 'thread_environment', 'rustc'):
        if a.get('environment', {}).get(key) != b.get('environment', {}).get(key):
            warnings.append('Different ' + key.replace('_', ' ') + '.')
    if a.get('effective_harness_rustflags') != b.get('effective_harness_rustflags'):
        warnings.append('Different effective compiler flags.')
    if a.get('numpy_configuration') != b.get('numpy_configuration'):
        warnings.append('Different NumPy backend configuration.')
    if a.get('numpy_thread_policy') != b.get('numpy_thread_policy'):
        warnings.append('Different NumPy thread-policy verification.')
    if a.get('explicit_execution') != b.get('explicit_execution'):
        warnings.append('Different explicit executor or affinity policies.')
    return warnings


def render_report(dest, data, baseline=None):
    """Write a compact Markdown/HTML report and comparable per-workload charts."""
    dest = Path(dest)
    dest.mkdir(parents=True, exist_ok=True)
    meta = data['metadata']
    groups = rows(data)
    libraries = [lib for lib in LIBRARIES if any(lib in values for _, values in groups)]
    libraries += sorted({lib for _, values in groups for lib in values} - set(libraries))
    escape = html.escape
    names = lambda lib: NAMES.get(lib, lib)
    version = next((p['version'] for p in meta.get('library_packages', []) if p['name'] == 'kernix'), 'workspace')
    env = meta.get('environment', {})
    description = f"{cpu_name(env)} · one BLAS/Rayon thread · allocating APIs unless marked reused · warmed individual calls"
    affinity = env.get('affinity_cpu')
    host_note = ('One measured host and one BLAS/Rayon thread. '
                 + (f'CPU affinity pinned to logical CPU {affinity}. ' if affinity is not None else 'CPU affinity was not pinned. ')
                 + 'Frequency and background load are uncontrolled; these observations establish no universal ranking.')
    if meta.get('explicit_execution'):
        host_note += ' Rows marked 2 workers use a private two-thread executor and reusable output; their recorded CPU affinity can differ. They compare Kernix versions, not single-thread competitors.'
        description += ' · explicit 2-worker rows labelled'
    warnings = baseline_warnings(data, baseline)
    summary = []
    table = []
    previous = baseline['results'] if baseline else {}
    for workload, values in groups:
        title, shape, dtype = label(workload)
        available = [micros(value) for value in values.values()]
        best = min(available)
        cells = []
        md = []
        for lib in libraries:
            value = values.get(lib)
            if value is None:
                cells.append('<td class="missing">—</td>');md.append('—');continue
            us = micros(value)
            fastest = us == best
            klass = 'best' if fastest else ''
            cells.append(f'<td class="{klass}" title="p95: {micros(value, "p95_ns"):.3f} µs">{us:.3f}</td>')
            md.append(f'**{us:.3f}**' if fastest else f'{us:.3f}')
        delta = change(values.get('kernix'), previous.get(workload + '/kernix'))
        if baseline:
            text = f'{delta:+.1f}%' if delta is not None else '—'
            cls = ('faster' if delta < 0 else 'slower') if delta is not None else 'missing'
            cells.append(f'<td class="{cls}">{text}</td>');md.append(text)
        table.append(f'<tr data-workload="{escape(workload)}"><th>{escape(title)}<small>{escape(shape)} · {escape(dtype)}</small></th>{"".join(cells)}</tr>')
        summary.append('| ' + ' · '.join(filter(None, (title, shape, dtype))) + ' | ' + ' | '.join(md) + ' |')
    columns = [names(lib) for lib in libraries] + (['Kernix Δ vs baseline'] if baseline else [])
    baseline_note = f"Baseline: {baseline['metadata']['run_id']}. Δ is latency change; negative is faster. Runs are sequential, not randomized/interleaved." if baseline else 'No previous-version baseline is paired with this view.'
    solver = meta.get('minimum_norm_algorithm', 'pivoted QR plus scaled right QR')
    numerical_note = (f'Minimum-norm rows include factorization and four RHS. Kernix uses {solver}; '
                      'NumPy uses native lstsq; nalgebra uses native SVD/solve. Fixtures have a wide rank gap; '
                      'null-space orthogonality and normal residual are validated independently. Algorithm costs '
                      'and rank policies differ; see DETAILS.md for thresholds. These are not general SVD comparisons.'
                      if any(w.startswith('min_norm_') for w, _ in groups) else '')
    md = [f'# Kernix {version} · CPU benchmarks', '', description, '',
          '**Microseconds per call, median. Lower is faster; bold marks the lowest observed median in each row.**', '',
          baseline_note, '', '| Workload | ' + ' | '.join(columns) + ' |',
          '|---|' + '---:|' * len(columns), *summary, '',
          '![Relative latency by workload](latency.png)', '',
          'Each panel has its own scale. Ratios are library / Kernix median; below 1 means the comparator was faster.', '',
          '## Reading these results', '',
          '- ' + host_note,
          '- Timing includes output allocation and destruction except rows explicitly marked reused. Inputs and imports are prepared outside timing. GEMM inputs are row-major; output layout and validation contracts can differ by library.',
          '- Per-call p95 describes 256 individual warmed calls. The 15 batch means form a separate distribution; neither is a confidence interval.',
          '- Rust allocation probes use a separate instrumented binary. Native engines may retain packing buffers; warmed probes exclude their first-use allocation and retention. NumPy native allocations are unavailable, never zero.', '',
          '[Interactive report](report.html) · [Detailed measurements and method](DETAILS.md) · [Raw samples and provenance](raw.json) · [Printable chart](latency.pdf)', '',
          f"Measured source `{meta['revision']}`. Run `{meta['run_id']}`. Rendering does not remeasure or modify raw evidence.", '']
    if warnings:
        md[4:4] = ['**Baseline caveats:** ' + ' '.join(warnings), '']
    if numerical_note:
        md[md.index('## Reading these results') + 2:md.index('## Reading these results') + 2] = [numerical_note, '']
    (dest / 'README.md').write_text('\n'.join(md), encoding='utf-8')
    details = []
    for workload, values in groups:
        title, shape, dtype = label(workload)
        lines = []
        for lib, value in values.items():
            timing, resources = value['timing'], value['resources']
            alloc = resources.get('rust_allocator')
            allocation = str(alloc['allocation_calls']) if alloc else 'unavailable'
            peak = f"{alloc['peak_live_requested_bytes']:,}" if alloc else 'unavailable'
            batch = timing['batch_means']
            tail = timing['per_call'].get('p99_ns')
            p99 = f'{tail/1000:.3f}' if tail is not None else 'unavailable'
            rate = throughput(workload, value)
            gflops = f'{rate:.2f}' if rate is not None else '—'
            lines.append(f'<tr><th>{escape(names(lib))}</th><td>{micros(value):.3f} / {micros(value,"p95_ns"):.3f} / {p99}</td><td>{gflops}</td><td>{batch["p50_ns"]/1000:.3f} / {batch["p95_ns"]/1000:.3f}</td><td>{allocation}</td><td>{peak}</td></tr>')
        details.append(f'<details><summary>{escape(title)} <span>{escape(shape)} · {escape(dtype)}</span></summary><div class="scroll"><table><thead><tr><th>Library</th><th>Call p50 / p95 / p99, µs</th><th>Nominal GEMM GFLOP/s</th><th>Batch mean p50 / p95, µs</th><th>Rust allocations</th><th>Peak requested bytes</th></tr></thead><tbody>{"".join(lines)}</tbody></table></div></details>')
    document = '''<!doctype html><html lang="en"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1">
<title>Kernix CPU benchmarks</title><style>
:root{color-scheme:light;--ink:#22263c;--muted:#626b80;--line:#e2e5ef;--purple:#5b47d6}*{box-sizing:border-box}body{margin:0;background:#f5f6fb;color:var(--ink);font:16px/1.6 system-ui,sans-serif}main{max-width:1320px;margin:auto;padding:48px 28px}header{margin-bottom:32px}.eyebrow{color:var(--purple);font-size:12px;letter-spacing:.15em;font-weight:750;text-transform:uppercase}h1{font-size:clamp(30px,4vw,48px);line-height:1.2;letter-spacing:-.04em;margin:12px 0}h2{font-size:23px;letter-spacing:-.02em}p{color:var(--muted)}.card{background:white;border:1px solid var(--line);border-radius:18px;padding:24px;margin:24px 0;box-shadow:0 4px 22px #25214a05}.scroll{overflow:auto}table{border-collapse:collapse;width:100%;font-size:14px;font-variant-numeric:tabular-nums}th,td{border-bottom:1px solid var(--line);padding:13px 12px;text-align:right;white-space:nowrap}th:first-child{text-align:left}thead th{color:var(--muted);font-size:12px;letter-spacing:.02em}tbody th{font-weight:600}small{display:block;color:var(--muted);font-size:12px;font-weight:400}.kernix{color:var(--purple)}.best{background:#f0edff;color:#4936b8;font-weight:750}.missing{color:#a3a8b8}.faster{color:#247552}.slower{color:#aa5a28}.note{font-size:13px;color:var(--muted)}.warning{color:#9b5c21}a{color:var(--purple);text-underline-offset:3px}input{font:inherit;border:1px solid var(--line);border-radius:8px;padding:10px 14px;width:min(100%,340px);margin:8px 0}img{max-width:100%;height:auto}details{border-top:1px solid var(--line);padding:14px 0}summary{cursor:pointer;font-weight:600}summary span{font-weight:400;color:var(--muted);font-size:13px;margin-left:12px}footer{font-size:13px;color:var(--muted);overflow-wrap:anywhere}@media print{body{background:white}main{padding:0}.card{box-shadow:none;break-inside:avoid}input{display:none}details{display:none}a{color:inherit}}
</style><main><header><div class="eyebrow">Kernix · performance evidence</div><h1>Numerics, measured.</h1><p>{{VERSION}}<br>{{DESCRIPTION}}</p></header>
<section class="card"><h2>At a glance</h2><p>Microseconds per call · median · lower is faster. Shaded cells mark the lowest observed median.</p><p class="note">{{BASELINE}}</p>{{WARNINGS}}<label for="search" class="note">Filter workloads</label><br><input id="search" type="search" placeholder="Try matmul, backward or f32" autocomplete="off"><div class="scroll"><table id="comparison"><thead><tr><th>Workload</th>{{COLUMNS}}</tr></thead><tbody>{{ROWS}}</tbody></table></div><p class="note">Hover over a timing for its per-call p95. Expand a workload for p99 and nominal GEMM throughput (2n³ divided by allocating API latency). A missing cell means unmeasured, not unsupported.</p></section>
<section class="card"><h2>The relative picture</h2><p>Each workload uses its own scale. Library / Kernix median: below 1 means the comparator was faster.</p><img src="latency.png" alt="Per-workload relative latency panels, with comparator wins and losses"><p class="note">{{HOST_NOTE}} GEMM inputs are row-major; output layout and validation contracts may differ. Timings include output allocation and destruction.</p></section>
<section class="card"><h2>Distributions &amp; resources</h2><p>256 individual warmed calls and 15 separate 10-ms batch means. Resource probes use a separate instrumented binary; NumPy native allocations are unavailable. Requested bytes include output and are not scratch-only memory or RSS. Native engines may retain thread-local packing memory; warmed probes exclude its first-use allocation and retention.</p>{{DETAILS}}</section>
<footer>{{NUMERICAL_NOTE}}<p><a href="DETAILS.md">Full method &amp; all resource fields</a> · <a href="raw.json">Raw samples &amp; provenance</a> · <a href="latency.pdf">Printable chart</a></p>{{PROVENANCE}}</footer></main><script>document.getElementById('search').addEventListener('input',function(){const q=this.value.toLowerCase();document.querySelectorAll('#comparison tbody tr').forEach(row=>{row.hidden=!(row.textContent+' '+row.dataset.workload).toLowerCase().includes(q)})})</script></html>'''
    replacements = dict(VERSION=escape('Kernix ' + version), DESCRIPTION=escape(description), BASELINE=escape(baseline_note),
                        WARNINGS='<p class="warning">' + escape(' '.join(warnings)) + '</p>' if warnings else '',
                        COLUMNS=''.join(f'<th class="{"kernix" if col == "Kernix" else ""}">{escape(col)}</th>' for col in columns),
                        ROWS=''.join(table), DETAILS=''.join(details), HOST_NOTE=escape(host_note), NUMERICAL_NOTE='<p>'+escape(numerical_note)+'</p>' if numerical_note else '',
                        PROVENANCE=escape(f"Measured source {meta['revision']} · run {meta['run_id']}. Rendering does not change or remeasure raw evidence."))
    # Single pass: replacement text containing another placeholder stays literal.
    import re
    document = re.sub(r'\{\{([A-Z_]+)\}\}', lambda m: replacements[m[1]], document)
    (dest / 'report.html').write_text(document, encoding='utf-8')
    render_chart(dest, groups)


def render_chart(dest, groups):
    import matplotlib
    matplotlib.use('Agg')
    import matplotlib.pyplot as plt
    import math
    fig, axes = plt.subplots(math.ceil(len(groups)/2), 2, figsize=(13, math.ceil(len(groups)/2)*2.65), layout='constrained', squeeze=False)
    fig.set_facecolor('#f5f6fb')
    for axis, (workload, values) in zip(axes.flat, groups):
        title, shape, dtype = label(workload)
        reference = values.get('kernix')
        if reference is None:
            axis.set_visible(False);continue
        base = micros(reference)
        libraries = [lib for lib in LIBRARIES if lib in values]
        libraries += sorted(set(values) - set(libraries))
        ratios = [micros(values[lib])/base for lib in libraries]
        axis.barh(range(len(libraries)), ratios, color=[COLORS.get(lib, '#626b80') for lib in libraries], height=.65)
        axis.set_yticks(range(len(libraries)), [NAMES.get(lib, lib) for lib in libraries], fontsize=9)
        axis.invert_yaxis();axis.axvline(1, color='#6b6580', lw=.7, ls='--')
        axis.set_xlim(0, max(ratios)*1.35)
        for i, (lib, ratio) in enumerate(zip(libraries, ratios)):
            axis.text(ratio + max(ratios)*.02, i, f'{micros(values[lib]):.2f} µs · {ratio:.2f}×', va='center', fontsize=8, color='#34364a')
        axis.set_title(f'{title} · {shape} · {dtype}', loc='left', fontsize=10, weight='bold', pad=12)
        axis.set_xlabel('Latency / Kernix', fontsize=8, color='#626b80');axis.tick_params(length=0, labelsize=8)
        axis.spines[['top', 'right', 'left']].set_visible(False);axis.spines['bottom'].set_color('#dadce8')
        axis.grid(axis='x', alpha=.15);axis.set_axisbelow(True)
    for axis in list(axes.flat)[len(groups):]:axis.set_visible(False)
    fig.savefig(dest/'latency.png', dpi=160);fig.savefig(dest/'latency.pdf');plt.close(fig)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('directory', type=Path, help='directory containing raw.json; only presentation files are rewritten')
    parser.add_argument('--baseline', type=Path, help='directory containing an exact previous-version raw.json')
    args = parser.parse_args()
    data = json.loads((args.directory/'raw.json').read_text(encoding='utf-8'))
    baseline = json.loads((args.baseline/'raw.json').read_text(encoding='utf-8')) if args.baseline else None
    from evidence import render_details
    render_details(args.directory, data)
    render_report(args.directory, data, baseline)


if __name__ == '__main__':
    main()
