use crate::layout::{Layout, LayoutError, LayoutShape};

use super::{HostBuffer, Storage};

/// Checked, read-only binding between a host allocation and a strided layout.
/// The binding borrows both objects and never transfers their ownership.
pub struct StorageView<'a, T, S: LayoutShape> {
    elements: &'a [T],
    layout: &'a Layout<S>,
}

/// Checked, exclusive binding. Even overlapping logical indices can be
/// accessed safely, one-at-a-time, via `get_mut(&mut self, ...)`. This type
/// intentionally exposes no unchecked multi-element mutable iterator.
pub struct StorageViewMut<'a, T, S: LayoutShape> {
    elements: &'a mut [T],
    layout: &'a Layout<S>,
}

impl<T, B: HostBuffer<T>> Storage<T, B> {
    /// Reject any layout that can reach beyond the host slice's allocation.
    /// The byte-extent check deliberately uses the actual host slice length,
    /// not merely the backend's reported logical element count.
    pub fn view<'a, S: LayoutShape>(
        &'a self,
        layout: &'a Layout<S>,
    ) -> Result<StorageView<'a, T, S>, LayoutError> {
        let elements = self.as_slice();
        layout.validate_cpu_storage::<T>(elements.len())?;
        Ok(StorageView { elements, layout })
    }

    pub fn view_mut<'a, S: LayoutShape>(
        &'a mut self,
        layout: &'a Layout<S>,
    ) -> Result<StorageViewMut<'a, T, S>, LayoutError> {
        let elements = self.as_mut_slice();
        layout.validate_cpu_storage::<T>(elements.len())?;
        Ok(StorageViewMut { elements, layout })
    }
}

impl<'a, T, S: LayoutShape> StorageView<'a, T, S> {
    pub fn layout(&self) -> &'a Layout<S> {
        self.layout
    }

    pub fn get(&self, indices: &[usize]) -> Result<&T, LayoutError> {
        let offset = self.layout.offset_of(indices)?;
        // `view()` has already checked the entire reachable storage span.
        Ok(&self.elements[offset])
    }
}

impl<'a, T, S: LayoutShape> StorageViewMut<'a, T, S> {
    pub fn layout(&self) -> &'a Layout<S> {
        self.layout
    }

    pub fn get(&self, indices: &[usize]) -> Result<&T, LayoutError> {
        let offset = self.layout.offset_of(indices)?;
        Ok(&self.elements[offset])
    }

    /// Borrowing `&mut self` prevents independent live `&mut T` values even
    /// when two logical indices address the same physical element.
    pub fn get_mut(&mut self, indices: &[usize]) -> Result<&mut T, LayoutError> {
        let offset = self.layout.offset_of(indices)?;
        Ok(&mut self.elements[offset])
    }

    pub fn into_read_only(self) -> StorageView<'a, T, S> {
        StorageView {
            elements: self.elements,
            layout: self.layout,
        }
    }
}
