#!/usr/bin/env python3
"""Readable views of retained evidence; rendering never changes the measurements."""

import argparse
import json
import re
from pathlib import Path

LIBRARIES = (
    "kernix",
    "kernix-accelerate",
    "ndarray",
    "nalgebra",
    "faer",
    "candle",
    "numpy",
)
NAMES = dict(
    kernix="Kernix",
    ndarray="ndarray",
    nalgebra="nalgebra",
    faer="faer",
    candle="Candle",
    numpy="NumPy",
)
NAMES["kernix-accelerate"] = "Kernix · explicit Accelerate"
COLORS = dict(
    kernix="#5b47d6",
    ndarray="#2c8c99",
    nalgebra="#bf8742",
    faer="#60884a",
    candle="#ca627f",
    numpy="#577da8",
)
COLORS["kernix-accelerate"] = "#8b72d6"
WORKLOADS = {
    "add_65536_f64": ("Elementwise addition", "65,536 elements", "f64"),
    "sum_65536_f64": ("Sum reduction", "65,536 elements", "f64"),
    "cholesky_64_f64": ("Cholesky factorization", "64 × 64", "f64"),
    "solve_64_f64": ("Linear system solve", "64 × 64", "f64"),
    "qr_8x4_f64": ("Thin QR", "8 × 4", "f64"),
    "qr_32x16_f64": ("Thin QR", "32 × 16", "f64"),
    "matmul_reuse_32_f64": (
        "Matrix multiplication · reused output",
        "32 × 32 · 32 × 32",
        "f64",
    ),
    "matmul_reuse_128_f64": (
        "Matrix multiplication · reused output",
        "128 × 128 · 128 × 128",
        "f64",
    ),
    "rgb1080_to224_f32": (
        "RGB resize & normalization",
        "1920 × 1080 → 1 × 3 × 224 × 224",
        "u8 → f32",
    ),
    "conv_1x3x32x32_8x3x3x3_f32": (
        "2D convolution",
        "1 × 3 × 32 × 32 · 8 × 3 × 3 × 3",
        "f32",
    ),
    "create_full_64x64_f64": ("Create filled tensor", "64 × 64", "f64"),
    "create_matrix_3x3_f64": ("Create matrix", "3 × 3", "f64"),
    "dense_sgd_step_32x16x8_f64": ("Dense training step", "32 × 16 × 8", "f64"),
    "cholesky_factor_512_f64": ("Cholesky factorization", "512 × 512", "f64"),
    "matmul_ab_128_f64": (
        "Independent matrix operands",
        "128 × 128 · 128 × 128",
        "f64",
    ),
    "matmul_ab_128_f32": (
        "Independent matrix operands",
        "128 × 128 · 128 × 128",
        "f32",
    ),
    "parallel_matmul_into_64_f64": (
        "Matmul · 2 workers · reused output",
        "64 × 64 · 64 × 64",
        "f64",
    ),
    "parallel_matmul_into_128_f64": (
        "Matmul · 2 workers · reused output",
        "128 × 128 · 128 × 128",
        "f64",
    ),
    "parallel_matmul_into_256_f64": (
        "Matmul · 2 workers · reused output",
        "256 × 256 · 256 × 256",
        "f64",
    ),
    "parallel_matmul_into_128_f32": (
        "Matmul · 2 workers · reused output",
        "128 × 128 · 128 × 128",
        "f32",
    ),
    "tensordot_16x32x8_32x8x16_f64": (
        "Coalesced tensor contraction",
        "16 × 32 × 8 · 32 × 8 × 16",
        "f64",
    ),
    "training_shared_backward_64x32_f64": (
        "Shared-variable backward",
        "sum(X * X), 64 × 32; prepared graph",
        "f64",
    ),
    "matmul_32_f64": ("Matrix multiplication", "32 × 32 · 32 × 32", "f64"),
    "matmul_64_f64": ("Matrix multiplication", "64 × 64 · 64 × 64", "f64"),
    "matmul_128_f64": ("Matrix multiplication", "128 × 128 · 128 × 128", "f64"),
    "matmul_256_f64": ("Matrix multiplication", "256 × 256 · 256 × 256", "f64"),
    "matmul_64_f32": ("Matrix multiplication", "64 × 64 · 64 × 64", "f32"),
    "matmul_128_f32": ("Matrix multiplication", "128 × 128 · 128 × 128", "f32"),
    "matmul_256_f32": ("Matrix multiplication", "256 × 256 · 256 × 256", "f32"),
    "matmul_batched_4x16x32x16_f64": (
        "Batched multiplication",
        "4 × 16 × 32 · 1 × 32 × 16",
        "f64",
    ),
    "matmul_batched_4x16x32x16_f32": (
        "Batched multiplication",
        "4 × 16 × 32 · 1 × 32 × 16",
        "f32",
    ),
    "pivoted_qr_96x32_f64": ("Column-pivoted QR", "96 × 32", "f64"),
    "pivoted_qr_256x64_f64": ("Column-pivoted QR", "256 × 64", "f64"),
    "min_norm_64x32_rank16_f64": (
        "Minimum-norm least squares",
        "64 × 32, rank 16, 4 RHS",
        "f64",
    ),
    "min_norm_32x64_rank32_f64": (
        "Minimum-norm least squares",
        "32 × 64, rank 32, 4 RHS",
        "f64",
    ),
    "qr_96x32_f64": ("Thin QR", "96 × 32", "f64"),
    "qr_256x64_f64": ("Thin QR", "256 × 64", "f64"),
    "cholesky_factor_64_f64": ("Cholesky factorization", "64 × 64", "f64"),
    "cholesky_factor_128_f64": ("Cholesky factorization", "128 × 128", "f64"),
    "cholesky_factor_256_f64": ("Cholesky factorization", "256 × 256", "f64"),
    "softmax_256x64_f64": ("Row softmax", "256 × 64", "f64"),
    "create_zeros_64x64_f32": ("Create zeros", "64 × 64", "f32"),
    "create_ones_64x64_f32": ("Create ones", "64 × 64", "f32"),
    "matmul_sum_backward_32x16x8_f64": (
        "Record + backward",
        "sum(A · W), 32 × 16 · 16 × 8",
        "f64",
    ),
    "training_forward_2x4x8x4_f64": (
        "Model inference",
        "batched normalized/sliced MLP",
        "f64",
    ),
    "training_record_2x4x8x4_f64": (
        "Model recording + forward",
        "batched normalized/sliced MLP",
        "f64",
    ),
    "training_backward_2x4x8x4_f64": (
        "Model backward",
        "prepared graph → fresh gradients",
        "f64",
    ),
    "training_sgd_2x4x8x4_f64": (
        "Parameter update",
        "transactional SGD, fixed gradients",
        "f64",
    ),
}


def cpu_name(environment):
    details = environment.get("hardware_details")
    if details:
        try:
            for row in json.loads(details).get("lscpu", []):
                if row.get("field") == "Model name:":
                    return row["data"]
        except (ValueError, AttributeError):
            return details.strip()
    return environment.get("cpu_model") or "CPU unspecified"


def label(workload):
    # Future workloads stay visible even before a presentation label is added.
    return WORKLOADS.get(workload, (workload.replace("_", " "), "", ""))


def rows(data):
    grouped = {}
    for key, value in data["results"].items():
        workload, library = key.rsplit("/", 1)
        grouped.setdefault(workload, {})[library] = value
    from presentation import family, FAMILIES

    order = list(WORKLOADS)
    families = list(FAMILIES)
    return sorted(
        grouped.items(),
        key=lambda item: (
            families.index(family(item[0])),
            order.index(item[0]) if item[0] in order else len(order),
            item[0],
        ),
    )


def micros(result, metric="p50_ns"):
    return result["timing"]["per_call"][metric] / 1000


def throughput(workload, value):
    """Nominal 2*n^3 operations per allocation-inclusive GEMM latency, GFLOP/s."""
    match = re.fullmatch(r"matmul_(\d+)_f(?:32|64)", workload)
    if not match:
        return None
    ns = value["timing"]["per_call"]["p50_ns"]
    return 2 * int(match[1]) ** 3 / ns if ns > 0 else None


def change(current, previous):
    """Signed latency change: negative is faster. Never infer missing baselines."""
    if current is None or previous is None:
        return None
    old = micros(previous)
    return (micros(current) / old - 1) * 100 if old > 0 else None


def baseline_warnings(data, baseline):
    if baseline is None:
        return []
    a, b = data["metadata"], baseline["metadata"]
    warnings = []
    for key in ("cpu_model", "affinity_cpu", "governor", "thread_environment", "rustc"):
        if a.get("environment", {}).get(key) != b.get("environment", {}).get(key):
            warnings.append("Different " + key.replace("_", " ") + ".")
    if a.get("effective_harness_rustflags") != b.get("effective_harness_rustflags"):
        warnings.append("Different effective compiler flags.")
    if a.get("numpy_configuration") != b.get("numpy_configuration"):
        warnings.append("Different NumPy backend configuration.")

    def thread_policy(metadata):
        policy = dict(metadata.get("numpy_thread_policy") or {})
        # Mach-O UUIDs and install paths differ between separately linked copies.
        # Keep binary hashes as provenance; compare verified policy and source.
        if policy.get("controller_source_sha256"):
            policy.pop("controller_sha256", None)
        return policy

    if thread_policy(a) != thread_policy(b):
        warnings.append("Different NumPy thread-policy verification.")
    if a.get("explicit_execution") != b.get("explicit_execution"):
        warnings.append("Different explicit executor or affinity policies.")
    return warnings


def render_report(dest, data, baseline=None, paired=None):
    """Render immutable evidence with separate version and peer comparisons."""
    from presentation import render, normalize

    data = normalize(data)
    baseline = normalize(baseline) if baseline else None
    dest = Path(dest)
    dest.mkdir(parents=True, exist_ok=True)
    original = dest / "original-report.txt"
    if not original.exists():
        for candidate in ["README.md", "DETAILS.md"]:
            if (dest / candidate).exists():
                original.write_bytes((dest / candidate).read_bytes())
                break
    render(
        dest,
        data,
        baseline,
        paired,
        dict(
            rows=rows,
            label=label,
            micros=micros,
            names=lambda lib: NAMES.get(lib, lib),
            cpu=cpu_name(data["metadata"].get("environment", {})),
            libraries=LIBRARIES,
            baseline_warnings=baseline_warnings,
            change=change,
        ),
    )
    if not original.exists():
        original.write_bytes((dest / "README.md").read_bytes())
    from presentation import receipt

    receipt(dest, data, baseline, paired)
    render_chart(dest, rows(data), data["metadata"], paired=paired)


def render_chart(dest, groups, metadata=None, paired=None):
    """Vector PDF and SVG plus a matching PNG, from one typographic scene."""
    from presentation import static_report

    data = dict(
        metadata=metadata or {},
        results={w + "/" + lib: v for w, values in groups for lib, v in values.items()},
    )
    static_report(
        dest,
        data,
        dict(
            rows=rows,
            label=label,
            names=lambda lib: NAMES.get(lib, lib),
            cpu=cpu_name(data["metadata"].get("environment", {})),
        ),
        paired,
    )


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "directory",
        type=Path,
        help="directory containing raw.json; only presentation files are rewritten",
    )
    parser.add_argument(
        "--baseline",
        type=Path,
        help="directory containing an exact previous-version raw.json",
    )
    parser.add_argument(
        "--paired", type=Path, help="directory containing matching paired.json"
    )
    args = parser.parse_args()
    from presentation import load

    data = load(args.directory / "raw.json")
    baseline = load(args.baseline / "raw.json") if args.baseline else None
    render_report(
        args.directory,
        data,
        baseline,
        load(args.paired / "paired.json") if args.paired else None,
    )


if __name__ == "__main__":
    main()
