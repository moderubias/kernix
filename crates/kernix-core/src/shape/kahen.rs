use core::ops::Index;

use smallvec::SmallVec;

use super::{
    INLINE_RANK, Kotei, Shape, ShapeError,
    axis::normalize_axis,
    broadcast::{broadcast_dims, can_broadcast_to},
    utils::{numel, validate_dims},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Kahen {
    dims: SmallVec<[usize; INLINE_RANK]>,
}

impl Kahen {
    pub fn new(dims: impl AsRef<[usize]>) -> Result<Self, ShapeError> {
        let dims = dims.as_ref();

        validate_dims(dims)?;

        Ok(Self::from_validated_dims(dims))
    }

    #[inline]
    pub fn scalar() -> Self {
        Self {
            dims: SmallVec::new(),
        }
    }

    #[inline]
    pub fn dims(&self) -> &[usize] {
        &self.dims
    }

    #[inline]
    pub fn rank(&self) -> usize {
        self.dims.len()
    }

    #[inline]
    pub fn is_scalar(&self) -> bool {
        self.dims.is_empty()
    }

    #[inline]
    pub fn is_zero_sized(&self) -> bool {
        self.dims.contains(&0)
    }

    #[inline]
    pub fn numel(&self) -> usize {
        numel(&self.dims)
    }

    #[inline]
    pub fn axis(&self, axis: isize) -> Result<usize, ShapeError> {
        normalize_axis(self.rank(), axis)
    }

    #[inline]
    pub fn dim(&self, axis: isize) -> Result<usize, ShapeError> {
        Ok(self.dims[self.axis(axis)?])
    }

    pub fn with_dim(&self, axis: isize, value: usize) -> Result<Self, ShapeError> {
        let axis = self.axis(axis)?;

        let mut dims = self.dims.clone();
        dims[axis] = value;

        validate_dims(&dims)?;

        Ok(Self { dims })
    }

    pub fn insert_axis(&self, axis: usize) -> Result<Self, ShapeError> {
        let rank = self.rank();

        if axis > rank {
            return Err(ShapeError::AxisPositionOutOfBounds { axis, rank });
        }

        let mut dims = self.dims.clone();
        dims.insert(axis, 1);

        Ok(Self { dims })
    }

    pub fn squeeze_axis(&self, axis: usize) -> Result<Self, ShapeError> {
        let rank = self.rank();

        if axis >= rank {
            return Err(ShapeError::AxisPositionOutOfBounds { axis, rank });
        }

        let dim = self.dims[axis];

        if dim != 1 {
            return Err(ShapeError::AxisNotSingleton { axis, dim });
        }

        let mut dims = self.dims.clone();
        dims.remove(axis);

        Ok(Self { dims })
    }

    pub fn permute(&self, axes: impl AsRef<[usize]>) -> Result<Self, ShapeError> {
        let axes = axes.as_ref();
        let rank = self.rank();

        if axes.len() != rank {
            return Err(ShapeError::RankMismatch {
                expected: rank,
                actual: axes.len(),
            });
        }

        let mut seen = SmallVec::<[bool; INLINE_RANK]>::new();
        seen.resize(rank, false);

        let mut dims = SmallVec::<[usize; INLINE_RANK]>::with_capacity(rank);

        for &axis in axes {
            if axis >= rank {
                return Err(ShapeError::AxisPositionOutOfBounds { axis, rank });
            }

            if seen[axis] {
                return Err(ShapeError::DuplicateAxis { axis });
            }

            seen[axis] = true;
            dims.push(self.dims[axis]);
        }

        Ok(Self { dims })
    }

    pub fn can_broadcast_to<S>(&self, target: &S) -> bool
    where
        S: Shape + ?Sized,
    {
        can_broadcast_to(self.dims(), target.dims())
    }

    pub fn broadcast<S>(&self, other: &S) -> Result<Self, ShapeError>
    where
        S: Shape + ?Sized,
    {
        let dims = broadcast_dims(self.dims(), other.dims())?;

        Ok(Self { dims })
    }

    pub fn broadcast_shapes<'a>(
        shapes: impl IntoIterator<Item = &'a Self>,
    ) -> Result<Self, ShapeError> {
        shapes
            .into_iter()
            .try_fold(Self::scalar(), |acc, shape| acc.broadcast(shape))
    }

    #[inline]
    pub(crate) fn from_validated_dims(dims: impl AsRef<[usize]>) -> Self {
        Self {
            dims: SmallVec::from_slice(dims.as_ref()),
        }
    }
}

impl Shape for Kahen {
    #[inline]
    fn dims(&self) -> &[usize] {
        &self.dims
    }
}

impl AsRef<[usize]> for Kahen {
    #[inline]
    fn as_ref(&self) -> &[usize] {
        &self.dims
    }
}

impl Index<usize> for Kahen {
    type Output = usize;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.dims[index]
    }
}

impl<'a> IntoIterator for &'a Kahen {
    type Item = &'a usize;
    type IntoIter = core::slice::Iter<'a, usize>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.dims.iter()
    }
}

impl<const N: usize> TryFrom<[usize; N]> for Kahen {
    type Error = ShapeError;

    #[inline]
    fn try_from(dims: [usize; N]) -> Result<Self, Self::Error> {
        Self::new(dims)
    }
}

impl TryFrom<&[usize]> for Kahen {
    type Error = ShapeError;

    #[inline]
    fn try_from(dims: &[usize]) -> Result<Self, Self::Error> {
        Self::new(dims)
    }
}

impl<const N: usize> From<Kotei<N>> for Kahen {
    #[inline]
    fn from(shape: Kotei<N>) -> Self {
        Self::from_validated_dims(shape.dims_array())
    }
}

impl<const N: usize> From<&Kotei<N>> for Kahen {
    #[inline]
    fn from(shape: &Kotei<N>) -> Self {
        Self::from_validated_dims(shape.dims_array())
    }
}
