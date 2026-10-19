// Kernix numerics-softmax overlay 04 (applied by tools/kernix-update-04/apply.py)
use alloc::{vec, vec::Vec};
use core::{
    mem::align_of,
    ops::{Add, Div, Mul, Neg, Rem, Sub},
};
use kernix_core::{layout::Order, shape::Kahen, tensor::Tensor};

use super::{
    OpsError, TensorInput,
    broadcast::{broadcast_shape, project_broadcast},
    error::flatten_build_error,
};

/// Fallible unary transform. `f` is called once for each logical element in
/// C-order. Its error is returned unchanged; previously constructed output
/// elements are dropped by the Tensor initializer.
///
/// The output is a newly owned, C-contiguous host tensor.
///
/// # Examples
///
/// ```
/// use kernix_core::{Order, shape::Kahen, tensor::Tensor};
/// use kernix_ops::try_map;
/// let mut a = Tensor::from_vec(vec![1.0,2.0,3.0,4.0], Kahen::new([2,2])?, Order::C)?;
/// let result = try_map(&a, |x| Ok(x*2.0))?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn try_map<T, U, A, F>(source: &A, mut f: F) -> Result<Tensor<U>, OpsError>
where
    A: TensorInput<T>,
    F: FnMut(&T) -> Result<U, OpsError>,
{
    try_map_indexed(source, |value, _indices| f(value))
}

/// As [`try_map`], but receives the logical coordinates of each input value.
/// Coordinates are valid only for the duration of the callback.
///
/// # Examples
///
/// ```
/// use kernix_core::{Order, shape::Kahen, tensor::Tensor};
/// use kernix_ops::try_map_indexed;
/// let mut a = Tensor::from_vec(vec![1.0,2.0,3.0,4.0], Kahen::new([2,2])?, Order::C)?;
/// let result = try_map_indexed(&a, |x, index| Ok(x+index[0] as f64))?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn try_map_indexed<T, U, A, F>(source: &A, mut f: F) -> Result<Tensor<U>, OpsError>
where
    A: TensorInput<T>,
    F: FnMut(&T, &[usize]) -> Result<U, OpsError>,
{
    let shape = Kahen::new(source.dims())?;
    Tensor::<U>::try_from_fn(shape, Order::C, align_of::<U>(), |coords| {
        let value = source.get(coords)?;
        f(value, coords)
    })
    .map_err(flatten_build_error)
}

/// Unary map over logical elements, returning a new C-contiguous CPU tensor.
///
/// # Examples
///
/// ```
/// use kernix_core::{Order, shape::Kahen, tensor::Tensor};
/// use kernix_ops::map;
/// let mut a = Tensor::from_vec(vec![1.0,2.0,3.0,4.0], Kahen::new([2,2])?, Order::C)?;
/// let result = map(&a, |x| x*2.0)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn map<T, U, A, F>(source: &A, mut f: F) -> Result<Tensor<U>, OpsError>
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

/// Fallible binary transform with NumPy-compatible right-aligned broadcasting.
/// On error, no partially initialized output escapes.
///
/// # Examples
///
/// ```
/// use kernix_core::{Order, shape::Kahen, tensor::Tensor};
/// use kernix_ops::try_zip_map;
/// let mut a = Tensor::from_vec(vec![1.0,2.0,3.0,4.0], Kahen::new([2,2])?, Order::C)?;
/// let result = try_zip_map(&a, &a, |x,y| Ok(x+y))?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn try_zip_map<L, R, U, A, B, F>(left: &A, right: &B, mut f: F) -> Result<Tensor<U>, OpsError>
where
    A: TensorInput<L>,
    B: TensorInput<R>,
    F: FnMut(&L, &R) -> Result<U, OpsError>,
{
    try_zip_map_indexed(left, right, |a, b, _indices| f(a, b))
}

/// Indexed counterpart of [`try_zip_map`]. The coordinate slice refers to
/// the broadcasted output shape (not either input shape).
///
/// # Examples
///
/// ```
/// use kernix_core::{Order, shape::Kahen, tensor::Tensor};
/// use kernix_ops::try_zip_map_indexed;
/// let mut a = Tensor::from_vec(vec![1.0,2.0,3.0,4.0], Kahen::new([2,2])?, Order::C)?;
/// let result = try_zip_map_indexed(&a, &a, |x,y,index| Ok(x+y+index[0] as f64))?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn try_zip_map_indexed<L, R, U, A, B, F>(
    left: &A,
    right: &B,
    mut f: F,
) -> Result<Tensor<U>, OpsError>
where
    A: TensorInput<L>,
    B: TensorInput<R>,
    F: FnMut(&L, &R, &[usize]) -> Result<U, OpsError>,
{
    let dims = broadcast_shape(left.dims(), right.dims())?;
    let shape = Kahen::new(&dims)?;
    let mut li = vec![0; left.dims().len()];
    let mut ri = vec![0; right.dims().len()];
    Tensor::<U>::try_from_fn(shape, Order::C, align_of::<U>(), |coords| {
        project_broadcast(coords, left.dims(), &mut li);
        project_broadcast(coords, right.dims(), &mut ri);
        let a = left.get(&li)?;
        let b = right.get(&ri)?;
        f(a, b, coords)
    })
    .map_err(flatten_build_error)
}

/// Binary elementwise map with right-aligned broadcasting. Both sources are
/// borrowed and result elements are freshly allocated in C-order.
///
/// # Examples
///
/// ```
/// use kernix_core::{Order, shape::Kahen, tensor::Tensor};
/// use kernix_ops::zip_map;
/// let mut a = Tensor::from_vec(vec![1.0,2.0,3.0,4.0], Kahen::new([2,2])?, Order::C)?;
/// let result = zip_map(&a, &a, |x,y| x+y)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn zip_map<L, R, U, A, B, F>(left: &A, right: &B, mut f: F) -> Result<Tensor<U>, OpsError>
where
    A: TensorInput<L>,
    B: TensorInput<R>,
    F: FnMut(&L, &R) -> U,
{
    // Equal-shape contiguous elementwise work requires no coordinate
    // projection. Broadcasting and strided inputs keep their old fallback.
    let result_shape = Kahen::new(broadcast_shape(left.dims(), right.dims())?)?;
    if left.dims() == right.dims() {
        if let (Some(a), Some(b)) = (left.as_c_contiguous_slice(), right.as_c_contiguous_slice()) {
            if a.len() == result_shape.numel() && b.len() == result_shape.numel() {
                let values: Vec<U> = a.iter().zip(b).map(|(a, b)| f(a, b)).collect();
                return Ok(Tensor::from_vec(values, result_shape, Order::C)?);
            }
        }
    }
    try_zip_map(left, right, |a, b| Ok(f(a, b)))
}

macro_rules! binary_op {
    ($name:ident, $op:ident, $method:ident) => {
        /// Allocate elementwise arithmetic with trailing-axis broadcasting.
        /// Scalar operations retain native Rust overflow/division semantics. For checked
        /// integer errors use the checked_* functions. Results are C-contiguous.
        pub fn $name<T, A, B>(left: &A, right: &B) -> Result<Tensor<T>, OpsError>
        where
            T: Clone + $op<Output = T>,
            A: TensorInput<T>,
            B: TensorInput<T>,
        {
            zip_map(left, right, |a, b| a.clone().$method(b.clone()))
        }
    };
}

binary_op!(add, Add, add);
binary_op!(sub, Sub, sub);
binary_op!(mul, Mul, mul);
binary_op!(div, Div, div);
binary_op!(rem, Rem, rem);

/// Allocate elementwise negation in logical C order. Integer overflow follows
/// native Rust operators; use checked_neg for fallible integer negation.
///
/// # Examples
///
/// ```
/// use kernix_core::{Order, shape::Kahen, tensor::Tensor};
/// use kernix_ops::neg;
/// let mut a = Tensor::from_vec(vec![1.0,2.0,3.0,4.0], Kahen::new([2,2])?, Order::C)?;
/// let result = neg(&a)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn neg<T, A>(source: &A) -> Result<Tensor<T>, OpsError>
where
    T: Clone + Neg<Output = T>,
    A: TensorInput<T>,
{
    map(source, |value| -value.clone())
}
