/// Preferred iteration order when constructing or reshaping a contiguous layout.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Order {
    /// Last axis changes fastest (row-major).
    C,
    /// First axis changes fastest (column-major).
    F,
}
