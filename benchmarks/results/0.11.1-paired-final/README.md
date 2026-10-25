# Kernix 0.11.1 · paired process comparison

9 fresh-process pairs per workload. Ratios below 1 mean less time.

[Interactive HTML (open locally)](report.html) · [Vector PDF](latency.pdf) · [Original process evidence](paired.json)

[![Paired process comparison preview](latency.png)](latency.pdf)


### Version changes at a glance

The vertical line marks unchanged time. Left is faster; right is slower. Ranges are exploratory 95% intervals over fresh-process pairs. An interval crossing the line leaves direction unclear.

![Matrix multiplication. Current divided by baseline; below 1 is faster.](charts/paired-matmul-1.png)

![Matrix multiplication. Current divided by baseline; below 1 is faster.](charts/paired-matmul-2.png)

![Explicit parallel execution. Current divided by baseline; below 1 is faster.](charts/paired-parallel-1.png)

![Decompositions & solvers. Current divided by baseline; below 1 is faster.](charts/paired-algebra-1.png)

![Training & autodiff. Current divided by baseline; below 1 is faster.](charts/paired-training-1.png)

## Exact process ratios

| Workload | Current / baseline [95% interval] | Interpretation |
|---|---:|---|
| Matrix multiplication · 32 × 32 · 32 × 32 · f64 | 0.9904882688649335× [0.9548349739432542, 1.0424050632911392] | Timing direction unclear |
| Matrix multiplication · 128 × 128 · 128 × 128 · f64 | 1.012244500410469× [0.996820129959906, 1.0277707380187007] | Timing direction unclear |
| Matrix multiplication · 128 × 128 · 128 × 128 · f32 | 1.0019341832842976× [0.9540607054963085, 1.0580849852211405] | Timing direction unclear |
| Matrix multiplication · 256 × 256 · 256 × 256 · f64 | 0.9959523669853934× [0.9901368615535494, 1.0083672209329557] | Timing direction unclear |
| Matrix multiplication · 256 × 256 · 256 × 256 · f32 | 0.9991795672844394× [0.9917177400090559, 1.0197192236299086] | Timing direction unclear |
| Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64 | 0.9321063394683027× [0.9020817270624518, 0.9764801297648013] | Observed time reduction |
| Independent matrix operands · 128 × 128 · 128 × 128 · f64 | 0.9973845772650938× [0.9513612643104681, 1.0198856125019133] | Timing direction unclear |
| Independent matrix operands · 128 × 128 · 128 × 128 · f32 | 0.9919104759336659× [0.971837475943946, 1.0259382052210737] | Timing direction unclear |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64 | 0.7574906367041199× [0.7263590680676107, 0.8196382428940568] | Observed time reduction |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f32 | 0.7099767981438515× [0.6951219512195121, 0.7725225225225225] | Observed time reduction |
| Cholesky factorization · 128 × 128 · f64 | 0.9840000693012699× [0.9571668921497207, 1.0007533641606483] | Timing direction unclear |
| Cholesky factorization · 256 × 256 · f64 | 0.9955355668450756× [0.9890255559008289, 1.0107981826317975] | Timing direction unclear |
| Cholesky factorization · 512 × 512 · f64 | 1.0037438332284923× [0.9868867083576638, 1.0199812625802072] | Timing direction unclear |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f64 | 0.9707103267549454× [0.5377926771777277, 0.9987317073170732] | Observed time reduction |
| Matmul · 2 workers · reused output · 256 × 256 · 256 × 256 · f64 | 0.9909823528068854× [0.9644791506637074, 1.0365048026342756] | Timing direction unclear |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f32 | 0.9912084047650446× [0.5711984041736996, 1.7238628349788434] | Timing direction unclear |

## Method

95% percentile bootstrap over paired process blocks; 10000 resamples; exploratory, no multiple-comparison adjustment

Current `0.11.1-final`; baseline `0.11.0-registry-0111`. All source ratio digits, paired observations and process ordering are preserved. No absolute peer latencies are inferred.
