"""Readable views of immutable evidence, separating inference from observations."""
import hashlib
import html
import json
import math
from pathlib import Path
import re
import subprocess

FAMILIES = {'matmul':'Matrix multiplication','parallel':'Explicit parallel execution',
    'algebra':'Decompositions and solvers','training':'Training and autodiff',
    'contraction':'Tensor contractions','statistics':'Statistics and activations',
    'construction':'Tensor construction','other':'Other workloads'}

def family(w):
    if w.startswith('parallel_'): return 'parallel'
    if w.startswith(('matmul_sum_backward_', 'training_')): return 'training'
    if w.startswith('matmul_'): return 'matmul'
    if w.startswith(('cholesky_', 'qr_', 'pivoted_qr_', 'min_norm_')): return 'algebra'
    if w.startswith('tensordot_'): return 'contraction'
    if w.startswith('create_'): return 'construction'
    if w.startswith('softmax_'): return 'statistics'
    return 'other'

def contract(w):
    if w.startswith('parallel_'): return '2 workers · reused output · pool preparation excluded'
    if w.startswith(('training_shared_backward_','training_backward_')): return '1 worker · prepared graph · fresh gradients'
    if w.startswith('min_norm_'): return '1 worker · 4 RHS · factorization included · algorithms differ'
    if w.startswith('matmul_batched_'): return '1 worker · allocated output · broadcast batches'
    if w.startswith('matmul_ab_'): return '1 worker · allocated output · independent operands'
    if w.startswith('matmul_'): return '1 worker · allocated output · self product'
    return '1 worker · allocated output' if not w.startswith('training_') else '1 worker · preparation as documented'

def paired_results(data, baseline, paired, warnings):
    if paired is None: return {}
    for key in ['run_id','revision','binary_sha256']:
        if paired['current'].get(key)!=data['metadata'].get(key):
            raise ValueError('paired evidence does not belong to the selected current run')
        if baseline and paired['baseline'].get(key)!=baseline['metadata'].get(key):
            raise ValueError('paired evidence does not belong to the selected baseline run')
    if warnings: raise ValueError('paired evidence has incompatible environments')
    for key,p in paired['results'].items():
        r,lo,hi=(p[k] for k in ['median_ratio','ci95_low','ci95_high'])
        if not all(math.isfinite(x) and x>0 for x in [r,lo,hi]) or not lo<=r<=hi:
            raise ValueError('invalid paired ratio or interval')
        if key+'/kernix' not in data['results']: raise ValueError('paired workload absent from current run')
    return paired['results']

def verdict(p):
    if p is None: return 'No paired comparison', 'neutral'
    if p['ci95_high']<1: return 'Observed time reduction', 'improved'
    if p['ci95_low']>1: return 'Observed time increase', 'regressed'
    return 'Timing direction unclear', 'neutral'

def ratio_text(p):
    return '—' if p is None else f"{p['median_ratio']:.3f}× [{p['ci95_low']:.3f}, {p['ci95_high']:.3f}]"

def render(dest,data,baseline,paired,h):
    esc=html.escape;meta=data['metadata'];groups=h['rows'](data)
    warnings=h['baseline_warnings'](data,baseline or ({'metadata':paired['baseline']} if paired else None))
    ratios=paired_results(data,baseline,paired,warnings)
    label=h['label'];micros=h['micros'];names=h['names'];cpu=h['cpu']
    libs=[lib for lib in h['libraries'] if any(lib in v for _,v in groups)]
    libs+=sorted({lib for _,v in groups for lib in v}-set(libs))
    version=next((p['version'] for p in meta.get('library_packages',[]) if p['name']=='kernix'),'workspace')
    env=meta.get('environment',{});aff=env.get('affinity_cpu')
    host=(f'CPU affinity pinned to logical CPU {aff}. ' if aff is not None else 'CPU affinity was not pinned. ')
    host+='Frequency and background load are uncontrolled; observations establish no universal ranking.'
    blas=meta.get('numpy_configuration',{}).get('Build Dependencies',{}).get('blas',{}).get('name','not recorded')
    solver=meta.get('minimum_norm_algorithm','pivoted QR plus scaled right QR')
    numerical=(f'Minimum-norm rows use {solver} in Kernix; NumPy native lstsq; nalgebra native SVD/solve. Algorithm costs and rank policies differ. Normal residuals and null-space orthogonality are independently checked. See DETAILS.md for thresholds.' if any(w.startswith('min_norm_') for w,_ in groups) else '')
    baseline_id=(baseline or {}).get('metadata',{}).get('run_id') or (paired or {}).get('baseline',{}).get('run_id')
    note=(f'Baseline: {baseline_id}. Sequential run changes are descriptive, not randomized/interleaved process comparisons.' if baseline_id else 'No previous-version baseline is paired with this view.')
    if warnings: note+=' Version deltas are disabled because environments differ.'
    md=[f'# Kernix {version} · CPU benchmarks','',f'{cpu} · warmed public API calls · conditions per workload','',
        '**Microseconds per call, median. Lower is faster. Bold marks a lowest observed median only among at least two measured libraries.**','',note,'',
        '| Workload | '+' | '.join(names(lib) for lib in libs)+(' | Sequential run change' if baseline else '')+' |',
        '|---|'+'---:|'*(len(libs)+(1 if baseline else 0))]
    peer_rows=[];version_rows=[];details=[];plots=[];losses=[]
    for w,values in groups:
        title,shape,dtype=label(w);caption=' · '.join(filter(None,[title,shape,dtype]))
        search=esc((w+' '+caption).lower(),quote=True)
        attrs=f'data-workload="{esc(w)}" data-family="{family(w)}" data-search="{search}"'
        times=[micros(v) for v in values.values()]
        if not all(math.isfinite(x) and x>=0 for x in times): raise ValueError('invalid measured latency')
        best=min(times);cells=[];md_cells=[]
        for lib in libs:
            v=values.get(lib)
            if v is None:cells.append('<td class="missing">—</td>');md_cells.append('—');continue
            us=micros(v);winner=len(values)>1 and us==best;kind='best' if winner else ''
            cells.append(f'<td class="{kind}">{us:.3g}</td>')
            md_cells.append(f'**{us:.3f}**' if winner else f'{us:.3f}')
        if baseline:
            delta=h['change'](values.get('kernix'),baseline['results'].get(w+'/kernix')) if not warnings else None
            md_cells.append(f'{delta:+.1f}%' if delta is not None else '—')
        md.append('| '+caption+' | '+' | '.join(md_cells)+' |')
        badge='No peer comparison measured' if len(values)==1 else 'Observed API latency · validation policies may differ'
        heading=f'<th scope="row">{esc(title)}<small>{esc(shape)} · {esc(dtype)}</small><small>{esc(contract(w))}</small></th>'
        peer_rows.append(f'<tr {attrs}>{heading}'+''.join(cells)+f'<td class="status">{badge}</td></tr>')
        p=ratios.get(w);status,kind=verdict(p)
        version_rows.append(f'<tr {attrs}>{heading}<td>{ratio_text(p)}</td><td class="status {kind}">{status}</td></tr>')
        k=values.get('kernix');others={lib:v for lib,v in values.items() if not lib.startswith('kernix')}
        if k and others:
            lib,v=min(others.items(),key=lambda item:micros(item[1]))
            if micros(v)<micros(k):losses.append((w,caption,lib,micros(k),micros(v)))
        resource=[]
        for lib,v in values.items():
            t=v['timing'];a=v['resources'].get('rust_allocator');q=t['per_call'];batch=t['batch_means'];tail=q.get('p99_ns')
            nums=[f'{q["p50_ns"]/1000:.3g}',f'{q["p95_ns"]/1000:.3g}',f'{tail/1000:.3g}' if tail is not None else 'unavailable',f'{batch["p50_ns"]/1000:.3g}',str(a['allocation_calls']) if a else 'unavailable',format(a['peak_live_requested_bytes'],',') if a else 'unavailable']
            resource.append(f'<tr><th scope="row">{esc(names(lib))}</th>'+''.join('<td>'+x+'</td>' for x in nums)+'</tr>')
        headers=['Library','Call median','Call p95','Call p99','Batch mean median','Allocations','Peak additional requested bytes']
        details.append(f'<details {attrs}><summary>{esc(caption)}</summary><p>{esc(contract(w))}. First API call is not a cold-cache measurement.</p><div class="scroll"><table><caption>Timing in µs and warmed Rust allocation probes</caption><thead><tr>'+''.join(f'<th scope="col">{x}</th>' for x in headers)+'</tr></thead><tbody>'+''.join(resource)+'</tbody></table></div></details>')
        if k and micros(k)>0:
            maximum=max(times);bars=[]
            for lib,v in values.items():
                us=micros(v);width=100*us/maximum
                bars.append(f'<div class="bar-row"><span>{esc(names(lib))}</span><div class="bar-track"><i style="width:{width:.3f}%" class="bar {"kernix" if lib.startswith("kernix") else ""}"></i></div><span>{us:.3g} µs · {us/micros(k):.2f}×</span></div>')
            plots.append(f'<article class="plot" {attrs}><h3>{esc(title)}</h3><p>{esc(shape)} · {esc(dtype)}<br>{esc(contract(w))}</p><p class="note">{badge}. Scale is local to this workload; ratios use portable Kernix as baseline.</p>'+''.join(bars)+'</article>')
    count=f'{len(ratios)} of {len(groups)} workloads have paired evidence.'
    md+=['','## Version comparison','',count+' Intervals resample process pairs, not calls. Exploratory; no multiple-comparison adjustment.','', '| Workload | Paired latency ratio [95% interval] | Interpretation |','|---|---:|---|']
    for w,_ in groups:md.append('| '+' · '.join(filter(None,label(w)))+' | '+ratio_text(ratios.get(w))+' | '+verdict(ratios.get(w))[0]+' |')
    md+=['','## Method and limits','',host,'',numerical,'','Ordinary allocating rows include output allocation and destruction. Explicit two-worker rows reuse output and exclude pool preparation. NumPy native allocations are unavailable, never zero. Warmed probes exclude first-use native workspace and retention.','',
        '[Interactive report](report.html) · [Full method](DETAILS.md) · [Raw samples](raw.json) · [Printable family charts](latency.pdf)','',
        'Measured source '+chr(96)+meta['revision']+chr(96)+'. Run '+chr(96)+meta['run_id']+chr(96)+'. Rendering does not change measurements.','']
    (dest/'README.md').write_text('\n'.join(md),encoding='utf-8')
    cards=[]
    for w in ['matmul_128_f64','matmul_128_f32','cholesky_factor_256_f64','cholesky_factor_512_f64']:
        if w not in ratios: continue
        p=ratios[w];status,kind=verdict(p);title,shape,dtype=label(w)
        cards.append(f'<article class="insight {kind}"><h3>{esc(title)} · {esc(shape)} · {esc(dtype)}</h3><strong>{status}</strong><p>Time {p["median_ratio"]:.3f}× baseline<br>95% interval [{p["ci95_low"]:.3f}, {p["ci95_high"]:.3f}]</p><small>{esc(contract(w))}</small></article>')
    if not cards: cards=['<p>No paired version comparison is attached. The library table shows host observations; a minimum median alone does not establish a repeatable lead.</p>']
    selected=[x for w in ['matmul_128_f64','matmul_128_f32','cholesky_factor_256_f64'] for x in losses if x[0]==w] or losses[:3]
    loss_html=''.join(f'<li><strong>{esc(caption)}</strong>: Kernix {kt:.3g} µs; {esc(names(lib))} {pt:.3g} µs.</li>' for _,caption,lib,kt,pt in selected) or '<li>No faster external peer median among the summary rows. See all workloads for coverage and limits.</li>'
    base=Path(__file__).parent
    tokens=dict(VERSION=esc(version),CSS=(base/'report_style.css').read_text(encoding='utf-8'),HAS_PAIRED=str(bool(ratios)).lower(),CPU=esc(cpu),HOST=esc(host),BASELINE=esc(note),PAIRED_COUNT=count,
        FACTS=f'<span>{len(groups)} workloads</span><span>{len(data["results"])} library/workload observations</span><span>NumPy BLAS: {esc(str(blas))}</span>',INSIGHTS=''.join(cards),LOSSES=loss_html,
        FAMILIES='<option value="">All workload families</option>'+''.join(f'<option value="{k}">{v}</option>' for k,v in FAMILIES.items() if any(family(w)==k for w,_ in groups)),
        VERSIONS=''.join(version_rows),COLUMNS=''.join(f'<th scope="col">{esc(names(lib))}</th>' for lib in libs),
        ROWS=''.join(peer_rows),PLOTS=''.join(plots),DETAILS=''.join(details),NUMERICAL=esc(numerical),
        WARNINGS='<p class="warning">'+esc(' '.join(warnings))+'</p>' if warnings else '',
        PROVENANCE=esc(f"Measured source {meta['revision']} · run {meta['run_id']}. Rendering does not change or remeasure raw evidence."))
    doc=re.sub(r'\{\{([A-Z_]+)\}\}',lambda m:tokens[m[1]],(base/'report_frontend.html').read_text(encoding='utf-8'))
    (dest/'report.html').write_text(doc,encoding='utf-8')
    files=['report.py','presentation.py','report_frontend.html','report_style.css']
    try:revision=subprocess.check_output(['git','rev-parse','HEAD'],cwd=base,text=True,stderr=subprocess.DEVNULL).strip()
    except (OSError,subprocess.CalledProcessError):revision=None
    receipt=dict(measured_revision=meta['revision'],renderer_revision=revision,renderer_sha256={f:hashlib.sha256((base/f).read_bytes()).hexdigest() for f in files},input_sha256=hashlib.sha256(json.dumps(data,sort_keys=True).encode()).hexdigest(),baseline_canonical_sha256=hashlib.sha256(json.dumps(baseline,sort_keys=True).encode()).hexdigest() if baseline else None,paired_canonical_sha256=hashlib.sha256(json.dumps(paired,sort_keys=True).encode()).hexdigest() if paired else None,paired_evidence_attached=bool(ratios),raw_modified=False)
    (dest/'presentation.json').write_text(json.dumps(receipt,indent=2)+'\n',encoding='utf-8')
