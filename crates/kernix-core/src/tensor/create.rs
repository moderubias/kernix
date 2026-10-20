use alloc::vec::Vec;
use core::mem::align_of;

use super::{Tensor, TensorError};
use crate::{
    Layout, LayoutError, Order,
    scalar::{One, Zero},
    shape::Kahen,
    storage::{CpuBuffer, Storage},
};

impl<T> Tensor<T> {
    /// Allocate C-order zeros using the scalar's additive identity.
    ///
    /// Shape/layout/storage are validated before generating elements. Calls
    /// [`Zero::zero`] once per element, requiring neither Clone nor Default.
    /// Returns shape, extent, unsupported zero-sized type or allocation errors.
    /// `[]` is scalar; any zero dimension makes the tensor empty. O(numel) time,
    /// one element allocation for nonempty tensors, plus shape metadata above rank six.
    /// For other orders/alignment, use [`Self::from_fn`] or [`Self::filled`].
    ///
    /// ```
    /// let a = kernix_core::tensor::Tensor::<f32>::zeros([2, 3])?;
    /// assert_eq!(a.storage_slice(), &[0.0; 6]);
    /// # Ok::<(), kernix_core::tensor::TensorError>(())
    /// ```
    pub fn zeros(dims: impl AsRef<[usize]>) -> Result<Self, TensorError>
    where
        T: Zero,
    {
        Self::from_identity(dims, T::zero)
    }

    /// Allocate C-order ones using the scalar's multiplicative identity.
    ///
    /// Calls [`One::one`] once per element, without Clone, Default or arithmetic
    /// bounds. Validation, complexity, scalar/empty and allocation contracts match
    /// [`Self::zeros`]. No dtype conversion or device transfer occurs.
    ///
    /// ```
    /// let a = kernix_core::tensor::Tensor::<i16>::ones([3])?;
    /// assert_eq!(a.storage_slice(), &[1, 1, 1]);
    /// # Ok::<(), kernix_core::tensor::TensorError>(())
    /// ```
    pub fn ones(dims: impl AsRef<[usize]>) -> Result<Self, TensorError>
    where
        T: One,
    {
        Self::from_identity(dims, T::one)
    }

    fn from_identity(
        dims: impl AsRef<[usize]>,
        mut identity: impl FnMut() -> T,
    ) -> Result<Self, TensorError> {
        let shape = Kahen::new(dims).map_err(LayoutError::from)?;
        let layout = Layout::contiguous(shape, Order::C)?;
        let storage =
            Storage::<T, CpuBuffer<T>>::try_from_fn(layout.numel(), align_of::<T>(), |_| {
                identity()
            })?;
        Self::from_host_storage(storage, layout)
    }

    /// Adopt C-order values with inferred dynamic shape metadata, without copying elements.
    ///
    /// Returns an error for invalid dimensions, a length mismatch, zero-sized element
    /// types or an unrepresentable storage extent. The vector is consumed on error.
    ///
    /// ```
    /// use kernix_core::tensor::Tensor;
    /// let a = Tensor::from_values([2, 2], vec![1, 2, 3, 4])?;
    /// assert_eq!(a.get(&[1, 0])?, &3);
    /// # Ok::<(), kernix_core::tensor::TensorError>(())
    /// ```
    pub fn from_values(dims: impl AsRef<[usize]>, values: Vec<T>) -> Result<Self, TensorError> {
        let shape = Kahen::new(dims).map_err(LayoutError::from)?;
        Self::from_vec(values, shape, Order::C)
    }

    /// Move a rectangular array into a C-order matrix. Elements need not be Clone.
    ///
    /// Rust checks row lengths at compile time. Empty rows and zero rows are supported;
    /// allocation follows `Vec::collect` and tensor validation follows `from_values`.
    ///
    /// ```
    /// let a = kernix_core::tensor::Tensor::from_rows([[1, 2], [3, 4]])?;
    /// assert_eq!(a.shape().dims(), &[2, 2]);
    /// # Ok::<(), kernix_core::tensor::TensorError>(())
    /// ```
    pub fn from_rows<const ROWS: usize, const COLS: usize>(
        rows: [[T; COLS]; ROWS],
    ) -> Result<Self, TensorError> {
        Self::from_values([ROWS, COLS], rows.into_iter().flatten().collect())
    }

    /// Allocate a C-order tensor using one cloned value and natural element alignment.
    ///
    /// Dimensions are validated before allocation. An empty dimension creates an empty
    /// tensor; `[]` creates a scalar. Returns shape, layout, storage or allocation errors.
    ///
    /// ```
    /// let a = kernix_core::tensor::Tensor::full([2, 3], 0.0f32)?;
    /// assert_eq!(a.storage_slice(), &[0.0; 6]);
    /// # Ok::<(), kernix_core::tensor::TensorError>(())
    /// ```
    pub fn full(dims: impl AsRef<[usize]>, value: T) -> Result<Self, TensorError>
    where
        T: Clone,
    {
        let shape = Kahen::new(dims).map_err(LayoutError::from)?;
        Self::filled(shape, Order::C, align_of::<T>(), value)
    }
}

/// Construct a C-order vector or an explicitly shaped tensor, returning a Result.
///
/// Values are evaluated once from left to right. Repetition requires Clone, like
/// `vec!`. The `shape = ...; values = ...` form adopts a Vec without element copies.
/// All forms return `Result<Tensor<T>, TensorError>` and work with no_std + alloc.
/// Allocation by `vec!` follows Rust's normal allocation failure behavior.
/// Array-valued elements stay elements: `tensor![[1,2],[3,4]]` has shape `[2]`
/// and element type `[i32;2]`. Use `matrix![1,2;3,4]` or `Tensor::from_rows`
/// for two scalar axes. Repetition evaluates the count before the fill value.
///
/// ```
/// use kernix_core::tensor;
/// let v = tensor![1, 2, 3]?;
/// let repeated = tensor![7; 4]?;
/// let a = tensor!(shape = [2, 2]; values = vec![1, 2, 3, 4])?;
/// assert_eq!(v.shape().dims(), &[3]);
/// assert_eq!(repeated.storage_slice(), &[7; 4]);
/// assert_eq!(a.shape().dims(), &[2, 2]);
/// # Ok::<(), kernix_core::tensor::TensorError>(())
/// ```
#[macro_export]
macro_rules! tensor {
    (shape = $dims:expr; values = $values:expr $(,)?) => {
        $crate::tensor::Tensor::from_values($dims, $values)
    };
    ($value:expr; $count:expr) => {{
        let count: usize = $count;
        $crate::tensor::Tensor::full([count], $value)
    }};
    ($($value:expr),* $(,)?) => {{
        let values = $crate::__private::vec![$($value),*];
        $crate::tensor::Tensor::from_values([values.len()], values)
    }};
}

/// Construct a C-order matrix with compile-time rectangularity checks.
///
/// Each expression is evaluated once, row by row. Elements are moved and need not
/// implement Clone or Copy. Returns `Result<Tensor<T>, TensorError>`.
///
/// ```
/// let a = kernix_core::matrix![1, 2; 3, 4]?;
/// assert_eq!(a.get(&[1, 0])?, &3);
/// # Ok::<(), kernix_core::tensor::TensorError>(())
/// ```
///
/// Ragged rows are rejected by Rust, before execution:
///
/// ```compile_fail
/// let _ = kernix_core::matrix![1, 2; 3];
/// ```
#[macro_export]
macro_rules! matrix {
    ($($($value:expr),+ $(,)?);+ $(;)?) => {
        $crate::tensor::Tensor::from_rows([$([$($value),+]),+])
    };
}

/// Allocate a C-order tensor filled with one value, returning a Result.
///
/// Value and dimension expressions are evaluated once. Shape/layout/storage errors
/// are returned without allocating the element buffer. Use `full!(0.0f32; 2, 3)`
/// for zeros and `full!(1.0f64; 2, 3)` for ones. With no dimensions, creates a scalar.
/// Dimension expressions are evaluated before the fill value, preserving the
/// existing macro's evaluation order. Associated `Tensor::zeros`/`ones` also
/// support custom identity types without Clone.
///
/// ```
/// let a = kernix_core::full!(0.0f32; 2, 3)?;
/// let scalar = kernix_core::full!(42;)?;
/// assert_eq!(a.shape().dims(), &[2, 3]);
/// assert_eq!(scalar.shape().dims(), &[]);
/// # Ok::<(), kernix_core::tensor::TensorError>(())
/// ```
#[macro_export]
macro_rules! full {
    ($value:expr; $($dim:expr),* $(,)?) => {
        $crate::tensor::Tensor::full([$($dim),*], $value)
    };
}

/// Construct a validated dynamic shape, returning `Result<Kahen, ShapeError>`.
///
/// `shape![]` is scalar, zero dimensions are supported, and products that overflow
/// usize are rejected. Each dimension expression is evaluated once.
///
/// ```
/// assert_eq!(kernix_core::shape![2, 3]?.numel(), 6);
/// assert_eq!(kernix_core::shape![]?.numel(), 1);
/// # Ok::<(), kernix_core::shape::ShapeError>(())
/// ```
#[macro_export]
macro_rules! shape {
    ($($dim:expr),* $(,)?) => {
        $crate::shape::Kahen::new([$($dim),*])
    };
}
