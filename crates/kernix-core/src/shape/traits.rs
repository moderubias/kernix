use super::{ShapeError, axis::normalize_axis, utils};

/// Read-only dimension queries shared by dynamic and fixed-rank shapes.
///
/// Implementors must expose stable dimensions whose product fits usize, or
/// contains a zero. No mutable dimension access is provided.
pub trait Shape {
    /// Borrow the dimension sizes in axis order, without allocating. O(1).
    fn dims(&self) -> &[usize];

    /// Return the number of axes. Scalars have rank zero. O(1).
    #[inline]
    fn rank(&self) -> usize {
        self.dims().len()
    }

    /// Return whether the shape has no axes (and therefore one element). O(1).
    #[inline]
    fn is_scalar(&self) -> bool {
        self.rank() == 0
    }

    /// Return whether any dimension is zero. O(rank).
    #[inline]
    fn is_zero_sized(&self) -> bool {
        self.dims().contains(&0)
    }

    /// Return the logical element count. Scalars contain one element; any zero
    /// dimension makes the count zero. Shape validation prevents product overflow.
    #[inline]
    fn numel(&self) -> usize {
        utils::numel(self.dims())
    }

    /// Normalize a signed axis into 0..rank; -1 selects the last axis.
    /// Returns an out-of-bounds error for invalid axes, including any axis on a scalar.
    #[inline]
    fn axis(&self, axis: isize) -> Result<usize, ShapeError> {
        normalize_axis(self.rank(), axis)
    }

    /// Return the dimension at a signed axis, or an axis error. O(1).
    #[inline]
    fn dim(&self, axis: isize) -> Result<usize, ShapeError> {
        Ok(self.dims()[self.axis(axis)?])
    }
}
