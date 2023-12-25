use crate::{prim::Prim, sealed::Sealed, Aint};

/// Wrapping conversion from one type into another.
///
/// When the source value is too large to be represented
/// by the destination type, the resulting value is
/// calculated by wrapping the source value as though
/// it were calculated with wrapping operations, such
/// as `i32::wrapping_add`. In practice, the source value
/// is simply truncated, bitwise.
///
/// # Example:
/// ```
/// # use aint::WrappingFrom;
/// let value = u8::wrapping_from(0x1234);
/// assert_eq!(value, 0x34);
/// ```
pub trait WrappingFrom<T> {
    fn wrapping_from(value: T) -> Self;
}

/// Wrapping conversion from one type into another.
///
/// When the source value is too large to be represented
/// by the destination type, the resulting value is
/// calculated by wrapping the source value as though
/// it were calculated with wrapping operations, such
/// as `i32::wrapping_add`. In practice, the source value
/// is simply truncated, bitwise.
///
/// # Example:
/// ```
/// # use aint::WrappingInto;
/// let value: u8 = 0x1234.wrapping_into();
/// assert_eq!(value, 0x34);
/// ```
pub trait WrappingInto<T> {
    fn wrapping_into(self) -> T;
}

impl<F, T> WrappingInto<T> for F
where
    T: WrappingFrom<F>,
{
    fn wrapping_into(self) -> T {
        T::wrapping_from(self)
    }
}

impl WrappingFrom<u8> for u8 {
    fn wrapping_from(value: u8) -> u8 {
        value as u8
    }
}

impl WrappingFrom<u16> for u8 {
    fn wrapping_from(value: u16) -> u8 {
        value as u8
    }
}

impl WrappingFrom<u32> for u8 {
    fn wrapping_from(value: u32) -> u8 {
        value as u8
    }
}

impl WrappingFrom<u64> for u8 {
    fn wrapping_from(value: u64) -> u8 {
        value as u8
    }
}

impl WrappingFrom<u128> for u8 {
    fn wrapping_from(value: u128) -> u8 {
        value as u8
    }
}

impl WrappingFrom<usize> for u8 {
    fn wrapping_from(value: usize) -> u8 {
        value as u8
    }
}

impl WrappingFrom<i8> for u8 {
    fn wrapping_from(value: i8) -> u8 {
        value as u8
    }
}

impl WrappingFrom<i16> for u8 {
    fn wrapping_from(value: i16) -> u8 {
        value as u8
    }
}

impl WrappingFrom<i32> for u8 {
    fn wrapping_from(value: i32) -> u8 {
        value as u8
    }
}

impl WrappingFrom<i64> for u8 {
    fn wrapping_from(value: i64) -> u8 {
        value as u8
    }
}

impl WrappingFrom<i128> for u8 {
    fn wrapping_from(value: i128) -> u8 {
        value as u8
    }
}

impl WrappingFrom<isize> for u8 {
    fn wrapping_from(value: isize) -> u8 {
        value as u8
    }
}

impl WrappingFrom<u8> for u16 {
    fn wrapping_from(value: u8) -> u16 {
        value as u16
    }
}

impl WrappingFrom<u16> for u16 {
    fn wrapping_from(value: u16) -> u16 {
        value as u16
    }
}

impl WrappingFrom<u32> for u16 {
    fn wrapping_from(value: u32) -> u16 {
        value as u16
    }
}

impl WrappingFrom<u64> for u16 {
    fn wrapping_from(value: u64) -> u16 {
        value as u16
    }
}

impl WrappingFrom<u128> for u16 {
    fn wrapping_from(value: u128) -> u16 {
        value as u16
    }
}

impl WrappingFrom<usize> for u16 {
    fn wrapping_from(value: usize) -> u16 {
        value as u16
    }
}

impl WrappingFrom<i8> for u16 {
    fn wrapping_from(value: i8) -> u16 {
        value as u16
    }
}

impl WrappingFrom<i16> for u16 {
    fn wrapping_from(value: i16) -> u16 {
        value as u16
    }
}

impl WrappingFrom<i32> for u16 {
    fn wrapping_from(value: i32) -> u16 {
        value as u16
    }
}

impl WrappingFrom<i64> for u16 {
    fn wrapping_from(value: i64) -> u16 {
        value as u16
    }
}

impl WrappingFrom<i128> for u16 {
    fn wrapping_from(value: i128) -> u16 {
        value as u16
    }
}

impl WrappingFrom<isize> for u16 {
    fn wrapping_from(value: isize) -> u16 {
        value as u16
    }
}

impl WrappingFrom<u8> for u32 {
    fn wrapping_from(value: u8) -> u32 {
        value as u32
    }
}

impl WrappingFrom<u16> for u32 {
    fn wrapping_from(value: u16) -> u32 {
        value as u32
    }
}

impl WrappingFrom<u32> for u32 {
    fn wrapping_from(value: u32) -> u32 {
        value as u32
    }
}

impl WrappingFrom<u64> for u32 {
    fn wrapping_from(value: u64) -> u32 {
        value as u32
    }
}

impl WrappingFrom<u128> for u32 {
    fn wrapping_from(value: u128) -> u32 {
        value as u32
    }
}

impl WrappingFrom<usize> for u32 {
    fn wrapping_from(value: usize) -> u32 {
        value as u32
    }
}

impl WrappingFrom<i8> for u32 {
    fn wrapping_from(value: i8) -> u32 {
        value as u32
    }
}

impl WrappingFrom<i16> for u32 {
    fn wrapping_from(value: i16) -> u32 {
        value as u32
    }
}

impl WrappingFrom<i32> for u32 {
    fn wrapping_from(value: i32) -> u32 {
        value as u32
    }
}

impl WrappingFrom<i64> for u32 {
    fn wrapping_from(value: i64) -> u32 {
        value as u32
    }
}

impl WrappingFrom<i128> for u32 {
    fn wrapping_from(value: i128) -> u32 {
        value as u32
    }
}

impl WrappingFrom<isize> for u32 {
    fn wrapping_from(value: isize) -> u32 {
        value as u32
    }
}

impl WrappingFrom<u8> for u64 {
    fn wrapping_from(value: u8) -> u64 {
        value as u64
    }
}

impl WrappingFrom<u16> for u64 {
    fn wrapping_from(value: u16) -> u64 {
        value as u64
    }
}

impl WrappingFrom<u32> for u64 {
    fn wrapping_from(value: u32) -> u64 {
        value as u64
    }
}

impl WrappingFrom<u64> for u64 {
    fn wrapping_from(value: u64) -> u64 {
        value as u64
    }
}

impl WrappingFrom<u128> for u64 {
    fn wrapping_from(value: u128) -> u64 {
        value as u64
    }
}

impl WrappingFrom<usize> for u64 {
    fn wrapping_from(value: usize) -> u64 {
        value as u64
    }
}

impl WrappingFrom<i8> for u64 {
    fn wrapping_from(value: i8) -> u64 {
        value as u64
    }
}

impl WrappingFrom<i16> for u64 {
    fn wrapping_from(value: i16) -> u64 {
        value as u64
    }
}

impl WrappingFrom<i32> for u64 {
    fn wrapping_from(value: i32) -> u64 {
        value as u64
    }
}

impl WrappingFrom<i64> for u64 {
    fn wrapping_from(value: i64) -> u64 {
        value as u64
    }
}

impl WrappingFrom<i128> for u64 {
    fn wrapping_from(value: i128) -> u64 {
        value as u64
    }
}

impl WrappingFrom<isize> for u64 {
    fn wrapping_from(value: isize) -> u64 {
        value as u64
    }
}

impl WrappingFrom<u8> for u128 {
    fn wrapping_from(value: u8) -> u128 {
        value as u128
    }
}

impl WrappingFrom<u16> for u128 {
    fn wrapping_from(value: u16) -> u128 {
        value as u128
    }
}

impl WrappingFrom<u32> for u128 {
    fn wrapping_from(value: u32) -> u128 {
        value as u128
    }
}

impl WrappingFrom<u64> for u128 {
    fn wrapping_from(value: u64) -> u128 {
        value as u128
    }
}

impl WrappingFrom<u128> for u128 {
    fn wrapping_from(value: u128) -> u128 {
        value as u128
    }
}

impl WrappingFrom<usize> for u128 {
    fn wrapping_from(value: usize) -> u128 {
        value as u128
    }
}

impl WrappingFrom<i8> for u128 {
    fn wrapping_from(value: i8) -> u128 {
        value as u128
    }
}

impl WrappingFrom<i16> for u128 {
    fn wrapping_from(value: i16) -> u128 {
        value as u128
    }
}

impl WrappingFrom<i32> for u128 {
    fn wrapping_from(value: i32) -> u128 {
        value as u128
    }
}

impl WrappingFrom<i64> for u128 {
    fn wrapping_from(value: i64) -> u128 {
        value as u128
    }
}

impl WrappingFrom<i128> for u128 {
    fn wrapping_from(value: i128) -> u128 {
        value as u128
    }
}

impl WrappingFrom<isize> for u128 {
    fn wrapping_from(value: isize) -> u128 {
        value as u128
    }
}

impl WrappingFrom<u8> for usize {
    fn wrapping_from(value: u8) -> usize {
        value as usize
    }
}

impl WrappingFrom<u16> for usize {
    fn wrapping_from(value: u16) -> usize {
        value as usize
    }
}

impl WrappingFrom<u32> for usize {
    fn wrapping_from(value: u32) -> usize {
        value as usize
    }
}

impl WrappingFrom<u64> for usize {
    fn wrapping_from(value: u64) -> usize {
        value as usize
    }
}

impl WrappingFrom<u128> for usize {
    fn wrapping_from(value: u128) -> usize {
        value as usize
    }
}

impl WrappingFrom<usize> for usize {
    fn wrapping_from(value: usize) -> usize {
        value as usize
    }
}

impl WrappingFrom<i8> for usize {
    fn wrapping_from(value: i8) -> usize {
        value as usize
    }
}

impl WrappingFrom<i16> for usize {
    fn wrapping_from(value: i16) -> usize {
        value as usize
    }
}

impl WrappingFrom<i32> for usize {
    fn wrapping_from(value: i32) -> usize {
        value as usize
    }
}

impl WrappingFrom<i64> for usize {
    fn wrapping_from(value: i64) -> usize {
        value as usize
    }
}

impl WrappingFrom<i128> for usize {
    fn wrapping_from(value: i128) -> usize {
        value as usize
    }
}

impl WrappingFrom<isize> for usize {
    fn wrapping_from(value: isize) -> usize {
        value as usize
    }
}

impl WrappingFrom<u8> for i8 {
    fn wrapping_from(value: u8) -> i8 {
        value as i8
    }
}

impl WrappingFrom<u16> for i8 {
    fn wrapping_from(value: u16) -> i8 {
        value as i8
    }
}

impl WrappingFrom<u32> for i8 {
    fn wrapping_from(value: u32) -> i8 {
        value as i8
    }
}

impl WrappingFrom<u64> for i8 {
    fn wrapping_from(value: u64) -> i8 {
        value as i8
    }
}

impl WrappingFrom<u128> for i8 {
    fn wrapping_from(value: u128) -> i8 {
        value as i8
    }
}

impl WrappingFrom<usize> for i8 {
    fn wrapping_from(value: usize) -> i8 {
        value as i8
    }
}

impl WrappingFrom<i8> for i8 {
    fn wrapping_from(value: i8) -> i8 {
        value as i8
    }
}

impl WrappingFrom<i16> for i8 {
    fn wrapping_from(value: i16) -> i8 {
        value as i8
    }
}

impl WrappingFrom<i32> for i8 {
    fn wrapping_from(value: i32) -> i8 {
        value as i8
    }
}

impl WrappingFrom<i64> for i8 {
    fn wrapping_from(value: i64) -> i8 {
        value as i8
    }
}

impl WrappingFrom<i128> for i8 {
    fn wrapping_from(value: i128) -> i8 {
        value as i8
    }
}

impl WrappingFrom<isize> for i8 {
    fn wrapping_from(value: isize) -> i8 {
        value as i8
    }
}

impl WrappingFrom<u8> for i16 {
    fn wrapping_from(value: u8) -> i16 {
        value as i16
    }
}

impl WrappingFrom<u16> for i16 {
    fn wrapping_from(value: u16) -> i16 {
        value as i16
    }
}

impl WrappingFrom<u32> for i16 {
    fn wrapping_from(value: u32) -> i16 {
        value as i16
    }
}

impl WrappingFrom<u64> for i16 {
    fn wrapping_from(value: u64) -> i16 {
        value as i16
    }
}

impl WrappingFrom<u128> for i16 {
    fn wrapping_from(value: u128) -> i16 {
        value as i16
    }
}

impl WrappingFrom<usize> for i16 {
    fn wrapping_from(value: usize) -> i16 {
        value as i16
    }
}

impl WrappingFrom<i8> for i16 {
    fn wrapping_from(value: i8) -> i16 {
        value as i16
    }
}

impl WrappingFrom<i16> for i16 {
    fn wrapping_from(value: i16) -> i16 {
        value as i16
    }
}

impl WrappingFrom<i32> for i16 {
    fn wrapping_from(value: i32) -> i16 {
        value as i16
    }
}

impl WrappingFrom<i64> for i16 {
    fn wrapping_from(value: i64) -> i16 {
        value as i16
    }
}

impl WrappingFrom<i128> for i16 {
    fn wrapping_from(value: i128) -> i16 {
        value as i16
    }
}

impl WrappingFrom<isize> for i16 {
    fn wrapping_from(value: isize) -> i16 {
        value as i16
    }
}

impl WrappingFrom<u8> for i32 {
    fn wrapping_from(value: u8) -> i32 {
        value as i32
    }
}

impl WrappingFrom<u16> for i32 {
    fn wrapping_from(value: u16) -> i32 {
        value as i32
    }
}

impl WrappingFrom<u32> for i32 {
    fn wrapping_from(value: u32) -> i32 {
        value as i32
    }
}

impl WrappingFrom<u64> for i32 {
    fn wrapping_from(value: u64) -> i32 {
        value as i32
    }
}

impl WrappingFrom<u128> for i32 {
    fn wrapping_from(value: u128) -> i32 {
        value as i32
    }
}

impl WrappingFrom<usize> for i32 {
    fn wrapping_from(value: usize) -> i32 {
        value as i32
    }
}

impl WrappingFrom<i8> for i32 {
    fn wrapping_from(value: i8) -> i32 {
        value as i32
    }
}

impl WrappingFrom<i16> for i32 {
    fn wrapping_from(value: i16) -> i32 {
        value as i32
    }
}

impl WrappingFrom<i32> for i32 {
    fn wrapping_from(value: i32) -> i32 {
        value as i32
    }
}

impl WrappingFrom<i64> for i32 {
    fn wrapping_from(value: i64) -> i32 {
        value as i32
    }
}

impl WrappingFrom<i128> for i32 {
    fn wrapping_from(value: i128) -> i32 {
        value as i32
    }
}

impl WrappingFrom<isize> for i32 {
    fn wrapping_from(value: isize) -> i32 {
        value as i32
    }
}

impl WrappingFrom<u8> for i64 {
    fn wrapping_from(value: u8) -> i64 {
        value as i64
    }
}

impl WrappingFrom<u16> for i64 {
    fn wrapping_from(value: u16) -> i64 {
        value as i64
    }
}

impl WrappingFrom<u32> for i64 {
    fn wrapping_from(value: u32) -> i64 {
        value as i64
    }
}

impl WrappingFrom<u64> for i64 {
    fn wrapping_from(value: u64) -> i64 {
        value as i64
    }
}

impl WrappingFrom<u128> for i64 {
    fn wrapping_from(value: u128) -> i64 {
        value as i64
    }
}

impl WrappingFrom<usize> for i64 {
    fn wrapping_from(value: usize) -> i64 {
        value as i64
    }
}

impl WrappingFrom<i8> for i64 {
    fn wrapping_from(value: i8) -> i64 {
        value as i64
    }
}

impl WrappingFrom<i16> for i64 {
    fn wrapping_from(value: i16) -> i64 {
        value as i64
    }
}

impl WrappingFrom<i32> for i64 {
    fn wrapping_from(value: i32) -> i64 {
        value as i64
    }
}

impl WrappingFrom<i64> for i64 {
    fn wrapping_from(value: i64) -> i64 {
        value as i64
    }
}

impl WrappingFrom<i128> for i64 {
    fn wrapping_from(value: i128) -> i64 {
        value as i64
    }
}

impl WrappingFrom<isize> for i64 {
    fn wrapping_from(value: isize) -> i64 {
        value as i64
    }
}

impl WrappingFrom<u8> for i128 {
    fn wrapping_from(value: u8) -> i128 {
        value as i128
    }
}

impl WrappingFrom<u16> for i128 {
    fn wrapping_from(value: u16) -> i128 {
        value as i128
    }
}

impl WrappingFrom<u32> for i128 {
    fn wrapping_from(value: u32) -> i128 {
        value as i128
    }
}

impl WrappingFrom<u64> for i128 {
    fn wrapping_from(value: u64) -> i128 {
        value as i128
    }
}

impl WrappingFrom<u128> for i128 {
    fn wrapping_from(value: u128) -> i128 {
        value as i128
    }
}

impl WrappingFrom<usize> for i128 {
    fn wrapping_from(value: usize) -> i128 {
        value as i128
    }
}

impl WrappingFrom<i8> for i128 {
    fn wrapping_from(value: i8) -> i128 {
        value as i128
    }
}

impl WrappingFrom<i16> for i128 {
    fn wrapping_from(value: i16) -> i128 {
        value as i128
    }
}

impl WrappingFrom<i32> for i128 {
    fn wrapping_from(value: i32) -> i128 {
        value as i128
    }
}

impl WrappingFrom<i64> for i128 {
    fn wrapping_from(value: i64) -> i128 {
        value as i128
    }
}

impl WrappingFrom<i128> for i128 {
    fn wrapping_from(value: i128) -> i128 {
        value as i128
    }
}

impl WrappingFrom<isize> for i128 {
    fn wrapping_from(value: isize) -> i128 {
        value as i128
    }
}

impl WrappingFrom<u8> for isize {
    fn wrapping_from(value: u8) -> isize {
        value as isize
    }
}

impl WrappingFrom<u16> for isize {
    fn wrapping_from(value: u16) -> isize {
        value as isize
    }
}

impl WrappingFrom<u32> for isize {
    fn wrapping_from(value: u32) -> isize {
        value as isize
    }
}

impl WrappingFrom<u64> for isize {
    fn wrapping_from(value: u64) -> isize {
        value as isize
    }
}

impl WrappingFrom<u128> for isize {
    fn wrapping_from(value: u128) -> isize {
        value as isize
    }
}

impl WrappingFrom<usize> for isize {
    fn wrapping_from(value: usize) -> isize {
        value as isize
    }
}

impl WrappingFrom<i8> for isize {
    fn wrapping_from(value: i8) -> isize {
        value as isize
    }
}

impl WrappingFrom<i16> for isize {
    fn wrapping_from(value: i16) -> isize {
        value as isize
    }
}

impl WrappingFrom<i32> for isize {
    fn wrapping_from(value: i32) -> isize {
        value as isize
    }
}

impl WrappingFrom<i64> for isize {
    fn wrapping_from(value: i64) -> isize {
        value as isize
    }
}

impl WrappingFrom<i128> for isize {
    fn wrapping_from(value: i128) -> isize {
        value as isize
    }
}

impl WrappingFrom<isize> for isize {
    fn wrapping_from(value: isize) -> isize {
        value as isize
    }
}

impl<F: Prim, R: Sealed, const WIDTH: u32> WrappingFrom<F> for Aint<R, WIDTH>
where
    R: WrappingFrom<F>,
{
    fn wrapping_from(value: F) -> Self {
        Self::_new_wrapping(value.wrapping_into())
    }
}

impl<T: Prim, R: Sealed, const WIDTH: u32> WrappingFrom<Aint<R, WIDTH>> for T
where
    T: WrappingFrom<R>,
{
    fn wrapping_from(value: Aint<R, WIDTH>) -> Self {
        value.0.wrapping_into()
    }
}
