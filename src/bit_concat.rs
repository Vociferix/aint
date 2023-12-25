use crate::{int::Int, width, WrappingInto};

/// Concatenate two integral types bitwise into a larger integral type.
///
/// # Example:
/// ```
/// # use aint::BitConcat;
/// let concat = u16::bit_concat(0b10101010u8, 0b01010101u8);
/// assert_eq!(concat, 0b10101010_01010101_u16);
/// ```
pub trait BitConcat<L, R>: Sized {
    fn bit_concat(left: L, right: R) -> Self;
}

impl<T: Int, L: Int, R: Int> BitConcat<L, R> for T
where
    <L::Width as width::Width>::Add<R::Width>: width::WidthEq<T::Width>,
    L: WrappingInto<T>,
    R: WrappingInto<T>,
{
    fn bit_concat(left: L, right: R) -> Self {
        let l: Self = left.wrapping_into();
        let r: Self = right.wrapping_into();
        (l << R::BITS) | r
    }
}
