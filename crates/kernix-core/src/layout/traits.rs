use crate::shape::{Kahen, Kotei, Shape, ShapeError};
use smallvec::SmallVec;

/// Associates a shape's rank representation with the same-rank stride storage.
///
/// Implementations must return validated shapes from `with_same_rank_dims`.
/// The method must reject a rank change.
pub trait LayoutShape: Shape + Clone {
    type Strides: AsRef<[isize]> + AsMut<[isize]> + Clone + core::fmt::Debug + PartialEq + Eq;

    fn zeroed_strides(&self) -> Self::Strides;

    fn with_same_rank_dims(&self, dims: &[usize]) -> Result<Self, ShapeError>;
}

impl LayoutShape for Kahen {
    type Strides = SmallVec<[isize; 6]>;

    fn zeroed_strides(&self) -> Self::Strides {
        core::iter::repeat_n(0, self.rank()).collect()
    }

    fn with_same_rank_dims(&self, dims: &[usize]) -> Result<Self, ShapeError> {
        if dims.len() != self.rank() {
            return Err(ShapeError::RankMismatch {
                expected: self.rank(),
                actual: dims.len(),
            });
        }
        Kahen::new(dims)
    }
}

impl<const N: usize> LayoutShape for Kotei<N> {
    type Strides = [isize; N];

    fn zeroed_strides(&self) -> Self::Strides {
        [0; N]
    }

    fn with_same_rank_dims(&self, dims: &[usize]) -> Result<Self, ShapeError> {
        if dims.len() != N {
            return Err(ShapeError::RankMismatch {
                expected: N,
                actual: dims.len(),
            });
        }
        let array: [usize; N] = dims.try_into().map_err(|_| ShapeError::RankMismatch {
            expected: N,
            actual: dims.len(),
        })?;
        Kotei::<N>::new(array)
    }
}
