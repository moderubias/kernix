#!/usr/bin/env python3
"""Apply Kernix update 04 to the verified October 19 baseline.

Never overwrites unrelated local source. Validates every hunk before writing any
tracked file; re-running after success is idempotent. Stdlib-only Python 3.
"""
from __future__ import annotations

import argparse
import pathlib
import subprocess
import sys


def replace_once(contents: str, old: str, new: str, filename: str, label: str) -> str:
    # Check the complete replacement first: several replacements retain the
    # original anchor as a suffix, so counting the anchor first is NOT
    # sufficient to implement idempotency.
    if new in contents:
        return contents
    n = contents.count(old)
    if n == 1:
        return contents.replace(old, new, 1)
    raise RuntimeError(f"{filename}: {label}: expected one unmodified match, found {n}; aborting")


def patch_math(s: str, f: str) -> str:
    s = replace_once(s, '    fn exp(self) -> Self;\n', '''    /// Accurate log(1+x), including x close to zero.\n    fn ln_1p(self) -> Self { Self::ln(Self::one() + self) }\n    /// Accurate exp(x)-1, including x close to zero.\n    fn exp_m1(self) -> Self { Self::exp(self) - Self::one() }\n    fn exp(self) -> Self;\n''', f, 'FloatMath methods')
    s = replace_once(s, '''    fn ln(self) -> Self {
        libm::logf(self)
    }
''', '''    fn ln(self) -> Self {
        libm::logf(self)
    }
    fn ln_1p(self) -> Self {
        libm::log1pf(self)
    }
    fn exp_m1(self) -> Self {
        libm::expm1f(self)
    }
''', f, 'f32 special functions')
    s = replace_once(s, '''    fn ln(self) -> Self {
        libm::log(self)
    }
''', '''    fn ln(self) -> Self {
        libm::log(self)
    }
    fn ln_1p(self) -> Self {
        libm::log1p(self)
    }
    fn exp_m1(self) -> Self {
        libm::expm1(self)
    }
''', f, 'f64 special functions')
    s = replace_once(s, '''pub fn softplus<T: FloatMath, A: TensorInput<T>>(source: &A) -> Result<Tensor<T>, OpsError> {
    map(source, |value| {
        if *value > T::zero() {
            *value + FloatMath::ln(T::one() + FloatMath::exp(-*value))
        } else {
            FloatMath::ln(T::one() + FloatMath::exp(*value))
        }
    })
}
''', '''pub(crate) fn scalar_softplus<T: FloatMath>(value: T) -> T {
    if value > T::zero() {
        value + FloatMath::ln_1p(FloatMath::exp(-value))
    } else {
        FloatMath::ln_1p(FloatMath::exp(value))
    }
}

pub fn softplus<T: FloatMath, A: TensorInput<T>>(source: &A) -> Result<Tensor<T>, OpsError> {
    map(source, |value| scalar_softplus(*value))
}
''', f, 'softplus stable replacement')
    return s


def patch_elementwise(s: str, f: str) -> str:
    s = replace_once(s, 'use alloc::vec;\n', 'use alloc::{vec, vec::Vec};\n', f, 'Vec import')
    s = replace_once(s, '''pub fn map<T, U, A, F>(source: &A, mut f: F) -> Result<Tensor<U>, OpsError>
where
    A: TensorInput<T>,
    F: FnMut(&T) -> U,
{
    try_map(source, |value| Ok(f(value)))
}
''', '''pub fn map<T, U, A, F>(source: &A, mut f: F) -> Result<Tensor<U>, OpsError>
where
    A: TensorInput<T>,
    F: FnMut(&T) -> U,
{
    // Direct logical C-order traversal; arbitrary strides retain the checked
    // `try_map` path. A panic still drops the partially built Vec safely.
    let shape = Kahen::new(source.dims())?;
    if let Some(values) = source.as_c_contiguous_slice() {
        if values.len() == shape.numel() {
            let result: Vec<U> = values.iter().map(&mut f).collect();
            return Ok(Tensor::from_vec(result, shape, Order::C)?);
        }
    }
    try_map(source, |value| Ok(f(value)))
}
''', f, 'map direct fast path')
    s = replace_once(s, '''pub fn zip_map<L, R, U, A, B, F>(left: &A, right: &B, mut f: F) -> Result<Tensor<U>, OpsError>
where
    A: TensorInput<L>,
    B: TensorInput<R>,
    F: FnMut(&L, &R) -> U,
{
    try_zip_map(left, right, |a, b| Ok(f(a, b)))
}
''', '''pub fn zip_map<L, R, U, A, B, F>(left: &A, right: &B, mut f: F) -> Result<Tensor<U>, OpsError>
where
    A: TensorInput<L>,
    B: TensorInput<R>,
    F: FnMut(&L, &R) -> U,
{
    // Equal-shape contiguous elementwise work requires no coordinate
    // projection. Broadcasting and strided inputs keep their old fallback.
    let result_shape = Kahen::new(broadcast_shape(left.dims(), right.dims())?)?;
    if left.dims() == right.dims() {
        if let (Some(a), Some(b)) =
            (left.as_c_contiguous_slice(), right.as_c_contiguous_slice())
        {
            if a.len() == result_shape.numel() && b.len() == result_shape.numel() {
                let values: Vec<U> = a.iter().zip(b).map(|(a, b)| f(a, b)).collect();
                return Ok(Tensor::from_vec(values, result_shape, Order::C)?);
            }
        }
    }
    try_zip_map(left, right, |a, b| Ok(f(a, b)))
}
''', f, 'zip_map direct fast path')
    return s


def patch_reduce(s: str, f: str) -> str:
    s = replace_once(s, '''    let mut coords = vec![0; source.dims().len()];
    let mut acc = T::zero();
    for _ in 0..shape.numel() {
        acc += source.get(&coords)?.clone();
        next_c(&mut coords, source.dims());
    }
    Ok(acc)
''', '''    if let Some(values) = source.as_c_contiguous_slice() {
        if values.len() == shape.numel() {
            let mut acc = T::zero();
            for value in values { acc += value.clone(); }
            return Ok(acc);
        }
    }
    let mut coords = vec![0; source.dims().len()];
    let mut acc = T::zero();
    for _ in 0..shape.numel() {
        acc += source.get(&coords)?.clone();
        next_c(&mut coords, source.dims());
    }
    Ok(acc)
''', f, 'sum_all fast path')
    s = replace_once(s, '''    let mut coords = vec![0; source.dims().len()];
    let mut acc = T::one();
    for _ in 0..shape.numel() {
        acc *= source.get(&coords)?.clone();
        next_c(&mut coords, source.dims());
    }
    Ok(acc)
''', '''    if let Some(values) = source.as_c_contiguous_slice() {
        if values.len() == shape.numel() {
            let mut acc = T::one();
            for value in values { acc *= value.clone(); }
            return Ok(acc);
        }
    }
    let mut coords = vec![0; source.dims().len()];
    let mut acc = T::one();
    for _ in 0..shape.numel() {
        acc *= source.get(&coords)?.clone();
        next_c(&mut coords, source.dims());
    }
    Ok(acc)
''', f, 'prod_all fast path')
    s = replace_once(s, '''    Ok(sum_all(source)? / T::from_count(shape.numel()))
''', '''    // Divide before accumulation. This avoids overflow of a large finite
    // intermediate sum when the final mean is representable. This is not a
    // compensated summation guarantee for all possible input sequences.
    let divisor = T::from_count(shape.numel());
    if let Some(values) = source.as_c_contiguous_slice() {
        if values.len() == shape.numel() {
            let mut mean = T::zero();
            for value in values { mean += value.clone() / divisor; }
            return Ok(mean);
        }
    }
    let mut coords = vec![0; source.dims().len()];
    let mut mean = T::zero();
    for _ in 0..shape.numel() {
        mean += source.get(&coords)?.clone() / divisor;
        next_c(&mut coords, source.dims());
    }
    Ok(mean)
''', f, 'mean_all')
    s = replace_once(s, '''    let sum = sum_axes(source, axes, keepdims)?;
    if count == 0 {
        return Ok(sum);
    } // empty output, no division performed
    map(&sum, |value| *value / T::from_count(count))
''', '''    if count == 0 {
        // Only possible for an empty output (nonempty outputs error above).
        return sum_axes(source, axes, keepdims);
    }
    let divisor = T::from_count(count);
    reduce_axes(source, axes, keepdims, T::zero(), |acc, value| {
        *acc += value / divisor;
    })
''', f, 'mean_axes')
    return s


def patch_input(s: str, f: str) -> str:
    s = replace_once(s, '''    layout::LayoutShape,
''', '''    layout::{LayoutShape, Order},
''', f, 'Order import')
    s = replace_once(s, '''pub trait TensorInput<T> {
    fn dims(&self) -> &[usize];
    fn get(&self, indices: &[usize]) -> Result<&T, TensorError>;
}
''', '''pub trait TensorInput<T> {
    fn dims(&self) -> &[usize];
    fn get(&self, indices: &[usize]) -> Result<&T, TensorError>;

    /// Optional C-order contiguous logical payload. The slice must have
    /// exactly product(dims()) elements in logical C traversal order.
    /// Device buffers and noncontiguous layouts return `None` by default.
    fn as_c_contiguous_slice(&self) -> Option<&[T]> {
        None
    }
}
''', f, 'TensorInput fast-path contract')
    for typename in ['Tensor', 'TensorView', 'TensorViewMut']:
        old = f'''    fn get(&self, indices: &[usize]) -> Result<&T, TensorError> {{
        {typename}::get(self, indices)
    }}
}}
'''
        new = f'''    fn get(&self, indices: &[usize]) -> Result<&T, TensorError> {{
        {typename}::get(self, indices)
    }}

    fn as_c_contiguous_slice(&self) -> Option<&[T]> {{
        self.as_contiguous_slice(Order::C).ok()
    }}
}}
'''
        s = replace_once(s, old, new, f, typename + ' fast path')
    return s


def patch_matmul(s: str, f: str) -> str:
    s = replace_once(s, '''    let output = Kahen::new(out_dims)?;
    let batch_rank = batch.len();
''', '''    let output = Kahen::new(out_dims)?;
    if output.numel() == 0 {
        return Ok(Tensor::from_vec(Vec::new(), output, Order::C)?);
    }

    // Fast direct-address CPU path for ordinary row-major rank-2 tensors.
    // Keep k as the innermost loop: floating-point accumulation order matches
    // the reference kernel. Arbitrary strides, batches and custom inputs
    // retain the fully general checked-indexing fallback below.
    if a.len() == 2 && b.len() == 2 {
        if let (Some(a_data), Some(b_data)) =
            (left.as_c_contiguous_slice(), right.as_c_contiguous_slice())
        {
            if m.checked_mul(k) == Some(a_data.len())
                && k.checked_mul(n) == Some(b_data.len())
            {
                let mut values = Vec::with_capacity(output.numel());
                for row in 0..m {
                    for col in 0..n {
                        let mut value = T::zero();
                        for inner in 0..k {
                            value += a_data[row * k + inner].clone()
                                * b_data[inner * n + col].clone();
                        }
                        values.push(value);
                    }
                }
                return Ok(Tensor::from_vec(values, output, Order::C)?);
            }
        }
    }
    let batch_rank = batch.len();
''', f, 'matmul direct-address path')
    return s


def patch_dot(s: str, f: str) -> str:
    s = replace_once(s, '''    let mut total = T::zero();
    for i in 0..left.dims()[0] {
''', '''    // Zero-allocation direct-address path for contiguous vector inputs.
    if let (Some(a), Some(b)) =
        (left.as_c_contiguous_slice(), right.as_c_contiguous_slice())
    {
        if a.len() == left.dims()[0] && b.len() == right.dims()[0] {
            let mut total = T::zero();
            for (lhs, rhs) in a.iter().zip(b) {
                total += lhs.clone() * rhs.clone();
            }
            return Ok(total);
        }
    }
    let mut total = T::zero();
    for i in 0..left.dims()[0] {
''', f, 'dot direct-address path')
    return s


def patch_ops(s: str, f: str) -> str:
    s = replace_once(s, 'mod math;\n', 'mod math;\nmod math_extra;\n', f, 'math_extra module')
    s = replace_once(s, '''pub use stats::{std_all, std_axes, variance_all, variance_axes};
''', '''pub use stats::{std_all, std_axes, variance_all, variance_axes};

pub use math_extra::{
    FloatTrig, atan2, cos, expm1, hypot, log1p, log1mexp, logaddexp,
    logdiffexp, logsigmoid, softplus_beta,
    log_softmax_temperature, logsumexp_online_all, logsumexp_online_axis,
    masked_log_softmax, masked_log_softmax_temperature, masked_softmax,
    masked_softmax_temperature, mish, rsqrt, sin, softmax_temperature,
    softmax_online, softmax_online_temperature, log_softmax_online,
};
''', f, 'math_extra exports')
    return s


def patch_kahen(s: str, f: str) -> str:
    s = replace_once(s, '''    pub fn squeeze_axis(&self, axis: usize) -> Result<Self, ShapeError> {
''', '''    /// Remove all singleton axes without changing the number of elements.
    pub fn squeeze(&self) -> Self {
        let dims: SmallVec<[usize; INLINE_RANK]> = self.dims
            .iter()
            .copied()
            .filter(|&dim| dim != 1)
            .collect();
        Self { dims }
    }

    /// Alias of `insert_axis`, corresponding to NumPy/PyTorch terminology.
    #[inline]
    pub fn unsqueeze(&self, axis: usize) -> Result<Self, ShapeError> {
        self.insert_axis(axis)
    }

    /// Swap two existing axes, retaining the element count.
    pub fn swap_axes(&self, left: usize, right: usize) -> Result<Self, ShapeError> {
        let rank = self.rank();
        if left >= rank {
            return Err(ShapeError::AxisPositionOutOfBounds { axis: left, rank });
        }
        if right >= rank {
            return Err(ShapeError::AxisPositionOutOfBounds { axis: right, rank });
        }
        let mut dims = self.dims.clone();
        dims.swap(left, right);
        Ok(Self { dims })
    }

    pub fn squeeze_axis(&self, axis: usize) -> Result<Self, ShapeError> {
''', f, 'shape convenience methods')
    return s


def patch_kotei(s: str, f: str) -> str:
    s = replace_once(s, '''    pub fn permute(&self, axes: [usize; N]) -> Result<Self, ShapeError> {
''', '''    /// Swap two existing axes without erasing the fixed rank.
    pub fn swap_axes(&self, left: usize, right: usize) -> Result<Self, ShapeError> {
        if left >= N {
            return Err(ShapeError::AxisPositionOutOfBounds { axis: left, rank: N });
        }
        if right >= N {
            return Err(ShapeError::AxisPositionOutOfBounds { axis: right, rank: N });
        }
        let mut dims = self.dims;
        dims.swap(left, right);
        Ok(Self { dims })
    }

    pub fn permute(&self, axes: [usize; N]) -> Result<Self, ShapeError> {
''', f, 'fixed-rank swap_axes')
    return s


EDITORS = {
    'crates/kernix-ops/src/ops/math.rs': patch_math,
    'crates/kernix-ops/src/ops/elementwise.rs': patch_elementwise,
    'crates/kernix-ops/src/ops/reduce.rs': patch_reduce,
    'crates/kernix-ops/src/ops/input.rs': patch_input,
    'crates/kernix-ops/src/ops/linalg.rs': patch_matmul,
    'crates/kernix-ops/src/ops/linalg_extra.rs': patch_dot,
    'crates/kernix-ops/src/ops.rs': patch_ops,
    'crates/kernix-core/src/shape/kahen.rs': patch_kahen,
    'crates/kernix-core/src/shape/kotei.rs': patch_kotei,
}


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument('--check', action='store_true', help='validate hunks without modifying files')
    args = ap.parse_args()
    root = pathlib.Path(__file__).resolve().parents[2]
    if not (root / 'Cargo.toml').is_file():
        raise RuntimeError(f'Not a Kernix repository root: {root}')
    try:
        head = subprocess.check_output(['git', '-C', str(root), 'rev-parse', '--short=12', 'HEAD'], text=True).strip()
    except (subprocess.CalledProcessError, FileNotFoundError):
        head = 'unknown'
    print(f'Kernix update 04: repository HEAD {head}')
    results = {}
    stamp = '// Kernix numerics-softmax overlay 04 (applied by tools/kernix-update-04/apply.py)'
    for filename, editor in EDITORS.items():
        path = root / filename
        before = path.read_text(encoding='utf-8')
        if before.startswith(stamp + '\n'):
            after = before  # Idempotent even after rustfmt changes whitespace.
        else:
            after = editor(before, filename)
            if after != before:
                after = stamp + '\n' + after
        results[path] = (before, after)
    required = [
        'crates/kernix-ops/src/ops/math_extra.rs',
        'crates/kernix-ops/tests/numeric_dispatch_04.rs',
        'crates/kernix-core/tests/shape_extras_04.rs',
    ]
    for file in required:
        if not (root / file).is_file():
            raise RuntimeError(f'Missing overlay file: {file}. Unzip the archive first.')
    changes = [path for path, (before, after) in results.items() if before != after]
    # Do not silently patch a user's uncommitted changes in any affected file.
    # Unrelated working-tree changes do not prevent applying this overlay.
    for path in changes:
        rel = str(path.relative_to(root))
        diff = subprocess.run(['git', '-C', str(root), 'diff', '--quiet', 'HEAD', '--', rel], check=False)
        if diff.returncode != 0:
            raise RuntimeError(f'Uncommitted/staged changes in {rel}; commit or stash them before applying.')
    print(f'Preflight succeeded; {len(changes)} tracked source files to modify.')
    if args.check:
        print('Dry run only; no files changed.')
        return 0
    for path in changes:
        before, after = results[path]
        path.write_text(after, encoding='utf-8')
        print('Modified:', path.relative_to(root))
    print('Apply complete. Run cargo fmt/test/clippy/no-default-features checks.')
    return 0


if __name__ == '__main__':
    try:
        raise SystemExit(main())
    except (RuntimeError, OSError) as error:
        print(f'ERROR: {error}', file=sys.stderr)
        raise SystemExit(1) from error
