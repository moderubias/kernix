use super::{ShapeError, axis::normalize_axis, utils};

pub trait Shape {
    fn dims(&self) -> &[usize];

    #[inline]
    fn rank(&self) -> usize {
        self.dims().len()
    }

    #[inline]
    fn is_scalar(&self) -> bool {
        self.rank() == 0
    }

    #[inline]
    fn is_zero_sized(&self) -> bool {
        self.dims().contains(&0)
    }

    #[inline]
    fn numel(&self) -> usize {
        utils::numel(self.dims())
    }

    #[inline]
    fn axis(&self, axis: isize) -> Result<usize, ShapeError> {
        normalize_axis(self.rank(), axis)
    }

    #[inline]
    fn dim(&self, axis: isize) -> Result<usize, ShapeError> {
        Ok(self.dims()[self.axis(axis)?])
    }
}
