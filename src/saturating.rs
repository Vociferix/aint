use crate::{prim::Prim, sealed::Sealed, Aint};

/// Saturating conversion from one type into another.
///
/// When the source value is too large to be represented
/// by the destination type, resulting value is saturated
/// to the bounds of the destination type.
///
/// # Example:
/// ```
/// # use aint::SaturatingFrom;
/// let value = u8::saturating_from(1000);
/// assert_eq!(value, u8::MAX);
/// ```
pub trait SaturatingFrom<T> {
    fn saturating_from(value: T) -> Self;
}

/// Saturating conversion from one type into another.
///
/// When the source value is too large to be represented
/// by the destination type, resulting value is saturated
/// to the bounds of the destination type.
///
/// # Example:
/// ```
/// # use aint::SaturatingInto;
/// let value: u8 = 1000.saturating_into();
/// assert_eq!(value, u8::MAX);
/// ```
pub trait SaturatingInto<T> {
    fn saturating_into(self) -> T;
}

impl<F, T> SaturatingInto<T> for F
where
    T: SaturatingFrom<F>,
{
    fn saturating_into(self) -> T {
        T::saturating_from(self)
    }
}

impl SaturatingFrom<u8> for u8 {
    fn saturating_from(value: u8) -> u8 {
        value
    }
}

impl SaturatingFrom<u16> for u8 {
    fn saturating_from(value: u16) -> u8 {
        core::cmp::min(value, u8::MAX as u16) as u8
    }
}

impl SaturatingFrom<u32> for u8 {
    fn saturating_from(value: u32) -> u8 {
        core::cmp::min(value, u8::MAX as u32) as u8
    }
}

impl SaturatingFrom<u64> for u8 {
    fn saturating_from(value: u64) -> u8 {
        core::cmp::min(value, u8::MAX as u64) as u8
    }
}

impl SaturatingFrom<u128> for u8 {
    fn saturating_from(value: u128) -> u8 {
        core::cmp::min(value, u8::MAX as u128) as u8
    }
}

impl SaturatingFrom<usize> for u8 {
    fn saturating_from(value: usize) -> u8 {
        core::cmp::min(value, u8::MAX as usize) as u8
    }
}

impl SaturatingFrom<i8> for u8 {
    fn saturating_from(value: i8) -> u8 {
        core::cmp::max(value, 0) as u8
    }
}

impl SaturatingFrom<i16> for u8 {
    fn saturating_from(value: i16) -> u8 {
        core::cmp::Ord::clamp(value, 0, u8::MAX as i16) as u8
    }
}

impl SaturatingFrom<i32> for u8 {
    fn saturating_from(value: i32) -> u8 {
        core::cmp::Ord::clamp(value, 0, u8::MAX as i32) as u8
    }
}

impl SaturatingFrom<i64> for u8 {
    fn saturating_from(value: i64) -> u8 {
        core::cmp::Ord::clamp(value, 0, u8::MAX as i64) as u8
    }
}

impl SaturatingFrom<i128> for u8 {
    fn saturating_from(value: i128) -> u8 {
        core::cmp::Ord::clamp(value, 0, u8::MAX as i128) as u8
    }
}

impl SaturatingFrom<isize> for u8 {
    fn saturating_from(value: isize) -> u8 {
        if core::mem::size_of::<isize>() > core::mem::size_of::<u8>() {
            core::cmp::Ord::clamp(value, 0, u8::MAX as isize) as u8
        } else {
            core::cmp::max(value, 0) as u8
        }
    }
}

impl SaturatingFrom<u8> for u16 {
    fn saturating_from(value: u8) -> u16 {
        value as u16
    }
}

impl SaturatingFrom<u16> for u16 {
    fn saturating_from(value: u16) -> u16 {
        value
    }
}

impl SaturatingFrom<u32> for u16 {
    fn saturating_from(value: u32) -> u16 {
        core::cmp::min(value, u16::MAX as u32) as u16
    }
}

impl SaturatingFrom<u64> for u16 {
    fn saturating_from(value: u64) -> u16 {
        core::cmp::min(value, u16::MAX as u64) as u16
    }
}

impl SaturatingFrom<u128> for u16 {
    fn saturating_from(value: u128) -> u16 {
        core::cmp::min(value, u16::MAX as u128) as u16
    }
}

impl SaturatingFrom<usize> for u16 {
    fn saturating_from(value: usize) -> u16 {
        core::cmp::min(value, u16::MAX as usize) as u16
    }
}

impl SaturatingFrom<i8> for u16 {
    fn saturating_from(value: i8) -> u16 {
        core::cmp::max(value, 0) as u16
    }
}

impl SaturatingFrom<i16> for u16 {
    fn saturating_from(value: i16) -> u16 {
        core::cmp::max(value, 0) as u16
    }
}

impl SaturatingFrom<i32> for u16 {
    fn saturating_from(value: i32) -> u16 {
        core::cmp::Ord::clamp(value, 0, u16::MAX as i32) as u16
    }
}

impl SaturatingFrom<i64> for u16 {
    fn saturating_from(value: i64) -> u16 {
        core::cmp::Ord::clamp(value, 0, u16::MAX as i64) as u16
    }
}

impl SaturatingFrom<i128> for u16 {
    fn saturating_from(value: i128) -> u16 {
        core::cmp::Ord::clamp(value, 0, u16::MAX as i128) as u16
    }
}

impl SaturatingFrom<isize> for u16 {
    fn saturating_from(value: isize) -> u16 {
        if core::mem::size_of::<isize>() > core::mem::size_of::<u16>() {
            core::cmp::Ord::clamp(value, 0, u16::MAX as isize) as u16
        } else {
            core::cmp::max(value, 0) as u16
        }
    }
}

impl SaturatingFrom<u8> for u32 {
    fn saturating_from(value: u8) -> u32 {
        value as u32
    }
}

impl SaturatingFrom<u16> for u32 {
    fn saturating_from(value: u16) -> u32 {
        value as u32
    }
}

impl SaturatingFrom<u32> for u32 {
    fn saturating_from(value: u32) -> u32 {
        value
    }
}

impl SaturatingFrom<u64> for u32 {
    fn saturating_from(value: u64) -> u32 {
        core::cmp::min(value, u32::MAX as u64) as u32
    }
}

impl SaturatingFrom<u128> for u32 {
    fn saturating_from(value: u128) -> u32 {
        core::cmp::min(value, u32::MAX as u128) as u32
    }
}

impl SaturatingFrom<usize> for u32 {
    fn saturating_from(value: usize) -> u32 {
        core::cmp::min(value, u32::MAX as usize) as u32
    }
}

impl SaturatingFrom<i8> for u32 {
    fn saturating_from(value: i8) -> u32 {
        core::cmp::max(value, 0) as u32
    }
}

impl SaturatingFrom<i16> for u32 {
    fn saturating_from(value: i16) -> u32 {
        core::cmp::max(value, 0) as u32
    }
}

impl SaturatingFrom<i32> for u32 {
    fn saturating_from(value: i32) -> u32 {
        core::cmp::max(value, 0) as u32
    }
}

impl SaturatingFrom<i64> for u32 {
    fn saturating_from(value: i64) -> u32 {
        core::cmp::Ord::clamp(value, 0, u32::MAX as i64) as u32
    }
}

impl SaturatingFrom<i128> for u32 {
    fn saturating_from(value: i128) -> u32 {
        core::cmp::Ord::clamp(value, 0, u32::MAX as i128) as u32
    }
}

impl SaturatingFrom<isize> for u32 {
    fn saturating_from(value: isize) -> u32 {
        if core::mem::size_of::<isize>() > core::mem::size_of::<u32>() {
            core::cmp::Ord::clamp(value, 0, u32::MAX as isize) as u32
        } else {
            core::cmp::max(value, 0) as u32
        }
    }
}

impl SaturatingFrom<u8> for u64 {
    fn saturating_from(value: u8) -> u64 {
        value as u64
    }
}

impl SaturatingFrom<u16> for u64 {
    fn saturating_from(value: u16) -> u64 {
        value as u64
    }
}

impl SaturatingFrom<u32> for u64 {
    fn saturating_from(value: u32) -> u64 {
        value as u64
    }
}

impl SaturatingFrom<u64> for u64 {
    fn saturating_from(value: u64) -> u64 {
        value
    }
}

impl SaturatingFrom<u128> for u64 {
    fn saturating_from(value: u128) -> u64 {
        core::cmp::min(value, u64::MAX as u128) as u64
    }
}

impl SaturatingFrom<usize> for u64 {
    fn saturating_from(value: usize) -> u64 {
        core::cmp::min(value, u64::MAX as usize) as u64
    }
}

impl SaturatingFrom<i8> for u64 {
    fn saturating_from(value: i8) -> u64 {
        core::cmp::max(value, 0) as u64
    }
}

impl SaturatingFrom<i16> for u64 {
    fn saturating_from(value: i16) -> u64 {
        core::cmp::max(value, 0) as u64
    }
}

impl SaturatingFrom<i32> for u64 {
    fn saturating_from(value: i32) -> u64 {
        core::cmp::max(value, 0) as u64
    }
}

impl SaturatingFrom<i64> for u64 {
    fn saturating_from(value: i64) -> u64 {
        core::cmp::max(value, 0) as u64
    }
}

impl SaturatingFrom<i128> for u64 {
    fn saturating_from(value: i128) -> u64 {
        core::cmp::Ord::clamp(value, 0, u64::MAX as i128) as u64
    }
}

impl SaturatingFrom<isize> for u64 {
    fn saturating_from(value: isize) -> u64 {
        if core::mem::size_of::<isize>() > core::mem::size_of::<u64>() {
            core::cmp::Ord::clamp(value, 0, u64::MAX as isize) as u64
        } else {
            core::cmp::max(value, 0) as u64
        }
    }
}

impl SaturatingFrom<u8> for u128 {
    fn saturating_from(value: u8) -> u128 {
        value as u128
    }
}

impl SaturatingFrom<u16> for u128 {
    fn saturating_from(value: u16) -> u128 {
        value as u128
    }
}

impl SaturatingFrom<u32> for u128 {
    fn saturating_from(value: u32) -> u128 {
        value as u128
    }
}

impl SaturatingFrom<u64> for u128 {
    fn saturating_from(value: u64) -> u128 {
        value as u128
    }
}

impl SaturatingFrom<u128> for u128 {
    fn saturating_from(value: u128) -> u128 {
        value
    }
}

impl SaturatingFrom<usize> for u128 {
    fn saturating_from(value: usize) -> u128 {
        core::cmp::min(value, u128::MAX as usize) as u128
    }
}

impl SaturatingFrom<i8> for u128 {
    fn saturating_from(value: i8) -> u128 {
        core::cmp::max(value, 0) as u128
    }
}

impl SaturatingFrom<i16> for u128 {
    fn saturating_from(value: i16) -> u128 {
        core::cmp::max(value, 0) as u128
    }
}

impl SaturatingFrom<i32> for u128 {
    fn saturating_from(value: i32) -> u128 {
        core::cmp::max(value, 0) as u128
    }
}

impl SaturatingFrom<i64> for u128 {
    fn saturating_from(value: i64) -> u128 {
        core::cmp::max(value, 0) as u128
    }
}

impl SaturatingFrom<i128> for u128 {
    fn saturating_from(value: i128) -> u128 {
        core::cmp::max(value, 0) as u128
    }
}

impl SaturatingFrom<isize> for u128 {
    fn saturating_from(value: isize) -> u128 {
        if core::mem::size_of::<isize>() > core::mem::size_of::<u128>() {
            core::cmp::Ord::clamp(value, 0, u128::MAX as isize) as u128
        } else {
            core::cmp::max(value, 0) as u128
        }
    }
}

impl SaturatingFrom<u8> for usize {
    fn saturating_from(value: u8) -> usize {
        value as usize
    }
}

impl SaturatingFrom<u16> for usize {
    fn saturating_from(value: u16) -> usize {
        value as usize
    }
}

impl SaturatingFrom<u32> for usize {
    fn saturating_from(value: u32) -> usize {
        value as usize
    }
}

impl SaturatingFrom<u64> for usize {
    fn saturating_from(value: u64) -> usize {
        value as usize
    }
}

impl SaturatingFrom<u128> for usize {
    fn saturating_from(value: u128) -> usize {
        value as usize
    }
}

impl SaturatingFrom<usize> for usize {
    fn saturating_from(value: usize) -> usize {
        value
    }
}

impl SaturatingFrom<i8> for usize {
    fn saturating_from(value: i8) -> usize {
        if core::mem::size_of::<i8>() <= core::mem::size_of::<usize>() {
            core::cmp::max(value, 0) as usize
        } else {
            core::cmp::Ord::clamp(value, 0, usize::MAX as i8) as usize
        }
    }
}

impl SaturatingFrom<i16> for usize {
    fn saturating_from(value: i16) -> usize {
        if core::mem::size_of::<i16>() <= core::mem::size_of::<usize>() {
            core::cmp::max(value, 0) as usize
        } else {
            core::cmp::Ord::clamp(value, 0, usize::MAX as i16) as usize
        }
    }
}

impl SaturatingFrom<i32> for usize {
    fn saturating_from(value: i32) -> usize {
        if core::mem::size_of::<i32>() <= core::mem::size_of::<usize>() {
            core::cmp::max(value, 0) as usize
        } else {
            core::cmp::Ord::clamp(value, 0, usize::MAX as i32) as usize
        }
    }
}

impl SaturatingFrom<i64> for usize {
    fn saturating_from(value: i64) -> usize {
        if core::mem::size_of::<i64>() <= core::mem::size_of::<usize>() {
            core::cmp::max(value, 0) as usize
        } else {
            core::cmp::Ord::clamp(value, 0, usize::MAX as i64) as usize
        }
    }
}

impl SaturatingFrom<i128> for usize {
    fn saturating_from(value: i128) -> usize {
        if core::mem::size_of::<i128>() <= core::mem::size_of::<usize>() {
            core::cmp::max(value, 0) as usize
        } else {
            core::cmp::Ord::clamp(value, 0, usize::MAX as i128) as usize
        }
    }
}

impl SaturatingFrom<isize> for usize {
    fn saturating_from(value: isize) -> usize {
        core::cmp::max(value, 0) as usize
    }
}

impl SaturatingFrom<u8> for i8 {
    fn saturating_from(value: u8) -> i8 {
        core::cmp::min(value, i8::MAX as u8) as i8
    }
}

impl SaturatingFrom<u16> for i8 {
    fn saturating_from(value: u16) -> i8 {
        core::cmp::min(value, i8::MAX as u16) as i8
    }
}

impl SaturatingFrom<u32> for i8 {
    fn saturating_from(value: u32) -> i8 {
        core::cmp::min(value, i8::MAX as u32) as i8
    }
}

impl SaturatingFrom<u64> for i8 {
    fn saturating_from(value: u64) -> i8 {
        core::cmp::min(value, i8::MAX as u64) as i8
    }
}

impl SaturatingFrom<u128> for i8 {
    fn saturating_from(value: u128) -> i8 {
        core::cmp::min(value, i8::MAX as u128) as i8
    }
}

impl SaturatingFrom<usize> for i8 {
    fn saturating_from(value: usize) -> i8 {
        core::cmp::min(value, i8::MAX as usize) as i8
    }
}

impl SaturatingFrom<i8> for i8 {
    fn saturating_from(value: i8) -> i8 {
        value
    }
}

impl SaturatingFrom<i16> for i8 {
    fn saturating_from(value: i16) -> i8 {
        core::cmp::Ord::clamp(value, i8::MIN as i16, i8::MAX as i16) as i8
    }
}

impl SaturatingFrom<i32> for i8 {
    fn saturating_from(value: i32) -> i8 {
        core::cmp::Ord::clamp(value, i8::MIN as i32, i8::MAX as i32) as i8
    }
}

impl SaturatingFrom<i64> for i8 {
    fn saturating_from(value: i64) -> i8 {
        core::cmp::Ord::clamp(value, i8::MIN as i64, i8::MAX as i64) as i8
    }
}

impl SaturatingFrom<i128> for i8 {
    fn saturating_from(value: i128) -> i8 {
        core::cmp::Ord::clamp(value, i8::MIN as i128, i8::MAX as i128) as i8
    }
}

impl SaturatingFrom<isize> for i8 {
    fn saturating_from(value: isize) -> i8 {
        if core::mem::size_of::<isize>() <= core::mem::size_of::<i8>() {
            value as i8
        } else {
            core::cmp::Ord::clamp(value, i8::MIN as isize, i8::MAX as isize) as i8
        }
    }
}

impl SaturatingFrom<u8> for i16 {
    fn saturating_from(value: u8) -> i16 {
        value as i16
    }
}

impl SaturatingFrom<u16> for i16 {
    fn saturating_from(value: u16) -> i16 {
        core::cmp::min(value, i16::MAX as u16) as i16
    }
}

impl SaturatingFrom<u32> for i16 {
    fn saturating_from(value: u32) -> i16 {
        core::cmp::min(value, i16::MAX as u32) as i16
    }
}

impl SaturatingFrom<u64> for i16 {
    fn saturating_from(value: u64) -> i16 {
        core::cmp::min(value, i16::MAX as u64) as i16
    }
}

impl SaturatingFrom<u128> for i16 {
    fn saturating_from(value: u128) -> i16 {
        core::cmp::min(value, i16::MAX as u128) as i16
    }
}

impl SaturatingFrom<usize> for i16 {
    fn saturating_from(value: usize) -> i16 {
        if core::mem::size_of::<usize>() < core::mem::size_of::<i16>() {
            value as i16
        } else {
            core::cmp::min(value, i16::MAX as usize) as i16
        }
    }
}

impl SaturatingFrom<i8> for i16 {
    fn saturating_from(value: i8) -> i16 {
        value as i16
    }
}

impl SaturatingFrom<i16> for i16 {
    fn saturating_from(value: i16) -> i16 {
        value
    }
}

impl SaturatingFrom<i32> for i16 {
    fn saturating_from(value: i32) -> i16 {
        core::cmp::Ord::clamp(value, i16::MIN as i32, i16::MAX as i32) as i16
    }
}

impl SaturatingFrom<i64> for i16 {
    fn saturating_from(value: i64) -> i16 {
        core::cmp::Ord::clamp(value, i16::MIN as i64, i16::MAX as i64) as i16
    }
}

impl SaturatingFrom<i128> for i16 {
    fn saturating_from(value: i128) -> i16 {
        core::cmp::Ord::clamp(value, i16::MIN as i128, i16::MAX as i128) as i16
    }
}

impl SaturatingFrom<isize> for i16 {
    fn saturating_from(value: isize) -> i16 {
        if core::mem::size_of::<isize>() <= core::mem::size_of::<i16>() {
            value as i16
        } else {
            core::cmp::Ord::clamp(value, i16::MIN as isize, i16::MAX as isize) as i16
        }
    }
}

impl SaturatingFrom<u8> for i32 {
    fn saturating_from(value: u8) -> i32 {
        value as i32
    }
}

impl SaturatingFrom<u16> for i32 {
    fn saturating_from(value: u16) -> i32 {
        value as i32
    }
}

impl SaturatingFrom<u32> for i32 {
    fn saturating_from(value: u32) -> i32 {
        core::cmp::min(value, i32::MAX as u32) as i32
    }
}

impl SaturatingFrom<u64> for i32 {
    fn saturating_from(value: u64) -> i32 {
        core::cmp::min(value, i32::MAX as u64) as i32
    }
}

impl SaturatingFrom<u128> for i32 {
    fn saturating_from(value: u128) -> i32 {
        core::cmp::min(value, i32::MAX as u128) as i32
    }
}

impl SaturatingFrom<usize> for i32 {
    fn saturating_from(value: usize) -> i32 {
        if core::mem::size_of::<usize>() < core::mem::size_of::<i32>() {
            value as i32
        } else {
            core::cmp::min(value, i32::MAX as usize) as i32
        }
    }
}

impl SaturatingFrom<i8> for i32 {
    fn saturating_from(value: i8) -> i32 {
        value as i32
    }
}

impl SaturatingFrom<i16> for i32 {
    fn saturating_from(value: i16) -> i32 {
        value as i32
    }
}

impl SaturatingFrom<i32> for i32 {
    fn saturating_from(value: i32) -> i32 {
        value
    }
}

impl SaturatingFrom<i64> for i32 {
    fn saturating_from(value: i64) -> i32 {
        core::cmp::Ord::clamp(value, i32::MIN as i64, i32::MAX as i64) as i32
    }
}

impl SaturatingFrom<i128> for i32 {
    fn saturating_from(value: i128) -> i32 {
        core::cmp::Ord::clamp(value, i32::MIN as i128, i32::MAX as i128) as i32
    }
}

impl SaturatingFrom<isize> for i32 {
    fn saturating_from(value: isize) -> i32 {
        if core::mem::size_of::<isize>() <= core::mem::size_of::<i32>() {
            value as i32
        } else {
            core::cmp::Ord::clamp(value, i32::MIN as isize, i32::MAX as isize) as i32
        }
    }
}

impl SaturatingFrom<u8> for i64 {
    fn saturating_from(value: u8) -> i64 {
        value as i64
    }
}

impl SaturatingFrom<u16> for i64 {
    fn saturating_from(value: u16) -> i64 {
        value as i64
    }
}

impl SaturatingFrom<u32> for i64 {
    fn saturating_from(value: u32) -> i64 {
        value as i64
    }
}

impl SaturatingFrom<u64> for i64 {
    fn saturating_from(value: u64) -> i64 {
        core::cmp::min(value, i64::MAX as u64) as i64
    }
}

impl SaturatingFrom<u128> for i64 {
    fn saturating_from(value: u128) -> i64 {
        core::cmp::min(value, i64::MAX as u128) as i64
    }
}

impl SaturatingFrom<usize> for i64 {
    fn saturating_from(value: usize) -> i64 {
        if core::mem::size_of::<usize>() < core::mem::size_of::<i64>() {
            value as i64
        } else {
            core::cmp::min(value, i64::MAX as usize) as i64
        }
    }
}

impl SaturatingFrom<i8> for i64 {
    fn saturating_from(value: i8) -> i64 {
        value as i64
    }
}

impl SaturatingFrom<i16> for i64 {
    fn saturating_from(value: i16) -> i64 {
        value as i64
    }
}

impl SaturatingFrom<i32> for i64 {
    fn saturating_from(value: i32) -> i64 {
        value as i64
    }
}

impl SaturatingFrom<i64> for i64 {
    fn saturating_from(value: i64) -> i64 {
        value
    }
}

impl SaturatingFrom<i128> for i64 {
    fn saturating_from(value: i128) -> i64 {
        core::cmp::Ord::clamp(value, i64::MIN as i128, i64::MAX as i128) as i64
    }
}

impl SaturatingFrom<isize> for i64 {
    fn saturating_from(value: isize) -> i64 {
        if core::mem::size_of::<isize>() <= core::mem::size_of::<i64>() {
            value as i64
        } else {
            core::cmp::Ord::clamp(value, i64::MIN as isize, i64::MAX as isize) as i64
        }
    }
}

impl SaturatingFrom<u8> for i128 {
    fn saturating_from(value: u8) -> i128 {
        value as i128
    }
}

impl SaturatingFrom<u16> for i128 {
    fn saturating_from(value: u16) -> i128 {
        value as i128
    }
}

impl SaturatingFrom<u32> for i128 {
    fn saturating_from(value: u32) -> i128 {
        value as i128
    }
}

impl SaturatingFrom<u64> for i128 {
    fn saturating_from(value: u64) -> i128 {
        value as i128
    }
}

impl SaturatingFrom<u128> for i128 {
    fn saturating_from(value: u128) -> i128 {
        core::cmp::min(value, i128::MAX as u128) as i128
    }
}

impl SaturatingFrom<usize> for i128 {
    fn saturating_from(value: usize) -> i128 {
        if core::mem::size_of::<usize>() < core::mem::size_of::<i128>() {
            value as i128
        } else {
            core::cmp::min(value, i128::MAX as usize) as i128
        }
    }
}

impl SaturatingFrom<i8> for i128 {
    fn saturating_from(value: i8) -> i128 {
        value as i128
    }
}

impl SaturatingFrom<i16> for i128 {
    fn saturating_from(value: i16) -> i128 {
        value as i128
    }
}

impl SaturatingFrom<i32> for i128 {
    fn saturating_from(value: i32) -> i128 {
        value as i128
    }
}

impl SaturatingFrom<i64> for i128 {
    fn saturating_from(value: i64) -> i128 {
        value as i128
    }
}

impl SaturatingFrom<i128> for i128 {
    fn saturating_from(value: i128) -> i128 {
        value
    }
}

impl SaturatingFrom<isize> for i128 {
    fn saturating_from(value: isize) -> i128 {
        if core::mem::size_of::<isize>() <= core::mem::size_of::<i128>() {
            value as i128
        } else {
            core::cmp::Ord::clamp(value, i128::MIN as isize, i128::MAX as isize) as i128
        }
    }
}

impl SaturatingFrom<u8> for isize {
    fn saturating_from(value: u8) -> isize {
        if core::mem::size_of::<u8>() < core::mem::size_of::<isize>() {
            value as isize
        } else {
            core::cmp::min(value, isize::MAX as u8) as isize
        }
    }
}

impl SaturatingFrom<u16> for isize {
    fn saturating_from(value: u16) -> isize {
        if core::mem::size_of::<u16>() < core::mem::size_of::<isize>() {
            value as isize
        } else {
            core::cmp::min(value, isize::MAX as u16) as isize
        }
    }
}

impl SaturatingFrom<u32> for isize {
    fn saturating_from(value: u32) -> isize {
        if core::mem::size_of::<u32>() < core::mem::size_of::<isize>() {
            value as isize
        } else {
            core::cmp::min(value, isize::MAX as u32) as isize
        }
    }
}

impl SaturatingFrom<u64> for isize {
    fn saturating_from(value: u64) -> isize {
        if core::mem::size_of::<u64>() < core::mem::size_of::<isize>() {
            value as isize
        } else {
            core::cmp::min(value, isize::MAX as u64) as isize
        }
    }
}

impl SaturatingFrom<u128> for isize {
    fn saturating_from(value: u128) -> isize {
        if core::mem::size_of::<u128>() < core::mem::size_of::<isize>() {
            value as isize
        } else {
            core::cmp::min(value, isize::MAX as u128) as isize
        }
    }
}

impl SaturatingFrom<usize> for isize {
    fn saturating_from(value: usize) -> isize {
        core::cmp::min(value, isize::MAX as usize) as isize
    }
}

impl SaturatingFrom<i8> for isize {
    fn saturating_from(value: i8) -> isize {
        if core::mem::size_of::<i8>() <= core::mem::size_of::<isize>() {
            value as isize
        } else {
            core::cmp::Ord::clamp(value, isize::MIN as i8, isize::MAX as i8) as isize
        }
    }
}

impl SaturatingFrom<i16> for isize {
    fn saturating_from(value: i16) -> isize {
        if core::mem::size_of::<i16>() <= core::mem::size_of::<isize>() {
            value as isize
        } else {
            core::cmp::Ord::clamp(value, isize::MIN as i16, isize::MAX as i16) as isize
        }
    }
}

impl SaturatingFrom<i32> for isize {
    fn saturating_from(value: i32) -> isize {
        if core::mem::size_of::<i32>() <= core::mem::size_of::<isize>() {
            value as isize
        } else {
            core::cmp::Ord::clamp(value, isize::MIN as i32, isize::MAX as i32) as isize
        }
    }
}

impl SaturatingFrom<i64> for isize {
    fn saturating_from(value: i64) -> isize {
        if core::mem::size_of::<i64>() <= core::mem::size_of::<isize>() {
            value as isize
        } else {
            core::cmp::Ord::clamp(value, isize::MIN as i64, isize::MAX as i64) as isize
        }
    }
}

impl SaturatingFrom<i128> for isize {
    fn saturating_from(value: i128) -> isize {
        if core::mem::size_of::<i128>() <= core::mem::size_of::<isize>() {
            value as isize
        } else {
            core::cmp::Ord::clamp(value, isize::MIN as i128, isize::MAX as i128) as isize
        }
    }
}

impl SaturatingFrom<isize> for isize {
    fn saturating_from(value: isize) -> isize {
        value
    }
}

impl<F: Prim, R: Sealed, const WIDTH: u32> SaturatingFrom<F> for Aint<R, WIDTH>
where
    R: SaturatingFrom<F>,
{
    fn saturating_from(value: F) -> Self {
        Self::_new_saturating(value.saturating_into())
    }
}

impl<T: Prim, R: Sealed, const WIDTH: u32> SaturatingFrom<Aint<R, WIDTH>> for T
where
    T: SaturatingFrom<R>,
{
    fn saturating_from(value: Aint<R, WIDTH>) -> Self {
        value.0.saturating_into()
    }
}
