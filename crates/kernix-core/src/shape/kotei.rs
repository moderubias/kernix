use core::ops::Index;

use super::{
    Kahen, Shape, ShapeError,
    axis::normalize_axis,
    broadcast::{broadcast_dims, can_broadcast_to},
    utils::{numel, validate_dims},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Kotei<const N: usize> {
    dims: [usize; N],
}

impl<const N: usize> Kotei<N> {
    pub fn new(dims: [usize; N]) -> Result<Self, ShapeError> {
        validate_dims(&dims)?;

        Ok(Self { dims })
    }

    #[inline]
    pub const fn dims_array(&self) -> &[usize; N] {
        &self.dims
    }

    #[inline]
    pub fn dims(&self) -> &[usize] {
        &self.dims
    }

    #[inline]
    pub const fn rank(&self) -> usize {
        N
    }

    #[inline]
    pub const fn is_scalar(&self) -> bool {
        N == 0
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
        normalize_axis(N, axis)
    }

    #[inline]
    pub fn dim(&self, axis: isize) -> Result<usize, ShapeError> {
        Ok(self.dims[self.axis(axis)?])
    }

    pub fn with_dim(&self, axis: isize, value: usize) -> Result<Self, ShapeError> {
        let axis = self.axis(axis)?;

        let mut dims = self.dims;
        dims[axis] = value;

        Self::new(dims)
    }

    pub fn permute(&self, axes: [usize; N]) -> Result<Self, ShapeError> {
        let mut seen = [false; N];
        let mut dims = [0usize; N];

        for (dst, &src) in axes.iter().enumerate() {
            if src >= N {
                return Err(ShapeError::AxisPositionOutOfBounds { axis: src, rank: N });
            }

            if seen[src] {
                return Err(ShapeError::DuplicateAxis { axis: src });
            }

            seen[src] = true;
            dims[dst] = self.dims[src];
        }

        Ok(Self { dims })
    }

    pub fn insert_axis(&self, axis: usize) -> Result<Kahen, ShapeError> {
        self.to_kahen().insert_axis(axis)
    }

    pub fn squeeze_axis(&self, axis: usize) -> Result<Kahen, ShapeError> {
        self.to_kahen().squeeze_axis(axis)
    }

    pub fn can_broadcast_to<S>(&self, target: &S) -> bool
    where
        S: Shape + ?Sized,
    {
        can_broadcast_to(self.dims(), target.dims())
    }

    pub fn broadcast<S>(&self, other: &S) -> Result<Kahen, ShapeError>
    where
        S: Shape + ?Sized,
    {
        let dims = broadcast_dims(self.dims(), other.dims())?;

        Ok(Kahen::from_validated_dims(dims))
    }

    #[inline]
    pub fn to_kahen(&self) -> Kahen {
        Kahen::from_validated_dims(self.dims)
    }
}

impl Kotei<0> {
    #[inline]
    pub const fn scalar() -> Self {
        Self { dims: [] }
    }
}

impl<const N: usize> Shape for Kotei<N> {
    #[inline]
    fn dims(&self) -> &[usize] {
        &self.dims
    }

    #[inline]
    fn rank(&self) -> usize {
        N
    }
}

impl<const N: usize> AsRef<[usize]> for Kotei<N> {
    #[inline]
    fn as_ref(&self) -> &[usize] {
        &self.dims
    }
}

impl<const N: usize> Index<usize> for Kotei<N> {
    type Output = usize;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.dims[index]
    }
}

impl<'a, const N: usize> IntoIterator for &'a Kotei<N> {
    type Item = &'a usize;
    type IntoIter = core::slice::Iter<'a, usize>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.dims.iter()
    }
}

impl<const N: usize> TryFrom<[usize; N]> for Kotei<N> {
    type Error = ShapeError;

    #[inline]
    fn try_from(dims: [usize; N]) -> Result<Self, Self::Error> {
        Self::new(dims)
    }
}

impl<const N: usize> TryFrom<&Kahen> for Kotei<N> {
    type Error = ShapeError;

    fn try_from(shape: &Kahen) -> Result<Self, Self::Error> {
        if shape.rank() != N {
            return Err(ShapeError::RankMismatch {
                expected: N,
                actual: shape.rank(),
            });
        }

        let dims: [usize; N] = shape
            .dims()
            .try_into()
            .expect("Kahen rank was checked before conversion");

        Ok(Self { dims })
    }
}

impl<const N: usize> TryFrom<Kahen> for Kotei<N> {
    type Error = ShapeError;

    #[inline]
    fn try_from(shape: Kahen) -> Result<Self, Self::Error> {
        Self::try_from(&shape)
    }
}
