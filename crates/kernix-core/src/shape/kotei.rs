use core::{fmt, ops::Index};

use super::{
    Kahen, Shape, ShapeError,
    axis::normalize_axis,
    broadcast::{broadcast_dims, can_broadcast_to},
    utils::{numel, validate_dims},
};

/// A validated fixed-rank shape with N dimensions stored inline in an array.
///
/// Kotei means fixed in Japanese. The rank is fixed, while dimension sizes are
/// runtime values. Rank-changing operations return a dynamic shape.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Kotei<const N: usize> {
    dims: [usize; N],
}

impl<const N: usize> Kotei<N> {
    /// Validate dimensions and construct a shape in O(rank). Any zero makes the
    /// element count zero; otherwise an overflowing product returns an error.
    ///
    /// # Examples
    ///
    /// ```
    /// use kernix_core::shape::{Kahen, Kotei};
    /// let shape = Kotei::<3>::new([2,1,3])?;
    /// # Ok::<(), kernix_core::shape::ShapeError>(())
    /// ```
    pub fn new(dims: [usize; N]) -> Result<Self, ShapeError> {
        validate_dims(&dims)?;

        Ok(Self { dims })
    }

    /// Borrow the fixed-size dimension array without copying. O(1).
    #[inline]
    ///
    /// # Examples
    ///
    /// ```
    /// use kernix_core::shape::{Kahen, Kotei};
    /// let shape = Kotei::<3>::new([2,1,3])?;
    /// assert_eq!(shape.dims_array(), &[2,1,3]);
    /// # Ok::<(), kernix_core::shape::ShapeError>(())
    /// ```
    pub const fn dims_array(&self) -> &[usize; N] {
        &self.dims
    }

    /// Borrow the dimension sizes in axis order, without allocating. O(1).
    #[inline]
    ///
    /// # Examples
    ///
    /// ```
    /// use kernix_core::shape::{Kahen, Kotei};
    /// let shape = Kotei::<3>::new([2,1,3])?;
    /// assert_eq!(shape.dims(), &[2,1,3]);
    /// # Ok::<(), kernix_core::shape::ShapeError>(())
    /// ```
    pub fn dims(&self) -> &[usize] {
        &self.dims
    }

    /// Return the number of axes. Scalars have rank zero. O(1).
    #[inline]
    ///
    /// # Examples
    ///
    /// ```
    /// use kernix_core::shape::{Kahen, Kotei};
    /// let shape = Kotei::<3>::new([2,1,3])?;
    /// assert_eq!(shape.rank(),3);
    /// # Ok::<(), kernix_core::shape::ShapeError>(())
    /// ```
    pub const fn rank(&self) -> usize {
        N
    }

    /// Return whether the shape has no axes (and therefore one element). O(1).
    #[inline]
    ///
    /// # Examples
    ///
    /// ```
    /// use kernix_core::shape::{Kahen, Kotei};
    /// let shape = Kotei::<3>::new([2,1,3])?;
    /// assert!(!shape.is_scalar());
    /// # Ok::<(), kernix_core::shape::ShapeError>(())
    /// ```
    pub const fn is_scalar(&self) -> bool {
        N == 0
    }

    /// Return whether any dimension is zero. O(rank).
    #[inline]
    ///
    /// # Examples
    ///
    /// ```
    /// use kernix_core::shape::{Kahen, Kotei};
    /// let shape = Kotei::<3>::new([2,1,3])?;
    /// assert!(!shape.is_zero_sized());
    /// # Ok::<(), kernix_core::shape::ShapeError>(())
    /// ```
    pub fn is_zero_sized(&self) -> bool {
        self.dims.contains(&0)
    }

    /// Return the logical element count. Scalars contain one element; any zero
    /// dimension makes the count zero. Shape validation prevents product overflow.
    #[inline]
    ///
    /// # Examples
    ///
    /// ```
    /// use kernix_core::shape::{Kahen, Kotei};
    /// let shape = Kotei::<3>::new([2,1,3])?;
    /// assert_eq!(shape.numel(),6);
    /// # Ok::<(), kernix_core::shape::ShapeError>(())
    /// ```
    pub fn numel(&self) -> usize {
        numel(&self.dims)
    }

    /// Normalize a signed axis into 0..rank; -1 selects the last axis.
    /// Returns an out-of-bounds error for invalid axes, including any axis on a scalar.
    #[inline]
    ///
    /// # Examples
    ///
    /// ```
    /// use kernix_core::shape::{Kahen, Kotei};
    /// let shape = Kotei::<3>::new([2,1,3])?;
    /// assert_eq!(shape.axis(-1)?,2);
    /// # Ok::<(), kernix_core::shape::ShapeError>(())
    /// ```
    pub fn axis(&self, axis: isize) -> Result<usize, ShapeError> {
        normalize_axis(N, axis)
    }

    /// Return the dimension at a signed axis, or an axis error. O(1).
    #[inline]
    ///
    /// # Examples
    ///
    /// ```
    /// use kernix_core::shape::{Kahen, Kotei};
    /// let shape = Kotei::<3>::new([2,1,3])?;
    /// assert_eq!(shape.dim(-1)?,3);
    /// # Ok::<(), kernix_core::shape::ShapeError>(())
    /// ```
    pub fn dim(&self, axis: isize) -> Result<usize, ShapeError> {
        Ok(self.dims[self.axis(axis)?])
    }

    /// Return a shape with one dimension replaced, revalidating element count.
    /// Zero is allowed. Invalid axes and overflowing nonzero products return errors. O(rank).
    ///
    /// # Examples
    ///
    /// ```
    /// use kernix_core::shape::{Kahen, Kotei};
    /// let shape = Kotei::<3>::new([2,1,3])?;
    /// assert_eq!(shape.with_dim(-1,4)?.numel(),8);
    /// # Ok::<(), kernix_core::shape::ShapeError>(())
    /// ```
    pub fn with_dim(&self, axis: isize, value: usize) -> Result<Self, ShapeError> {
        let axis = self.axis(axis)?;

        let mut dims = self.dims;
        dims[axis] = value;

        Self::new(dims)
    }

    /// Swap two existing axes without erasing the fixed rank.
    ///
    /// # Examples
    ///
    /// ```
    /// use kernix_core::shape::{Kahen, Kotei};
    /// let shape = Kotei::<3>::new([2,1,3])?;
    /// assert_eq!(shape.swap_axes(0,2)?.dims(),&[3,1,2]);
    /// # Ok::<(), kernix_core::shape::ShapeError>(())
    /// ```
    pub fn swap_axes(&self, left: usize, right: usize) -> Result<Self, ShapeError> {
        if left >= N {
            return Err(ShapeError::AxisPositionOutOfBounds {
                axis: left,
                rank: N,
            });
        }
        if right >= N {
            return Err(ShapeError::AxisPositionOutOfBounds {
                axis: right,
                rank: N,
            });
        }
        let mut dims = self.dims;
        dims.swap(left, right);
        Ok(Self { dims })
    }

    /// Reorder axes according to a complete permutation.
    /// Returns an error for duplicate, missing or out-of-range axes. O(rank).
    ///
    /// # Examples
    ///
    /// ```
    /// use kernix_core::shape::{Kahen, Kotei};
    /// let shape = Kotei::<3>::new([2,1,3])?;
    /// assert_eq!(shape.permute([2,0,1])?.dims(),&[3,2,1]);
    /// # Ok::<(), kernix_core::shape::ShapeError>(())
    /// ```
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

    /// Insert a singleton axis at a position in 0..=rank.
    /// Returns an axis error outside that range. Element count is unchanged. O(rank).
    ///
    /// # Examples
    ///
    /// ```
    /// use kernix_core::shape::{Kahen, Kotei};
    /// let shape = Kotei::<3>::new([2,1,3])?;
    /// assert_eq!(shape.insert_axis(0)?.rank(),4);
    /// # Ok::<(), kernix_core::shape::ShapeError>(())
    /// ```
    pub fn insert_axis(&self, axis: usize) -> Result<Kahen, ShapeError> {
        self.to_kahen().insert_axis(axis)
    }

    /// Remove an axis, returning a dynamic-rank shape and revalidating numel.
    ///
    /// # Examples
    ///
    /// ```
    /// use kernix_core::shape::{Kahen, Kotei};
    /// let shape = Kotei::<3>::new([2,1,3])?;
    /// assert_eq!(shape.remove_axis(1)?.dims(),&[2,3]);
    /// # Ok::<(), kernix_core::shape::ShapeError>(())
    /// ```
    pub fn remove_axis(&self, axis: usize) -> Result<Kahen, ShapeError> {
        self.to_kahen().remove_axis(axis)
    }

    /// Remove all singleton axes, returning a dynamic-rank shape.
    ///
    /// # Examples
    ///
    /// ```
    /// use kernix_core::shape::{Kahen, Kotei};
    /// let shape = Kotei::<3>::new([2,1,3])?;
    /// assert_eq!(shape.squeeze().dims(),&[2,3]);
    /// # Ok::<(), kernix_core::shape::ShapeError>(())
    /// ```
    pub fn squeeze(&self) -> Kahen {
        self.to_kahen().squeeze()
    }

    /// Insert a singleton axis; alias of [`Self::insert_axis`].
    ///
    /// # Examples
    ///
    /// ```
    /// use kernix_core::shape::{Kahen, Kotei};
    /// let shape = Kotei::<3>::new([2,1,3])?;
    /// assert_eq!(shape.unsqueeze(3)?.rank(),4);
    /// # Ok::<(), kernix_core::shape::ShapeError>(())
    /// ```
    pub fn unsqueeze(&self, axis: usize) -> Result<Kahen, ShapeError> {
        self.insert_axis(axis)
    }

    /// Remove a singleton axis, returning an error for an invalid axis or a
    /// non-singleton dimension. Element count is unchanged. O(rank).
    ///
    /// # Examples
    ///
    /// ```
    /// use kernix_core::shape::{Kahen, Kotei};
    /// let shape = Kotei::<3>::new([2,1,3])?;
    /// assert_eq!(shape.squeeze_axis(1)?.dims(),&[2,3]);
    /// # Ok::<(), kernix_core::shape::ShapeError>(())
    /// ```
    pub fn squeeze_axis(&self, axis: usize) -> Result<Kahen, ShapeError> {
        self.to_kahen().squeeze_axis(axis)
    }

    /// Test directional broadcasting to the target: source rank cannot exceed
    /// target rank, and each trailing source dimension must equal the target or one.
    /// A singleton broadcasts to zero, but zero cannot broadcast to one. O(max rank).
    ///
    /// # Examples
    ///
    /// ```
    /// use kernix_core::shape::{Kahen, Kotei};
    /// let shape = Kotei::<3>::new([2,1,3])?;
    /// assert!(shape.can_broadcast_to(&Kahen::new([2,4,3])?));
    /// # Ok::<(), kernix_core::shape::ShapeError>(())
    /// ```
    pub fn can_broadcast_to<S>(&self, target: &S) -> bool
    where
        S: Shape + ?Sized,
    {
        can_broadcast_to(self.dims(), target.dims())
    }

    /// Compute the smallest common trailing-axis broadcast shape.
    /// Equal dimensions are retained; a singleton takes the other dimension (including zero).
    /// Incompatible dimensions or an overflowing result return an error. O(max rank).
    ///
    /// # Examples
    ///
    /// ```
    /// use kernix_core::shape::{Kahen, Kotei};
    /// let shape = Kotei::<3>::new([2,1,3])?;
    /// assert_eq!(shape.broadcast(&Kahen::new([4,3])?)?.dims(),&[2,4,3]);
    /// # Ok::<(), kernix_core::shape::ShapeError>(())
    /// ```
    pub fn broadcast<S>(&self, other: &S) -> Result<Kahen, ShapeError>
    where
        S: Shape + ?Sized,
    {
        let dims = broadcast_dims(self.dims(), other.dims())?;

        Ok(Kahen::from_validated_dims(dims))
    }

    /// Copy dimensions into a dynamic-rank shape. O(rank), with inline storage up to rank six.
    #[inline]
    ///
    /// # Examples
    ///
    /// ```
    /// use kernix_core::shape::{Kahen, Kotei};
    /// let shape = Kotei::<3>::new([2,1,3])?;
    /// assert_eq!(shape.to_kahen().dims(),shape.dims());
    /// # Ok::<(), kernix_core::shape::ShapeError>(())
    /// ```
    pub fn to_kahen(&self) -> Kahen {
        Kahen::from_validated_dims(self.dims)
    }
}

impl Kotei<0> {
    /// Construct a rank-zero scalar with one element and no dimensions.
    #[inline]
    ///
    /// # Examples
    ///
    /// ```
    /// use kernix_core::shape::{Kahen, Kotei};
    /// let shape = Kotei::<0>::scalar();
    /// assert_eq!(shape.numel(),1);
    /// # Ok::<(), kernix_core::shape::ShapeError>(())
    /// ```
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

impl<const N: usize> TryFrom<&[usize]> for Kotei<N> {
    type Error = ShapeError;
    fn try_from(dims: &[usize]) -> Result<Self, Self::Error> {
        let dims = dims.try_into().map_err(|_| ShapeError::RankMismatch {
            expected: N,
            actual: dims.len(),
        })?;
        Self::new(dims)
    }
}

impl<const N: usize> fmt::Display for Kotei<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.dims())
    }
}

#[cfg(feature = "serde")]
impl<const N: usize> serde::Serialize for Kotei<N> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serde::Serialize::serialize(self.dims(), serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de, const N: usize> serde::Deserialize<'de> for Kotei<N> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let shape = <Kahen as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(shape).map_err(serde::de::Error::custom)
    }
}
