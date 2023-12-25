use crate::Aint;

fn make_try_from_int_err() -> core::num::TryFromIntError {
    let val: i8 = -1;
    let Err(e) = u8::try_from(val) else {
        unreachable!()
    };
    e
}

impl From<bool> for Aint<u8, 1> {
    fn from(value: bool) -> Self {
        Self(if value { 1 } else { 0 })
    }
}

impl From<Aint<u8, 1>> for bool {
    fn from(value: Aint<u8, 1>) -> Self {
        value.0 != 0
    }
}

impl<const WIDTH: u32> TryFrom<u8> for Aint<u8, WIDTH> {
    type Error = core::num::TryFromIntError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value <= Self::MAX.0 {
            Ok(Self(value))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<u16> for Aint<u8, WIDTH> {
    type Error = core::num::TryFromIntError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value <= Self::MAX.0 as u16 {
            Ok(Self(value as u8))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<u32> for Aint<u8, WIDTH> {
    type Error = core::num::TryFromIntError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value <= Self::MAX.0 as u32 {
            Ok(Self(value as u8))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<u64> for Aint<u8, WIDTH> {
    type Error = core::num::TryFromIntError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if value <= Self::MAX.0 as u64 {
            Ok(Self(value as u8))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<u128> for Aint<u8, WIDTH> {
    type Error = core::num::TryFromIntError;

    fn try_from(value: u128) -> Result<Self, Self::Error> {
        if value <= Self::MAX.0 as u128 {
            Ok(Self(value as u8))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<usize> for Aint<u8, WIDTH> {
    type Error = core::num::TryFromIntError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value <= Self::MAX.0 as usize {
            Ok(Self(value as u8))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<i8> for Aint<u8, WIDTH> {
    type Error = core::num::TryFromIntError;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        if value >= 0 {
            Ok(Self(value as u8))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<i16> for Aint<u8, WIDTH> {
    type Error = core::num::TryFromIntError;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        if value >= 0 && value <= Self::MAX.0 as i16 {
            Ok(Self(value as u8))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<i32> for Aint<u8, WIDTH> {
    type Error = core::num::TryFromIntError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value >= 0 && value <= Self::MAX.0 as i32 {
            Ok(Self(value as u8))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<i64> for Aint<u8, WIDTH> {
    type Error = core::num::TryFromIntError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if value >= 0 && value <= Self::MAX.0 as i64 {
            Ok(Self(value as u8))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<i128> for Aint<u8, WIDTH> {
    type Error = core::num::TryFromIntError;

    fn try_from(value: i128) -> Result<Self, Self::Error> {
        if value >= 0 && value <= Self::MAX.0 as i128 {
            Ok(Self(value as u8))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<isize> for Aint<u8, WIDTH> {
    type Error = core::num::TryFromIntError;

    fn try_from(value: isize) -> Result<Self, Self::Error> {
        if value >= 0 && value <= Self::MAX.0 as isize {
            Ok(Self(value as u8))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> From<Aint<u8, WIDTH>> for u8 {
    fn from(value: Aint<u8, WIDTH>) -> Self {
        value.0
    }
}

impl<const WIDTH: u32> From<Aint<u8, WIDTH>> for u16 {
    fn from(value: Aint<u8, WIDTH>) -> Self {
        value.0 as u16
    }
}

impl<const WIDTH: u32> From<Aint<u8, WIDTH>> for u32 {
    fn from(value: Aint<u8, WIDTH>) -> Self {
        value.0 as u32
    }
}

impl<const WIDTH: u32> From<Aint<u8, WIDTH>> for u64 {
    fn from(value: Aint<u8, WIDTH>) -> Self {
        value.0 as u64
    }
}

impl<const WIDTH: u32> From<Aint<u8, WIDTH>> for u128 {
    fn from(value: Aint<u8, WIDTH>) -> Self {
        value.0 as u128
    }
}

impl<const WIDTH: u32> From<Aint<u8, WIDTH>> for usize {
    fn from(value: Aint<u8, WIDTH>) -> Self {
        value.0 as usize
    }
}

impl<const WIDTH: u32> From<Aint<u8, WIDTH>> for i8 {
    fn from(value: Aint<u8, WIDTH>) -> Self {
        value.0 as i8
    }
}

impl<const WIDTH: u32> From<Aint<u8, WIDTH>> for i16 {
    fn from(value: Aint<u8, WIDTH>) -> Self {
        value.0 as i16
    }
}

impl<const WIDTH: u32> From<Aint<u8, WIDTH>> for i32 {
    fn from(value: Aint<u8, WIDTH>) -> Self {
        value.0 as i32
    }
}

impl<const WIDTH: u32> From<Aint<u8, WIDTH>> for i64 {
    fn from(value: Aint<u8, WIDTH>) -> Self {
        value.0 as i64
    }
}

impl<const WIDTH: u32> From<Aint<u8, WIDTH>> for i128 {
    fn from(value: Aint<u8, WIDTH>) -> Self {
        value.0 as i128
    }
}

impl<const WIDTH: u32> From<Aint<u8, WIDTH>> for isize {
    fn from(value: Aint<u8, WIDTH>) -> Self {
        value.0 as isize
    }
}

impl<const WIDTH: u32> From<u8> for Aint<u16, WIDTH> {
    fn from(value: u8) -> Self {
        Self(value as u16)
    }
}

impl<const WIDTH: u32> TryFrom<u16> for Aint<u16, WIDTH> {
    type Error = core::num::TryFromIntError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value <= Self::MAX.0 {
            Ok(Self(value))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<u32> for Aint<u16, WIDTH> {
    type Error = core::num::TryFromIntError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value <= Self::MAX.0 as u32 {
            Ok(Self(value as u16))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<u64> for Aint<u16, WIDTH> {
    type Error = core::num::TryFromIntError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if value <= Self::MAX.0 as u64 {
            Ok(Self(value as u16))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<u128> for Aint<u16, WIDTH> {
    type Error = core::num::TryFromIntError;

    fn try_from(value: u128) -> Result<Self, Self::Error> {
        if value <= Self::MAX.0 as u128 {
            Ok(Self(value as u16))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<usize> for Aint<u16, WIDTH> {
    type Error = core::num::TryFromIntError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value <= Self::MAX.0 as usize {
            Ok(Self(value as u16))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<i8> for Aint<u16, WIDTH> {
    type Error = core::num::TryFromIntError;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        if value >= 0 {
            Ok(Self(value as u16))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<i16> for Aint<u16, WIDTH> {
    type Error = core::num::TryFromIntError;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        if value >= 0 {
            Ok(Self(value as u16))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<i32> for Aint<u16, WIDTH> {
    type Error = core::num::TryFromIntError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value >= 0 && value <= Self::MAX.0 as i32 {
            Ok(Self(value as u16))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<i64> for Aint<u16, WIDTH> {
    type Error = core::num::TryFromIntError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if value >= 0 && value <= Self::MAX.0 as i64 {
            Ok(Self(value as u16))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<i128> for Aint<u16, WIDTH> {
    type Error = core::num::TryFromIntError;

    fn try_from(value: i128) -> Result<Self, Self::Error> {
        if value >= 0 && value <= Self::MAX.0 as i128 {
            Ok(Self(value as u16))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<isize> for Aint<u16, WIDTH> {
    type Error = core::num::TryFromIntError;

    fn try_from(value: isize) -> Result<Self, Self::Error> {
        if value >= 0 && value <= Self::MAX.0 as isize {
            Ok(Self(value as u16))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<Aint<u16, WIDTH>> for u8 {
    type Error = core::num::TryFromIntError;

    fn try_from(value: Aint<u16, WIDTH>) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl<const WIDTH: u32> From<Aint<u16, WIDTH>> for u16 {
    fn from(value: Aint<u16, WIDTH>) -> Self {
        value.0
    }
}

impl<const WIDTH: u32> From<Aint<u16, WIDTH>> for u32 {
    fn from(value: Aint<u16, WIDTH>) -> Self {
        value.0 as u32
    }
}

impl<const WIDTH: u32> From<Aint<u16, WIDTH>> for u64 {
    fn from(value: Aint<u16, WIDTH>) -> Self {
        value.0 as u64
    }
}

impl<const WIDTH: u32> From<Aint<u16, WIDTH>> for u128 {
    fn from(value: Aint<u16, WIDTH>) -> Self {
        value.0 as u128
    }
}

impl<const WIDTH: u32> From<Aint<u16, WIDTH>> for usize {
    fn from(value: Aint<u16, WIDTH>) -> Self {
        value.0 as usize
    }
}

impl<const WIDTH: u32> TryFrom<Aint<u16, WIDTH>> for i8 {
    type Error = core::num::TryFromIntError;

    fn try_from(value: Aint<u16, WIDTH>) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl<const WIDTH: u32> From<Aint<u16, WIDTH>> for i16 {
    fn from(value: Aint<u16, WIDTH>) -> Self {
        value.0 as i16
    }
}

impl<const WIDTH: u32> From<Aint<u16, WIDTH>> for i32 {
    fn from(value: Aint<u16, WIDTH>) -> Self {
        value.0 as i32
    }
}

impl<const WIDTH: u32> From<Aint<u16, WIDTH>> for i64 {
    fn from(value: Aint<u16, WIDTH>) -> Self {
        value.0 as i64
    }
}

impl<const WIDTH: u32> From<Aint<u16, WIDTH>> for i128 {
    fn from(value: Aint<u16, WIDTH>) -> Self {
        value.0 as i128
    }
}

impl<const WIDTH: u32> From<Aint<u16, WIDTH>> for isize {
    fn from(value: Aint<u16, WIDTH>) -> Self {
        value.0 as isize
    }
}

impl<const WIDTH: u32> From<u8> for Aint<u32, WIDTH> {
    fn from(value: u8) -> Self {
        Self(value as u32)
    }
}

impl<const WIDTH: u32> From<u16> for Aint<u32, WIDTH> {
    fn from(value: u16) -> Self {
        Self(value as u32)
    }
}

impl<const WIDTH: u32> TryFrom<u32> for Aint<u32, WIDTH> {
    type Error = core::num::TryFromIntError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value <= Self::MAX.0 {
            Ok(Self(value))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<u64> for Aint<u32, WIDTH> {
    type Error = core::num::TryFromIntError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if value <= Self::MAX.0 as u64 {
            Ok(Self(value as u32))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<u128> for Aint<u32, WIDTH> {
    type Error = core::num::TryFromIntError;

    fn try_from(value: u128) -> Result<Self, Self::Error> {
        if value <= Self::MAX.0 as u128 {
            Ok(Self(value as u32))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<usize> for Aint<u32, WIDTH> {
    type Error = core::num::TryFromIntError;

    #[cfg(target_pointer_width = "16")]
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Ok(Self(value as u32))
    }

    #[cfg(not(target_pointer_width = "16"))]
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value <= Self::MAX.0 as usize {
            Ok(Self(value as u32))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<i8> for Aint<u32, WIDTH> {
    type Error = core::num::TryFromIntError;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        if value >= 0 {
            Ok(Self(value as u32))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<i16> for Aint<u32, WIDTH> {
    type Error = core::num::TryFromIntError;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        if value >= 0 {
            Ok(Self(value as u32))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<i32> for Aint<u32, WIDTH> {
    type Error = core::num::TryFromIntError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value >= 0 {
            Ok(Self(value as u32))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<i64> for Aint<u32, WIDTH> {
    type Error = core::num::TryFromIntError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if value >= 0 && value <= Self::MAX.0 as i64 {
            Ok(Self(value as u32))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<i128> for Aint<u32, WIDTH> {
    type Error = core::num::TryFromIntError;

    fn try_from(value: i128) -> Result<Self, Self::Error> {
        if value >= 0 && value <= Self::MAX.0 as i128 {
            Ok(Self(value as u32))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<isize> for Aint<u32, WIDTH> {
    type Error = core::num::TryFromIntError;

    #[cfg(target_pointer_width = "16")]
    fn try_from(value: isize) -> Result<Self, Self::Error> {
        if value >= 0 {
            Ok(Self(value as u32))
        } else {
            Err(make_try_from_int_err())
        }
    }

    #[cfg(not(target_pointer_width = "16"))]
    fn try_from(value: isize) -> Result<Self, Self::Error> {
        if value >= 0 && value <= Self::MAX.0 as isize {
            Ok(Self(value as u32))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<Aint<u32, WIDTH>> for u8 {
    type Error = core::num::TryFromIntError;

    fn try_from(value: Aint<u32, WIDTH>) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl<const WIDTH: u32> TryFrom<Aint<u32, WIDTH>> for u16 {
    type Error = core::num::TryFromIntError;

    fn try_from(value: Aint<u32, WIDTH>) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl<const WIDTH: u32> From<Aint<u32, WIDTH>> for u32 {
    fn from(value: Aint<u32, WIDTH>) -> Self {
        value.0
    }
}

impl<const WIDTH: u32> From<Aint<u32, WIDTH>> for u64 {
    fn from(value: Aint<u32, WIDTH>) -> Self {
        value.0 as u64
    }
}

impl<const WIDTH: u32> From<Aint<u32, WIDTH>> for u128 {
    fn from(value: Aint<u32, WIDTH>) -> Self {
        value.0 as u128
    }
}

impl<const WIDTH: u32> TryFrom<Aint<u32, WIDTH>> for usize {
    type Error = core::num::TryFromIntError;

    fn try_from(value: Aint<u32, WIDTH>) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl<const WIDTH: u32> TryFrom<Aint<u32, WIDTH>> for i8 {
    type Error = core::num::TryFromIntError;

    fn try_from(value: Aint<u32, WIDTH>) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl<const WIDTH: u32> TryFrom<Aint<u32, WIDTH>> for i16 {
    type Error = core::num::TryFromIntError;

    fn try_from(value: Aint<u32, WIDTH>) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl<const WIDTH: u32> From<Aint<u32, WIDTH>> for i32 {
    fn from(value: Aint<u32, WIDTH>) -> Self {
        value.0 as i32
    }
}

impl<const WIDTH: u32> From<Aint<u32, WIDTH>> for i64 {
    fn from(value: Aint<u32, WIDTH>) -> Self {
        value.0 as i64
    }
}

impl<const WIDTH: u32> From<Aint<u32, WIDTH>> for i128 {
    fn from(value: Aint<u32, WIDTH>) -> Self {
        value.0 as i128
    }
}

impl<const WIDTH: u32> TryFrom<Aint<u32, WIDTH>> for isize {
    type Error = core::num::TryFromIntError;

    fn try_from(value: Aint<u32, WIDTH>) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl<const WIDTH: u32> From<u8> for Aint<u64, WIDTH> {
    fn from(value: u8) -> Self {
        Self(value as u64)
    }
}

impl<const WIDTH: u32> From<u16> for Aint<u64, WIDTH> {
    fn from(value: u16) -> Self {
        Self(value as u64)
    }
}

impl<const WIDTH: u32> From<u32> for Aint<u64, WIDTH> {
    fn from(value: u32) -> Self {
        Self(value as u64)
    }
}

impl<const WIDTH: u32> TryFrom<u64> for Aint<u64, WIDTH> {
    type Error = core::num::TryFromIntError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if value <= Self::MAX.0 {
            Ok(Self(value))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<u128> for Aint<u64, WIDTH> {
    type Error = core::num::TryFromIntError;

    fn try_from(value: u128) -> Result<Self, Self::Error> {
        if value <= Self::MAX.0 as u128 {
            Ok(Self(value as u64))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<usize> for Aint<u64, WIDTH> {
    type Error = core::num::TryFromIntError;

    #[cfg(any(target_pointer_width = "16", target_pointer_width = "32"))]
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Ok(Self(value as u64))
    }

    #[cfg(not(any(target_pointer_width = "16", target_pointer_width = "32")))]
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value <= Self::MAX.0 as usize {
            Ok(Self(value as u64))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<i8> for Aint<u64, WIDTH> {
    type Error = core::num::TryFromIntError;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        if value >= 0 {
            Ok(Self(value as u64))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<i16> for Aint<u64, WIDTH> {
    type Error = core::num::TryFromIntError;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        if value >= 0 {
            Ok(Self(value as u64))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<i32> for Aint<u64, WIDTH> {
    type Error = core::num::TryFromIntError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value >= 0 {
            Ok(Self(value as u64))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<i64> for Aint<u64, WIDTH> {
    type Error = core::num::TryFromIntError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if value >= 0 {
            Ok(Self(value as u64))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<i128> for Aint<u64, WIDTH> {
    type Error = core::num::TryFromIntError;

    fn try_from(value: i128) -> Result<Self, Self::Error> {
        if value >= 0 && value <= Self::MAX.0 as i128 {
            Ok(Self(value as u64))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<isize> for Aint<u64, WIDTH> {
    type Error = core::num::TryFromIntError;

    #[cfg(any(target_pointer_width = "16", target_pointer_width = "32"))]
    fn try_from(value: isize) -> Result<Self, Self::Error> {
        if value >= 0 {
            Ok(Self(value as u64))
        } else {
            Err(make_try_from_int_err())
        }
    }

    #[cfg(not(any(target_pointer_width = "16", target_pointer_width = "32")))]
    fn try_from(value: isize) -> Result<Self, Self::Error> {
        if value >= 0 && value <= Self::MAX.0 as isize {
            Ok(Self(value as u64))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<Aint<u64, WIDTH>> for u8 {
    type Error = core::num::TryFromIntError;

    fn try_from(value: Aint<u64, WIDTH>) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl<const WIDTH: u32> TryFrom<Aint<u64, WIDTH>> for u16 {
    type Error = core::num::TryFromIntError;

    fn try_from(value: Aint<u64, WIDTH>) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl<const WIDTH: u32> TryFrom<Aint<u64, WIDTH>> for u32 {
    type Error = core::num::TryFromIntError;

    fn try_from(value: Aint<u64, WIDTH>) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl<const WIDTH: u32> From<Aint<u64, WIDTH>> for u64 {
    fn from(value: Aint<u64, WIDTH>) -> Self {
        value.0
    }
}

impl<const WIDTH: u32> From<Aint<u64, WIDTH>> for u128 {
    fn from(value: Aint<u64, WIDTH>) -> Self {
        value.0 as u128
    }
}

impl<const WIDTH: u32> TryFrom<Aint<u64, WIDTH>> for usize {
    type Error = core::num::TryFromIntError;

    fn try_from(value: Aint<u64, WIDTH>) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl<const WIDTH: u32> TryFrom<Aint<u64, WIDTH>> for i8 {
    type Error = core::num::TryFromIntError;

    fn try_from(value: Aint<u64, WIDTH>) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl<const WIDTH: u32> TryFrom<Aint<u64, WIDTH>> for i16 {
    type Error = core::num::TryFromIntError;

    fn try_from(value: Aint<u64, WIDTH>) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl<const WIDTH: u32> TryFrom<Aint<u64, WIDTH>> for i32 {
    type Error = core::num::TryFromIntError;

    fn try_from(value: Aint<u64, WIDTH>) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl<const WIDTH: u32> From<Aint<u64, WIDTH>> for i64 {
    fn from(value: Aint<u64, WIDTH>) -> Self {
        value.0 as i64
    }
}

impl<const WIDTH: u32> From<Aint<u64, WIDTH>> for i128 {
    fn from(value: Aint<u64, WIDTH>) -> Self {
        value.0 as i128
    }
}

impl<const WIDTH: u32> TryFrom<Aint<u64, WIDTH>> for isize {
    type Error = core::num::TryFromIntError;

    fn try_from(value: Aint<u64, WIDTH>) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl<const WIDTH: u32> From<u8> for Aint<u128, WIDTH> {
    fn from(value: u8) -> Self {
        Self(value as u128)
    }
}

impl<const WIDTH: u32> From<u16> for Aint<u128, WIDTH> {
    fn from(value: u16) -> Self {
        Self(value as u128)
    }
}

impl<const WIDTH: u32> From<u32> for Aint<u128, WIDTH> {
    fn from(value: u32) -> Self {
        Self(value as u128)
    }
}

impl<const WIDTH: u32> From<u64> for Aint<u128, WIDTH> {
    fn from(value: u64) -> Self {
        Self(value as u128)
    }
}

impl<const WIDTH: u32> TryFrom<u128> for Aint<u128, WIDTH> {
    type Error = core::num::TryFromIntError;

    fn try_from(value: u128) -> Result<Self, Self::Error> {
        if value <= Self::MAX.0 {
            Ok(Self(value))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<usize> for Aint<u128, WIDTH> {
    type Error = core::num::TryFromIntError;

    #[cfg(any(
        target_pointer_width = "16",
        target_pointer_width = "32",
        target_pointer_width = "64"
    ))]
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Ok(Self(value as u128))
    }

    #[cfg(not(any(
        target_pointer_width = "16",
        target_pointer_width = "32",
        target_pointer_width = "64"
    )))]
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value <= Self::MAX.0 as usize {
            Ok(Self(value as u128))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<i8> for Aint<u128, WIDTH> {
    type Error = core::num::TryFromIntError;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        if value >= 0 {
            Ok(Self(value as u128))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<i16> for Aint<u128, WIDTH> {
    type Error = core::num::TryFromIntError;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        if value >= 0 {
            Ok(Self(value as u128))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<i32> for Aint<u128, WIDTH> {
    type Error = core::num::TryFromIntError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value >= 0 {
            Ok(Self(value as u128))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<i64> for Aint<u128, WIDTH> {
    type Error = core::num::TryFromIntError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if value >= 0 {
            Ok(Self(value as u128))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<i128> for Aint<u128, WIDTH> {
    type Error = core::num::TryFromIntError;

    fn try_from(value: i128) -> Result<Self, Self::Error> {
        if value >= 0 {
            Ok(Self(value as u128))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<isize> for Aint<u128, WIDTH> {
    type Error = core::num::TryFromIntError;

    #[cfg(any(
        target_pointer_width = "16",
        target_pointer_width = "32",
        target_pointer_width = "64"
    ))]
    fn try_from(value: isize) -> Result<Self, Self::Error> {
        if value >= 0 {
            Ok(Self(value as u128))
        } else {
            Err(make_try_from_int_err())
        }
    }

    #[cfg(not(any(
        target_pointer_width = "16",
        target_pointer_width = "32",
        target_pointer_width = "64"
    )))]
    fn try_from(value: isize) -> Result<Self, Self::Error> {
        if value >= 0 && value <= Self::MAX.0 as isize {
            Ok(Self(value as u128))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<Aint<u128, WIDTH>> for u8 {
    type Error = core::num::TryFromIntError;

    fn try_from(value: Aint<u128, WIDTH>) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl<const WIDTH: u32> TryFrom<Aint<u128, WIDTH>> for u16 {
    type Error = core::num::TryFromIntError;

    fn try_from(value: Aint<u128, WIDTH>) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl<const WIDTH: u32> TryFrom<Aint<u128, WIDTH>> for u32 {
    type Error = core::num::TryFromIntError;

    fn try_from(value: Aint<u128, WIDTH>) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl<const WIDTH: u32> TryFrom<Aint<u128, WIDTH>> for u64 {
    type Error = core::num::TryFromIntError;

    fn try_from(value: Aint<u128, WIDTH>) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl<const WIDTH: u32> From<Aint<u128, WIDTH>> for u128 {
    fn from(value: Aint<u128, WIDTH>) -> Self {
        value.0
    }
}

impl<const WIDTH: u32> TryFrom<Aint<u128, WIDTH>> for usize {
    type Error = core::num::TryFromIntError;

    fn try_from(value: Aint<u128, WIDTH>) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl<const WIDTH: u32> TryFrom<Aint<u128, WIDTH>> for i8 {
    type Error = core::num::TryFromIntError;

    fn try_from(value: Aint<u128, WIDTH>) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl<const WIDTH: u32> TryFrom<Aint<u128, WIDTH>> for i16 {
    type Error = core::num::TryFromIntError;

    fn try_from(value: Aint<u128, WIDTH>) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl<const WIDTH: u32> TryFrom<Aint<u128, WIDTH>> for i32 {
    type Error = core::num::TryFromIntError;

    fn try_from(value: Aint<u128, WIDTH>) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl<const WIDTH: u32> TryFrom<Aint<u128, WIDTH>> for i64 {
    type Error = core::num::TryFromIntError;

    fn try_from(value: Aint<u128, WIDTH>) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl<const WIDTH: u32> From<Aint<u128, WIDTH>> for i128 {
    fn from(value: Aint<u128, WIDTH>) -> Self {
        value.0 as i128
    }
}

impl<const WIDTH: u32> TryFrom<Aint<u128, WIDTH>> for isize {
    type Error = core::num::TryFromIntError;

    fn try_from(value: Aint<u128, WIDTH>) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl<const WIDTH: u32> TryFrom<u8> for Aint<i8, WIDTH> {
    type Error = core::num::TryFromIntError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value <= Self::MAX.0 as u8 {
            Ok(Self(value as i8))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<u16> for Aint<i8, WIDTH> {
    type Error = core::num::TryFromIntError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value <= Self::MAX.0 as u16 {
            Ok(Self(value as i8))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<u32> for Aint<i8, WIDTH> {
    type Error = core::num::TryFromIntError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value <= Self::MAX.0 as u32 {
            Ok(Self(value as i8))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<u64> for Aint<i8, WIDTH> {
    type Error = core::num::TryFromIntError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if value <= Self::MAX.0 as u64 {
            Ok(Self(value as i8))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<u128> for Aint<i8, WIDTH> {
    type Error = core::num::TryFromIntError;

    fn try_from(value: u128) -> Result<Self, Self::Error> {
        if value <= Self::MAX.0 as u128 {
            Ok(Self(value as i8))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<usize> for Aint<i8, WIDTH> {
    type Error = core::num::TryFromIntError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value <= Self::MAX.0 as usize {
            Ok(Self(value as i8))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<i8> for Aint<i8, WIDTH> {
    type Error = core::num::TryFromIntError;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        if value <= Self::MAX.0 && value >= Self::MIN.0 {
            Ok(Self(value))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<i16> for Aint<i8, WIDTH> {
    type Error = core::num::TryFromIntError;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        if value <= Self::MAX.0 as i16 && value >= Self::MIN.0 as i16 {
            Ok(Self(value as i8))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<i32> for Aint<i8, WIDTH> {
    type Error = core::num::TryFromIntError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value <= Self::MAX.0 as i32 && value >= Self::MIN.0 as i32 {
            Ok(Self(value as i8))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<i64> for Aint<i8, WIDTH> {
    type Error = core::num::TryFromIntError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if value <= Self::MAX.0 as i64 && value >= Self::MIN.0 as i64 {
            Ok(Self(value as i8))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<i128> for Aint<i8, WIDTH> {
    type Error = core::num::TryFromIntError;

    fn try_from(value: i128) -> Result<Self, Self::Error> {
        if value <= Self::MAX.0 as i128 && value >= Self::MIN.0 as i128 {
            Ok(Self(value as i8))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<isize> for Aint<i8, WIDTH> {
    type Error = core::num::TryFromIntError;

    fn try_from(value: isize) -> Result<Self, Self::Error> {
        if value <= Self::MAX.0 as isize && value >= Self::MIN.0 as isize {
            Ok(Self(value as i8))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<Aint<i8, WIDTH>> for u8 {
    type Error = core::num::TryFromIntError;

    fn try_from(value: Aint<i8, WIDTH>) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl<const WIDTH: u32> TryFrom<Aint<i8, WIDTH>> for u16 {
    type Error = core::num::TryFromIntError;

    fn try_from(value: Aint<i8, WIDTH>) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl<const WIDTH: u32> TryFrom<Aint<i8, WIDTH>> for u32 {
    type Error = core::num::TryFromIntError;

    fn try_from(value: Aint<i8, WIDTH>) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl<const WIDTH: u32> TryFrom<Aint<i8, WIDTH>> for u64 {
    type Error = core::num::TryFromIntError;

    fn try_from(value: Aint<i8, WIDTH>) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl<const WIDTH: u32> TryFrom<Aint<i8, WIDTH>> for u128 {
    type Error = core::num::TryFromIntError;

    fn try_from(value: Aint<i8, WIDTH>) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl<const WIDTH: u32> TryFrom<Aint<i8, WIDTH>> for usize {
    type Error = core::num::TryFromIntError;

    fn try_from(value: Aint<i8, WIDTH>) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl<const WIDTH: u32> From<Aint<i8, WIDTH>> for i8 {
    fn from(value: Aint<i8, WIDTH>) -> Self {
        value.0
    }
}

impl<const WIDTH: u32> From<Aint<i8, WIDTH>> for i16 {
    fn from(value: Aint<i8, WIDTH>) -> Self {
        value.0 as i16
    }
}

impl<const WIDTH: u32> From<Aint<i8, WIDTH>> for i32 {
    fn from(value: Aint<i8, WIDTH>) -> Self {
        value.0 as i32
    }
}

impl<const WIDTH: u32> From<Aint<i8, WIDTH>> for i64 {
    fn from(value: Aint<i8, WIDTH>) -> Self {
        value.0 as i64
    }
}

impl<const WIDTH: u32> From<Aint<i8, WIDTH>> for i128 {
    fn from(value: Aint<i8, WIDTH>) -> Self {
        value.0 as i128
    }
}

impl<const WIDTH: u32> From<Aint<i8, WIDTH>> for isize {
    fn from(value: Aint<i8, WIDTH>) -> Self {
        value.0 as isize
    }
}

impl<const WIDTH: u32> From<u8> for Aint<i16, WIDTH> {
    fn from(value: u8) -> Self {
        Self(value as i16)
    }
}

impl<const WIDTH: u32> TryFrom<u16> for Aint<i16, WIDTH> {
    type Error = core::num::TryFromIntError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value <= Self::MAX.0 as u16 {
            Ok(Self(value as i16))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<u32> for Aint<i16, WIDTH> {
    type Error = core::num::TryFromIntError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value <= Self::MAX.0 as u32 {
            Ok(Self(value as i16))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<u64> for Aint<i16, WIDTH> {
    type Error = core::num::TryFromIntError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if value <= Self::MAX.0 as u64 {
            Ok(Self(value as i16))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<u128> for Aint<i16, WIDTH> {
    type Error = core::num::TryFromIntError;

    fn try_from(value: u128) -> Result<Self, Self::Error> {
        if value <= Self::MAX.0 as u128 {
            Ok(Self(value as i16))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<usize> for Aint<i16, WIDTH> {
    type Error = core::num::TryFromIntError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value <= Self::MAX.0 as usize {
            Ok(Self(value as i16))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> From<i8> for Aint<i16, WIDTH> {
    fn from(value: i8) -> Self {
        Self(value as i16)
    }
}

impl<const WIDTH: u32> TryFrom<i16> for Aint<i16, WIDTH> {
    type Error = core::num::TryFromIntError;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        if value <= Self::MAX.0 && value >= Self::MIN.0 {
            Ok(Self(value))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<i32> for Aint<i16, WIDTH> {
    type Error = core::num::TryFromIntError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value <= Self::MAX.0 as i32 && value >= Self::MIN.0 as i32 {
            Ok(Self(value as i16))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<i64> for Aint<i16, WIDTH> {
    type Error = core::num::TryFromIntError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if value <= Self::MAX.0 as i64 && value >= Self::MIN.0 as i64 {
            Ok(Self(value as i16))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<i128> for Aint<i16, WIDTH> {
    type Error = core::num::TryFromIntError;

    fn try_from(value: i128) -> Result<Self, Self::Error> {
        if value <= Self::MAX.0 as i128 && value >= Self::MIN.0 as i128 {
            Ok(Self(value as i16))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<isize> for Aint<i16, WIDTH> {
    type Error = core::num::TryFromIntError;

    fn try_from(value: isize) -> Result<Self, Self::Error> {
        if value <= Self::MAX.0 as isize && value >= Self::MIN.0 as isize {
            Ok(Self(value as i16))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<Aint<i16, WIDTH>> for u8 {
    type Error = core::num::TryFromIntError;

    fn try_from(value: Aint<i16, WIDTH>) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl<const WIDTH: u32> TryFrom<Aint<i16, WIDTH>> for u16 {
    type Error = core::num::TryFromIntError;

    fn try_from(value: Aint<i16, WIDTH>) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl<const WIDTH: u32> TryFrom<Aint<i16, WIDTH>> for u32 {
    type Error = core::num::TryFromIntError;

    fn try_from(value: Aint<i16, WIDTH>) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl<const WIDTH: u32> TryFrom<Aint<i16, WIDTH>> for u64 {
    type Error = core::num::TryFromIntError;

    fn try_from(value: Aint<i16, WIDTH>) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl<const WIDTH: u32> TryFrom<Aint<i16, WIDTH>> for u128 {
    type Error = core::num::TryFromIntError;

    fn try_from(value: Aint<i16, WIDTH>) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl<const WIDTH: u32> TryFrom<Aint<i16, WIDTH>> for usize {
    type Error = core::num::TryFromIntError;

    fn try_from(value: Aint<i16, WIDTH>) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl<const WIDTH: u32> TryFrom<Aint<i16, WIDTH>> for i8 {
    type Error = core::num::TryFromIntError;

    fn try_from(value: Aint<i16, WIDTH>) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl<const WIDTH: u32> From<Aint<i16, WIDTH>> for i16 {
    fn from(value: Aint<i16, WIDTH>) -> Self {
        value.0
    }
}

impl<const WIDTH: u32> From<Aint<i16, WIDTH>> for i32 {
    fn from(value: Aint<i16, WIDTH>) -> Self {
        value.0 as i32
    }
}

impl<const WIDTH: u32> From<Aint<i16, WIDTH>> for i64 {
    fn from(value: Aint<i16, WIDTH>) -> Self {
        value.0 as i64
    }
}

impl<const WIDTH: u32> From<Aint<i16, WIDTH>> for i128 {
    fn from(value: Aint<i16, WIDTH>) -> Self {
        value.0 as i128
    }
}

impl<const WIDTH: u32> From<Aint<i16, WIDTH>> for isize {
    fn from(value: Aint<i16, WIDTH>) -> Self {
        value.0 as isize
    }
}

impl<const WIDTH: u32> From<u8> for Aint<i32, WIDTH> {
    fn from(value: u8) -> Self {
        Self(value as i32)
    }
}

impl<const WIDTH: u32> From<u16> for Aint<i32, WIDTH> {
    fn from(value: u16) -> Self {
        Self(value as i32)
    }
}

impl<const WIDTH: u32> TryFrom<u32> for Aint<i32, WIDTH> {
    type Error = core::num::TryFromIntError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value <= Self::MAX.0 as u32 {
            Ok(Self(value as i32))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<u64> for Aint<i32, WIDTH> {
    type Error = core::num::TryFromIntError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if value <= Self::MAX.0 as u64 {
            Ok(Self(value as i32))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<u128> for Aint<i32, WIDTH> {
    type Error = core::num::TryFromIntError;

    fn try_from(value: u128) -> Result<Self, Self::Error> {
        if value <= Self::MAX.0 as u128 {
            Ok(Self(value as i32))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<usize> for Aint<i32, WIDTH> {
    type Error = core::num::TryFromIntError;

    #[cfg(target_pointer_width = "16")]
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Ok(Self(value as i32))
    }

    #[cfg(not(target_pointer_width = "16"))]
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value <= Self::MAX.0 as usize {
            Ok(Self(value as i32))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> From<i8> for Aint<i32, WIDTH> {
    fn from(value: i8) -> Self {
        Self(value as i32)
    }
}

impl<const WIDTH: u32> From<i16> for Aint<i32, WIDTH> {
    fn from(value: i16) -> Self {
        Self(value as i32)
    }
}

impl<const WIDTH: u32> TryFrom<i32> for Aint<i32, WIDTH> {
    type Error = core::num::TryFromIntError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value <= Self::MAX.0 && value >= Self::MIN.0 {
            Ok(Self(value))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<i64> for Aint<i32, WIDTH> {
    type Error = core::num::TryFromIntError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if value <= Self::MAX.0 as i64 && value >= Self::MIN.0 as i64 {
            Ok(Self(value as i32))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<i128> for Aint<i32, WIDTH> {
    type Error = core::num::TryFromIntError;

    fn try_from(value: i128) -> Result<Self, Self::Error> {
        if value <= Self::MAX.0 as i128 && value >= Self::MIN.0 as i128 {
            Ok(Self(value as i32))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<isize> for Aint<i32, WIDTH> {
    type Error = core::num::TryFromIntError;

    #[cfg(target_pointer_width = "16")]
    fn try_from(value: isize) -> Result<Self, Self::Error> {
        Ok(Self(value as i32))
    }

    #[cfg(not(target_pointer_width = "16"))]
    fn try_from(value: isize) -> Result<Self, Self::Error> {
        if value <= Self::MAX.0 as isize && value >= Self::MIN.0 as isize {
            Ok(Self(value as i32))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<Aint<i32, WIDTH>> for u8 {
    type Error = core::num::TryFromIntError;

    fn try_from(value: Aint<i32, WIDTH>) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl<const WIDTH: u32> TryFrom<Aint<i32, WIDTH>> for u16 {
    type Error = core::num::TryFromIntError;

    fn try_from(value: Aint<i32, WIDTH>) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl<const WIDTH: u32> TryFrom<Aint<i32, WIDTH>> for u32 {
    type Error = core::num::TryFromIntError;

    fn try_from(value: Aint<i32, WIDTH>) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl<const WIDTH: u32> TryFrom<Aint<i32, WIDTH>> for u64 {
    type Error = core::num::TryFromIntError;

    fn try_from(value: Aint<i32, WIDTH>) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl<const WIDTH: u32> TryFrom<Aint<i32, WIDTH>> for u128 {
    type Error = core::num::TryFromIntError;

    fn try_from(value: Aint<i32, WIDTH>) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl<const WIDTH: u32> TryFrom<Aint<i32, WIDTH>> for usize {
    type Error = core::num::TryFromIntError;

    fn try_from(value: Aint<i32, WIDTH>) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl<const WIDTH: u32> TryFrom<Aint<i32, WIDTH>> for i8 {
    type Error = core::num::TryFromIntError;

    fn try_from(value: Aint<i32, WIDTH>) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl<const WIDTH: u32> TryFrom<Aint<i32, WIDTH>> for i16 {
    type Error = core::num::TryFromIntError;

    fn try_from(value: Aint<i32, WIDTH>) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl<const WIDTH: u32> From<Aint<i32, WIDTH>> for i32 {
    fn from(value: Aint<i32, WIDTH>) -> Self {
        value.0
    }
}

impl<const WIDTH: u32> From<Aint<i32, WIDTH>> for i64 {
    fn from(value: Aint<i32, WIDTH>) -> Self {
        value.0 as i64
    }
}

impl<const WIDTH: u32> From<Aint<i32, WIDTH>> for i128 {
    fn from(value: Aint<i32, WIDTH>) -> Self {
        value.0 as i128
    }
}

impl<const WIDTH: u32> TryFrom<Aint<i32, WIDTH>> for isize {
    type Error = core::num::TryFromIntError;

    fn try_from(value: Aint<i32, WIDTH>) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl<const WIDTH: u32> From<u8> for Aint<i64, WIDTH> {
    fn from(value: u8) -> Self {
        Self(value as i64)
    }
}

impl<const WIDTH: u32> From<u16> for Aint<i64, WIDTH> {
    fn from(value: u16) -> Self {
        Self(value as i64)
    }
}

impl<const WIDTH: u32> From<u32> for Aint<i64, WIDTH> {
    fn from(value: u32) -> Self {
        Self(value as i64)
    }
}

impl<const WIDTH: u32> TryFrom<u64> for Aint<i64, WIDTH> {
    type Error = core::num::TryFromIntError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if value <= Self::MAX.0 as u64 {
            Ok(Self(value as i64))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<u128> for Aint<i64, WIDTH> {
    type Error = core::num::TryFromIntError;

    fn try_from(value: u128) -> Result<Self, Self::Error> {
        if value <= Self::MAX.0 as u128 {
            Ok(Self(value as i64))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<usize> for Aint<i64, WIDTH> {
    type Error = core::num::TryFromIntError;

    #[cfg(any(target_pointer_width = "16", target_pointer_width = "32"))]
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Ok(Self(value as i64))
    }

    #[cfg(not(any(target_pointer_width = "16", target_pointer_width = "32")))]
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value <= Self::MAX.0 as usize {
            Ok(Self(value as i64))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> From<i8> for Aint<i64, WIDTH> {
    fn from(value: i8) -> Self {
        Self(value as i64)
    }
}

impl<const WIDTH: u32> From<i16> for Aint<i64, WIDTH> {
    fn from(value: i16) -> Self {
        Self(value as i64)
    }
}

impl<const WIDTH: u32> From<i32> for Aint<i64, WIDTH> {
    fn from(value: i32) -> Self {
        Self(value as i64)
    }
}

impl<const WIDTH: u32> TryFrom<i64> for Aint<i64, WIDTH> {
    type Error = core::num::TryFromIntError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if value <= Self::MAX.0 && value >= Self::MIN.0 {
            Ok(Self(value))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<i128> for Aint<i64, WIDTH> {
    type Error = core::num::TryFromIntError;

    fn try_from(value: i128) -> Result<Self, Self::Error> {
        if value <= Self::MAX.0 as i128 && value >= Self::MIN.0 as i128 {
            Ok(Self(value as i64))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<isize> for Aint<i64, WIDTH> {
    type Error = core::num::TryFromIntError;

    #[cfg(any(target_pointer_width = "16", target_pointer_width = "32"))]
    fn try_from(value: isize) -> Result<Self, Self::Error> {
        Ok(Self(value as i64))
    }

    #[cfg(not(any(target_pointer_width = "16", target_pointer_width = "32")))]
    fn try_from(value: isize) -> Result<Self, Self::Error> {
        if value <= Self::MAX.0 as isize && value >= Self::MIN.0 as isize {
            Ok(Self(value as i64))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<Aint<i64, WIDTH>> for u8 {
    type Error = core::num::TryFromIntError;

    fn try_from(value: Aint<i64, WIDTH>) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl<const WIDTH: u32> TryFrom<Aint<i64, WIDTH>> for u16 {
    type Error = core::num::TryFromIntError;

    fn try_from(value: Aint<i64, WIDTH>) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl<const WIDTH: u32> TryFrom<Aint<i64, WIDTH>> for u32 {
    type Error = core::num::TryFromIntError;

    fn try_from(value: Aint<i64, WIDTH>) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl<const WIDTH: u32> TryFrom<Aint<i64, WIDTH>> for u64 {
    type Error = core::num::TryFromIntError;

    fn try_from(value: Aint<i64, WIDTH>) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl<const WIDTH: u32> TryFrom<Aint<i64, WIDTH>> for u128 {
    type Error = core::num::TryFromIntError;

    fn try_from(value: Aint<i64, WIDTH>) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl<const WIDTH: u32> TryFrom<Aint<i64, WIDTH>> for usize {
    type Error = core::num::TryFromIntError;

    fn try_from(value: Aint<i64, WIDTH>) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl<const WIDTH: u32> TryFrom<Aint<i64, WIDTH>> for i8 {
    type Error = core::num::TryFromIntError;

    fn try_from(value: Aint<i64, WIDTH>) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl<const WIDTH: u32> TryFrom<Aint<i64, WIDTH>> for i16 {
    type Error = core::num::TryFromIntError;

    fn try_from(value: Aint<i64, WIDTH>) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl<const WIDTH: u32> TryFrom<Aint<i64, WIDTH>> for i32 {
    type Error = core::num::TryFromIntError;

    fn try_from(value: Aint<i64, WIDTH>) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl<const WIDTH: u32> From<Aint<i64, WIDTH>> for i64 {
    fn from(value: Aint<i64, WIDTH>) -> Self {
        value.0
    }
}

impl<const WIDTH: u32> From<Aint<i64, WIDTH>> for i128 {
    fn from(value: Aint<i64, WIDTH>) -> Self {
        value.0 as i128
    }
}

impl<const WIDTH: u32> TryFrom<Aint<i64, WIDTH>> for isize {
    type Error = core::num::TryFromIntError;

    fn try_from(value: Aint<i64, WIDTH>) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl<const WIDTH: u32> From<u8> for Aint<i128, WIDTH> {
    fn from(value: u8) -> Self {
        Self(value as i128)
    }
}

impl<const WIDTH: u32> From<u16> for Aint<i128, WIDTH> {
    fn from(value: u16) -> Self {
        Self(value as i128)
    }
}

impl<const WIDTH: u32> From<u32> for Aint<i128, WIDTH> {
    fn from(value: u32) -> Self {
        Self(value as i128)
    }
}

impl<const WIDTH: u32> From<u64> for Aint<i128, WIDTH> {
    fn from(value: u64) -> Self {
        Self(value as i128)
    }
}

impl<const WIDTH: u32> TryFrom<u128> for Aint<i128, WIDTH> {
    type Error = core::num::TryFromIntError;

    fn try_from(value: u128) -> Result<Self, Self::Error> {
        if value <= Self::MAX.0 as u128 {
            Ok(Self(value as i128))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<usize> for Aint<i128, WIDTH> {
    type Error = core::num::TryFromIntError;

    #[cfg(any(
        target_pointer_width = "16",
        target_pointer_width = "32",
        target_pointer_width = "64"
    ))]
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Ok(Self(value as i128))
    }

    #[cfg(not(any(
        target_pointer_width = "16",
        target_pointer_width = "32",
        target_pointer_width = "64"
    )))]
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value <= Self::MAX.0 as usize {
            Ok(Self(value as i128))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> From<i8> for Aint<i128, WIDTH> {
    fn from(value: i8) -> Self {
        Self(value as i128)
    }
}

impl<const WIDTH: u32> From<i16> for Aint<i128, WIDTH> {
    fn from(value: i16) -> Self {
        Self(value as i128)
    }
}

impl<const WIDTH: u32> From<i32> for Aint<i128, WIDTH> {
    fn from(value: i32) -> Self {
        Self(value as i128)
    }
}

impl<const WIDTH: u32> From<i64> for Aint<i128, WIDTH> {
    fn from(value: i64) -> Self {
        Self(value as i128)
    }
}

impl<const WIDTH: u32> TryFrom<i128> for Aint<i128, WIDTH> {
    type Error = core::num::TryFromIntError;

    fn try_from(value: i128) -> Result<Self, Self::Error> {
        if value <= Self::MAX.0 && value >= Self::MIN.0 {
            Ok(Self(value))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<isize> for Aint<i128, WIDTH> {
    type Error = core::num::TryFromIntError;

    #[cfg(any(
        target_pointer_width = "16",
        target_pointer_width = "32",
        target_pointer_width = "64"
    ))]
    fn try_from(value: isize) -> Result<Self, Self::Error> {
        Ok(Self(value as i128))
    }

    #[cfg(not(any(
        target_pointer_width = "16",
        target_pointer_width = "32",
        target_pointer_width = "64"
    )))]
    fn try_from(value: isize) -> Result<Self, Self::Error> {
        if value <= Self::MAX.0 as isize && value >= Self::MIN.0 as isize {
            Ok(Self(value as i128))
        } else {
            Err(make_try_from_int_err())
        }
    }
}

impl<const WIDTH: u32> TryFrom<Aint<i128, WIDTH>> for u8 {
    type Error = core::num::TryFromIntError;

    fn try_from(value: Aint<i128, WIDTH>) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl<const WIDTH: u32> TryFrom<Aint<i128, WIDTH>> for u16 {
    type Error = core::num::TryFromIntError;

    fn try_from(value: Aint<i128, WIDTH>) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl<const WIDTH: u32> TryFrom<Aint<i128, WIDTH>> for u32 {
    type Error = core::num::TryFromIntError;

    fn try_from(value: Aint<i128, WIDTH>) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl<const WIDTH: u32> TryFrom<Aint<i128, WIDTH>> for u64 {
    type Error = core::num::TryFromIntError;

    fn try_from(value: Aint<i128, WIDTH>) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl<const WIDTH: u32> TryFrom<Aint<i128, WIDTH>> for u128 {
    type Error = core::num::TryFromIntError;

    fn try_from(value: Aint<i128, WIDTH>) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl<const WIDTH: u32> TryFrom<Aint<i128, WIDTH>> for usize {
    type Error = core::num::TryFromIntError;

    fn try_from(value: Aint<i128, WIDTH>) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl<const WIDTH: u32> TryFrom<Aint<i128, WIDTH>> for i8 {
    type Error = core::num::TryFromIntError;

    fn try_from(value: Aint<i128, WIDTH>) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl<const WIDTH: u32> TryFrom<Aint<i128, WIDTH>> for i16 {
    type Error = core::num::TryFromIntError;

    fn try_from(value: Aint<i128, WIDTH>) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl<const WIDTH: u32> TryFrom<Aint<i128, WIDTH>> for i32 {
    type Error = core::num::TryFromIntError;

    fn try_from(value: Aint<i128, WIDTH>) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl<const WIDTH: u32> TryFrom<Aint<i128, WIDTH>> for i64 {
    type Error = core::num::TryFromIntError;

    fn try_from(value: Aint<i128, WIDTH>) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl<const WIDTH: u32> From<Aint<i128, WIDTH>> for i128 {
    fn from(value: Aint<i128, WIDTH>) -> Self {
        value.0
    }
}

impl<const WIDTH: u32> TryFrom<Aint<i128, WIDTH>> for isize {
    type Error = core::num::TryFromIntError;

    fn try_from(value: Aint<i128, WIDTH>) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}
