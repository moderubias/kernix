use core::iter::FusedIterator;
use smallvec::SmallVec;

use super::{TensorError, index::unravel};
use crate::layout::{Layout, LayoutShape, Order};

/// Iterates logical elements in C- or F-order, without copying data.
/// Supports strided, reversed, broadcast, empty, and scalar layouts.
pub struct TensorIter<'a, T, S: LayoutShape> {
    elements: &'a [T],
    layout: Layout<S>,
    order: Order,
    coords: SmallVec<[usize; 6]>,
    front: usize,
    back: usize,
}

impl<'a, T, S: LayoutShape> TensorIter<'a, T, S> {
    pub fn new(elements: &'a [T], layout: Layout<S>, order: Order) -> Result<Self, TensorError> {
        layout.validate_cpu_storage::<T>(elements.len())?;
        let coords = core::iter::repeat_n(0, layout.rank()).collect();
        let back = layout.numel();
        Ok(Self {
            elements,
            layout,
            order,
            coords,
            front: 0,
            back,
        })
    }

    fn at(&mut self, index: usize) -> &'a T {
        unravel(
            index,
            self.layout.shape().dims(),
            self.order,
            &mut self.coords,
        );
        let offset = self
            .layout
            .offset_of(&self.coords)
            .expect("validated iterator indices must map into the layout");
        let elements: &'a [T] = self.elements;
        &elements[offset]
    }
}

impl<'a, T, S: LayoutShape> Iterator for TensorIter<'a, T, S> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.front == self.back {
            return None;
        }
        let index = self.front;
        self.front += 1;
        Some(self.at(index))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let left = self.back - self.front;
        (left, Some(left))
    }
}

impl<'a, T, S: LayoutShape> DoubleEndedIterator for TensorIter<'a, T, S> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.front == self.back {
            return None;
        }
        self.back -= 1;
        let index = self.back;
        Some(self.at(index))
    }
}

impl<'a, T, S: LayoutShape> ExactSizeIterator for TensorIter<'a, T, S> {}
impl<'a, T, S: LayoutShape> FusedIterator for TensorIter<'a, T, S> {}
