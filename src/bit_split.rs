use crate::{int::Int, width, WrappingInto};

/// Split an integral type bitwise into two smaller integral types.
///
/// # Example:
/// ```
/// # use aint::BitSplit;
/// let (upper, lower): (u8, u8) = 0b10101010_01010101_u16.bit_split();
/// assert_eq!(upper, 0b10101010);
/// assert_eq!(lower, 0b01010101);
/// ```
pub trait BitSplit<L, R>: Sized {
    fn bit_split(self) -> (L, R);
}

impl<T: Int, L: Int, R: Int> BitSplit<L, R> for T
where
    <L::Width as width::Width>::Add<R::Width>: width::WidthEq<T::Width>,
    T: WrappingInto<L> + WrappingInto<R>,
{
    fn bit_split(self) -> (L, R) {
        ((self >> R::BITS).wrapping_into(), self.wrapping_into())
    }
}
