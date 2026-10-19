use alloc::vec::Vec;
use core::{fmt, ops::Index};

use smallvec::SmallVec;

use super::{
    INLINE_RANK, Kotei, Shape, ShapeError,
    axis::normalize_axis,
    broadcast::{broadcast_dims, can_broadcast_to},
    utils::{numel, validate_dims},
};

/// A validated dynamic-rank shape. Kahen means variable in Japanese.
///
/// Ranks up to six are stored inline; larger ranks use the heap. A scalar has no
/// axes and one element. Dimensions may be zero; the nonzero product must fit usize.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Kahen {
    dims: SmallVec<[usize; INLINE_RANK]>,
}

impl Kahen {
    /// Validate dimensions and construct a shape in O(rank). Any zero makes the
    /// element count zero; otherwise an overflowing product returns an error.
    ///
    /// # Examples
    ///
    /// ```
    /// use kernix_core::shape::{Kahen, Kotei};
    /// let shape = Kahen::new([2,1,3])?;
    /// # Ok::<(), kernix_core::shape::ShapeError>(())
    /// ```
    pub fn new(dims: impl AsRef<[usize]>) -> Result<Self, ShapeError> {
        let dims = dims.as_ref();

        validate_dims(dims)?;

        Ok(Self::from_validated_dims(dims))
    }

    /// Construct a rank-zero scalar with one element and no dimensions.
    #[inline]
    ///
    /// # Examples
    ///
    /// ```
    /// use kernix_core::shape::{Kahen, Kotei};
    /// let shape = Kahen::scalar();
    /// assert_eq!(shape.numel(),1);
    /// # Ok::<(), kernix_core::shape::ShapeError>(())
    /// ```
    pub const fn scalar() -> Self {
        Self {
            dims: SmallVec::new_const(),
        }
    }

    /// Borrow the dimension sizes in axis order, without allocating. O(1).
    #[inline]
    ///
    /// # Examples
    ///
    /// ```
    /// use kernix_core::shape::{Kahen, Kotei};
    /// let shape = Kahen::new([2,1,3])?;
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
    /// let shape = Kahen::new([2,1,3])?;
    /// assert_eq!(shape.rank(),3);
    /// # Ok::<(), kernix_core::shape::ShapeError>(())
    /// ```
    pub fn rank(&self) -> usize {
        self.dims.len()
    }

    /// Return whether the shape has no axes (and therefore one element). O(1).
    #[inline]
    ///
    /// # Examples
    ///
    /// ```
    /// use kernix_core::shape::{Kahen, Kotei};
    /// let shape = Kahen::new([2,1,3])?;
    /// assert!(!shape.is_scalar());
    /// # Ok::<(), kernix_core::shape::ShapeError>(())
    /// ```
    pub fn is_scalar(&self) -> bool {
        self.dims.is_empty()
    }

    /// Return whether any dimension is zero. O(rank).
    #[inline]
    ///
    /// # Examples
    ///
    /// ```
    /// use kernix_core::shape::{Kahen, Kotei};
    /// let shape = Kahen::new([2,1,3])?;
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
    /// let shape = Kahen::new([2,1,3])?;
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
    /// let shape = Kahen::new([2,1,3])?;
    /// assert_eq!(shape.axis(-1)?,2);
    /// # Ok::<(), kernix_core::shape::ShapeError>(())
    /// ```
    pub fn axis(&self, axis: isize) -> Result<usize, ShapeError> {
        normalize_axis(self.rank(), axis)
    }

    /// Return the dimension at a signed axis, or an axis error. O(1).
    #[inline]
    ///
    /// # Examples
    ///
    /// ```
    /// use kernix_core::shape::{Kahen, Kotei};
    /// let shape = Kahen::new([2,1,3])?;
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
    /// let shape = Kahen::new([2,1,3])?;
    /// assert_eq!(shape.with_dim(-1,4)?.numel(),8);
    /// # Ok::<(), kernix_core::shape::ShapeError>(())
    /// ```
    pub fn with_dim(&self, axis: isize, value: usize) -> Result<Self, ShapeError> {
        let axis = self.axis(axis)?;

        let mut dims = self.dims.clone();
        dims[axis] = value;

        validate_dims(&dims)?;

        Ok(Self { dims })
    }

    /// Insert a singleton axis at a position in 0..=rank.
    /// Returns an axis error outside that range. Element count is unchanged. O(rank).
    ///
    /// # Examples
    ///
    /// ```
    /// use kernix_core::shape::{Kahen, Kotei};
    /// let shape = Kahen::new([2,1,3])?;
    /// assert_eq!(shape.insert_axis(0)?.rank(),4);
    /// # Ok::<(), kernix_core::shape::ShapeError>(())
    /// ```
    pub fn insert_axis(&self, axis: usize) -> Result<Self, ShapeError> {
        let rank = self.rank();

        if axis > rank {
            return Err(ShapeError::AxisPositionOutOfBounds { axis, rank });
        }

        let mut dims = self.dims.clone();
        dims.insert(axis, 1);

        Ok(Self { dims })
    }

    /// Remove all singleton axes without changing the number of elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use kernix_core::shape::{Kahen, Kotei};
    /// let shape = Kahen::new([2,1,3])?;
    /// assert_eq!(shape.squeeze().dims(),&[2,3]);
    /// # Ok::<(), kernix_core::shape::ShapeError>(())
    /// ```
    pub fn squeeze(&self) -> Self {
        let dims: SmallVec<[usize; INLINE_RANK]> =
            self.dims.iter().copied().filter(|&dim| dim != 1).collect();
        Self { dims }
    }

    /// Alias of `insert_axis`, corresponding to NumPy/PyTorch terminology.
    #[inline]
    ///
    /// # Examples
    ///
    /// ```
    /// use kernix_core::shape::{Kahen, Kotei};
    /// let shape = Kahen::new([2,1,3])?;
    /// assert_eq!(shape.unsqueeze(3)?.rank(),4);
    /// # Ok::<(), kernix_core::shape::ShapeError>(())
    /// ```
    pub fn unsqueeze(&self, axis: usize) -> Result<Self, ShapeError> {
        self.insert_axis(axis)
    }

    /// Swap two existing axes, retaining the element count.
    ///
    /// # Examples
    ///
    /// ```
    /// use kernix_core::shape::{Kahen, Kotei};
    /// let shape = Kahen::new([2,1,3])?;
    /// assert_eq!(shape.swap_axes(0,2)?.dims(),&[3,1,2]);
    /// # Ok::<(), kernix_core::shape::ShapeError>(())
    /// ```
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

    /// Remove any axis and validate the remaining element count.
    ///
    /// Unlike [`Self::squeeze_axis`], this can change the number of elements.
    /// Removing a zero-sized axis can expose an overflowing product.
    /// Returns an axis or element-count error; runs in O(rank).
    ///
    /// ```
    /// use kernix_core::shape::Kahen;
    /// let shape = Kahen::new([2, 3])?;
    /// assert_eq!(shape.remove_axis(0)?.dims(), &[3]);
    /// # Ok::<(), kernix_core::shape::ShapeError>(())
    /// ```
    pub fn remove_axis(&self, axis: usize) -> Result<Self, ShapeError> {
        if axis >= self.rank() {
            return Err(ShapeError::AxisPositionOutOfBounds {
                axis,
                rank: self.rank(),
            });
        }
        let mut dims = self.dims.clone();
        dims.remove(axis);
        Self::new(dims)
    }

    /// Remove a singleton axis, returning an error for an invalid axis or a
    /// non-singleton dimension. Element count is unchanged. O(rank).
    ///
    /// # Examples
    ///
    /// ```
    /// use kernix_core::shape::{Kahen, Kotei};
    /// let shape = Kahen::new([2,1,3])?;
    /// assert_eq!(shape.squeeze_axis(1)?.dims(),&[2,3]);
    /// # Ok::<(), kernix_core::shape::ShapeError>(())
    /// ```
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

    /// Reorder axes according to a complete permutation.
    /// Returns an error for duplicate, missing or out-of-range axes. O(rank).
    ///
    /// # Examples
    ///
    /// ```
    /// use kernix_core::shape::{Kahen, Kotei};
    /// let shape = Kahen::new([2,1,3])?;
    /// assert_eq!(shape.permute([2,0,1])?.dims(),&[3,2,1]);
    /// # Ok::<(), kernix_core::shape::ShapeError>(())
    /// ```
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

    /// Test directional broadcasting to the target: source rank cannot exceed
    /// target rank, and each trailing source dimension must equal the target or one.
    /// A singleton broadcasts to zero, but zero cannot broadcast to one. O(max rank).
    ///
    /// # Examples
    ///
    /// ```
    /// use kernix_core::shape::{Kahen, Kotei};
    /// let shape = Kahen::new([2,1,3])?;
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
    /// let shape = Kahen::new([2,1,3])?;
    /// assert_eq!(shape.broadcast(&Kahen::new([4,3])?)?.dims(),&[2,4,3]);
    /// # Ok::<(), kernix_core::shape::ShapeError>(())
    /// ```
    pub fn broadcast<S>(&self, other: &S) -> Result<Self, ShapeError>
    where
        S: Shape + ?Sized,
    {
        let dims = broadcast_dims(self.dims(), other.dims())?;

        Ok(Self { dims })
    }

    /// Broadcast a sequence of shapes, returning a scalar for an empty sequence.
    /// Returns the first incompatibility or overflowing result.
    ///
    /// # Examples
    ///
    /// ```
    /// use kernix_core::shape::{Kahen, Kotei};
    /// let shape = Kahen::new([2,1,3])?;
    /// assert_eq!(Kahen::broadcast_shapes([&shape])?.dims(), shape.dims());
    /// # Ok::<(), kernix_core::shape::ShapeError>(())
    /// ```
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

impl TryFrom<Vec<usize>> for Kahen {
    type Error = ShapeError;
    fn try_from(dims: Vec<usize>) -> Result<Self, Self::Error> {
        validate_dims(&dims)?;
        Ok(Self {
            dims: SmallVec::from_vec(dims),
        })
    }
}

impl fmt::Display for Kahen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.dims())
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Kahen {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serde::Serialize::serialize(self.dims(), serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Kahen {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let dims = <Vec<usize> as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(dims).map_err(serde::de::Error::custom)
    }
}
