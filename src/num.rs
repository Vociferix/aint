use crate::sealed::Sealed;
use crate::Aint;

impl num_traits::FromBytes for Aint<u32, 24> {
    type Bytes = [u8; 3];

    fn from_be_bytes(bytes: &[u8; 3]) -> Self {
        <Aint<u32, 24>>::from_be_bytes(bytes.clone())
    }

    fn from_le_bytes(bytes: &[u8; 3]) -> Self {
        <Aint<u32, 24>>::from_le_bytes(bytes.clone())
    }

    fn from_ne_bytes(bytes: &[u8; 3]) -> Self {
        <Aint<u32, 24>>::from_ne_bytes(bytes.clone())
    }
}

impl num_traits::ToBytes for Aint<u32, 24> {
    type Bytes = [u8; 3];

    fn to_be_bytes(&self) -> [u8; 3] {
        <Aint<u32, 24>>::to_be_bytes(*self)
    }

    fn to_le_bytes(&self) -> [u8; 3] {
        <Aint<u32, 24>>::to_le_bytes(*self)
    }

    fn to_ne_bytes(&self) -> [u8; 3] {
        <Aint<u32, 24>>::to_ne_bytes(*self)
    }
}

impl num_traits::FromBytes for Aint<u64, 40> {
    type Bytes = [u8; 5];

    fn from_be_bytes(bytes: &[u8; 5]) -> Self {
        <Aint<u64, 40>>::from_be_bytes(bytes.clone())
    }

    fn from_le_bytes(bytes: &[u8; 5]) -> Self {
        <Aint<u64, 40>>::from_le_bytes(bytes.clone())
    }

    fn from_ne_bytes(bytes: &[u8; 5]) -> Self {
        <Aint<u64, 40>>::from_ne_bytes(bytes.clone())
    }
}

impl num_traits::ToBytes for Aint<u64, 40> {
    type Bytes = [u8; 5];

    fn to_be_bytes(&self) -> [u8; 5] {
        <Aint<u64, 40>>::to_be_bytes(*self)
    }

    fn to_le_bytes(&self) -> [u8; 5] {
        <Aint<u64, 40>>::to_le_bytes(*self)
    }

    fn to_ne_bytes(&self) -> [u8; 5] {
        <Aint<u64, 40>>::to_ne_bytes(*self)
    }
}

impl num_traits::FromBytes for Aint<u64, 48> {
    type Bytes = [u8; 6];

    fn from_be_bytes(bytes: &[u8; 6]) -> Self {
        <Aint<u64, 48>>::from_be_bytes(bytes.clone())
    }

    fn from_le_bytes(bytes: &[u8; 6]) -> Self {
        <Aint<u64, 48>>::from_le_bytes(bytes.clone())
    }

    fn from_ne_bytes(bytes: &[u8; 6]) -> Self {
        <Aint<u64, 48>>::from_ne_bytes(bytes.clone())
    }
}

impl num_traits::ToBytes for Aint<u64, 48> {
    type Bytes = [u8; 6];

    fn to_be_bytes(&self) -> [u8; 6] {
        <Aint<u64, 48>>::to_be_bytes(*self)
    }

    fn to_le_bytes(&self) -> [u8; 6] {
        <Aint<u64, 48>>::to_le_bytes(*self)
    }

    fn to_ne_bytes(&self) -> [u8; 6] {
        <Aint<u64, 48>>::to_ne_bytes(*self)
    }
}

impl num_traits::FromBytes for Aint<u64, 56> {
    type Bytes = [u8; 7];

    fn from_be_bytes(bytes: &[u8; 7]) -> Self {
        <Aint<u64, 56>>::from_be_bytes(bytes.clone())
    }

    fn from_le_bytes(bytes: &[u8; 7]) -> Self {
        <Aint<u64, 56>>::from_le_bytes(bytes.clone())
    }

    fn from_ne_bytes(bytes: &[u8; 7]) -> Self {
        <Aint<u64, 56>>::from_ne_bytes(bytes.clone())
    }
}

impl num_traits::ToBytes for Aint<u64, 56> {
    type Bytes = [u8; 7];

    fn to_be_bytes(&self) -> [u8; 7] {
        <Aint<u64, 56>>::to_be_bytes(*self)
    }

    fn to_le_bytes(&self) -> [u8; 7] {
        <Aint<u64, 56>>::to_le_bytes(*self)
    }

    fn to_ne_bytes(&self) -> [u8; 7] {
        <Aint<u64, 56>>::to_ne_bytes(*self)
    }
}

impl num_traits::FromBytes for Aint<u128, 72> {
    type Bytes = [u8; 9];

    fn from_be_bytes(bytes: &[u8; 9]) -> Self {
        <Aint<u128, 72>>::from_be_bytes(bytes.clone())
    }

    fn from_le_bytes(bytes: &[u8; 9]) -> Self {
        <Aint<u128, 72>>::from_le_bytes(bytes.clone())
    }

    fn from_ne_bytes(bytes: &[u8; 9]) -> Self {
        <Aint<u128, 72>>::from_ne_bytes(bytes.clone())
    }
}

impl num_traits::ToBytes for Aint<u128, 72> {
    type Bytes = [u8; 9];

    fn to_be_bytes(&self) -> [u8; 9] {
        <Aint<u128, 72>>::to_be_bytes(*self)
    }

    fn to_le_bytes(&self) -> [u8; 9] {
        <Aint<u128, 72>>::to_le_bytes(*self)
    }

    fn to_ne_bytes(&self) -> [u8; 9] {
        <Aint<u128, 72>>::to_ne_bytes(*self)
    }
}

impl num_traits::FromBytes for Aint<u128, 80> {
    type Bytes = [u8; 10];

    fn from_be_bytes(bytes: &[u8; 10]) -> Self {
        <Aint<u128, 80>>::from_be_bytes(bytes.clone())
    }

    fn from_le_bytes(bytes: &[u8; 10]) -> Self {
        <Aint<u128, 80>>::from_le_bytes(bytes.clone())
    }

    fn from_ne_bytes(bytes: &[u8; 10]) -> Self {
        <Aint<u128, 80>>::from_ne_bytes(bytes.clone())
    }
}

impl num_traits::ToBytes for Aint<u128, 80> {
    type Bytes = [u8; 10];

    fn to_be_bytes(&self) -> [u8; 10] {
        <Aint<u128, 80>>::to_be_bytes(*self)
    }

    fn to_le_bytes(&self) -> [u8; 10] {
        <Aint<u128, 80>>::to_le_bytes(*self)
    }

    fn to_ne_bytes(&self) -> [u8; 10] {
        <Aint<u128, 80>>::to_ne_bytes(*self)
    }
}

impl num_traits::FromBytes for Aint<u128, 88> {
    type Bytes = [u8; 11];

    fn from_be_bytes(bytes: &[u8; 11]) -> Self {
        <Aint<u128, 88>>::from_be_bytes(bytes.clone())
    }

    fn from_le_bytes(bytes: &[u8; 11]) -> Self {
        <Aint<u128, 88>>::from_le_bytes(bytes.clone())
    }

    fn from_ne_bytes(bytes: &[u8; 11]) -> Self {
        <Aint<u128, 88>>::from_ne_bytes(bytes.clone())
    }
}

impl num_traits::ToBytes for Aint<u128, 88> {
    type Bytes = [u8; 11];

    fn to_be_bytes(&self) -> [u8; 11] {
        <Aint<u128, 88>>::to_be_bytes(*self)
    }

    fn to_le_bytes(&self) -> [u8; 11] {
        <Aint<u128, 88>>::to_le_bytes(*self)
    }

    fn to_ne_bytes(&self) -> [u8; 11] {
        <Aint<u128, 88>>::to_ne_bytes(*self)
    }
}

impl num_traits::FromBytes for Aint<u128, 96> {
    type Bytes = [u8; 12];

    fn from_be_bytes(bytes: &[u8; 12]) -> Self {
        <Aint<u128, 96>>::from_be_bytes(bytes.clone())
    }

    fn from_le_bytes(bytes: &[u8; 12]) -> Self {
        <Aint<u128, 96>>::from_le_bytes(bytes.clone())
    }

    fn from_ne_bytes(bytes: &[u8; 12]) -> Self {
        <Aint<u128, 96>>::from_ne_bytes(bytes.clone())
    }
}

impl num_traits::ToBytes for Aint<u128, 96> {
    type Bytes = [u8; 12];

    fn to_be_bytes(&self) -> [u8; 12] {
        <Aint<u128, 96>>::to_be_bytes(*self)
    }

    fn to_le_bytes(&self) -> [u8; 12] {
        <Aint<u128, 96>>::to_le_bytes(*self)
    }

    fn to_ne_bytes(&self) -> [u8; 12] {
        <Aint<u128, 96>>::to_ne_bytes(*self)
    }
}

impl num_traits::FromBytes for Aint<u128, 104> {
    type Bytes = [u8; 13];

    fn from_be_bytes(bytes: &[u8; 13]) -> Self {
        <Aint<u128, 104>>::from_be_bytes(bytes.clone())
    }

    fn from_le_bytes(bytes: &[u8; 13]) -> Self {
        <Aint<u128, 104>>::from_le_bytes(bytes.clone())
    }

    fn from_ne_bytes(bytes: &[u8; 13]) -> Self {
        <Aint<u128, 104>>::from_ne_bytes(bytes.clone())
    }
}

impl num_traits::ToBytes for Aint<u128, 104> {
    type Bytes = [u8; 13];

    fn to_be_bytes(&self) -> [u8; 13] {
        <Aint<u128, 104>>::to_be_bytes(*self)
    }

    fn to_le_bytes(&self) -> [u8; 13] {
        <Aint<u128, 104>>::to_le_bytes(*self)
    }

    fn to_ne_bytes(&self) -> [u8; 13] {
        <Aint<u128, 104>>::to_ne_bytes(*self)
    }
}

impl num_traits::FromBytes for Aint<u128, 112> {
    type Bytes = [u8; 14];

    fn from_be_bytes(bytes: &[u8; 14]) -> Self {
        <Aint<u128, 112>>::from_be_bytes(bytes.clone())
    }

    fn from_le_bytes(bytes: &[u8; 14]) -> Self {
        <Aint<u128, 112>>::from_le_bytes(bytes.clone())
    }

    fn from_ne_bytes(bytes: &[u8; 14]) -> Self {
        <Aint<u128, 112>>::from_ne_bytes(bytes.clone())
    }
}

impl num_traits::ToBytes for Aint<u128, 112> {
    type Bytes = [u8; 14];

    fn to_be_bytes(&self) -> [u8; 14] {
        <Aint<u128, 112>>::to_be_bytes(*self)
    }

    fn to_le_bytes(&self) -> [u8; 14] {
        <Aint<u128, 112>>::to_le_bytes(*self)
    }

    fn to_ne_bytes(&self) -> [u8; 14] {
        <Aint<u128, 112>>::to_ne_bytes(*self)
    }
}

impl num_traits::FromBytes for Aint<u128, 120> {
    type Bytes = [u8; 15];

    fn from_be_bytes(bytes: &[u8; 15]) -> Self {
        <Aint<u128, 120>>::from_be_bytes(bytes.clone())
    }

    fn from_le_bytes(bytes: &[u8; 15]) -> Self {
        <Aint<u128, 120>>::from_le_bytes(bytes.clone())
    }

    fn from_ne_bytes(bytes: &[u8; 15]) -> Self {
        <Aint<u128, 120>>::from_ne_bytes(bytes.clone())
    }
}

impl num_traits::ToBytes for Aint<u128, 120> {
    type Bytes = [u8; 15];

    fn to_be_bytes(&self) -> [u8; 15] {
        <Aint<u128, 120>>::to_be_bytes(*self)
    }

    fn to_le_bytes(&self) -> [u8; 15] {
        <Aint<u128, 120>>::to_le_bytes(*self)
    }

    fn to_ne_bytes(&self) -> [u8; 15] {
        <Aint<u128, 120>>::to_ne_bytes(*self)
    }
}

impl num_traits::FromBytes for Aint<i32, 24> {
    type Bytes = [u8; 3];

    fn from_be_bytes(bytes: &[u8; 3]) -> Self {
        <Aint<i32, 24>>::from_be_bytes(bytes.clone())
    }

    fn from_le_bytes(bytes: &[u8; 3]) -> Self {
        <Aint<i32, 24>>::from_le_bytes(bytes.clone())
    }

    fn from_ne_bytes(bytes: &[u8; 3]) -> Self {
        <Aint<i32, 24>>::from_ne_bytes(bytes.clone())
    }
}

impl num_traits::ToBytes for Aint<i32, 24> {
    type Bytes = [u8; 3];

    fn to_be_bytes(&self) -> [u8; 3] {
        <Aint<i32, 24>>::to_be_bytes(*self)
    }

    fn to_le_bytes(&self) -> [u8; 3] {
        <Aint<i32, 24>>::to_le_bytes(*self)
    }

    fn to_ne_bytes(&self) -> [u8; 3] {
        <Aint<i32, 24>>::to_ne_bytes(*self)
    }
}

impl num_traits::FromBytes for Aint<i64, 40> {
    type Bytes = [u8; 5];

    fn from_be_bytes(bytes: &[u8; 5]) -> Self {
        <Aint<i64, 40>>::from_be_bytes(bytes.clone())
    }

    fn from_le_bytes(bytes: &[u8; 5]) -> Self {
        <Aint<i64, 40>>::from_le_bytes(bytes.clone())
    }

    fn from_ne_bytes(bytes: &[u8; 5]) -> Self {
        <Aint<i64, 40>>::from_ne_bytes(bytes.clone())
    }
}

impl num_traits::ToBytes for Aint<i64, 40> {
    type Bytes = [u8; 5];

    fn to_be_bytes(&self) -> [u8; 5] {
        <Aint<i64, 40>>::to_be_bytes(*self)
    }

    fn to_le_bytes(&self) -> [u8; 5] {
        <Aint<i64, 40>>::to_le_bytes(*self)
    }

    fn to_ne_bytes(&self) -> [u8; 5] {
        <Aint<i64, 40>>::to_ne_bytes(*self)
    }
}

impl num_traits::FromBytes for Aint<i64, 48> {
    type Bytes = [u8; 6];

    fn from_be_bytes(bytes: &[u8; 6]) -> Self {
        <Aint<i64, 48>>::from_be_bytes(bytes.clone())
    }

    fn from_le_bytes(bytes: &[u8; 6]) -> Self {
        <Aint<i64, 48>>::from_le_bytes(bytes.clone())
    }

    fn from_ne_bytes(bytes: &[u8; 6]) -> Self {
        <Aint<i64, 48>>::from_ne_bytes(bytes.clone())
    }
}

impl num_traits::ToBytes for Aint<i64, 48> {
    type Bytes = [u8; 6];

    fn to_be_bytes(&self) -> [u8; 6] {
        <Aint<i64, 48>>::to_be_bytes(*self)
    }

    fn to_le_bytes(&self) -> [u8; 6] {
        <Aint<i64, 48>>::to_le_bytes(*self)
    }

    fn to_ne_bytes(&self) -> [u8; 6] {
        <Aint<i64, 48>>::to_ne_bytes(*self)
    }
}

impl num_traits::FromBytes for Aint<i64, 56> {
    type Bytes = [u8; 7];

    fn from_be_bytes(bytes: &[u8; 7]) -> Self {
        <Aint<i64, 56>>::from_be_bytes(bytes.clone())
    }

    fn from_le_bytes(bytes: &[u8; 7]) -> Self {
        <Aint<i64, 56>>::from_le_bytes(bytes.clone())
    }

    fn from_ne_bytes(bytes: &[u8; 7]) -> Self {
        <Aint<i64, 56>>::from_ne_bytes(bytes.clone())
    }
}

impl num_traits::ToBytes for Aint<i64, 56> {
    type Bytes = [u8; 7];

    fn to_be_bytes(&self) -> [u8; 7] {
        <Aint<i64, 56>>::to_be_bytes(*self)
    }

    fn to_le_bytes(&self) -> [u8; 7] {
        <Aint<i64, 56>>::to_le_bytes(*self)
    }

    fn to_ne_bytes(&self) -> [u8; 7] {
        <Aint<i64, 56>>::to_ne_bytes(*self)
    }
}

impl num_traits::FromBytes for Aint<i128, 72> {
    type Bytes = [u8; 9];

    fn from_be_bytes(bytes: &[u8; 9]) -> Self {
        <Aint<i128, 72>>::from_be_bytes(bytes.clone())
    }

    fn from_le_bytes(bytes: &[u8; 9]) -> Self {
        <Aint<i128, 72>>::from_le_bytes(bytes.clone())
    }

    fn from_ne_bytes(bytes: &[u8; 9]) -> Self {
        <Aint<i128, 72>>::from_ne_bytes(bytes.clone())
    }
}

impl num_traits::ToBytes for Aint<i128, 72> {
    type Bytes = [u8; 9];

    fn to_be_bytes(&self) -> [u8; 9] {
        <Aint<i128, 72>>::to_be_bytes(*self)
    }

    fn to_le_bytes(&self) -> [u8; 9] {
        <Aint<i128, 72>>::to_le_bytes(*self)
    }

    fn to_ne_bytes(&self) -> [u8; 9] {
        <Aint<i128, 72>>::to_ne_bytes(*self)
    }
}

impl num_traits::FromBytes for Aint<i128, 80> {
    type Bytes = [u8; 10];

    fn from_be_bytes(bytes: &[u8; 10]) -> Self {
        <Aint<i128, 80>>::from_be_bytes(bytes.clone())
    }

    fn from_le_bytes(bytes: &[u8; 10]) -> Self {
        <Aint<i128, 80>>::from_le_bytes(bytes.clone())
    }

    fn from_ne_bytes(bytes: &[u8; 10]) -> Self {
        <Aint<i128, 80>>::from_ne_bytes(bytes.clone())
    }
}

impl num_traits::ToBytes for Aint<i128, 80> {
    type Bytes = [u8; 10];

    fn to_be_bytes(&self) -> [u8; 10] {
        <Aint<i128, 80>>::to_be_bytes(*self)
    }

    fn to_le_bytes(&self) -> [u8; 10] {
        <Aint<i128, 80>>::to_le_bytes(*self)
    }

    fn to_ne_bytes(&self) -> [u8; 10] {
        <Aint<i128, 80>>::to_ne_bytes(*self)
    }
}

impl num_traits::FromBytes for Aint<i128, 88> {
    type Bytes = [u8; 11];

    fn from_be_bytes(bytes: &[u8; 11]) -> Self {
        <Aint<i128, 88>>::from_be_bytes(bytes.clone())
    }

    fn from_le_bytes(bytes: &[u8; 11]) -> Self {
        <Aint<i128, 88>>::from_le_bytes(bytes.clone())
    }

    fn from_ne_bytes(bytes: &[u8; 11]) -> Self {
        <Aint<i128, 88>>::from_ne_bytes(bytes.clone())
    }
}

impl num_traits::ToBytes for Aint<i128, 88> {
    type Bytes = [u8; 11];

    fn to_be_bytes(&self) -> [u8; 11] {
        <Aint<i128, 88>>::to_be_bytes(*self)
    }

    fn to_le_bytes(&self) -> [u8; 11] {
        <Aint<i128, 88>>::to_le_bytes(*self)
    }

    fn to_ne_bytes(&self) -> [u8; 11] {
        <Aint<i128, 88>>::to_ne_bytes(*self)
    }
}

impl num_traits::FromBytes for Aint<i128, 96> {
    type Bytes = [u8; 12];

    fn from_be_bytes(bytes: &[u8; 12]) -> Self {
        <Aint<i128, 96>>::from_be_bytes(bytes.clone())
    }

    fn from_le_bytes(bytes: &[u8; 12]) -> Self {
        <Aint<i128, 96>>::from_le_bytes(bytes.clone())
    }

    fn from_ne_bytes(bytes: &[u8; 12]) -> Self {
        <Aint<i128, 96>>::from_ne_bytes(bytes.clone())
    }
}

impl num_traits::ToBytes for Aint<i128, 96> {
    type Bytes = [u8; 12];

    fn to_be_bytes(&self) -> [u8; 12] {
        <Aint<i128, 96>>::to_be_bytes(*self)
    }

    fn to_le_bytes(&self) -> [u8; 12] {
        <Aint<i128, 96>>::to_le_bytes(*self)
    }

    fn to_ne_bytes(&self) -> [u8; 12] {
        <Aint<i128, 96>>::to_ne_bytes(*self)
    }
}

impl num_traits::FromBytes for Aint<i128, 104> {
    type Bytes = [u8; 13];

    fn from_be_bytes(bytes: &[u8; 13]) -> Self {
        <Aint<i128, 104>>::from_be_bytes(bytes.clone())
    }

    fn from_le_bytes(bytes: &[u8; 13]) -> Self {
        <Aint<i128, 104>>::from_le_bytes(bytes.clone())
    }

    fn from_ne_bytes(bytes: &[u8; 13]) -> Self {
        <Aint<i128, 104>>::from_ne_bytes(bytes.clone())
    }
}

impl num_traits::ToBytes for Aint<i128, 104> {
    type Bytes = [u8; 13];

    fn to_be_bytes(&self) -> [u8; 13] {
        <Aint<i128, 104>>::to_be_bytes(*self)
    }

    fn to_le_bytes(&self) -> [u8; 13] {
        <Aint<i128, 104>>::to_le_bytes(*self)
    }

    fn to_ne_bytes(&self) -> [u8; 13] {
        <Aint<i128, 104>>::to_ne_bytes(*self)
    }
}

impl num_traits::FromBytes for Aint<i128, 112> {
    type Bytes = [u8; 14];

    fn from_be_bytes(bytes: &[u8; 14]) -> Self {
        <Aint<i128, 112>>::from_be_bytes(bytes.clone())
    }

    fn from_le_bytes(bytes: &[u8; 14]) -> Self {
        <Aint<i128, 112>>::from_le_bytes(bytes.clone())
    }

    fn from_ne_bytes(bytes: &[u8; 14]) -> Self {
        <Aint<i128, 112>>::from_ne_bytes(bytes.clone())
    }
}

impl num_traits::ToBytes for Aint<i128, 112> {
    type Bytes = [u8; 14];

    fn to_be_bytes(&self) -> [u8; 14] {
        <Aint<i128, 112>>::to_be_bytes(*self)
    }

    fn to_le_bytes(&self) -> [u8; 14] {
        <Aint<i128, 112>>::to_le_bytes(*self)
    }

    fn to_ne_bytes(&self) -> [u8; 14] {
        <Aint<i128, 112>>::to_ne_bytes(*self)
    }
}

impl num_traits::FromBytes for Aint<i128, 120> {
    type Bytes = [u8; 15];

    fn from_be_bytes(bytes: &[u8; 15]) -> Self {
        <Aint<i128, 120>>::from_be_bytes(bytes.clone())
    }

    fn from_le_bytes(bytes: &[u8; 15]) -> Self {
        <Aint<i128, 120>>::from_le_bytes(bytes.clone())
    }

    fn from_ne_bytes(bytes: &[u8; 15]) -> Self {
        <Aint<i128, 120>>::from_ne_bytes(bytes.clone())
    }
}

impl num_traits::ToBytes for Aint<i128, 120> {
    type Bytes = [u8; 15];

    fn to_be_bytes(&self) -> [u8; 15] {
        <Aint<i128, 120>>::to_be_bytes(*self)
    }

    fn to_le_bytes(&self) -> [u8; 15] {
        <Aint<i128, 120>>::to_le_bytes(*self)
    }

    fn to_ne_bytes(&self) -> [u8; 15] {
        <Aint<i128, 120>>::to_ne_bytes(*self)
    }
}

impl<R: Sealed, const WIDTH: u32> num_traits::Zero for Aint<R, WIDTH> {
    fn zero() -> Self {
        Self(R::ZERO)
    }

    fn is_zero(&self) -> bool {
        self.0 == R::ZERO
    }
}

impl<R: Sealed, const WIDTH: u32> num_traits::One for Aint<R, WIDTH> {
    fn one() -> Self {
        Self::_new_wrapping(R::ONE)
    }
}

impl<const WIDTH: u32> num_traits::Num for Aint<u8, WIDTH> {
    type FromStrRadixErr = core::num::ParseIntError;

    fn from_str_radix(s: &str, radix: u32) -> Result<Self, core::num::ParseIntError> {
        <Aint<u8, WIDTH>>::from_str_radix(s, radix)
    }
}

impl<const WIDTH: u32> num_traits::Num for Aint<u16, WIDTH> {
    type FromStrRadixErr = core::num::ParseIntError;

    fn from_str_radix(s: &str, radix: u32) -> Result<Self, core::num::ParseIntError> {
        <Aint<u16, WIDTH>>::from_str_radix(s, radix)
    }
}

impl<const WIDTH: u32> num_traits::Num for Aint<u32, WIDTH> {
    type FromStrRadixErr = core::num::ParseIntError;

    fn from_str_radix(s: &str, radix: u32) -> Result<Self, core::num::ParseIntError> {
        <Aint<u32, WIDTH>>::from_str_radix(s, radix)
    }
}

impl<const WIDTH: u32> num_traits::Num for Aint<u64, WIDTH> {
    type FromStrRadixErr = core::num::ParseIntError;

    fn from_str_radix(s: &str, radix: u32) -> Result<Self, core::num::ParseIntError> {
        <Aint<u64, WIDTH>>::from_str_radix(s, radix)
    }
}

impl<const WIDTH: u32> num_traits::Num for Aint<u128, WIDTH> {
    type FromStrRadixErr = core::num::ParseIntError;

    fn from_str_radix(s: &str, radix: u32) -> Result<Self, core::num::ParseIntError> {
        <Aint<u128, WIDTH>>::from_str_radix(s, radix)
    }
}

impl<const WIDTH: u32> num_traits::Num for Aint<i8, WIDTH> {
    type FromStrRadixErr = core::num::ParseIntError;

    fn from_str_radix(s: &str, radix: u32) -> Result<Self, core::num::ParseIntError> {
        <Aint<i8, WIDTH>>::from_str_radix(s, radix)
    }
}

impl<const WIDTH: u32> num_traits::Num for Aint<i16, WIDTH> {
    type FromStrRadixErr = core::num::ParseIntError;

    fn from_str_radix(s: &str, radix: u32) -> Result<Self, core::num::ParseIntError> {
        <Aint<i16, WIDTH>>::from_str_radix(s, radix)
    }
}

impl<const WIDTH: u32> num_traits::Num for Aint<i32, WIDTH> {
    type FromStrRadixErr = core::num::ParseIntError;

    fn from_str_radix(s: &str, radix: u32) -> Result<Self, core::num::ParseIntError> {
        <Aint<i32, WIDTH>>::from_str_radix(s, radix)
    }
}

impl<const WIDTH: u32> num_traits::Num for Aint<i64, WIDTH> {
    type FromStrRadixErr = core::num::ParseIntError;

    fn from_str_radix(s: &str, radix: u32) -> Result<Self, core::num::ParseIntError> {
        <Aint<i64, WIDTH>>::from_str_radix(s, radix)
    }
}

impl<const WIDTH: u32> num_traits::Num for Aint<i128, WIDTH> {
    type FromStrRadixErr = core::num::ParseIntError;

    fn from_str_radix(s: &str, radix: u32) -> Result<Self, core::num::ParseIntError> {
        <Aint<i128, WIDTH>>::from_str_radix(s, radix)
    }
}

impl<const WIDTH: u32> num_integer::Integer for Aint<u8, WIDTH> {
    fn div_floor(&self, other: &Self) -> Self {
        let val = num_integer::Integer::div_floor(&self.0, &other.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to divide with overflow"
        );
        Self::_new_wrapping(val)
    }

    fn mod_floor(&self, other: &Self) -> Self {
        let val = num_integer::Integer::mod_floor(&self.0, &other.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to compute modulus with overflow"
        );
        Self::_new_wrapping(val)
    }

    fn gcd(&self, other: &Self) -> Self {
        let val = num_integer::Integer::gcd(&self.0, &other.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to compute GCD with overflow"
        );
        Self::_new_wrapping(val)
    }

    fn lcm(&self, other: &Self) -> Self {
        let val = num_integer::Integer::lcm(&self.0, &other.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to compute LCM with overflow"
        );
        Self::_new_wrapping(val)
    }

    fn divides(&self, other: &Self) -> bool {
        num_integer::Integer::divides(&self.0, &other.0)
    }

    fn is_multiple_of(&self, other: &Self) -> bool {
        num_integer::Integer::is_multiple_of(&self.0, &other.0)
    }

    fn is_even(&self) -> bool {
        num_integer::Integer::is_even(&self.0)
    }

    fn is_odd(&self) -> bool {
        num_integer::Integer::is_odd(&self.0)
    }

    fn div_rem(&self, other: &Self) -> (Self, Self) {
        let (q, r) = num_integer::Integer::div_rem(&self.0, &other.0);
        debug_assert!(
            q <= Self::MAX.0 && q >= Self::MIN.0,
            "attempt to divide with overflow"
        );
        debug_assert!(
            r <= Self::MAX.0 && r >= Self::MIN.0,
            "attempt to compute modulus with overflow"
        );
        (Self::_new_wrapping(q), Self::_new_wrapping(r))
    }

    fn div_ceil(&self, other: &Self) -> Self {
        let val = num_integer::Integer::div_ceil(&self.0, &other.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to divide with overflow"
        );
        Self::_new_wrapping(val)
    }

    fn gcd_lcm(&self, other: &Self) -> (Self, Self) {
        let (g, l) = num_integer::Integer::gcd_lcm(&self.0, &other.0);
        debug_assert!(
            g <= Self::MAX.0 && g >= Self::MIN.0,
            "attempt to compute GCD with overflow"
        );
        debug_assert!(
            l <= Self::MAX.0 && l >= Self::MIN.0,
            "attempt to compute LCM with overflow"
        );
        (Self::_new_wrapping(g), Self::_new_wrapping(l))
    }

    fn extended_gcd(&self, other: &Self) -> num_integer::ExtendedGcd<Self> {
        let res = num_integer::Integer::extended_gcd(&self.0, &other.0);
        debug_assert!(
            (res.gcd <= Self::MAX.0 && res.gcd >= Self::MIN.0)
                || (res.x <= Self::MAX.0 && res.x >= Self::MIN.0)
                || (res.y <= Self::MAX.0 && res.y >= Self::MIN.0),
            "attempt to compute extended GCD with overflow"
        );
        num_integer::ExtendedGcd {
            gcd: Self::_new_wrapping(res.gcd),
            x: Self::_new_wrapping(res.x),
            y: Self::_new_wrapping(res.y),
        }
    }

    fn div_mod_floor(&self, other: &Self) -> (Self, Self) {
        let (q, r) = num_integer::Integer::div_mod_floor(&self.0, &other.0);
        debug_assert!(
            q <= Self::MAX.0 && q >= Self::MIN.0,
            "attempt to divide with overflow"
        );
        debug_assert!(
            r <= Self::MAX.0 && r >= Self::MIN.0,
            "attempt to compute modulus with overflow"
        );
        (Self::_new_wrapping(q), Self::_new_wrapping(r))
    }

    fn next_multiple_of(&self, other: &Self) -> Self {
        let val = num_integer::Integer::next_multiple_of(&self.0, &other.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to find next multiple with overflow"
        );
        Self::_new_wrapping(val)
    }

    fn prev_multiple_of(&self, other: &Self) -> Self {
        let val = num_integer::Integer::prev_multiple_of(&self.0, &other.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to find previous multiple with overflow"
        );
        Self::_new_wrapping(val)
    }
}

impl<const WIDTH: u32> num_integer::Integer for Aint<u16, WIDTH> {
    fn div_floor(&self, other: &Self) -> Self {
        let val = num_integer::Integer::div_floor(&self.0, &other.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to divide with overflow"
        );
        Self::_new_wrapping(val)
    }

    fn mod_floor(&self, other: &Self) -> Self {
        let val = num_integer::Integer::mod_floor(&self.0, &other.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to compute modulus with overflow"
        );
        Self::_new_wrapping(val)
    }

    fn gcd(&self, other: &Self) -> Self {
        let val = num_integer::Integer::gcd(&self.0, &other.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to compute GCD with overflow"
        );
        Self::_new_wrapping(val)
    }

    fn lcm(&self, other: &Self) -> Self {
        let val = num_integer::Integer::lcm(&self.0, &other.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to compute LCM with overflow"
        );
        Self::_new_wrapping(val)
    }

    fn divides(&self, other: &Self) -> bool {
        num_integer::Integer::divides(&self.0, &other.0)
    }

    fn is_multiple_of(&self, other: &Self) -> bool {
        num_integer::Integer::is_multiple_of(&self.0, &other.0)
    }

    fn is_even(&self) -> bool {
        num_integer::Integer::is_even(&self.0)
    }

    fn is_odd(&self) -> bool {
        num_integer::Integer::is_odd(&self.0)
    }

    fn div_rem(&self, other: &Self) -> (Self, Self) {
        let (q, r) = num_integer::Integer::div_rem(&self.0, &other.0);
        debug_assert!(
            q <= Self::MAX.0 && q >= Self::MIN.0,
            "attempt to divide with overflow"
        );
        debug_assert!(
            r <= Self::MAX.0 && r >= Self::MIN.0,
            "attempt to compute modulus with overflow"
        );
        (Self::_new_wrapping(q), Self::_new_wrapping(r))
    }

    fn div_ceil(&self, other: &Self) -> Self {
        let val = num_integer::Integer::div_ceil(&self.0, &other.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to divide with overflow"
        );
        Self::_new_wrapping(val)
    }

    fn gcd_lcm(&self, other: &Self) -> (Self, Self) {
        let (g, l) = num_integer::Integer::gcd_lcm(&self.0, &other.0);
        debug_assert!(
            g <= Self::MAX.0 && g >= Self::MIN.0,
            "attempt to compute GCD with overflow"
        );
        debug_assert!(
            l <= Self::MAX.0 && l >= Self::MIN.0,
            "attempt to compute LCM with overflow"
        );
        (Self::_new_wrapping(g), Self::_new_wrapping(l))
    }

    fn extended_gcd(&self, other: &Self) -> num_integer::ExtendedGcd<Self> {
        let res = num_integer::Integer::extended_gcd(&self.0, &other.0);
        debug_assert!(
            (res.gcd <= Self::MAX.0 && res.gcd >= Self::MIN.0)
                || (res.x <= Self::MAX.0 && res.x >= Self::MIN.0)
                || (res.y <= Self::MAX.0 && res.y >= Self::MIN.0),
            "attempt to compute extended GCD with overflow"
        );
        num_integer::ExtendedGcd {
            gcd: Self::_new_wrapping(res.gcd),
            x: Self::_new_wrapping(res.x),
            y: Self::_new_wrapping(res.y),
        }
    }

    fn div_mod_floor(&self, other: &Self) -> (Self, Self) {
        let (q, r) = num_integer::Integer::div_mod_floor(&self.0, &other.0);
        debug_assert!(
            q <= Self::MAX.0 && q >= Self::MIN.0,
            "attempt to divide with overflow"
        );
        debug_assert!(
            r <= Self::MAX.0 && r >= Self::MIN.0,
            "attempt to compute modulus with overflow"
        );
        (Self::_new_wrapping(q), Self::_new_wrapping(r))
    }

    fn next_multiple_of(&self, other: &Self) -> Self {
        let val = num_integer::Integer::next_multiple_of(&self.0, &other.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to find next multiple with overflow"
        );
        Self::_new_wrapping(val)
    }

    fn prev_multiple_of(&self, other: &Self) -> Self {
        let val = num_integer::Integer::prev_multiple_of(&self.0, &other.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to find previous multiple with overflow"
        );
        Self::_new_wrapping(val)
    }
}

impl<const WIDTH: u32> num_integer::Integer for Aint<u32, WIDTH> {
    fn div_floor(&self, other: &Self) -> Self {
        let val = num_integer::Integer::div_floor(&self.0, &other.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to divide with overflow"
        );
        Self::_new_wrapping(val)
    }

    fn mod_floor(&self, other: &Self) -> Self {
        let val = num_integer::Integer::mod_floor(&self.0, &other.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to compute modulus with overflow"
        );
        Self::_new_wrapping(val)
    }

    fn gcd(&self, other: &Self) -> Self {
        let val = num_integer::Integer::gcd(&self.0, &other.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to compute GCD with overflow"
        );
        Self::_new_wrapping(val)
    }

    fn lcm(&self, other: &Self) -> Self {
        let val = num_integer::Integer::lcm(&self.0, &other.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to compute LCM with overflow"
        );
        Self::_new_wrapping(val)
    }

    fn divides(&self, other: &Self) -> bool {
        num_integer::Integer::divides(&self.0, &other.0)
    }

    fn is_multiple_of(&self, other: &Self) -> bool {
        num_integer::Integer::is_multiple_of(&self.0, &other.0)
    }

    fn is_even(&self) -> bool {
        num_integer::Integer::is_even(&self.0)
    }

    fn is_odd(&self) -> bool {
        num_integer::Integer::is_odd(&self.0)
    }

    fn div_rem(&self, other: &Self) -> (Self, Self) {
        let (q, r) = num_integer::Integer::div_rem(&self.0, &other.0);
        debug_assert!(
            q <= Self::MAX.0 && q >= Self::MIN.0,
            "attempt to divide with overflow"
        );
        debug_assert!(
            r <= Self::MAX.0 && r >= Self::MIN.0,
            "attempt to compute modulus with overflow"
        );
        (Self::_new_wrapping(q), Self::_new_wrapping(r))
    }

    fn div_ceil(&self, other: &Self) -> Self {
        let val = num_integer::Integer::div_ceil(&self.0, &other.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to divide with overflow"
        );
        Self::_new_wrapping(val)
    }

    fn gcd_lcm(&self, other: &Self) -> (Self, Self) {
        let (g, l) = num_integer::Integer::gcd_lcm(&self.0, &other.0);
        debug_assert!(
            g <= Self::MAX.0 && g >= Self::MIN.0,
            "attempt to compute GCD with overflow"
        );
        debug_assert!(
            l <= Self::MAX.0 && l >= Self::MIN.0,
            "attempt to compute LCM with overflow"
        );
        (Self::_new_wrapping(g), Self::_new_wrapping(l))
    }

    fn extended_gcd(&self, other: &Self) -> num_integer::ExtendedGcd<Self> {
        let res = num_integer::Integer::extended_gcd(&self.0, &other.0);
        debug_assert!(
            (res.gcd <= Self::MAX.0 && res.gcd >= Self::MIN.0)
                || (res.x <= Self::MAX.0 && res.x >= Self::MIN.0)
                || (res.y <= Self::MAX.0 && res.y >= Self::MIN.0),
            "attempt to compute extended GCD with overflow"
        );
        num_integer::ExtendedGcd {
            gcd: Self::_new_wrapping(res.gcd),
            x: Self::_new_wrapping(res.x),
            y: Self::_new_wrapping(res.y),
        }
    }

    fn div_mod_floor(&self, other: &Self) -> (Self, Self) {
        let (q, r) = num_integer::Integer::div_mod_floor(&self.0, &other.0);
        debug_assert!(
            q <= Self::MAX.0 && q >= Self::MIN.0,
            "attempt to divide with overflow"
        );
        debug_assert!(
            r <= Self::MAX.0 && r >= Self::MIN.0,
            "attempt to compute modulus with overflow"
        );
        (Self::_new_wrapping(q), Self::_new_wrapping(r))
    }

    fn next_multiple_of(&self, other: &Self) -> Self {
        let val = num_integer::Integer::next_multiple_of(&self.0, &other.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to find next multiple with overflow"
        );
        Self::_new_wrapping(val)
    }

    fn prev_multiple_of(&self, other: &Self) -> Self {
        let val = num_integer::Integer::prev_multiple_of(&self.0, &other.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to find previous multiple with overflow"
        );
        Self::_new_wrapping(val)
    }
}

impl<const WIDTH: u32> num_integer::Integer for Aint<u64, WIDTH> {
    fn div_floor(&self, other: &Self) -> Self {
        let val = num_integer::Integer::div_floor(&self.0, &other.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to divide with overflow"
        );
        Self::_new_wrapping(val)
    }

    fn mod_floor(&self, other: &Self) -> Self {
        let val = num_integer::Integer::mod_floor(&self.0, &other.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to compute modulus with overflow"
        );
        Self::_new_wrapping(val)
    }

    fn gcd(&self, other: &Self) -> Self {
        let val = num_integer::Integer::gcd(&self.0, &other.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to compute GCD with overflow"
        );
        Self::_new_wrapping(val)
    }

    fn lcm(&self, other: &Self) -> Self {
        let val = num_integer::Integer::lcm(&self.0, &other.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to compute LCM with overflow"
        );
        Self::_new_wrapping(val)
    }

    fn divides(&self, other: &Self) -> bool {
        num_integer::Integer::divides(&self.0, &other.0)
    }

    fn is_multiple_of(&self, other: &Self) -> bool {
        num_integer::Integer::is_multiple_of(&self.0, &other.0)
    }

    fn is_even(&self) -> bool {
        num_integer::Integer::is_even(&self.0)
    }

    fn is_odd(&self) -> bool {
        num_integer::Integer::is_odd(&self.0)
    }

    fn div_rem(&self, other: &Self) -> (Self, Self) {
        let (q, r) = num_integer::Integer::div_rem(&self.0, &other.0);
        debug_assert!(
            q <= Self::MAX.0 && q >= Self::MIN.0,
            "attempt to divide with overflow"
        );
        debug_assert!(
            r <= Self::MAX.0 && r >= Self::MIN.0,
            "attempt to compute modulus with overflow"
        );
        (Self::_new_wrapping(q), Self::_new_wrapping(r))
    }

    fn div_ceil(&self, other: &Self) -> Self {
        let val = num_integer::Integer::div_ceil(&self.0, &other.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to divide with overflow"
        );
        Self::_new_wrapping(val)
    }

    fn gcd_lcm(&self, other: &Self) -> (Self, Self) {
        let (g, l) = num_integer::Integer::gcd_lcm(&self.0, &other.0);
        debug_assert!(
            g <= Self::MAX.0 && g >= Self::MIN.0,
            "attempt to compute GCD with overflow"
        );
        debug_assert!(
            l <= Self::MAX.0 && l >= Self::MIN.0,
            "attempt to compute LCM with overflow"
        );
        (Self::_new_wrapping(g), Self::_new_wrapping(l))
    }

    fn extended_gcd(&self, other: &Self) -> num_integer::ExtendedGcd<Self> {
        let res = num_integer::Integer::extended_gcd(&self.0, &other.0);
        debug_assert!(
            (res.gcd <= Self::MAX.0 && res.gcd >= Self::MIN.0)
                || (res.x <= Self::MAX.0 && res.x >= Self::MIN.0)
                || (res.y <= Self::MAX.0 && res.y >= Self::MIN.0),
            "attempt to compute extended GCD with overflow"
        );
        num_integer::ExtendedGcd {
            gcd: Self::_new_wrapping(res.gcd),
            x: Self::_new_wrapping(res.x),
            y: Self::_new_wrapping(res.y),
        }
    }

    fn div_mod_floor(&self, other: &Self) -> (Self, Self) {
        let (q, r) = num_integer::Integer::div_mod_floor(&self.0, &other.0);
        debug_assert!(
            q <= Self::MAX.0 && q >= Self::MIN.0,
            "attempt to divide with overflow"
        );
        debug_assert!(
            r <= Self::MAX.0 && r >= Self::MIN.0,
            "attempt to compute modulus with overflow"
        );
        (Self::_new_wrapping(q), Self::_new_wrapping(r))
    }

    fn next_multiple_of(&self, other: &Self) -> Self {
        let val = num_integer::Integer::next_multiple_of(&self.0, &other.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to find next multiple with overflow"
        );
        Self::_new_wrapping(val)
    }

    fn prev_multiple_of(&self, other: &Self) -> Self {
        let val = num_integer::Integer::prev_multiple_of(&self.0, &other.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to find previous multiple with overflow"
        );
        Self::_new_wrapping(val)
    }
}

impl<const WIDTH: u32> num_integer::Integer for Aint<u128, WIDTH> {
    fn div_floor(&self, other: &Self) -> Self {
        let val = num_integer::Integer::div_floor(&self.0, &other.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to divide with overflow"
        );
        Self::_new_wrapping(val)
    }

    fn mod_floor(&self, other: &Self) -> Self {
        let val = num_integer::Integer::mod_floor(&self.0, &other.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to compute modulus with overflow"
        );
        Self::_new_wrapping(val)
    }

    fn gcd(&self, other: &Self) -> Self {
        let val = num_integer::Integer::gcd(&self.0, &other.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to compute GCD with overflow"
        );
        Self::_new_wrapping(val)
    }

    fn lcm(&self, other: &Self) -> Self {
        let val = num_integer::Integer::lcm(&self.0, &other.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to compute LCM with overflow"
        );
        Self::_new_wrapping(val)
    }

    fn divides(&self, other: &Self) -> bool {
        num_integer::Integer::divides(&self.0, &other.0)
    }

    fn is_multiple_of(&self, other: &Self) -> bool {
        num_integer::Integer::is_multiple_of(&self.0, &other.0)
    }

    fn is_even(&self) -> bool {
        num_integer::Integer::is_even(&self.0)
    }

    fn is_odd(&self) -> bool {
        num_integer::Integer::is_odd(&self.0)
    }

    fn div_rem(&self, other: &Self) -> (Self, Self) {
        let (q, r) = num_integer::Integer::div_rem(&self.0, &other.0);
        debug_assert!(
            q <= Self::MAX.0 && q >= Self::MIN.0,
            "attempt to divide with overflow"
        );
        debug_assert!(
            r <= Self::MAX.0 && r >= Self::MIN.0,
            "attempt to compute modulus with overflow"
        );
        (Self::_new_wrapping(q), Self::_new_wrapping(r))
    }

    fn div_ceil(&self, other: &Self) -> Self {
        let val = num_integer::Integer::div_ceil(&self.0, &other.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to divide with overflow"
        );
        Self::_new_wrapping(val)
    }

    fn gcd_lcm(&self, other: &Self) -> (Self, Self) {
        let (g, l) = num_integer::Integer::gcd_lcm(&self.0, &other.0);
        debug_assert!(
            g <= Self::MAX.0 && g >= Self::MIN.0,
            "attempt to compute GCD with overflow"
        );
        debug_assert!(
            l <= Self::MAX.0 && l >= Self::MIN.0,
            "attempt to compute LCM with overflow"
        );
        (Self::_new_wrapping(g), Self::_new_wrapping(l))
    }

    fn extended_gcd(&self, other: &Self) -> num_integer::ExtendedGcd<Self> {
        let res = num_integer::Integer::extended_gcd(&self.0, &other.0);
        debug_assert!(
            (res.gcd <= Self::MAX.0 && res.gcd >= Self::MIN.0)
                || (res.x <= Self::MAX.0 && res.x >= Self::MIN.0)
                || (res.y <= Self::MAX.0 && res.y >= Self::MIN.0),
            "attempt to compute extended GCD with overflow"
        );
        num_integer::ExtendedGcd {
            gcd: Self::_new_wrapping(res.gcd),
            x: Self::_new_wrapping(res.x),
            y: Self::_new_wrapping(res.y),
        }
    }

    fn div_mod_floor(&self, other: &Self) -> (Self, Self) {
        let (q, r) = num_integer::Integer::div_mod_floor(&self.0, &other.0);
        debug_assert!(
            q <= Self::MAX.0 && q >= Self::MIN.0,
            "attempt to divide with overflow"
        );
        debug_assert!(
            r <= Self::MAX.0 && r >= Self::MIN.0,
            "attempt to compute modulus with overflow"
        );
        (Self::_new_wrapping(q), Self::_new_wrapping(r))
    }

    fn next_multiple_of(&self, other: &Self) -> Self {
        let val = num_integer::Integer::next_multiple_of(&self.0, &other.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to find next multiple with overflow"
        );
        Self::_new_wrapping(val)
    }

    fn prev_multiple_of(&self, other: &Self) -> Self {
        let val = num_integer::Integer::prev_multiple_of(&self.0, &other.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to find previous multiple with overflow"
        );
        Self::_new_wrapping(val)
    }
}

impl<const WIDTH: u32> num_integer::Integer for Aint<i8, WIDTH> {
    fn div_floor(&self, other: &Self) -> Self {
        let val = num_integer::Integer::div_floor(&self.0, &other.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to divide with overflow"
        );
        Self::_new_wrapping(val)
    }

    fn mod_floor(&self, other: &Self) -> Self {
        let val = num_integer::Integer::mod_floor(&self.0, &other.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to compute modulus with overflow"
        );
        Self::_new_wrapping(val)
    }

    fn gcd(&self, other: &Self) -> Self {
        let val = num_integer::Integer::gcd(&self.0, &other.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to compute GCD with overflow"
        );
        Self::_new_wrapping(val)
    }

    fn lcm(&self, other: &Self) -> Self {
        let val = num_integer::Integer::lcm(&self.0, &other.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to compute LCM with overflow"
        );
        Self::_new_wrapping(val)
    }

    fn divides(&self, other: &Self) -> bool {
        num_integer::Integer::divides(&self.0, &other.0)
    }

    fn is_multiple_of(&self, other: &Self) -> bool {
        num_integer::Integer::is_multiple_of(&self.0, &other.0)
    }

    fn is_even(&self) -> bool {
        num_integer::Integer::is_even(&self.0)
    }

    fn is_odd(&self) -> bool {
        num_integer::Integer::is_odd(&self.0)
    }

    fn div_rem(&self, other: &Self) -> (Self, Self) {
        let (q, r) = num_integer::Integer::div_rem(&self.0, &other.0);
        debug_assert!(
            q <= Self::MAX.0 && q >= Self::MIN.0,
            "attempt to divide with overflow"
        );
        debug_assert!(
            r <= Self::MAX.0 && r >= Self::MIN.0,
            "attempt to compute modulus with overflow"
        );
        (Self::_new_wrapping(q), Self::_new_wrapping(r))
    }

    fn div_ceil(&self, other: &Self) -> Self {
        let val = num_integer::Integer::div_ceil(&self.0, &other.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to divide with overflow"
        );
        Self::_new_wrapping(val)
    }

    fn gcd_lcm(&self, other: &Self) -> (Self, Self) {
        let (g, l) = num_integer::Integer::gcd_lcm(&self.0, &other.0);
        debug_assert!(
            g <= Self::MAX.0 && g >= Self::MIN.0,
            "attempt to compute GCD with overflow"
        );
        debug_assert!(
            l <= Self::MAX.0 && l >= Self::MIN.0,
            "attempt to compute LCM with overflow"
        );
        (Self::_new_wrapping(g), Self::_new_wrapping(l))
    }

    fn extended_gcd(&self, other: &Self) -> num_integer::ExtendedGcd<Self> {
        let res = num_integer::Integer::extended_gcd(&self.0, &other.0);
        debug_assert!(
            (res.gcd <= Self::MAX.0 && res.gcd >= Self::MIN.0)
                || (res.x <= Self::MAX.0 && res.x >= Self::MIN.0)
                || (res.y <= Self::MAX.0 && res.y >= Self::MIN.0),
            "attempt to compute extended GCD with overflow"
        );
        num_integer::ExtendedGcd {
            gcd: Self::_new_wrapping(res.gcd),
            x: Self::_new_wrapping(res.x),
            y: Self::_new_wrapping(res.y),
        }
    }

    fn div_mod_floor(&self, other: &Self) -> (Self, Self) {
        let (q, r) = num_integer::Integer::div_mod_floor(&self.0, &other.0);
        debug_assert!(
            q <= Self::MAX.0 && q >= Self::MIN.0,
            "attempt to divide with overflow"
        );
        debug_assert!(
            r <= Self::MAX.0 && r >= Self::MIN.0,
            "attempt to compute modulus with overflow"
        );
        (Self::_new_wrapping(q), Self::_new_wrapping(r))
    }

    fn next_multiple_of(&self, other: &Self) -> Self {
        let val = num_integer::Integer::next_multiple_of(&self.0, &other.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to find next multiple with overflow"
        );
        Self::_new_wrapping(val)
    }

    fn prev_multiple_of(&self, other: &Self) -> Self {
        let val = num_integer::Integer::prev_multiple_of(&self.0, &other.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to find previous multiple with overflow"
        );
        Self::_new_wrapping(val)
    }
}

impl<const WIDTH: u32> num_integer::Integer for Aint<i16, WIDTH> {
    fn div_floor(&self, other: &Self) -> Self {
        let val = num_integer::Integer::div_floor(&self.0, &other.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to divide with overflow"
        );
        Self::_new_wrapping(val)
    }

    fn mod_floor(&self, other: &Self) -> Self {
        let val = num_integer::Integer::mod_floor(&self.0, &other.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to compute modulus with overflow"
        );
        Self::_new_wrapping(val)
    }

    fn gcd(&self, other: &Self) -> Self {
        let val = num_integer::Integer::gcd(&self.0, &other.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to compute GCD with overflow"
        );
        Self::_new_wrapping(val)
    }

    fn lcm(&self, other: &Self) -> Self {
        let val = num_integer::Integer::lcm(&self.0, &other.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to compute LCM with overflow"
        );
        Self::_new_wrapping(val)
    }

    fn divides(&self, other: &Self) -> bool {
        num_integer::Integer::divides(&self.0, &other.0)
    }

    fn is_multiple_of(&self, other: &Self) -> bool {
        num_integer::Integer::is_multiple_of(&self.0, &other.0)
    }

    fn is_even(&self) -> bool {
        num_integer::Integer::is_even(&self.0)
    }

    fn is_odd(&self) -> bool {
        num_integer::Integer::is_odd(&self.0)
    }

    fn div_rem(&self, other: &Self) -> (Self, Self) {
        let (q, r) = num_integer::Integer::div_rem(&self.0, &other.0);
        debug_assert!(
            q <= Self::MAX.0 && q >= Self::MIN.0,
            "attempt to divide with overflow"
        );
        debug_assert!(
            r <= Self::MAX.0 && r >= Self::MIN.0,
            "attempt to compute modulus with overflow"
        );
        (Self::_new_wrapping(q), Self::_new_wrapping(r))
    }

    fn div_ceil(&self, other: &Self) -> Self {
        let val = num_integer::Integer::div_ceil(&self.0, &other.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to divide with overflow"
        );
        Self::_new_wrapping(val)
    }

    fn gcd_lcm(&self, other: &Self) -> (Self, Self) {
        let (g, l) = num_integer::Integer::gcd_lcm(&self.0, &other.0);
        debug_assert!(
            g <= Self::MAX.0 && g >= Self::MIN.0,
            "attempt to compute GCD with overflow"
        );
        debug_assert!(
            l <= Self::MAX.0 && l >= Self::MIN.0,
            "attempt to compute LCM with overflow"
        );
        (Self::_new_wrapping(g), Self::_new_wrapping(l))
    }

    fn extended_gcd(&self, other: &Self) -> num_integer::ExtendedGcd<Self> {
        let res = num_integer::Integer::extended_gcd(&self.0, &other.0);
        debug_assert!(
            (res.gcd <= Self::MAX.0 && res.gcd >= Self::MIN.0)
                || (res.x <= Self::MAX.0 && res.x >= Self::MIN.0)
                || (res.y <= Self::MAX.0 && res.y >= Self::MIN.0),
            "attempt to compute extended GCD with overflow"
        );
        num_integer::ExtendedGcd {
            gcd: Self::_new_wrapping(res.gcd),
            x: Self::_new_wrapping(res.x),
            y: Self::_new_wrapping(res.y),
        }
    }

    fn div_mod_floor(&self, other: &Self) -> (Self, Self) {
        let (q, r) = num_integer::Integer::div_mod_floor(&self.0, &other.0);
        debug_assert!(
            q <= Self::MAX.0 && q >= Self::MIN.0,
            "attempt to divide with overflow"
        );
        debug_assert!(
            r <= Self::MAX.0 && r >= Self::MIN.0,
            "attempt to compute modulus with overflow"
        );
        (Self::_new_wrapping(q), Self::_new_wrapping(r))
    }

    fn next_multiple_of(&self, other: &Self) -> Self {
        let val = num_integer::Integer::next_multiple_of(&self.0, &other.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to find next multiple with overflow"
        );
        Self::_new_wrapping(val)
    }

    fn prev_multiple_of(&self, other: &Self) -> Self {
        let val = num_integer::Integer::prev_multiple_of(&self.0, &other.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to find previous multiple with overflow"
        );
        Self::_new_wrapping(val)
    }
}

impl<const WIDTH: u32> num_integer::Integer for Aint<i32, WIDTH> {
    fn div_floor(&self, other: &Self) -> Self {
        let val = num_integer::Integer::div_floor(&self.0, &other.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to divide with overflow"
        );
        Self::_new_wrapping(val)
    }

    fn mod_floor(&self, other: &Self) -> Self {
        let val = num_integer::Integer::mod_floor(&self.0, &other.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to compute modulus with overflow"
        );
        Self::_new_wrapping(val)
    }

    fn gcd(&self, other: &Self) -> Self {
        let val = num_integer::Integer::gcd(&self.0, &other.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to compute GCD with overflow"
        );
        Self::_new_wrapping(val)
    }

    fn lcm(&self, other: &Self) -> Self {
        let val = num_integer::Integer::lcm(&self.0, &other.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to compute LCM with overflow"
        );
        Self::_new_wrapping(val)
    }

    fn divides(&self, other: &Self) -> bool {
        num_integer::Integer::divides(&self.0, &other.0)
    }

    fn is_multiple_of(&self, other: &Self) -> bool {
        num_integer::Integer::is_multiple_of(&self.0, &other.0)
    }

    fn is_even(&self) -> bool {
        num_integer::Integer::is_even(&self.0)
    }

    fn is_odd(&self) -> bool {
        num_integer::Integer::is_odd(&self.0)
    }

    fn div_rem(&self, other: &Self) -> (Self, Self) {
        let (q, r) = num_integer::Integer::div_rem(&self.0, &other.0);
        debug_assert!(
            q <= Self::MAX.0 && q >= Self::MIN.0,
            "attempt to divide with overflow"
        );
        debug_assert!(
            r <= Self::MAX.0 && r >= Self::MIN.0,
            "attempt to compute modulus with overflow"
        );
        (Self::_new_wrapping(q), Self::_new_wrapping(r))
    }

    fn div_ceil(&self, other: &Self) -> Self {
        let val = num_integer::Integer::div_ceil(&self.0, &other.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to divide with overflow"
        );
        Self::_new_wrapping(val)
    }

    fn gcd_lcm(&self, other: &Self) -> (Self, Self) {
        let (g, l) = num_integer::Integer::gcd_lcm(&self.0, &other.0);
        debug_assert!(
            g <= Self::MAX.0 && g >= Self::MIN.0,
            "attempt to compute GCD with overflow"
        );
        debug_assert!(
            l <= Self::MAX.0 && l >= Self::MIN.0,
            "attempt to compute LCM with overflow"
        );
        (Self::_new_wrapping(g), Self::_new_wrapping(l))
    }

    fn extended_gcd(&self, other: &Self) -> num_integer::ExtendedGcd<Self> {
        let res = num_integer::Integer::extended_gcd(&self.0, &other.0);
        debug_assert!(
            (res.gcd <= Self::MAX.0 && res.gcd >= Self::MIN.0)
                || (res.x <= Self::MAX.0 && res.x >= Self::MIN.0)
                || (res.y <= Self::MAX.0 && res.y >= Self::MIN.0),
            "attempt to compute extended GCD with overflow"
        );
        num_integer::ExtendedGcd {
            gcd: Self::_new_wrapping(res.gcd),
            x: Self::_new_wrapping(res.x),
            y: Self::_new_wrapping(res.y),
        }
    }

    fn div_mod_floor(&self, other: &Self) -> (Self, Self) {
        let (q, r) = num_integer::Integer::div_mod_floor(&self.0, &other.0);
        debug_assert!(
            q <= Self::MAX.0 && q >= Self::MIN.0,
            "attempt to divide with overflow"
        );
        debug_assert!(
            r <= Self::MAX.0 && r >= Self::MIN.0,
            "attempt to compute modulus with overflow"
        );
        (Self::_new_wrapping(q), Self::_new_wrapping(r))
    }

    fn next_multiple_of(&self, other: &Self) -> Self {
        let val = num_integer::Integer::next_multiple_of(&self.0, &other.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to find next multiple with overflow"
        );
        Self::_new_wrapping(val)
    }

    fn prev_multiple_of(&self, other: &Self) -> Self {
        let val = num_integer::Integer::prev_multiple_of(&self.0, &other.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to find previous multiple with overflow"
        );
        Self::_new_wrapping(val)
    }
}

impl<const WIDTH: u32> num_integer::Integer for Aint<i64, WIDTH> {
    fn div_floor(&self, other: &Self) -> Self {
        let val = num_integer::Integer::div_floor(&self.0, &other.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to divide with overflow"
        );
        Self::_new_wrapping(val)
    }

    fn mod_floor(&self, other: &Self) -> Self {
        let val = num_integer::Integer::mod_floor(&self.0, &other.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to compute modulus with overflow"
        );
        Self::_new_wrapping(val)
    }

    fn gcd(&self, other: &Self) -> Self {
        let val = num_integer::Integer::gcd(&self.0, &other.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to compute GCD with overflow"
        );
        Self::_new_wrapping(val)
    }

    fn lcm(&self, other: &Self) -> Self {
        let val = num_integer::Integer::lcm(&self.0, &other.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to compute LCM with overflow"
        );
        Self::_new_wrapping(val)
    }

    fn divides(&self, other: &Self) -> bool {
        num_integer::Integer::divides(&self.0, &other.0)
    }

    fn is_multiple_of(&self, other: &Self) -> bool {
        num_integer::Integer::is_multiple_of(&self.0, &other.0)
    }

    fn is_even(&self) -> bool {
        num_integer::Integer::is_even(&self.0)
    }

    fn is_odd(&self) -> bool {
        num_integer::Integer::is_odd(&self.0)
    }

    fn div_rem(&self, other: &Self) -> (Self, Self) {
        let (q, r) = num_integer::Integer::div_rem(&self.0, &other.0);
        debug_assert!(
            q <= Self::MAX.0 && q >= Self::MIN.0,
            "attempt to divide with overflow"
        );
        debug_assert!(
            r <= Self::MAX.0 && r >= Self::MIN.0,
            "attempt to compute modulus with overflow"
        );
        (Self::_new_wrapping(q), Self::_new_wrapping(r))
    }

    fn div_ceil(&self, other: &Self) -> Self {
        let val = num_integer::Integer::div_ceil(&self.0, &other.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to divide with overflow"
        );
        Self::_new_wrapping(val)
    }

    fn gcd_lcm(&self, other: &Self) -> (Self, Self) {
        let (g, l) = num_integer::Integer::gcd_lcm(&self.0, &other.0);
        debug_assert!(
            g <= Self::MAX.0 && g >= Self::MIN.0,
            "attempt to compute GCD with overflow"
        );
        debug_assert!(
            l <= Self::MAX.0 && l >= Self::MIN.0,
            "attempt to compute LCM with overflow"
        );
        (Self::_new_wrapping(g), Self::_new_wrapping(l))
    }

    fn extended_gcd(&self, other: &Self) -> num_integer::ExtendedGcd<Self> {
        let res = num_integer::Integer::extended_gcd(&self.0, &other.0);
        debug_assert!(
            (res.gcd <= Self::MAX.0 && res.gcd >= Self::MIN.0)
                || (res.x <= Self::MAX.0 && res.x >= Self::MIN.0)
                || (res.y <= Self::MAX.0 && res.y >= Self::MIN.0),
            "attempt to compute extended GCD with overflow"
        );
        num_integer::ExtendedGcd {
            gcd: Self::_new_wrapping(res.gcd),
            x: Self::_new_wrapping(res.x),
            y: Self::_new_wrapping(res.y),
        }
    }

    fn div_mod_floor(&self, other: &Self) -> (Self, Self) {
        let (q, r) = num_integer::Integer::div_mod_floor(&self.0, &other.0);
        debug_assert!(
            q <= Self::MAX.0 && q >= Self::MIN.0,
            "attempt to divide with overflow"
        );
        debug_assert!(
            r <= Self::MAX.0 && r >= Self::MIN.0,
            "attempt to compute modulus with overflow"
        );
        (Self::_new_wrapping(q), Self::_new_wrapping(r))
    }

    fn next_multiple_of(&self, other: &Self) -> Self {
        let val = num_integer::Integer::next_multiple_of(&self.0, &other.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to find next multiple with overflow"
        );
        Self::_new_wrapping(val)
    }

    fn prev_multiple_of(&self, other: &Self) -> Self {
        let val = num_integer::Integer::prev_multiple_of(&self.0, &other.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to find previous multiple with overflow"
        );
        Self::_new_wrapping(val)
    }
}

impl<const WIDTH: u32> num_integer::Integer for Aint<i128, WIDTH> {
    fn div_floor(&self, other: &Self) -> Self {
        let val = num_integer::Integer::div_floor(&self.0, &other.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to divide with overflow"
        );
        Self::_new_wrapping(val)
    }

    fn mod_floor(&self, other: &Self) -> Self {
        let val = num_integer::Integer::mod_floor(&self.0, &other.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to compute modulus with overflow"
        );
        Self::_new_wrapping(val)
    }

    fn gcd(&self, other: &Self) -> Self {
        let val = num_integer::Integer::gcd(&self.0, &other.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to compute GCD with overflow"
        );
        Self::_new_wrapping(val)
    }

    fn lcm(&self, other: &Self) -> Self {
        let val = num_integer::Integer::lcm(&self.0, &other.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to compute LCM with overflow"
        );
        Self::_new_wrapping(val)
    }

    fn divides(&self, other: &Self) -> bool {
        num_integer::Integer::divides(&self.0, &other.0)
    }

    fn is_multiple_of(&self, other: &Self) -> bool {
        num_integer::Integer::is_multiple_of(&self.0, &other.0)
    }

    fn is_even(&self) -> bool {
        num_integer::Integer::is_even(&self.0)
    }

    fn is_odd(&self) -> bool {
        num_integer::Integer::is_odd(&self.0)
    }

    fn div_rem(&self, other: &Self) -> (Self, Self) {
        let (q, r) = num_integer::Integer::div_rem(&self.0, &other.0);
        debug_assert!(
            q <= Self::MAX.0 && q >= Self::MIN.0,
            "attempt to divide with overflow"
        );
        debug_assert!(
            r <= Self::MAX.0 && r >= Self::MIN.0,
            "attempt to compute modulus with overflow"
        );
        (Self::_new_wrapping(q), Self::_new_wrapping(r))
    }

    fn div_ceil(&self, other: &Self) -> Self {
        let val = num_integer::Integer::div_ceil(&self.0, &other.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to divide with overflow"
        );
        Self::_new_wrapping(val)
    }

    fn gcd_lcm(&self, other: &Self) -> (Self, Self) {
        let (g, l) = num_integer::Integer::gcd_lcm(&self.0, &other.0);
        debug_assert!(
            g <= Self::MAX.0 && g >= Self::MIN.0,
            "attempt to compute GCD with overflow"
        );
        debug_assert!(
            l <= Self::MAX.0 && l >= Self::MIN.0,
            "attempt to compute LCM with overflow"
        );
        (Self::_new_wrapping(g), Self::_new_wrapping(l))
    }

    fn extended_gcd(&self, other: &Self) -> num_integer::ExtendedGcd<Self> {
        let res = num_integer::Integer::extended_gcd(&self.0, &other.0);
        debug_assert!(
            (res.gcd <= Self::MAX.0 && res.gcd >= Self::MIN.0)
                || (res.x <= Self::MAX.0 && res.x >= Self::MIN.0)
                || (res.y <= Self::MAX.0 && res.y >= Self::MIN.0),
            "attempt to compute extended GCD with overflow"
        );
        num_integer::ExtendedGcd {
            gcd: Self::_new_wrapping(res.gcd),
            x: Self::_new_wrapping(res.x),
            y: Self::_new_wrapping(res.y),
        }
    }

    fn div_mod_floor(&self, other: &Self) -> (Self, Self) {
        let (q, r) = num_integer::Integer::div_mod_floor(&self.0, &other.0);
        debug_assert!(
            q <= Self::MAX.0 && q >= Self::MIN.0,
            "attempt to divide with overflow"
        );
        debug_assert!(
            r <= Self::MAX.0 && r >= Self::MIN.0,
            "attempt to compute modulus with overflow"
        );
        (Self::_new_wrapping(q), Self::_new_wrapping(r))
    }

    fn next_multiple_of(&self, other: &Self) -> Self {
        let val = num_integer::Integer::next_multiple_of(&self.0, &other.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to find next multiple with overflow"
        );
        Self::_new_wrapping(val)
    }

    fn prev_multiple_of(&self, other: &Self) -> Self {
        let val = num_integer::Integer::prev_multiple_of(&self.0, &other.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to find previous multiple with overflow"
        );
        Self::_new_wrapping(val)
    }
}

impl<const WIDTH: u32> num_integer::Roots for Aint<u8, WIDTH> {
    fn nth_root(&self, n: u32) -> Self {
        Self(num_integer::Roots::nth_root(&self.0, n))
    }

    fn sqrt(&self) -> Self {
        Self(num_integer::Roots::sqrt(&self.0))
    }

    fn cbrt(&self) -> Self {
        Self(num_integer::Roots::cbrt(&self.0))
    }
}

impl<const WIDTH: u32> num_integer::Roots for Aint<u16, WIDTH> {
    fn nth_root(&self, n: u32) -> Self {
        Self(num_integer::Roots::nth_root(&self.0, n))
    }

    fn sqrt(&self) -> Self {
        Self(num_integer::Roots::sqrt(&self.0))
    }

    fn cbrt(&self) -> Self {
        Self(num_integer::Roots::cbrt(&self.0))
    }
}

impl<const WIDTH: u32> num_integer::Roots for Aint<u32, WIDTH> {
    fn nth_root(&self, n: u32) -> Self {
        Self(num_integer::Roots::nth_root(&self.0, n))
    }

    fn sqrt(&self) -> Self {
        Self(num_integer::Roots::sqrt(&self.0))
    }

    fn cbrt(&self) -> Self {
        Self(num_integer::Roots::cbrt(&self.0))
    }
}

impl<const WIDTH: u32> num_integer::Roots for Aint<u64, WIDTH> {
    fn nth_root(&self, n: u32) -> Self {
        Self(num_integer::Roots::nth_root(&self.0, n))
    }

    fn sqrt(&self) -> Self {
        Self(num_integer::Roots::sqrt(&self.0))
    }

    fn cbrt(&self) -> Self {
        Self(num_integer::Roots::cbrt(&self.0))
    }
}

impl<const WIDTH: u32> num_integer::Roots for Aint<u128, WIDTH> {
    fn nth_root(&self, n: u32) -> Self {
        Self(num_integer::Roots::nth_root(&self.0, n))
    }

    fn sqrt(&self) -> Self {
        Self(num_integer::Roots::sqrt(&self.0))
    }

    fn cbrt(&self) -> Self {
        Self(num_integer::Roots::cbrt(&self.0))
    }
}

impl<const WIDTH: u32> num_integer::Roots for Aint<i8, WIDTH> {
    fn nth_root(&self, n: u32) -> Self {
        Self(num_integer::Roots::nth_root(&self.0, n))
    }

    fn sqrt(&self) -> Self {
        Self(num_integer::Roots::sqrt(&self.0))
    }

    fn cbrt(&self) -> Self {
        Self(num_integer::Roots::cbrt(&self.0))
    }
}

impl<const WIDTH: u32> num_integer::Roots for Aint<i16, WIDTH> {
    fn nth_root(&self, n: u32) -> Self {
        Self(num_integer::Roots::nth_root(&self.0, n))
    }

    fn sqrt(&self) -> Self {
        Self(num_integer::Roots::sqrt(&self.0))
    }

    fn cbrt(&self) -> Self {
        Self(num_integer::Roots::cbrt(&self.0))
    }
}

impl<const WIDTH: u32> num_integer::Roots for Aint<i32, WIDTH> {
    fn nth_root(&self, n: u32) -> Self {
        Self(num_integer::Roots::nth_root(&self.0, n))
    }

    fn sqrt(&self) -> Self {
        Self(num_integer::Roots::sqrt(&self.0))
    }

    fn cbrt(&self) -> Self {
        Self(num_integer::Roots::cbrt(&self.0))
    }
}

impl<const WIDTH: u32> num_integer::Roots for Aint<i64, WIDTH> {
    fn nth_root(&self, n: u32) -> Self {
        Self(num_integer::Roots::nth_root(&self.0, n))
    }

    fn sqrt(&self) -> Self {
        Self(num_integer::Roots::sqrt(&self.0))
    }

    fn cbrt(&self) -> Self {
        Self(num_integer::Roots::cbrt(&self.0))
    }
}

impl<const WIDTH: u32> num_integer::Roots for Aint<i128, WIDTH> {
    fn nth_root(&self, n: u32) -> Self {
        Self(num_integer::Roots::nth_root(&self.0, n))
    }

    fn sqrt(&self) -> Self {
        Self(num_integer::Roots::sqrt(&self.0))
    }

    fn cbrt(&self) -> Self {
        Self(num_integer::Roots::cbrt(&self.0))
    }
}

impl<R: Sealed, const WIDTH: u32> num_traits::Bounded for Aint<R, WIDTH> {
    fn min_value() -> Self {
        Self::_min()
    }

    fn max_value() -> Self {
        Self::_max()
    }
}

impl<R: Sealed, const WIDTH: u32> num_traits::FromPrimitive for Aint<R, WIDTH> {
    fn from_u8(n: u8) -> Option<Self> {
        match n.try_into() {
            Ok(val) => Self::_new(val),
            _ => None,
        }
    }

    fn from_u16(n: u16) -> Option<Self> {
        match n.try_into() {
            Ok(val) => Self::_new(val),
            _ => None,
        }
    }

    fn from_u32(n: u32) -> Option<Self> {
        match n.try_into() {
            Ok(val) => Self::_new(val),
            _ => None,
        }
    }

    fn from_u64(n: u64) -> Option<Self> {
        match n.try_into() {
            Ok(val) => Self::_new(val),
            _ => None,
        }
    }

    fn from_u128(n: u128) -> Option<Self> {
        match n.try_into() {
            Ok(val) => Self::_new(val),
            _ => None,
        }
    }

    fn from_usize(n: usize) -> Option<Self> {
        match n.try_into() {
            Ok(val) => Self::_new(val),
            _ => None,
        }
    }

    fn from_i8(n: i8) -> Option<Self> {
        match n.try_into() {
            Ok(val) => Self::_new(val),
            _ => None,
        }
    }

    fn from_i16(n: i16) -> Option<Self> {
        match n.try_into() {
            Ok(val) => Self::_new(val),
            _ => None,
        }
    }

    fn from_i32(n: i32) -> Option<Self> {
        match n.try_into() {
            Ok(val) => Self::_new(val),
            _ => None,
        }
    }

    fn from_i64(n: i64) -> Option<Self> {
        match n.try_into() {
            Ok(val) => Self::_new(val),
            _ => None,
        }
    }

    fn from_i128(n: i128) -> Option<Self> {
        match n.try_into() {
            Ok(val) => Self::_new(val),
            _ => None,
        }
    }

    fn from_isize(n: isize) -> Option<Self> {
        match n.try_into() {
            Ok(val) => Self::_new(val),
            _ => None,
        }
    }
}

impl<R: Sealed, const WIDTH: u32> num_traits::ToPrimitive for Aint<R, WIDTH> {
    fn to_u8(&self) -> Option<u8> {
        self.0.try_into().ok()
    }

    fn to_u16(&self) -> Option<u16> {
        self.0.try_into().ok()
    }

    fn to_u32(&self) -> Option<u32> {
        self.0.try_into().ok()
    }

    fn to_u64(&self) -> Option<u64> {
        self.0.try_into().ok()
    }

    fn to_u128(&self) -> Option<u128> {
        self.0.try_into().ok()
    }

    fn to_usize(&self) -> Option<usize> {
        self.0.try_into().ok()
    }

    fn to_i8(&self) -> Option<i8> {
        self.0.try_into().ok()
    }

    fn to_i16(&self) -> Option<i16> {
        self.0.try_into().ok()
    }

    fn to_i32(&self) -> Option<i32> {
        self.0.try_into().ok()
    }

    fn to_i64(&self) -> Option<i64> {
        self.0.try_into().ok()
    }

    fn to_i128(&self) -> Option<i128> {
        self.0.try_into().ok()
    }

    fn to_isize(&self) -> Option<isize> {
        self.0.try_into().ok()
    }
}

impl<const WIDTH: u32> num_traits::NumCast for Aint<u8, WIDTH> {
    fn from<T: num_traits::ToPrimitive>(n: T) -> Option<Self> {
        match n.to_u8() {
            Some(n) => Self::new(n),
            None => None,
        }
    }
}

impl<const WIDTH: u32> num_traits::NumCast for Aint<u16, WIDTH> {
    fn from<T: num_traits::ToPrimitive>(n: T) -> Option<Self> {
        match n.to_u16() {
            Some(n) => Self::new(n),
            None => None,
        }
    }
}

impl<const WIDTH: u32> num_traits::NumCast for Aint<u32, WIDTH> {
    fn from<T: num_traits::ToPrimitive>(n: T) -> Option<Self> {
        match n.to_u32() {
            Some(n) => Self::new(n),
            None => None,
        }
    }
}

impl<const WIDTH: u32> num_traits::NumCast for Aint<u64, WIDTH> {
    fn from<T: num_traits::ToPrimitive>(n: T) -> Option<Self> {
        match n.to_u64() {
            Some(n) => Self::new(n),
            None => None,
        }
    }
}

impl<const WIDTH: u32> num_traits::NumCast for Aint<u128, WIDTH> {
    fn from<T: num_traits::ToPrimitive>(n: T) -> Option<Self> {
        match n.to_u128() {
            Some(n) => Self::new(n),
            None => None,
        }
    }
}

impl<const WIDTH: u32> num_traits::NumCast for Aint<i8, WIDTH> {
    fn from<T: num_traits::ToPrimitive>(n: T) -> Option<Self> {
        match n.to_i8() {
            Some(n) => Self::new(n),
            None => None,
        }
    }
}

impl<const WIDTH: u32> num_traits::NumCast for Aint<i16, WIDTH> {
    fn from<T: num_traits::ToPrimitive>(n: T) -> Option<Self> {
        match n.to_i16() {
            Some(n) => Self::new(n),
            None => None,
        }
    }
}

impl<const WIDTH: u32> num_traits::NumCast for Aint<i32, WIDTH> {
    fn from<T: num_traits::ToPrimitive>(n: T) -> Option<Self> {
        match n.to_i32() {
            Some(n) => Self::new(n),
            None => None,
        }
    }
}

impl<const WIDTH: u32> num_traits::NumCast for Aint<i64, WIDTH> {
    fn from<T: num_traits::ToPrimitive>(n: T) -> Option<Self> {
        match n.to_i64() {
            Some(n) => Self::new(n),
            None => None,
        }
    }
}

impl<const WIDTH: u32> num_traits::NumCast for Aint<i128, WIDTH> {
    fn from<T: num_traits::ToPrimitive>(n: T) -> Option<Self> {
        match n.to_i128() {
            Some(n) => Self::new(n),
            None => None,
        }
    }
}

impl<const WIDTH: u32> num_traits::CheckedNeg for Aint<u8, WIDTH> {
    fn checked_neg(&self) -> Option<Self> {
        <Aint<u8, WIDTH>>::checked_neg(*self)
    }
}

impl<const WIDTH: u32> num_traits::CheckedNeg for Aint<u16, WIDTH> {
    fn checked_neg(&self) -> Option<Self> {
        <Aint<u16, WIDTH>>::checked_neg(*self)
    }
}

impl<const WIDTH: u32> num_traits::CheckedNeg for Aint<u32, WIDTH> {
    fn checked_neg(&self) -> Option<Self> {
        <Aint<u32, WIDTH>>::checked_neg(*self)
    }
}

impl<const WIDTH: u32> num_traits::CheckedNeg for Aint<u64, WIDTH> {
    fn checked_neg(&self) -> Option<Self> {
        <Aint<u64, WIDTH>>::checked_neg(*self)
    }
}

impl<const WIDTH: u32> num_traits::CheckedNeg for Aint<u128, WIDTH> {
    fn checked_neg(&self) -> Option<Self> {
        <Aint<u128, WIDTH>>::checked_neg(*self)
    }
}

impl<const WIDTH: u32> num_traits::CheckedNeg for Aint<i8, WIDTH> {
    fn checked_neg(&self) -> Option<Self> {
        <Aint<i8, WIDTH>>::checked_neg(*self)
    }
}

impl<const WIDTH: u32> num_traits::CheckedNeg for Aint<i16, WIDTH> {
    fn checked_neg(&self) -> Option<Self> {
        <Aint<i16, WIDTH>>::checked_neg(*self)
    }
}

impl<const WIDTH: u32> num_traits::CheckedNeg for Aint<i32, WIDTH> {
    fn checked_neg(&self) -> Option<Self> {
        <Aint<i32, WIDTH>>::checked_neg(*self)
    }
}

impl<const WIDTH: u32> num_traits::CheckedNeg for Aint<i64, WIDTH> {
    fn checked_neg(&self) -> Option<Self> {
        <Aint<i64, WIDTH>>::checked_neg(*self)
    }
}

impl<const WIDTH: u32> num_traits::CheckedNeg for Aint<i128, WIDTH> {
    fn checked_neg(&self) -> Option<Self> {
        <Aint<i128, WIDTH>>::checked_neg(*self)
    }
}

impl<const WIDTH: u32> num_traits::CheckedAdd for Aint<u8, WIDTH> {
    fn checked_add(&self, v: &Self) -> Option<Self> {
        <Aint<u8, WIDTH>>::checked_add(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::CheckedAdd for Aint<u16, WIDTH> {
    fn checked_add(&self, v: &Self) -> Option<Self> {
        <Aint<u16, WIDTH>>::checked_add(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::CheckedAdd for Aint<u32, WIDTH> {
    fn checked_add(&self, v: &Self) -> Option<Self> {
        <Aint<u32, WIDTH>>::checked_add(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::CheckedAdd for Aint<u64, WIDTH> {
    fn checked_add(&self, v: &Self) -> Option<Self> {
        <Aint<u64, WIDTH>>::checked_add(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::CheckedAdd for Aint<u128, WIDTH> {
    fn checked_add(&self, v: &Self) -> Option<Self> {
        <Aint<u128, WIDTH>>::checked_add(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::CheckedAdd for Aint<i8, WIDTH> {
    fn checked_add(&self, v: &Self) -> Option<Self> {
        <Aint<i8, WIDTH>>::checked_add(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::CheckedAdd for Aint<i16, WIDTH> {
    fn checked_add(&self, v: &Self) -> Option<Self> {
        <Aint<i16, WIDTH>>::checked_add(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::CheckedAdd for Aint<i32, WIDTH> {
    fn checked_add(&self, v: &Self) -> Option<Self> {
        <Aint<i32, WIDTH>>::checked_add(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::CheckedAdd for Aint<i64, WIDTH> {
    fn checked_add(&self, v: &Self) -> Option<Self> {
        <Aint<i64, WIDTH>>::checked_add(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::CheckedAdd for Aint<i128, WIDTH> {
    fn checked_add(&self, v: &Self) -> Option<Self> {
        <Aint<i128, WIDTH>>::checked_add(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::CheckedSub for Aint<u8, WIDTH> {
    fn checked_sub(&self, v: &Self) -> Option<Self> {
        <Aint<u8, WIDTH>>::checked_sub(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::CheckedSub for Aint<u16, WIDTH> {
    fn checked_sub(&self, v: &Self) -> Option<Self> {
        <Aint<u16, WIDTH>>::checked_sub(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::CheckedSub for Aint<u32, WIDTH> {
    fn checked_sub(&self, v: &Self) -> Option<Self> {
        <Aint<u32, WIDTH>>::checked_sub(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::CheckedSub for Aint<u64, WIDTH> {
    fn checked_sub(&self, v: &Self) -> Option<Self> {
        <Aint<u64, WIDTH>>::checked_sub(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::CheckedSub for Aint<u128, WIDTH> {
    fn checked_sub(&self, v: &Self) -> Option<Self> {
        <Aint<u128, WIDTH>>::checked_sub(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::CheckedSub for Aint<i8, WIDTH> {
    fn checked_sub(&self, v: &Self) -> Option<Self> {
        <Aint<i8, WIDTH>>::checked_sub(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::CheckedSub for Aint<i16, WIDTH> {
    fn checked_sub(&self, v: &Self) -> Option<Self> {
        <Aint<i16, WIDTH>>::checked_sub(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::CheckedSub for Aint<i32, WIDTH> {
    fn checked_sub(&self, v: &Self) -> Option<Self> {
        <Aint<i32, WIDTH>>::checked_sub(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::CheckedSub for Aint<i64, WIDTH> {
    fn checked_sub(&self, v: &Self) -> Option<Self> {
        <Aint<i64, WIDTH>>::checked_sub(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::CheckedSub for Aint<i128, WIDTH> {
    fn checked_sub(&self, v: &Self) -> Option<Self> {
        <Aint<i128, WIDTH>>::checked_sub(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::CheckedMul for Aint<u8, WIDTH> {
    fn checked_mul(&self, v: &Self) -> Option<Self> {
        <Aint<u8, WIDTH>>::checked_mul(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::CheckedMul for Aint<u16, WIDTH> {
    fn checked_mul(&self, v: &Self) -> Option<Self> {
        <Aint<u16, WIDTH>>::checked_mul(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::CheckedMul for Aint<u32, WIDTH> {
    fn checked_mul(&self, v: &Self) -> Option<Self> {
        <Aint<u32, WIDTH>>::checked_mul(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::CheckedMul for Aint<u64, WIDTH> {
    fn checked_mul(&self, v: &Self) -> Option<Self> {
        <Aint<u64, WIDTH>>::checked_mul(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::CheckedMul for Aint<u128, WIDTH> {
    fn checked_mul(&self, v: &Self) -> Option<Self> {
        <Aint<u128, WIDTH>>::checked_mul(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::CheckedMul for Aint<i8, WIDTH> {
    fn checked_mul(&self, v: &Self) -> Option<Self> {
        <Aint<i8, WIDTH>>::checked_mul(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::CheckedMul for Aint<i16, WIDTH> {
    fn checked_mul(&self, v: &Self) -> Option<Self> {
        <Aint<i16, WIDTH>>::checked_mul(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::CheckedMul for Aint<i32, WIDTH> {
    fn checked_mul(&self, v: &Self) -> Option<Self> {
        <Aint<i32, WIDTH>>::checked_mul(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::CheckedMul for Aint<i64, WIDTH> {
    fn checked_mul(&self, v: &Self) -> Option<Self> {
        <Aint<i64, WIDTH>>::checked_mul(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::CheckedMul for Aint<i128, WIDTH> {
    fn checked_mul(&self, v: &Self) -> Option<Self> {
        <Aint<i128, WIDTH>>::checked_mul(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::CheckedDiv for Aint<u8, WIDTH> {
    fn checked_div(&self, v: &Self) -> Option<Self> {
        <Aint<u8, WIDTH>>::checked_div(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::CheckedDiv for Aint<u16, WIDTH> {
    fn checked_div(&self, v: &Self) -> Option<Self> {
        <Aint<u16, WIDTH>>::checked_div(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::CheckedDiv for Aint<u32, WIDTH> {
    fn checked_div(&self, v: &Self) -> Option<Self> {
        <Aint<u32, WIDTH>>::checked_div(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::CheckedDiv for Aint<u64, WIDTH> {
    fn checked_div(&self, v: &Self) -> Option<Self> {
        <Aint<u64, WIDTH>>::checked_div(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::CheckedDiv for Aint<u128, WIDTH> {
    fn checked_div(&self, v: &Self) -> Option<Self> {
        <Aint<u128, WIDTH>>::checked_div(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::CheckedDiv for Aint<i8, WIDTH> {
    fn checked_div(&self, v: &Self) -> Option<Self> {
        <Aint<i8, WIDTH>>::checked_div(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::CheckedDiv for Aint<i16, WIDTH> {
    fn checked_div(&self, v: &Self) -> Option<Self> {
        <Aint<i16, WIDTH>>::checked_div(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::CheckedDiv for Aint<i32, WIDTH> {
    fn checked_div(&self, v: &Self) -> Option<Self> {
        <Aint<i32, WIDTH>>::checked_div(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::CheckedDiv for Aint<i64, WIDTH> {
    fn checked_div(&self, v: &Self) -> Option<Self> {
        <Aint<i64, WIDTH>>::checked_div(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::CheckedDiv for Aint<i128, WIDTH> {
    fn checked_div(&self, v: &Self) -> Option<Self> {
        <Aint<i128, WIDTH>>::checked_div(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::CheckedRem for Aint<u8, WIDTH> {
    fn checked_rem(&self, v: &Self) -> Option<Self> {
        <Aint<u8, WIDTH>>::checked_rem(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::CheckedRem for Aint<u16, WIDTH> {
    fn checked_rem(&self, v: &Self) -> Option<Self> {
        <Aint<u16, WIDTH>>::checked_rem(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::CheckedRem for Aint<u32, WIDTH> {
    fn checked_rem(&self, v: &Self) -> Option<Self> {
        <Aint<u32, WIDTH>>::checked_rem(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::CheckedRem for Aint<u64, WIDTH> {
    fn checked_rem(&self, v: &Self) -> Option<Self> {
        <Aint<u64, WIDTH>>::checked_rem(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::CheckedRem for Aint<u128, WIDTH> {
    fn checked_rem(&self, v: &Self) -> Option<Self> {
        <Aint<u128, WIDTH>>::checked_rem(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::CheckedRem for Aint<i8, WIDTH> {
    fn checked_rem(&self, v: &Self) -> Option<Self> {
        <Aint<i8, WIDTH>>::checked_rem(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::CheckedRem for Aint<i16, WIDTH> {
    fn checked_rem(&self, v: &Self) -> Option<Self> {
        <Aint<i16, WIDTH>>::checked_rem(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::CheckedRem for Aint<i32, WIDTH> {
    fn checked_rem(&self, v: &Self) -> Option<Self> {
        <Aint<i32, WIDTH>>::checked_rem(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::CheckedRem for Aint<i64, WIDTH> {
    fn checked_rem(&self, v: &Self) -> Option<Self> {
        <Aint<i64, WIDTH>>::checked_rem(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::CheckedRem for Aint<i128, WIDTH> {
    fn checked_rem(&self, v: &Self) -> Option<Self> {
        <Aint<i128, WIDTH>>::checked_rem(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::CheckedShl for Aint<u8, WIDTH> {
    fn checked_shl(&self, rhs: u32) -> Option<Self> {
        <Aint<u8, WIDTH>>::checked_shl(*self, rhs)
    }
}

impl<const WIDTH: u32> num_traits::CheckedShl for Aint<u16, WIDTH> {
    fn checked_shl(&self, rhs: u32) -> Option<Self> {
        <Aint<u16, WIDTH>>::checked_shl(*self, rhs)
    }
}

impl<const WIDTH: u32> num_traits::CheckedShl for Aint<u32, WIDTH> {
    fn checked_shl(&self, rhs: u32) -> Option<Self> {
        <Aint<u32, WIDTH>>::checked_shl(*self, rhs)
    }
}

impl<const WIDTH: u32> num_traits::CheckedShl for Aint<u64, WIDTH> {
    fn checked_shl(&self, rhs: u32) -> Option<Self> {
        <Aint<u64, WIDTH>>::checked_shl(*self, rhs)
    }
}

impl<const WIDTH: u32> num_traits::CheckedShl for Aint<u128, WIDTH> {
    fn checked_shl(&self, rhs: u32) -> Option<Self> {
        <Aint<u128, WIDTH>>::checked_shl(*self, rhs)
    }
}

impl<const WIDTH: u32> num_traits::CheckedShl for Aint<i8, WIDTH> {
    fn checked_shl(&self, rhs: u32) -> Option<Self> {
        <Aint<i8, WIDTH>>::checked_shl(*self, rhs)
    }
}

impl<const WIDTH: u32> num_traits::CheckedShl for Aint<i16, WIDTH> {
    fn checked_shl(&self, rhs: u32) -> Option<Self> {
        <Aint<i16, WIDTH>>::checked_shl(*self, rhs)
    }
}

impl<const WIDTH: u32> num_traits::CheckedShl for Aint<i32, WIDTH> {
    fn checked_shl(&self, rhs: u32) -> Option<Self> {
        <Aint<i32, WIDTH>>::checked_shl(*self, rhs)
    }
}

impl<const WIDTH: u32> num_traits::CheckedShl for Aint<i64, WIDTH> {
    fn checked_shl(&self, rhs: u32) -> Option<Self> {
        <Aint<i64, WIDTH>>::checked_shl(*self, rhs)
    }
}

impl<const WIDTH: u32> num_traits::CheckedShl for Aint<i128, WIDTH> {
    fn checked_shl(&self, rhs: u32) -> Option<Self> {
        <Aint<i128, WIDTH>>::checked_shl(*self, rhs)
    }
}

impl<const WIDTH: u32> num_traits::CheckedShr for Aint<u8, WIDTH> {
    fn checked_shr(&self, rhs: u32) -> Option<Self> {
        <Aint<u8, WIDTH>>::checked_shr(*self, rhs)
    }
}

impl<const WIDTH: u32> num_traits::CheckedShr for Aint<u16, WIDTH> {
    fn checked_shr(&self, rhs: u32) -> Option<Self> {
        <Aint<u16, WIDTH>>::checked_shr(*self, rhs)
    }
}

impl<const WIDTH: u32> num_traits::CheckedShr for Aint<u32, WIDTH> {
    fn checked_shr(&self, rhs: u32) -> Option<Self> {
        <Aint<u32, WIDTH>>::checked_shr(*self, rhs)
    }
}

impl<const WIDTH: u32> num_traits::CheckedShr for Aint<u64, WIDTH> {
    fn checked_shr(&self, rhs: u32) -> Option<Self> {
        <Aint<u64, WIDTH>>::checked_shr(*self, rhs)
    }
}

impl<const WIDTH: u32> num_traits::CheckedShr for Aint<u128, WIDTH> {
    fn checked_shr(&self, rhs: u32) -> Option<Self> {
        <Aint<u128, WIDTH>>::checked_shr(*self, rhs)
    }
}

impl<const WIDTH: u32> num_traits::CheckedShr for Aint<i8, WIDTH> {
    fn checked_shr(&self, rhs: u32) -> Option<Self> {
        <Aint<i8, WIDTH>>::checked_shr(*self, rhs)
    }
}

impl<const WIDTH: u32> num_traits::CheckedShr for Aint<i16, WIDTH> {
    fn checked_shr(&self, rhs: u32) -> Option<Self> {
        <Aint<i16, WIDTH>>::checked_shr(*self, rhs)
    }
}

impl<const WIDTH: u32> num_traits::CheckedShr for Aint<i32, WIDTH> {
    fn checked_shr(&self, rhs: u32) -> Option<Self> {
        <Aint<i32, WIDTH>>::checked_shr(*self, rhs)
    }
}

impl<const WIDTH: u32> num_traits::CheckedShr for Aint<i64, WIDTH> {
    fn checked_shr(&self, rhs: u32) -> Option<Self> {
        <Aint<i64, WIDTH>>::checked_shr(*self, rhs)
    }
}

impl<const WIDTH: u32> num_traits::CheckedShr for Aint<i128, WIDTH> {
    fn checked_shr(&self, rhs: u32) -> Option<Self> {
        <Aint<i128, WIDTH>>::checked_shr(*self, rhs)
    }
}

impl<const WIDTH: u32> num_traits::Euclid for Aint<u8, WIDTH> {
    fn div_euclid(&self, v: &Self) -> Self {
        <Aint<u8, WIDTH>>::div_euclid(*self, *v)
    }

    fn rem_euclid(&self, v: &Self) -> Self {
        <Aint<u8, WIDTH>>::rem_euclid(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::Euclid for Aint<u16, WIDTH> {
    fn div_euclid(&self, v: &Self) -> Self {
        <Aint<u16, WIDTH>>::div_euclid(*self, *v)
    }

    fn rem_euclid(&self, v: &Self) -> Self {
        <Aint<u16, WIDTH>>::rem_euclid(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::Euclid for Aint<u32, WIDTH> {
    fn div_euclid(&self, v: &Self) -> Self {
        <Aint<u32, WIDTH>>::div_euclid(*self, *v)
    }

    fn rem_euclid(&self, v: &Self) -> Self {
        <Aint<u32, WIDTH>>::rem_euclid(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::Euclid for Aint<u64, WIDTH> {
    fn div_euclid(&self, v: &Self) -> Self {
        <Aint<u64, WIDTH>>::div_euclid(*self, *v)
    }

    fn rem_euclid(&self, v: &Self) -> Self {
        <Aint<u64, WIDTH>>::rem_euclid(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::Euclid for Aint<u128, WIDTH> {
    fn div_euclid(&self, v: &Self) -> Self {
        <Aint<u128, WIDTH>>::div_euclid(*self, *v)
    }

    fn rem_euclid(&self, v: &Self) -> Self {
        <Aint<u128, WIDTH>>::rem_euclid(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::Euclid for Aint<i8, WIDTH> {
    fn div_euclid(&self, v: &Self) -> Self {
        <Aint<i8, WIDTH>>::div_euclid(*self, *v)
    }

    fn rem_euclid(&self, v: &Self) -> Self {
        <Aint<i8, WIDTH>>::rem_euclid(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::Euclid for Aint<i16, WIDTH> {
    fn div_euclid(&self, v: &Self) -> Self {
        <Aint<i16, WIDTH>>::div_euclid(*self, *v)
    }

    fn rem_euclid(&self, v: &Self) -> Self {
        <Aint<i16, WIDTH>>::rem_euclid(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::Euclid for Aint<i32, WIDTH> {
    fn div_euclid(&self, v: &Self) -> Self {
        <Aint<i32, WIDTH>>::div_euclid(*self, *v)
    }

    fn rem_euclid(&self, v: &Self) -> Self {
        <Aint<i32, WIDTH>>::rem_euclid(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::Euclid for Aint<i64, WIDTH> {
    fn div_euclid(&self, v: &Self) -> Self {
        <Aint<i64, WIDTH>>::div_euclid(*self, *v)
    }

    fn rem_euclid(&self, v: &Self) -> Self {
        <Aint<i64, WIDTH>>::rem_euclid(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::Euclid for Aint<i128, WIDTH> {
    fn div_euclid(&self, v: &Self) -> Self {
        <Aint<i128, WIDTH>>::div_euclid(*self, *v)
    }

    fn rem_euclid(&self, v: &Self) -> Self {
        <Aint<i128, WIDTH>>::rem_euclid(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::CheckedEuclid for Aint<u8, WIDTH> {
    fn checked_div_euclid(&self, v: &Self) -> Option<Self> {
        <Aint<u8, WIDTH>>::checked_div_euclid(*self, *v)
    }

    fn checked_rem_euclid(&self, v: &Self) -> Option<Self> {
        <Aint<u8, WIDTH>>::checked_rem_euclid(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::CheckedEuclid for Aint<u16, WIDTH> {
    fn checked_div_euclid(&self, v: &Self) -> Option<Self> {
        <Aint<u16, WIDTH>>::checked_div_euclid(*self, *v)
    }

    fn checked_rem_euclid(&self, v: &Self) -> Option<Self> {
        <Aint<u16, WIDTH>>::checked_rem_euclid(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::CheckedEuclid for Aint<u32, WIDTH> {
    fn checked_div_euclid(&self, v: &Self) -> Option<Self> {
        <Aint<u32, WIDTH>>::checked_div_euclid(*self, *v)
    }

    fn checked_rem_euclid(&self, v: &Self) -> Option<Self> {
        <Aint<u32, WIDTH>>::checked_rem_euclid(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::CheckedEuclid for Aint<u64, WIDTH> {
    fn checked_div_euclid(&self, v: &Self) -> Option<Self> {
        <Aint<u64, WIDTH>>::checked_div_euclid(*self, *v)
    }

    fn checked_rem_euclid(&self, v: &Self) -> Option<Self> {
        <Aint<u64, WIDTH>>::checked_rem_euclid(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::CheckedEuclid for Aint<u128, WIDTH> {
    fn checked_div_euclid(&self, v: &Self) -> Option<Self> {
        <Aint<u128, WIDTH>>::checked_div_euclid(*self, *v)
    }

    fn checked_rem_euclid(&self, v: &Self) -> Option<Self> {
        <Aint<u128, WIDTH>>::checked_rem_euclid(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::CheckedEuclid for Aint<i8, WIDTH> {
    fn checked_div_euclid(&self, v: &Self) -> Option<Self> {
        <Aint<i8, WIDTH>>::checked_div_euclid(*self, *v)
    }

    fn checked_rem_euclid(&self, v: &Self) -> Option<Self> {
        <Aint<i8, WIDTH>>::checked_rem_euclid(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::CheckedEuclid for Aint<i16, WIDTH> {
    fn checked_div_euclid(&self, v: &Self) -> Option<Self> {
        <Aint<i16, WIDTH>>::checked_div_euclid(*self, *v)
    }

    fn checked_rem_euclid(&self, v: &Self) -> Option<Self> {
        <Aint<i16, WIDTH>>::checked_rem_euclid(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::CheckedEuclid for Aint<i32, WIDTH> {
    fn checked_div_euclid(&self, v: &Self) -> Option<Self> {
        <Aint<i32, WIDTH>>::checked_div_euclid(*self, *v)
    }

    fn checked_rem_euclid(&self, v: &Self) -> Option<Self> {
        <Aint<i32, WIDTH>>::checked_rem_euclid(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::CheckedEuclid for Aint<i64, WIDTH> {
    fn checked_div_euclid(&self, v: &Self) -> Option<Self> {
        <Aint<i64, WIDTH>>::checked_div_euclid(*self, *v)
    }

    fn checked_rem_euclid(&self, v: &Self) -> Option<Self> {
        <Aint<i64, WIDTH>>::checked_rem_euclid(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::CheckedEuclid for Aint<i128, WIDTH> {
    fn checked_div_euclid(&self, v: &Self) -> Option<Self> {
        <Aint<i128, WIDTH>>::checked_div_euclid(*self, *v)
    }

    fn checked_rem_euclid(&self, v: &Self) -> Option<Self> {
        <Aint<i128, WIDTH>>::checked_rem_euclid(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::MulAdd for Aint<u8, WIDTH> {
    type Output = Self;

    fn mul_add(self, a: Self, b: Self) -> Self {
        let val = num_traits::MulAdd::mul_add(self.0, a.0, b.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to compute fused multiply-add with overflow"
        );
        Self::new_wrapping(val)
    }
}

impl<const WIDTH: u32> num_traits::MulAdd for Aint<u16, WIDTH> {
    type Output = Self;

    fn mul_add(self, a: Self, b: Self) -> Self {
        let val = num_traits::MulAdd::mul_add(self.0, a.0, b.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to compute fused multiply-add with overflow"
        );
        Self::new_wrapping(val)
    }
}

impl<const WIDTH: u32> num_traits::MulAdd for Aint<u32, WIDTH> {
    type Output = Self;

    fn mul_add(self, a: Self, b: Self) -> Self {
        let val = num_traits::MulAdd::mul_add(self.0, a.0, b.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to compute fused multiply-add with overflow"
        );
        Self::new_wrapping(val)
    }
}

impl<const WIDTH: u32> num_traits::MulAdd for Aint<u64, WIDTH> {
    type Output = Self;

    fn mul_add(self, a: Self, b: Self) -> Self {
        let val = num_traits::MulAdd::mul_add(self.0, a.0, b.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to compute fused multiply-add with overflow"
        );
        Self::new_wrapping(val)
    }
}

impl<const WIDTH: u32> num_traits::MulAdd for Aint<u128, WIDTH> {
    type Output = Self;

    fn mul_add(self, a: Self, b: Self) -> Self {
        let val = num_traits::MulAdd::mul_add(self.0, a.0, b.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to compute fused multiply-add with overflow"
        );
        Self::new_wrapping(val)
    }
}

impl<const WIDTH: u32> num_traits::MulAdd for Aint<i8, WIDTH> {
    type Output = Self;

    fn mul_add(self, a: Self, b: Self) -> Self {
        let val = num_traits::MulAdd::mul_add(self.0, a.0, b.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to compute fused multiply-add with overflow"
        );
        Self::new_wrapping(val)
    }
}

impl<const WIDTH: u32> num_traits::MulAdd for Aint<i16, WIDTH> {
    type Output = Self;

    fn mul_add(self, a: Self, b: Self) -> Self {
        let val = num_traits::MulAdd::mul_add(self.0, a.0, b.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to compute fused multiply-add with overflow"
        );
        Self::new_wrapping(val)
    }
}

impl<const WIDTH: u32> num_traits::MulAdd for Aint<i32, WIDTH> {
    type Output = Self;

    fn mul_add(self, a: Self, b: Self) -> Self {
        let val = num_traits::MulAdd::mul_add(self.0, a.0, b.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to compute fused multiply-add with overflow"
        );
        Self::new_wrapping(val)
    }
}

impl<const WIDTH: u32> num_traits::MulAdd for Aint<i64, WIDTH> {
    type Output = Self;

    fn mul_add(self, a: Self, b: Self) -> Self {
        let val = num_traits::MulAdd::mul_add(self.0, a.0, b.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to compute fused multiply-add with overflow"
        );
        Self::new_wrapping(val)
    }
}

impl<const WIDTH: u32> num_traits::MulAdd for Aint<i128, WIDTH> {
    type Output = Self;

    fn mul_add(self, a: Self, b: Self) -> Self {
        let val = num_traits::MulAdd::mul_add(self.0, a.0, b.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to compute fused multiply-add with overflow"
        );
        Self::new_wrapping(val)
    }
}

impl<const WIDTH: u32> num_traits::MulAddAssign for Aint<u8, WIDTH> {
    fn mul_add_assign(&mut self, a: Self, b: Self) {
        let mut val = self.0;
        num_traits::MulAddAssign::mul_add_assign(&mut val, a.0, b.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to compute fused multiply-add with overflow"
        );
        *self = Self::new_wrapping(val);
    }
}

impl<const WIDTH: u32> num_traits::MulAddAssign for Aint<u16, WIDTH> {
    fn mul_add_assign(&mut self, a: Self, b: Self) {
        let mut val = self.0;
        num_traits::MulAddAssign::mul_add_assign(&mut val, a.0, b.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to compute fused multiply-add with overflow"
        );
        *self = Self::new_wrapping(val);
    }
}

impl<const WIDTH: u32> num_traits::MulAddAssign for Aint<u32, WIDTH> {
    fn mul_add_assign(&mut self, a: Self, b: Self) {
        let mut val = self.0;
        num_traits::MulAddAssign::mul_add_assign(&mut val, a.0, b.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to compute fused multiply-add with overflow"
        );
        *self = Self::new_wrapping(val);
    }
}

impl<const WIDTH: u32> num_traits::MulAddAssign for Aint<u64, WIDTH> {
    fn mul_add_assign(&mut self, a: Self, b: Self) {
        let mut val = self.0;
        num_traits::MulAddAssign::mul_add_assign(&mut val, a.0, b.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to compute fused multiply-add with overflow"
        );
        *self = Self::new_wrapping(val);
    }
}

impl<const WIDTH: u32> num_traits::MulAddAssign for Aint<u128, WIDTH> {
    fn mul_add_assign(&mut self, a: Self, b: Self) {
        let mut val = self.0;
        num_traits::MulAddAssign::mul_add_assign(&mut val, a.0, b.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to compute fused multiply-add with overflow"
        );
        *self = Self::new_wrapping(val);
    }
}

impl<const WIDTH: u32> num_traits::MulAddAssign for Aint<i8, WIDTH> {
    fn mul_add_assign(&mut self, a: Self, b: Self) {
        let mut val = self.0;
        num_traits::MulAddAssign::mul_add_assign(&mut val, a.0, b.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to compute fused multiply-add with overflow"
        );
        *self = Self::new_wrapping(val);
    }
}

impl<const WIDTH: u32> num_traits::MulAddAssign for Aint<i16, WIDTH> {
    fn mul_add_assign(&mut self, a: Self, b: Self) {
        let mut val = self.0;
        num_traits::MulAddAssign::mul_add_assign(&mut val, a.0, b.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to compute fused multiply-add with overflow"
        );
        *self = Self::new_wrapping(val);
    }
}

impl<const WIDTH: u32> num_traits::MulAddAssign for Aint<i32, WIDTH> {
    fn mul_add_assign(&mut self, a: Self, b: Self) {
        let mut val = self.0;
        num_traits::MulAddAssign::mul_add_assign(&mut val, a.0, b.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to compute fused multiply-add with overflow"
        );
        *self = Self::new_wrapping(val);
    }
}

impl<const WIDTH: u32> num_traits::MulAddAssign for Aint<i64, WIDTH> {
    fn mul_add_assign(&mut self, a: Self, b: Self) {
        let mut val = self.0;
        num_traits::MulAddAssign::mul_add_assign(&mut val, a.0, b.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to compute fused multiply-add with overflow"
        );
        *self = Self::new_wrapping(val);
    }
}

impl<const WIDTH: u32> num_traits::MulAddAssign for Aint<i128, WIDTH> {
    fn mul_add_assign(&mut self, a: Self, b: Self) {
        let mut val = self.0;
        num_traits::MulAddAssign::mul_add_assign(&mut val, a.0, b.0);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to compute fused multiply-add with overflow"
        );
        *self = Self::new_wrapping(val);
    }
}

impl<const WIDTH: u32> num_traits::Saturating for Aint<u8, WIDTH> {
    fn saturating_add(self, v: Self) -> Self {
        <Aint<u8, WIDTH>>::saturating_add(self, v)
    }

    fn saturating_sub(self, v: Self) -> Self {
        <Aint<u8, WIDTH>>::saturating_sub(self, v)
    }
}

impl<const WIDTH: u32> num_traits::Saturating for Aint<u16, WIDTH> {
    fn saturating_add(self, v: Self) -> Self {
        <Aint<u16, WIDTH>>::saturating_add(self, v)
    }

    fn saturating_sub(self, v: Self) -> Self {
        <Aint<u16, WIDTH>>::saturating_sub(self, v)
    }
}

impl<const WIDTH: u32> num_traits::Saturating for Aint<u32, WIDTH> {
    fn saturating_add(self, v: Self) -> Self {
        <Aint<u32, WIDTH>>::saturating_add(self, v)
    }

    fn saturating_sub(self, v: Self) -> Self {
        <Aint<u32, WIDTH>>::saturating_sub(self, v)
    }
}

impl<const WIDTH: u32> num_traits::Saturating for Aint<u64, WIDTH> {
    fn saturating_add(self, v: Self) -> Self {
        <Aint<u64, WIDTH>>::saturating_add(self, v)
    }

    fn saturating_sub(self, v: Self) -> Self {
        <Aint<u64, WIDTH>>::saturating_sub(self, v)
    }
}

impl<const WIDTH: u32> num_traits::Saturating for Aint<u128, WIDTH> {
    fn saturating_add(self, v: Self) -> Self {
        <Aint<u128, WIDTH>>::saturating_add(self, v)
    }

    fn saturating_sub(self, v: Self) -> Self {
        <Aint<u128, WIDTH>>::saturating_sub(self, v)
    }
}

impl<const WIDTH: u32> num_traits::Saturating for Aint<i8, WIDTH> {
    fn saturating_add(self, v: Self) -> Self {
        <Aint<i8, WIDTH>>::saturating_add(self, v)
    }

    fn saturating_sub(self, v: Self) -> Self {
        <Aint<i8, WIDTH>>::saturating_sub(self, v)
    }
}

impl<const WIDTH: u32> num_traits::Saturating for Aint<i16, WIDTH> {
    fn saturating_add(self, v: Self) -> Self {
        <Aint<i16, WIDTH>>::saturating_add(self, v)
    }

    fn saturating_sub(self, v: Self) -> Self {
        <Aint<i16, WIDTH>>::saturating_sub(self, v)
    }
}

impl<const WIDTH: u32> num_traits::Saturating for Aint<i32, WIDTH> {
    fn saturating_add(self, v: Self) -> Self {
        <Aint<i32, WIDTH>>::saturating_add(self, v)
    }

    fn saturating_sub(self, v: Self) -> Self {
        <Aint<i32, WIDTH>>::saturating_sub(self, v)
    }
}

impl<const WIDTH: u32> num_traits::Saturating for Aint<i64, WIDTH> {
    fn saturating_add(self, v: Self) -> Self {
        <Aint<i64, WIDTH>>::saturating_add(self, v)
    }

    fn saturating_sub(self, v: Self) -> Self {
        <Aint<i64, WIDTH>>::saturating_sub(self, v)
    }
}

impl<const WIDTH: u32> num_traits::Saturating for Aint<i128, WIDTH> {
    fn saturating_add(self, v: Self) -> Self {
        <Aint<i128, WIDTH>>::saturating_add(self, v)
    }

    fn saturating_sub(self, v: Self) -> Self {
        <Aint<i128, WIDTH>>::saturating_sub(self, v)
    }
}

impl<const WIDTH: u32> num_traits::SaturatingAdd for Aint<u8, WIDTH> {
    fn saturating_add(&self, v: &Self) -> Self {
        <Aint<u8, WIDTH>>::saturating_add(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::SaturatingAdd for Aint<u16, WIDTH> {
    fn saturating_add(&self, v: &Self) -> Self {
        <Aint<u16, WIDTH>>::saturating_add(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::SaturatingAdd for Aint<u32, WIDTH> {
    fn saturating_add(&self, v: &Self) -> Self {
        <Aint<u32, WIDTH>>::saturating_add(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::SaturatingAdd for Aint<u64, WIDTH> {
    fn saturating_add(&self, v: &Self) -> Self {
        <Aint<u64, WIDTH>>::saturating_add(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::SaturatingAdd for Aint<u128, WIDTH> {
    fn saturating_add(&self, v: &Self) -> Self {
        <Aint<u128, WIDTH>>::saturating_add(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::SaturatingAdd for Aint<i8, WIDTH> {
    fn saturating_add(&self, v: &Self) -> Self {
        <Aint<i8, WIDTH>>::saturating_add(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::SaturatingAdd for Aint<i16, WIDTH> {
    fn saturating_add(&self, v: &Self) -> Self {
        <Aint<i16, WIDTH>>::saturating_add(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::SaturatingAdd for Aint<i32, WIDTH> {
    fn saturating_add(&self, v: &Self) -> Self {
        <Aint<i32, WIDTH>>::saturating_add(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::SaturatingAdd for Aint<i64, WIDTH> {
    fn saturating_add(&self, v: &Self) -> Self {
        <Aint<i64, WIDTH>>::saturating_add(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::SaturatingAdd for Aint<i128, WIDTH> {
    fn saturating_add(&self, v: &Self) -> Self {
        <Aint<i128, WIDTH>>::saturating_add(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::SaturatingSub for Aint<u8, WIDTH> {
    fn saturating_sub(&self, v: &Self) -> Self {
        <Aint<u8, WIDTH>>::saturating_sub(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::SaturatingSub for Aint<u16, WIDTH> {
    fn saturating_sub(&self, v: &Self) -> Self {
        <Aint<u16, WIDTH>>::saturating_sub(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::SaturatingSub for Aint<u32, WIDTH> {
    fn saturating_sub(&self, v: &Self) -> Self {
        <Aint<u32, WIDTH>>::saturating_sub(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::SaturatingSub for Aint<u64, WIDTH> {
    fn saturating_sub(&self, v: &Self) -> Self {
        <Aint<u64, WIDTH>>::saturating_sub(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::SaturatingSub for Aint<u128, WIDTH> {
    fn saturating_sub(&self, v: &Self) -> Self {
        <Aint<u128, WIDTH>>::saturating_sub(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::SaturatingSub for Aint<i8, WIDTH> {
    fn saturating_sub(&self, v: &Self) -> Self {
        <Aint<i8, WIDTH>>::saturating_sub(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::SaturatingSub for Aint<i16, WIDTH> {
    fn saturating_sub(&self, v: &Self) -> Self {
        <Aint<i16, WIDTH>>::saturating_sub(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::SaturatingSub for Aint<i32, WIDTH> {
    fn saturating_sub(&self, v: &Self) -> Self {
        <Aint<i32, WIDTH>>::saturating_sub(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::SaturatingSub for Aint<i64, WIDTH> {
    fn saturating_sub(&self, v: &Self) -> Self {
        <Aint<i64, WIDTH>>::saturating_sub(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::SaturatingSub for Aint<i128, WIDTH> {
    fn saturating_sub(&self, v: &Self) -> Self {
        <Aint<i128, WIDTH>>::saturating_sub(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::SaturatingMul for Aint<u8, WIDTH> {
    fn saturating_mul(&self, v: &Self) -> Self {
        <Aint<u8, WIDTH>>::saturating_mul(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::SaturatingMul for Aint<u16, WIDTH> {
    fn saturating_mul(&self, v: &Self) -> Self {
        <Aint<u16, WIDTH>>::saturating_mul(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::SaturatingMul for Aint<u32, WIDTH> {
    fn saturating_mul(&self, v: &Self) -> Self {
        <Aint<u32, WIDTH>>::saturating_mul(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::SaturatingMul for Aint<u64, WIDTH> {
    fn saturating_mul(&self, v: &Self) -> Self {
        <Aint<u64, WIDTH>>::saturating_mul(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::SaturatingMul for Aint<u128, WIDTH> {
    fn saturating_mul(&self, v: &Self) -> Self {
        <Aint<u128, WIDTH>>::saturating_mul(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::SaturatingMul for Aint<i8, WIDTH> {
    fn saturating_mul(&self, v: &Self) -> Self {
        <Aint<i8, WIDTH>>::saturating_mul(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::SaturatingMul for Aint<i16, WIDTH> {
    fn saturating_mul(&self, v: &Self) -> Self {
        <Aint<i16, WIDTH>>::saturating_mul(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::SaturatingMul for Aint<i32, WIDTH> {
    fn saturating_mul(&self, v: &Self) -> Self {
        <Aint<i32, WIDTH>>::saturating_mul(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::SaturatingMul for Aint<i64, WIDTH> {
    fn saturating_mul(&self, v: &Self) -> Self {
        <Aint<i64, WIDTH>>::saturating_mul(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::SaturatingMul for Aint<i128, WIDTH> {
    fn saturating_mul(&self, v: &Self) -> Self {
        <Aint<i128, WIDTH>>::saturating_mul(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::WrappingNeg for Aint<u8, WIDTH> {
    fn wrapping_neg(&self) -> Self {
        <Aint<u8, WIDTH>>::wrapping_neg(*self)
    }
}

impl<const WIDTH: u32> num_traits::WrappingNeg for Aint<u16, WIDTH> {
    fn wrapping_neg(&self) -> Self {
        <Aint<u16, WIDTH>>::wrapping_neg(*self)
    }
}

impl<const WIDTH: u32> num_traits::WrappingNeg for Aint<u32, WIDTH> {
    fn wrapping_neg(&self) -> Self {
        <Aint<u32, WIDTH>>::wrapping_neg(*self)
    }
}

impl<const WIDTH: u32> num_traits::WrappingNeg for Aint<u64, WIDTH> {
    fn wrapping_neg(&self) -> Self {
        <Aint<u64, WIDTH>>::wrapping_neg(*self)
    }
}

impl<const WIDTH: u32> num_traits::WrappingNeg for Aint<u128, WIDTH> {
    fn wrapping_neg(&self) -> Self {
        <Aint<u128, WIDTH>>::wrapping_neg(*self)
    }
}

impl<const WIDTH: u32> num_traits::WrappingNeg for Aint<i8, WIDTH> {
    fn wrapping_neg(&self) -> Self {
        <Aint<i8, WIDTH>>::wrapping_neg(*self)
    }
}

impl<const WIDTH: u32> num_traits::WrappingNeg for Aint<i16, WIDTH> {
    fn wrapping_neg(&self) -> Self {
        <Aint<i16, WIDTH>>::wrapping_neg(*self)
    }
}

impl<const WIDTH: u32> num_traits::WrappingNeg for Aint<i32, WIDTH> {
    fn wrapping_neg(&self) -> Self {
        <Aint<i32, WIDTH>>::wrapping_neg(*self)
    }
}

impl<const WIDTH: u32> num_traits::WrappingNeg for Aint<i64, WIDTH> {
    fn wrapping_neg(&self) -> Self {
        <Aint<i64, WIDTH>>::wrapping_neg(*self)
    }
}

impl<const WIDTH: u32> num_traits::WrappingNeg for Aint<i128, WIDTH> {
    fn wrapping_neg(&self) -> Self {
        <Aint<i128, WIDTH>>::wrapping_neg(*self)
    }
}

impl<const WIDTH: u32> num_traits::WrappingAdd for Aint<u8, WIDTH> {
    fn wrapping_add(&self, v: &Self) -> Self {
        <Aint<u8, WIDTH>>::wrapping_add(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::WrappingAdd for Aint<u16, WIDTH> {
    fn wrapping_add(&self, v: &Self) -> Self {
        <Aint<u16, WIDTH>>::wrapping_add(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::WrappingAdd for Aint<u32, WIDTH> {
    fn wrapping_add(&self, v: &Self) -> Self {
        <Aint<u32, WIDTH>>::wrapping_add(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::WrappingAdd for Aint<u64, WIDTH> {
    fn wrapping_add(&self, v: &Self) -> Self {
        <Aint<u64, WIDTH>>::wrapping_add(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::WrappingAdd for Aint<u128, WIDTH> {
    fn wrapping_add(&self, v: &Self) -> Self {
        <Aint<u128, WIDTH>>::wrapping_add(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::WrappingAdd for Aint<i8, WIDTH> {
    fn wrapping_add(&self, v: &Self) -> Self {
        <Aint<i8, WIDTH>>::wrapping_add(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::WrappingAdd for Aint<i16, WIDTH> {
    fn wrapping_add(&self, v: &Self) -> Self {
        <Aint<i16, WIDTH>>::wrapping_add(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::WrappingAdd for Aint<i32, WIDTH> {
    fn wrapping_add(&self, v: &Self) -> Self {
        <Aint<i32, WIDTH>>::wrapping_add(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::WrappingAdd for Aint<i64, WIDTH> {
    fn wrapping_add(&self, v: &Self) -> Self {
        <Aint<i64, WIDTH>>::wrapping_add(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::WrappingAdd for Aint<i128, WIDTH> {
    fn wrapping_add(&self, v: &Self) -> Self {
        <Aint<i128, WIDTH>>::wrapping_add(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::WrappingSub for Aint<u8, WIDTH> {
    fn wrapping_sub(&self, v: &Self) -> Self {
        <Aint<u8, WIDTH>>::wrapping_sub(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::WrappingSub for Aint<u16, WIDTH> {
    fn wrapping_sub(&self, v: &Self) -> Self {
        <Aint<u16, WIDTH>>::wrapping_sub(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::WrappingSub for Aint<u32, WIDTH> {
    fn wrapping_sub(&self, v: &Self) -> Self {
        <Aint<u32, WIDTH>>::wrapping_sub(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::WrappingSub for Aint<u64, WIDTH> {
    fn wrapping_sub(&self, v: &Self) -> Self {
        <Aint<u64, WIDTH>>::wrapping_sub(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::WrappingSub for Aint<u128, WIDTH> {
    fn wrapping_sub(&self, v: &Self) -> Self {
        <Aint<u128, WIDTH>>::wrapping_sub(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::WrappingSub for Aint<i8, WIDTH> {
    fn wrapping_sub(&self, v: &Self) -> Self {
        <Aint<i8, WIDTH>>::wrapping_sub(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::WrappingSub for Aint<i16, WIDTH> {
    fn wrapping_sub(&self, v: &Self) -> Self {
        <Aint<i16, WIDTH>>::wrapping_sub(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::WrappingSub for Aint<i32, WIDTH> {
    fn wrapping_sub(&self, v: &Self) -> Self {
        <Aint<i32, WIDTH>>::wrapping_sub(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::WrappingSub for Aint<i64, WIDTH> {
    fn wrapping_sub(&self, v: &Self) -> Self {
        <Aint<i64, WIDTH>>::wrapping_sub(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::WrappingSub for Aint<i128, WIDTH> {
    fn wrapping_sub(&self, v: &Self) -> Self {
        <Aint<i128, WIDTH>>::wrapping_sub(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::WrappingMul for Aint<u8, WIDTH> {
    fn wrapping_mul(&self, v: &Self) -> Self {
        <Aint<u8, WIDTH>>::wrapping_mul(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::WrappingMul for Aint<u16, WIDTH> {
    fn wrapping_mul(&self, v: &Self) -> Self {
        <Aint<u16, WIDTH>>::wrapping_mul(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::WrappingMul for Aint<u32, WIDTH> {
    fn wrapping_mul(&self, v: &Self) -> Self {
        <Aint<u32, WIDTH>>::wrapping_mul(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::WrappingMul for Aint<u64, WIDTH> {
    fn wrapping_mul(&self, v: &Self) -> Self {
        <Aint<u64, WIDTH>>::wrapping_mul(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::WrappingMul for Aint<u128, WIDTH> {
    fn wrapping_mul(&self, v: &Self) -> Self {
        <Aint<u128, WIDTH>>::wrapping_mul(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::WrappingMul for Aint<i8, WIDTH> {
    fn wrapping_mul(&self, v: &Self) -> Self {
        <Aint<i8, WIDTH>>::wrapping_mul(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::WrappingMul for Aint<i16, WIDTH> {
    fn wrapping_mul(&self, v: &Self) -> Self {
        <Aint<i16, WIDTH>>::wrapping_mul(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::WrappingMul for Aint<i32, WIDTH> {
    fn wrapping_mul(&self, v: &Self) -> Self {
        <Aint<i32, WIDTH>>::wrapping_mul(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::WrappingMul for Aint<i64, WIDTH> {
    fn wrapping_mul(&self, v: &Self) -> Self {
        <Aint<i64, WIDTH>>::wrapping_mul(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::WrappingMul for Aint<i128, WIDTH> {
    fn wrapping_mul(&self, v: &Self) -> Self {
        <Aint<i128, WIDTH>>::wrapping_mul(*self, *v)
    }
}

impl<const WIDTH: u32> num_traits::WrappingShl for Aint<u8, WIDTH> {
    fn wrapping_shl(&self, rhs: u32) -> Self {
        <Aint<u8, WIDTH>>::wrapping_shl(*self, rhs)
    }
}

impl<const WIDTH: u32> num_traits::WrappingShl for Aint<u16, WIDTH> {
    fn wrapping_shl(&self, rhs: u32) -> Self {
        <Aint<u16, WIDTH>>::wrapping_shl(*self, rhs)
    }
}

impl<const WIDTH: u32> num_traits::WrappingShl for Aint<u32, WIDTH> {
    fn wrapping_shl(&self, rhs: u32) -> Self {
        <Aint<u32, WIDTH>>::wrapping_shl(*self, rhs)
    }
}

impl<const WIDTH: u32> num_traits::WrappingShl for Aint<u64, WIDTH> {
    fn wrapping_shl(&self, rhs: u32) -> Self {
        <Aint<u64, WIDTH>>::wrapping_shl(*self, rhs)
    }
}

impl<const WIDTH: u32> num_traits::WrappingShl for Aint<u128, WIDTH> {
    fn wrapping_shl(&self, rhs: u32) -> Self {
        <Aint<u128, WIDTH>>::wrapping_shl(*self, rhs)
    }
}

impl<const WIDTH: u32> num_traits::WrappingShl for Aint<i8, WIDTH> {
    fn wrapping_shl(&self, rhs: u32) -> Self {
        <Aint<i8, WIDTH>>::wrapping_shl(*self, rhs)
    }
}

impl<const WIDTH: u32> num_traits::WrappingShl for Aint<i16, WIDTH> {
    fn wrapping_shl(&self, rhs: u32) -> Self {
        <Aint<i16, WIDTH>>::wrapping_shl(*self, rhs)
    }
}

impl<const WIDTH: u32> num_traits::WrappingShl for Aint<i32, WIDTH> {
    fn wrapping_shl(&self, rhs: u32) -> Self {
        <Aint<i32, WIDTH>>::wrapping_shl(*self, rhs)
    }
}

impl<const WIDTH: u32> num_traits::WrappingShl for Aint<i64, WIDTH> {
    fn wrapping_shl(&self, rhs: u32) -> Self {
        <Aint<i64, WIDTH>>::wrapping_shl(*self, rhs)
    }
}

impl<const WIDTH: u32> num_traits::WrappingShl for Aint<i128, WIDTH> {
    fn wrapping_shl(&self, rhs: u32) -> Self {
        <Aint<i128, WIDTH>>::wrapping_shl(*self, rhs)
    }
}

impl<const WIDTH: u32> num_traits::WrappingShr for Aint<u8, WIDTH> {
    fn wrapping_shr(&self, rhs: u32) -> Self {
        <Aint<u8, WIDTH>>::wrapping_shr(*self, rhs)
    }
}

impl<const WIDTH: u32> num_traits::WrappingShr for Aint<u16, WIDTH> {
    fn wrapping_shr(&self, rhs: u32) -> Self {
        <Aint<u16, WIDTH>>::wrapping_shr(*self, rhs)
    }
}

impl<const WIDTH: u32> num_traits::WrappingShr for Aint<u32, WIDTH> {
    fn wrapping_shr(&self, rhs: u32) -> Self {
        <Aint<u32, WIDTH>>::wrapping_shr(*self, rhs)
    }
}

impl<const WIDTH: u32> num_traits::WrappingShr for Aint<u64, WIDTH> {
    fn wrapping_shr(&self, rhs: u32) -> Self {
        <Aint<u64, WIDTH>>::wrapping_shr(*self, rhs)
    }
}

impl<const WIDTH: u32> num_traits::WrappingShr for Aint<u128, WIDTH> {
    fn wrapping_shr(&self, rhs: u32) -> Self {
        <Aint<u128, WIDTH>>::wrapping_shr(*self, rhs)
    }
}

impl<const WIDTH: u32> num_traits::WrappingShr for Aint<i8, WIDTH> {
    fn wrapping_shr(&self, rhs: u32) -> Self {
        <Aint<i8, WIDTH>>::wrapping_shr(*self, rhs)
    }
}

impl<const WIDTH: u32> num_traits::WrappingShr for Aint<i16, WIDTH> {
    fn wrapping_shr(&self, rhs: u32) -> Self {
        <Aint<i16, WIDTH>>::wrapping_shr(*self, rhs)
    }
}

impl<const WIDTH: u32> num_traits::WrappingShr for Aint<i32, WIDTH> {
    fn wrapping_shr(&self, rhs: u32) -> Self {
        <Aint<i32, WIDTH>>::wrapping_shr(*self, rhs)
    }
}

impl<const WIDTH: u32> num_traits::WrappingShr for Aint<i64, WIDTH> {
    fn wrapping_shr(&self, rhs: u32) -> Self {
        <Aint<i64, WIDTH>>::wrapping_shr(*self, rhs)
    }
}

impl<const WIDTH: u32> num_traits::WrappingShr for Aint<i128, WIDTH> {
    fn wrapping_shr(&self, rhs: u32) -> Self {
        <Aint<i128, WIDTH>>::wrapping_shr(*self, rhs)
    }
}

impl<T: core::convert::TryInto<u32> + crate::WrappingInto<u32> + Copy, const WIDTH: u32>
    num_traits::Pow<T> for Aint<u8, WIDTH>
{
    type Output = Self;

    fn pow(self, rhs: T) -> Self {
        let Ok(exp) = rhs.try_into() else {
            debug_assert!(false, "attempt to exponentiate with overflow");
            return Self::wrapping_pow(self, rhs.wrapping_into());
        };
        <Aint<u8, WIDTH>>::pow(self, exp)
    }
}

impl<T: core::convert::TryInto<u32> + crate::WrappingInto<u32> + Copy, const WIDTH: u32>
    num_traits::Pow<T> for Aint<u16, WIDTH>
{
    type Output = Self;

    fn pow(self, rhs: T) -> Self {
        let Ok(exp) = rhs.try_into() else {
            debug_assert!(false, "attempt to exponentiate with overflow");
            return Self::wrapping_pow(self, rhs.wrapping_into());
        };
        <Aint<u16, WIDTH>>::pow(self, exp)
    }
}

impl<T: core::convert::TryInto<u32> + crate::WrappingInto<u32> + Copy, const WIDTH: u32>
    num_traits::Pow<T> for Aint<u32, WIDTH>
{
    type Output = Self;

    fn pow(self, rhs: T) -> Self {
        let Ok(exp) = rhs.try_into() else {
            debug_assert!(false, "attempt to exponentiate with overflow");
            return Self::wrapping_pow(self, rhs.wrapping_into());
        };
        <Aint<u32, WIDTH>>::pow(self, exp)
    }
}

impl<T: core::convert::TryInto<u32> + crate::WrappingInto<u32> + Copy, const WIDTH: u32>
    num_traits::Pow<T> for Aint<u64, WIDTH>
{
    type Output = Self;

    fn pow(self, rhs: T) -> Self {
        let Ok(exp) = rhs.try_into() else {
            debug_assert!(false, "attempt to exponentiate with overflow");
            return Self::wrapping_pow(self, rhs.wrapping_into());
        };
        <Aint<u64, WIDTH>>::pow(self, exp)
    }
}

impl<T: core::convert::TryInto<u32> + crate::WrappingInto<u32> + Copy, const WIDTH: u32>
    num_traits::Pow<T> for Aint<u128, WIDTH>
{
    type Output = Self;

    fn pow(self, rhs: T) -> Self {
        let Ok(exp) = rhs.try_into() else {
            debug_assert!(false, "attempt to exponentiate with overflow");
            return Self::wrapping_pow(self, rhs.wrapping_into());
        };
        <Aint<u128, WIDTH>>::pow(self, exp)
    }
}

impl<T: core::convert::TryInto<u32> + crate::WrappingInto<u32> + Copy, const WIDTH: u32>
    num_traits::Pow<T> for Aint<i8, WIDTH>
{
    type Output = Self;

    fn pow(self, rhs: T) -> Self {
        let Ok(exp) = rhs.try_into() else {
            debug_assert!(false, "attempt to exponentiate with overflow");
            return Self::wrapping_pow(self, rhs.wrapping_into());
        };
        <Aint<i8, WIDTH>>::pow(self, exp)
    }
}

impl<T: core::convert::TryInto<u32> + crate::WrappingInto<u32> + Copy, const WIDTH: u32>
    num_traits::Pow<T> for Aint<i16, WIDTH>
{
    type Output = Self;

    fn pow(self, rhs: T) -> Self {
        let Ok(exp) = rhs.try_into() else {
            debug_assert!(false, "attempt to exponentiate with overflow");
            return Self::wrapping_pow(self, rhs.wrapping_into());
        };
        <Aint<i16, WIDTH>>::pow(self, exp)
    }
}

impl<T: core::convert::TryInto<u32> + crate::WrappingInto<u32> + Copy, const WIDTH: u32>
    num_traits::Pow<T> for Aint<i32, WIDTH>
{
    type Output = Self;

    fn pow(self, rhs: T) -> Self {
        let Ok(exp) = rhs.try_into() else {
            debug_assert!(false, "attempt to exponentiate with overflow");
            return Self::wrapping_pow(self, rhs.wrapping_into());
        };
        <Aint<i32, WIDTH>>::pow(self, exp)
    }
}

impl<T: core::convert::TryInto<u32> + crate::WrappingInto<u32> + Copy, const WIDTH: u32>
    num_traits::Pow<T> for Aint<i64, WIDTH>
{
    type Output = Self;

    fn pow(self, rhs: T) -> Self {
        let Ok(exp) = rhs.try_into() else {
            debug_assert!(false, "attempt to exponentiate with overflow");
            return Self::wrapping_pow(self, rhs.wrapping_into());
        };
        <Aint<i64, WIDTH>>::pow(self, exp)
    }
}

impl<T: core::convert::TryInto<u32> + crate::WrappingInto<u32> + Copy, const WIDTH: u32>
    num_traits::Pow<T> for Aint<i128, WIDTH>
{
    type Output = Self;

    fn pow(self, rhs: T) -> Self {
        let Ok(exp) = rhs.try_into() else {
            debug_assert!(false, "attempt to exponentiate with overflow");
            return Self::wrapping_pow(self, rhs.wrapping_into());
        };
        <Aint<i128, WIDTH>>::pow(self, exp)
    }
}

impl<T, R: Sealed, const WIDTH: u32> num_traits::Pow<T> for &Aint<R, WIDTH>
where
    Aint<R, WIDTH>: num_traits::Pow<T>,
{
    type Output = <Aint<R, WIDTH> as num_traits::Pow<T>>::Output;

    fn pow(self, rhs: T) -> Self::Output {
        <Aint<R, WIDTH> as num_traits::Pow<T>>::pow(*self, rhs)
    }
}

impl<const WIDTH: u32> num_traits::Unsigned for Aint<u8, WIDTH> {}

impl<const WIDTH: u32> num_traits::Unsigned for Aint<u16, WIDTH> {}

impl<const WIDTH: u32> num_traits::Unsigned for Aint<u32, WIDTH> {}

impl<const WIDTH: u32> num_traits::Unsigned for Aint<u64, WIDTH> {}

impl<const WIDTH: u32> num_traits::Unsigned for Aint<u128, WIDTH> {}

impl<const WIDTH: u32> num_traits::Signed for Aint<i8, WIDTH> {
    fn abs(&self) -> Self {
        <Aint<i8, WIDTH>>::abs(*self)
    }

    fn abs_sub(&self, other: &Self) -> Self {
        Self(<Aint<i8, WIDTH>>::abs_diff(*self, *other).0 as i8)
    }

    fn signum(&self) -> Self {
        <Aint<i8, WIDTH>>::signum(*self)
    }

    fn is_positive(&self) -> bool {
        <Aint<i8, WIDTH>>::is_positive(*self)
    }

    fn is_negative(&self) -> bool {
        <Aint<i8, WIDTH>>::is_negative(*self)
    }
}

impl<const WIDTH: u32> num_traits::Signed for Aint<i16, WIDTH> {
    fn abs(&self) -> Self {
        <Aint<i16, WIDTH>>::abs(*self)
    }

    fn abs_sub(&self, other: &Self) -> Self {
        Self(<Aint<i16, WIDTH>>::abs_diff(*self, *other).0 as i16)
    }

    fn signum(&self) -> Self {
        <Aint<i16, WIDTH>>::signum(*self)
    }

    fn is_positive(&self) -> bool {
        <Aint<i16, WIDTH>>::is_positive(*self)
    }

    fn is_negative(&self) -> bool {
        <Aint<i16, WIDTH>>::is_negative(*self)
    }
}

impl<const WIDTH: u32> num_traits::Signed for Aint<i32, WIDTH> {
    fn abs(&self) -> Self {
        <Aint<i32, WIDTH>>::abs(*self)
    }

    fn abs_sub(&self, other: &Self) -> Self {
        Self(<Aint<i32, WIDTH>>::abs_diff(*self, *other).0 as i32)
    }

    fn signum(&self) -> Self {
        <Aint<i32, WIDTH>>::signum(*self)
    }

    fn is_positive(&self) -> bool {
        <Aint<i32, WIDTH>>::is_positive(*self)
    }

    fn is_negative(&self) -> bool {
        <Aint<i32, WIDTH>>::is_negative(*self)
    }
}

impl<const WIDTH: u32> num_traits::Signed for Aint<i64, WIDTH> {
    fn abs(&self) -> Self {
        <Aint<i64, WIDTH>>::abs(*self)
    }

    fn abs_sub(&self, other: &Self) -> Self {
        Self(<Aint<i64, WIDTH>>::abs_diff(*self, *other).0 as i64)
    }

    fn signum(&self) -> Self {
        <Aint<i64, WIDTH>>::signum(*self)
    }

    fn is_positive(&self) -> bool {
        <Aint<i64, WIDTH>>::is_positive(*self)
    }

    fn is_negative(&self) -> bool {
        <Aint<i64, WIDTH>>::is_negative(*self)
    }
}

impl<const WIDTH: u32> num_traits::Signed for Aint<i128, WIDTH> {
    fn abs(&self) -> Self {
        <Aint<i128, WIDTH>>::abs(*self)
    }

    fn abs_sub(&self, other: &Self) -> Self {
        Self(<Aint<i128, WIDTH>>::abs_diff(*self, *other).0 as i128)
    }

    fn signum(&self) -> Self {
        <Aint<i128, WIDTH>>::signum(*self)
    }

    fn is_positive(&self) -> bool {
        <Aint<i128, WIDTH>>::is_positive(*self)
    }

    fn is_negative(&self) -> bool {
        <Aint<i128, WIDTH>>::is_negative(*self)
    }
}

impl<R1: Sealed, R2: Sealed, const WIDTH1: u32, const WIDTH2: u32>
    num_traits::AsPrimitive<Aint<R2, WIDTH2>> for Aint<R1, WIDTH1>
where
    R1: num_traits::AsPrimitive<R2>,
{
    fn as_(self) -> Aint<R2, WIDTH2> {
        <Aint<R2, WIDTH2>>::_new_wrapping(self.0.as_())
    }
}

impl<R: Sealed, const WIDTH: u32> num_traits::AsPrimitive<u8> for Aint<R, WIDTH>
where
    R: num_traits::AsPrimitive<u8>,
{
    fn as_(self) -> u8 {
        self.0.as_()
    }
}

impl<R: Sealed, const WIDTH: u32> num_traits::AsPrimitive<u16> for Aint<R, WIDTH>
where
    R: num_traits::AsPrimitive<u16>,
{
    fn as_(self) -> u16 {
        self.0.as_()
    }
}

impl<R: Sealed, const WIDTH: u32> num_traits::AsPrimitive<u32> for Aint<R, WIDTH>
where
    R: num_traits::AsPrimitive<u32>,
{
    fn as_(self) -> u32 {
        self.0.as_()
    }
}

impl<R: Sealed, const WIDTH: u32> num_traits::AsPrimitive<u64> for Aint<R, WIDTH>
where
    R: num_traits::AsPrimitive<u64>,
{
    fn as_(self) -> u64 {
        self.0.as_()
    }
}

impl<R: Sealed, const WIDTH: u32> num_traits::AsPrimitive<u128> for Aint<R, WIDTH>
where
    R: num_traits::AsPrimitive<u128>,
{
    fn as_(self) -> u128 {
        self.0.as_()
    }
}

impl<R: Sealed, const WIDTH: u32> num_traits::AsPrimitive<usize> for Aint<R, WIDTH>
where
    R: num_traits::AsPrimitive<usize>,
{
    fn as_(self) -> usize {
        self.0.as_()
    }
}

impl<R: Sealed, const WIDTH: u32> num_traits::AsPrimitive<i8> for Aint<R, WIDTH>
where
    R: num_traits::AsPrimitive<i8>,
{
    fn as_(self) -> i8 {
        self.0.as_()
    }
}

impl<R: Sealed, const WIDTH: u32> num_traits::AsPrimitive<i16> for Aint<R, WIDTH>
where
    R: num_traits::AsPrimitive<i16>,
{
    fn as_(self) -> i16 {
        self.0.as_()
    }
}

impl<R: Sealed, const WIDTH: u32> num_traits::AsPrimitive<i32> for Aint<R, WIDTH>
where
    R: num_traits::AsPrimitive<i32>,
{
    fn as_(self) -> i32 {
        self.0.as_()
    }
}

impl<R: Sealed, const WIDTH: u32> num_traits::AsPrimitive<i64> for Aint<R, WIDTH>
where
    R: num_traits::AsPrimitive<i64>,
{
    fn as_(self) -> i64 {
        self.0.as_()
    }
}

impl<R: Sealed, const WIDTH: u32> num_traits::AsPrimitive<i128> for Aint<R, WIDTH>
where
    R: num_traits::AsPrimitive<i128>,
{
    fn as_(self) -> i128 {
        self.0.as_()
    }
}

impl<R: Sealed, const WIDTH: u32> num_traits::AsPrimitive<isize> for Aint<R, WIDTH>
where
    R: num_traits::AsPrimitive<isize>,
{
    fn as_(self) -> isize {
        self.0.as_()
    }
}

impl<R: Sealed, const WIDTH: u32> num_traits::AsPrimitive<Aint<R, WIDTH>> for u8
where
    Self: num_traits::AsPrimitive<R>,
{
    fn as_(self) -> Aint<R, WIDTH> {
        <Aint<R, WIDTH>>::_new_wrapping(self.as_())
    }
}

impl<R: Sealed, const WIDTH: u32> num_traits::AsPrimitive<Aint<R, WIDTH>> for u16
where
    Self: num_traits::AsPrimitive<R>,
{
    fn as_(self) -> Aint<R, WIDTH> {
        <Aint<R, WIDTH>>::_new_wrapping(self.as_())
    }
}

impl<R: Sealed, const WIDTH: u32> num_traits::AsPrimitive<Aint<R, WIDTH>> for u32
where
    Self: num_traits::AsPrimitive<R>,
{
    fn as_(self) -> Aint<R, WIDTH> {
        <Aint<R, WIDTH>>::_new_wrapping(self.as_())
    }
}

impl<R: Sealed, const WIDTH: u32> num_traits::AsPrimitive<Aint<R, WIDTH>> for u64
where
    Self: num_traits::AsPrimitive<R>,
{
    fn as_(self) -> Aint<R, WIDTH> {
        <Aint<R, WIDTH>>::_new_wrapping(self.as_())
    }
}

impl<R: Sealed, const WIDTH: u32> num_traits::AsPrimitive<Aint<R, WIDTH>> for u128
where
    Self: num_traits::AsPrimitive<R>,
{
    fn as_(self) -> Aint<R, WIDTH> {
        <Aint<R, WIDTH>>::_new_wrapping(self.as_())
    }
}

impl<R: Sealed, const WIDTH: u32> num_traits::AsPrimitive<Aint<R, WIDTH>> for usize
where
    Self: num_traits::AsPrimitive<R>,
{
    fn as_(self) -> Aint<R, WIDTH> {
        <Aint<R, WIDTH>>::_new_wrapping(self.as_())
    }
}

impl<R: Sealed, const WIDTH: u32> num_traits::AsPrimitive<Aint<R, WIDTH>> for i8
where
    Self: num_traits::AsPrimitive<R>,
{
    fn as_(self) -> Aint<R, WIDTH> {
        <Aint<R, WIDTH>>::_new_wrapping(self.as_())
    }
}

impl<R: Sealed, const WIDTH: u32> num_traits::AsPrimitive<Aint<R, WIDTH>> for i16
where
    Self: num_traits::AsPrimitive<R>,
{
    fn as_(self) -> Aint<R, WIDTH> {
        <Aint<R, WIDTH>>::_new_wrapping(self.as_())
    }
}

impl<R: Sealed, const WIDTH: u32> num_traits::AsPrimitive<Aint<R, WIDTH>> for i32
where
    Self: num_traits::AsPrimitive<R>,
{
    fn as_(self) -> Aint<R, WIDTH> {
        <Aint<R, WIDTH>>::_new_wrapping(self.as_())
    }
}

impl<R: Sealed, const WIDTH: u32> num_traits::AsPrimitive<Aint<R, WIDTH>> for i64
where
    Self: num_traits::AsPrimitive<R>,
{
    fn as_(self) -> Aint<R, WIDTH> {
        <Aint<R, WIDTH>>::_new_wrapping(self.as_())
    }
}

impl<R: Sealed, const WIDTH: u32> num_traits::AsPrimitive<Aint<R, WIDTH>> for i128
where
    Self: num_traits::AsPrimitive<R>,
{
    fn as_(self) -> Aint<R, WIDTH> {
        <Aint<R, WIDTH>>::_new_wrapping(self.as_())
    }
}

impl<R: Sealed, const WIDTH: u32> num_traits::AsPrimitive<Aint<R, WIDTH>> for isize
where
    Self: num_traits::AsPrimitive<R>,
{
    fn as_(self) -> Aint<R, WIDTH> {
        <Aint<R, WIDTH>>::_new_wrapping(self.as_())
    }
}

impl num_traits::PrimInt for Aint<u32, 24> {
    fn count_ones(self) -> u32 {
        <Aint<u32, 24>>::count_ones(self)
    }

    fn count_zeros(self) -> u32 {
        <Aint<u32, 24>>::count_zeros(self)
    }

    fn leading_zeros(self) -> u32 {
        <Aint<u32, 24>>::leading_zeros(self)
    }

    fn trailing_zeros(self) -> u32 {
        <Aint<u32, 24>>::trailing_zeros(self)
    }

    fn leading_ones(self) -> u32 {
        <Aint<u32, 24>>::leading_ones(self)
    }

    fn trailing_ones(self) -> u32 {
        <Aint<u32, 24>>::trailing_ones(self)
    }

    fn reverse_bits(self) -> Self {
        <Aint<u32, 24>>::reverse_bits(self)
    }

    fn rotate_left(self, n: u32) -> Self {
        <Aint<u32, 24>>::rotate_left(self, n)
    }

    fn rotate_right(self, n: u32) -> Self {
        <Aint<u32, 24>>::rotate_right(self, n)
    }

    fn signed_shl(self, n: u32) -> Self {
        <Aint<u32, 24>>::shl(self, n)
    }

    fn signed_shr(self, n: u32) -> Self {
        if (self.0 & (1u32 << (Self::BITS - 1))) == 0 {
            <Aint<u32, 24>>::shr(self, n)
        } else {
            Self((self.0 >> n) | (!(Self::MAX.0 >> n) & Self::MAX.0))
        }
    }

    fn unsigned_shl(self, n: u32) -> Self {
        <Aint<u32, 24>>::shl(self, n)
    }

    fn unsigned_shr(self, n: u32) -> Self {
        <Aint<u32, 24>>::shr(self, n)
    }

    fn swap_bytes(self) -> Self {
        <Aint<u32, 24>>::swap_bytes(self)
    }

    fn from_be(x: Self) -> Self {
        <Aint<u32, 24>>::from_be(x)
    }

    fn from_le(x: Self) -> Self {
        <Aint<u32, 24>>::from_le(x)
    }

    fn to_be(self) -> Self {
        <Aint<u32, 24>>::to_be(self)
    }

    fn to_le(self) -> Self {
        <Aint<u32, 24>>::to_be(self)
    }

    fn pow(self, exp: u32) -> Self {
        <Aint<u32, 24>>::pow(self, exp)
    }
}

impl num_traits::PrimInt for Aint<u64, 40> {
    fn count_ones(self) -> u32 {
        <Aint<u64, 40>>::count_ones(self)
    }

    fn count_zeros(self) -> u32 {
        <Aint<u64, 40>>::count_zeros(self)
    }

    fn leading_zeros(self) -> u32 {
        <Aint<u64, 40>>::leading_zeros(self)
    }

    fn trailing_zeros(self) -> u32 {
        <Aint<u64, 40>>::trailing_zeros(self)
    }

    fn leading_ones(self) -> u32 {
        <Aint<u64, 40>>::leading_ones(self)
    }

    fn trailing_ones(self) -> u32 {
        <Aint<u64, 40>>::trailing_ones(self)
    }

    fn reverse_bits(self) -> Self {
        <Aint<u64, 40>>::reverse_bits(self)
    }

    fn rotate_left(self, n: u32) -> Self {
        <Aint<u64, 40>>::rotate_left(self, n)
    }

    fn rotate_right(self, n: u32) -> Self {
        <Aint<u64, 40>>::rotate_right(self, n)
    }

    fn signed_shl(self, n: u32) -> Self {
        <Aint<u64, 40>>::shl(self, n)
    }

    fn signed_shr(self, n: u32) -> Self {
        if (self.0 & (1u64 << (Self::BITS - 1))) == 0 {
            <Aint<u64, 40>>::shr(self, n)
        } else {
            Self((self.0 >> n) | (!(Self::MAX.0 >> n) & Self::MAX.0))
        }
    }

    fn unsigned_shl(self, n: u32) -> Self {
        <Aint<u64, 40>>::shl(self, n)
    }

    fn unsigned_shr(self, n: u32) -> Self {
        <Aint<u64, 40>>::shr(self, n)
    }

    fn swap_bytes(self) -> Self {
        <Aint<u64, 40>>::swap_bytes(self)
    }

    fn from_be(x: Self) -> Self {
        <Aint<u64, 40>>::from_be(x)
    }

    fn from_le(x: Self) -> Self {
        <Aint<u64, 40>>::from_le(x)
    }

    fn to_be(self) -> Self {
        <Aint<u64, 40>>::to_be(self)
    }

    fn to_le(self) -> Self {
        <Aint<u64, 40>>::to_be(self)
    }

    fn pow(self, exp: u32) -> Self {
        <Aint<u64, 40>>::pow(self, exp)
    }
}

impl num_traits::PrimInt for Aint<u64, 48> {
    fn count_ones(self) -> u32 {
        <Aint<u64, 48>>::count_ones(self)
    }

    fn count_zeros(self) -> u32 {
        <Aint<u64, 48>>::count_zeros(self)
    }

    fn leading_zeros(self) -> u32 {
        <Aint<u64, 48>>::leading_zeros(self)
    }

    fn trailing_zeros(self) -> u32 {
        <Aint<u64, 48>>::trailing_zeros(self)
    }

    fn leading_ones(self) -> u32 {
        <Aint<u64, 48>>::leading_ones(self)
    }

    fn trailing_ones(self) -> u32 {
        <Aint<u64, 48>>::trailing_ones(self)
    }

    fn reverse_bits(self) -> Self {
        <Aint<u64, 48>>::reverse_bits(self)
    }

    fn rotate_left(self, n: u32) -> Self {
        <Aint<u64, 48>>::rotate_left(self, n)
    }

    fn rotate_right(self, n: u32) -> Self {
        <Aint<u64, 48>>::rotate_right(self, n)
    }

    fn signed_shl(self, n: u32) -> Self {
        <Aint<u64, 48>>::shl(self, n)
    }

    fn signed_shr(self, n: u32) -> Self {
        if (self.0 & (1u64 << (Self::BITS - 1))) == 0 {
            <Aint<u64, 48>>::shr(self, n)
        } else {
            Self((self.0 >> n) | (!(Self::MAX.0 >> n) & Self::MAX.0))
        }
    }

    fn unsigned_shl(self, n: u32) -> Self {
        <Aint<u64, 48>>::shl(self, n)
    }

    fn unsigned_shr(self, n: u32) -> Self {
        <Aint<u64, 48>>::shr(self, n)
    }

    fn swap_bytes(self) -> Self {
        <Aint<u64, 48>>::swap_bytes(self)
    }

    fn from_be(x: Self) -> Self {
        <Aint<u64, 48>>::from_be(x)
    }

    fn from_le(x: Self) -> Self {
        <Aint<u64, 48>>::from_le(x)
    }

    fn to_be(self) -> Self {
        <Aint<u64, 48>>::to_be(self)
    }

    fn to_le(self) -> Self {
        <Aint<u64, 48>>::to_be(self)
    }

    fn pow(self, exp: u32) -> Self {
        <Aint<u64, 48>>::pow(self, exp)
    }
}

impl num_traits::PrimInt for Aint<u64, 56> {
    fn count_ones(self) -> u32 {
        <Aint<u64, 56>>::count_ones(self)
    }

    fn count_zeros(self) -> u32 {
        <Aint<u64, 56>>::count_zeros(self)
    }

    fn leading_zeros(self) -> u32 {
        <Aint<u64, 56>>::leading_zeros(self)
    }

    fn trailing_zeros(self) -> u32 {
        <Aint<u64, 56>>::trailing_zeros(self)
    }

    fn leading_ones(self) -> u32 {
        <Aint<u64, 56>>::leading_ones(self)
    }

    fn trailing_ones(self) -> u32 {
        <Aint<u64, 56>>::trailing_ones(self)
    }

    fn reverse_bits(self) -> Self {
        <Aint<u64, 56>>::reverse_bits(self)
    }

    fn rotate_left(self, n: u32) -> Self {
        <Aint<u64, 56>>::rotate_left(self, n)
    }

    fn rotate_right(self, n: u32) -> Self {
        <Aint<u64, 56>>::rotate_right(self, n)
    }

    fn signed_shl(self, n: u32) -> Self {
        <Aint<u64, 56>>::shl(self, n)
    }

    fn signed_shr(self, n: u32) -> Self {
        if (self.0 & (1u64 << (Self::BITS - 1))) == 0 {
            <Aint<u64, 56>>::shr(self, n)
        } else {
            Self((self.0 >> n) | (!(Self::MAX.0 >> n) & Self::MAX.0))
        }
    }

    fn unsigned_shl(self, n: u32) -> Self {
        <Aint<u64, 56>>::shl(self, n)
    }

    fn unsigned_shr(self, n: u32) -> Self {
        <Aint<u64, 56>>::shr(self, n)
    }

    fn swap_bytes(self) -> Self {
        <Aint<u64, 56>>::swap_bytes(self)
    }

    fn from_be(x: Self) -> Self {
        <Aint<u64, 56>>::from_be(x)
    }

    fn from_le(x: Self) -> Self {
        <Aint<u64, 56>>::from_le(x)
    }

    fn to_be(self) -> Self {
        <Aint<u64, 56>>::to_be(self)
    }

    fn to_le(self) -> Self {
        <Aint<u64, 56>>::to_be(self)
    }

    fn pow(self, exp: u32) -> Self {
        <Aint<u64, 56>>::pow(self, exp)
    }
}

impl num_traits::PrimInt for Aint<u128, 72> {
    fn count_ones(self) -> u32 {
        <Aint<u128, 72>>::count_ones(self)
    }

    fn count_zeros(self) -> u32 {
        <Aint<u128, 72>>::count_zeros(self)
    }

    fn leading_zeros(self) -> u32 {
        <Aint<u128, 72>>::leading_zeros(self)
    }

    fn trailing_zeros(self) -> u32 {
        <Aint<u128, 72>>::trailing_zeros(self)
    }

    fn leading_ones(self) -> u32 {
        <Aint<u128, 72>>::leading_ones(self)
    }

    fn trailing_ones(self) -> u32 {
        <Aint<u128, 72>>::trailing_ones(self)
    }

    fn reverse_bits(self) -> Self {
        <Aint<u128, 72>>::reverse_bits(self)
    }

    fn rotate_left(self, n: u32) -> Self {
        <Aint<u128, 72>>::rotate_left(self, n)
    }

    fn rotate_right(self, n: u32) -> Self {
        <Aint<u128, 72>>::rotate_right(self, n)
    }

    fn signed_shl(self, n: u32) -> Self {
        <Aint<u128, 72>>::shl(self, n)
    }

    fn signed_shr(self, n: u32) -> Self {
        if (self.0 & (1u128 << (Self::BITS - 1))) == 0 {
            <Aint<u128, 72>>::shr(self, n)
        } else {
            Self((self.0 >> n) | (!(Self::MAX.0 >> n) & Self::MAX.0))
        }
    }

    fn unsigned_shl(self, n: u32) -> Self {
        <Aint<u128, 72>>::shl(self, n)
    }

    fn unsigned_shr(self, n: u32) -> Self {
        <Aint<u128, 72>>::shr(self, n)
    }

    fn swap_bytes(self) -> Self {
        <Aint<u128, 72>>::swap_bytes(self)
    }

    fn from_be(x: Self) -> Self {
        <Aint<u128, 72>>::from_be(x)
    }

    fn from_le(x: Self) -> Self {
        <Aint<u128, 72>>::from_le(x)
    }

    fn to_be(self) -> Self {
        <Aint<u128, 72>>::to_be(self)
    }

    fn to_le(self) -> Self {
        <Aint<u128, 72>>::to_be(self)
    }

    fn pow(self, exp: u32) -> Self {
        <Aint<u128, 72>>::pow(self, exp)
    }
}

impl num_traits::PrimInt for Aint<u128, 80> {
    fn count_ones(self) -> u32 {
        <Aint<u128, 80>>::count_ones(self)
    }

    fn count_zeros(self) -> u32 {
        <Aint<u128, 80>>::count_zeros(self)
    }

    fn leading_zeros(self) -> u32 {
        <Aint<u128, 80>>::leading_zeros(self)
    }

    fn trailing_zeros(self) -> u32 {
        <Aint<u128, 80>>::trailing_zeros(self)
    }

    fn leading_ones(self) -> u32 {
        <Aint<u128, 80>>::leading_ones(self)
    }

    fn trailing_ones(self) -> u32 {
        <Aint<u128, 80>>::trailing_ones(self)
    }

    fn reverse_bits(self) -> Self {
        <Aint<u128, 80>>::reverse_bits(self)
    }

    fn rotate_left(self, n: u32) -> Self {
        <Aint<u128, 80>>::rotate_left(self, n)
    }

    fn rotate_right(self, n: u32) -> Self {
        <Aint<u128, 80>>::rotate_right(self, n)
    }

    fn signed_shl(self, n: u32) -> Self {
        <Aint<u128, 80>>::shl(self, n)
    }

    fn signed_shr(self, n: u32) -> Self {
        if (self.0 & (1u128 << (Self::BITS - 1))) == 0 {
            <Aint<u128, 80>>::shr(self, n)
        } else {
            Self((self.0 >> n) | (!(Self::MAX.0 >> n) & Self::MAX.0))
        }
    }

    fn unsigned_shl(self, n: u32) -> Self {
        <Aint<u128, 80>>::shl(self, n)
    }

    fn unsigned_shr(self, n: u32) -> Self {
        <Aint<u128, 80>>::shr(self, n)
    }

    fn swap_bytes(self) -> Self {
        <Aint<u128, 80>>::swap_bytes(self)
    }

    fn from_be(x: Self) -> Self {
        <Aint<u128, 80>>::from_be(x)
    }

    fn from_le(x: Self) -> Self {
        <Aint<u128, 80>>::from_le(x)
    }

    fn to_be(self) -> Self {
        <Aint<u128, 80>>::to_be(self)
    }

    fn to_le(self) -> Self {
        <Aint<u128, 80>>::to_be(self)
    }

    fn pow(self, exp: u32) -> Self {
        <Aint<u128, 80>>::pow(self, exp)
    }
}

impl num_traits::PrimInt for Aint<u128, 88> {
    fn count_ones(self) -> u32 {
        <Aint<u128, 88>>::count_ones(self)
    }

    fn count_zeros(self) -> u32 {
        <Aint<u128, 88>>::count_zeros(self)
    }

    fn leading_zeros(self) -> u32 {
        <Aint<u128, 88>>::leading_zeros(self)
    }

    fn trailing_zeros(self) -> u32 {
        <Aint<u128, 88>>::trailing_zeros(self)
    }

    fn leading_ones(self) -> u32 {
        <Aint<u128, 88>>::leading_ones(self)
    }

    fn trailing_ones(self) -> u32 {
        <Aint<u128, 88>>::trailing_ones(self)
    }

    fn reverse_bits(self) -> Self {
        <Aint<u128, 88>>::reverse_bits(self)
    }

    fn rotate_left(self, n: u32) -> Self {
        <Aint<u128, 88>>::rotate_left(self, n)
    }

    fn rotate_right(self, n: u32) -> Self {
        <Aint<u128, 88>>::rotate_right(self, n)
    }

    fn signed_shl(self, n: u32) -> Self {
        <Aint<u128, 88>>::shl(self, n)
    }

    fn signed_shr(self, n: u32) -> Self {
        if (self.0 & (1u128 << (Self::BITS - 1))) == 0 {
            <Aint<u128, 88>>::shr(self, n)
        } else {
            Self((self.0 >> n) | (!(Self::MAX.0 >> n) & Self::MAX.0))
        }
    }

    fn unsigned_shl(self, n: u32) -> Self {
        <Aint<u128, 88>>::shl(self, n)
    }

    fn unsigned_shr(self, n: u32) -> Self {
        <Aint<u128, 88>>::shr(self, n)
    }

    fn swap_bytes(self) -> Self {
        <Aint<u128, 88>>::swap_bytes(self)
    }

    fn from_be(x: Self) -> Self {
        <Aint<u128, 88>>::from_be(x)
    }

    fn from_le(x: Self) -> Self {
        <Aint<u128, 88>>::from_le(x)
    }

    fn to_be(self) -> Self {
        <Aint<u128, 88>>::to_be(self)
    }

    fn to_le(self) -> Self {
        <Aint<u128, 88>>::to_be(self)
    }

    fn pow(self, exp: u32) -> Self {
        <Aint<u128, 88>>::pow(self, exp)
    }
}

impl num_traits::PrimInt for Aint<u128, 96> {
    fn count_ones(self) -> u32 {
        <Aint<u128, 96>>::count_ones(self)
    }

    fn count_zeros(self) -> u32 {
        <Aint<u128, 96>>::count_zeros(self)
    }

    fn leading_zeros(self) -> u32 {
        <Aint<u128, 96>>::leading_zeros(self)
    }

    fn trailing_zeros(self) -> u32 {
        <Aint<u128, 96>>::trailing_zeros(self)
    }

    fn leading_ones(self) -> u32 {
        <Aint<u128, 96>>::leading_ones(self)
    }

    fn trailing_ones(self) -> u32 {
        <Aint<u128, 96>>::trailing_ones(self)
    }

    fn reverse_bits(self) -> Self {
        <Aint<u128, 96>>::reverse_bits(self)
    }

    fn rotate_left(self, n: u32) -> Self {
        <Aint<u128, 96>>::rotate_left(self, n)
    }

    fn rotate_right(self, n: u32) -> Self {
        <Aint<u128, 96>>::rotate_right(self, n)
    }

    fn signed_shl(self, n: u32) -> Self {
        <Aint<u128, 96>>::shl(self, n)
    }

    fn signed_shr(self, n: u32) -> Self {
        if (self.0 & (1u128 << (Self::BITS - 1))) == 0 {
            <Aint<u128, 96>>::shr(self, n)
        } else {
            Self((self.0 >> n) | (!(Self::MAX.0 >> n) & Self::MAX.0))
        }
    }

    fn unsigned_shl(self, n: u32) -> Self {
        <Aint<u128, 96>>::shl(self, n)
    }

    fn unsigned_shr(self, n: u32) -> Self {
        <Aint<u128, 96>>::shr(self, n)
    }

    fn swap_bytes(self) -> Self {
        <Aint<u128, 96>>::swap_bytes(self)
    }

    fn from_be(x: Self) -> Self {
        <Aint<u128, 96>>::from_be(x)
    }

    fn from_le(x: Self) -> Self {
        <Aint<u128, 96>>::from_le(x)
    }

    fn to_be(self) -> Self {
        <Aint<u128, 96>>::to_be(self)
    }

    fn to_le(self) -> Self {
        <Aint<u128, 96>>::to_be(self)
    }

    fn pow(self, exp: u32) -> Self {
        <Aint<u128, 96>>::pow(self, exp)
    }
}

impl num_traits::PrimInt for Aint<u128, 104> {
    fn count_ones(self) -> u32 {
        <Aint<u128, 104>>::count_ones(self)
    }

    fn count_zeros(self) -> u32 {
        <Aint<u128, 104>>::count_zeros(self)
    }

    fn leading_zeros(self) -> u32 {
        <Aint<u128, 104>>::leading_zeros(self)
    }

    fn trailing_zeros(self) -> u32 {
        <Aint<u128, 104>>::trailing_zeros(self)
    }

    fn leading_ones(self) -> u32 {
        <Aint<u128, 104>>::leading_ones(self)
    }

    fn trailing_ones(self) -> u32 {
        <Aint<u128, 104>>::trailing_ones(self)
    }

    fn reverse_bits(self) -> Self {
        <Aint<u128, 104>>::reverse_bits(self)
    }

    fn rotate_left(self, n: u32) -> Self {
        <Aint<u128, 104>>::rotate_left(self, n)
    }

    fn rotate_right(self, n: u32) -> Self {
        <Aint<u128, 104>>::rotate_right(self, n)
    }

    fn signed_shl(self, n: u32) -> Self {
        <Aint<u128, 104>>::shl(self, n)
    }

    fn signed_shr(self, n: u32) -> Self {
        if (self.0 & (1u128 << (Self::BITS - 1))) == 0 {
            <Aint<u128, 104>>::shr(self, n)
        } else {
            Self((self.0 >> n) | (!(Self::MAX.0 >> n) & Self::MAX.0))
        }
    }

    fn unsigned_shl(self, n: u32) -> Self {
        <Aint<u128, 104>>::shl(self, n)
    }

    fn unsigned_shr(self, n: u32) -> Self {
        <Aint<u128, 104>>::shr(self, n)
    }

    fn swap_bytes(self) -> Self {
        <Aint<u128, 104>>::swap_bytes(self)
    }

    fn from_be(x: Self) -> Self {
        <Aint<u128, 104>>::from_be(x)
    }

    fn from_le(x: Self) -> Self {
        <Aint<u128, 104>>::from_le(x)
    }

    fn to_be(self) -> Self {
        <Aint<u128, 104>>::to_be(self)
    }

    fn to_le(self) -> Self {
        <Aint<u128, 104>>::to_be(self)
    }

    fn pow(self, exp: u32) -> Self {
        <Aint<u128, 104>>::pow(self, exp)
    }
}

impl num_traits::PrimInt for Aint<u128, 112> {
    fn count_ones(self) -> u32 {
        <Aint<u128, 112>>::count_ones(self)
    }

    fn count_zeros(self) -> u32 {
        <Aint<u128, 112>>::count_zeros(self)
    }

    fn leading_zeros(self) -> u32 {
        <Aint<u128, 112>>::leading_zeros(self)
    }

    fn trailing_zeros(self) -> u32 {
        <Aint<u128, 112>>::trailing_zeros(self)
    }

    fn leading_ones(self) -> u32 {
        <Aint<u128, 112>>::leading_ones(self)
    }

    fn trailing_ones(self) -> u32 {
        <Aint<u128, 112>>::trailing_ones(self)
    }

    fn reverse_bits(self) -> Self {
        <Aint<u128, 112>>::reverse_bits(self)
    }

    fn rotate_left(self, n: u32) -> Self {
        <Aint<u128, 112>>::rotate_left(self, n)
    }

    fn rotate_right(self, n: u32) -> Self {
        <Aint<u128, 112>>::rotate_right(self, n)
    }

    fn signed_shl(self, n: u32) -> Self {
        <Aint<u128, 112>>::shl(self, n)
    }

    fn signed_shr(self, n: u32) -> Self {
        if (self.0 & (1u128 << (Self::BITS - 1))) == 0 {
            <Aint<u128, 112>>::shr(self, n)
        } else {
            Self((self.0 >> n) | (!(Self::MAX.0 >> n) & Self::MAX.0))
        }
    }

    fn unsigned_shl(self, n: u32) -> Self {
        <Aint<u128, 112>>::shl(self, n)
    }

    fn unsigned_shr(self, n: u32) -> Self {
        <Aint<u128, 112>>::shr(self, n)
    }

    fn swap_bytes(self) -> Self {
        <Aint<u128, 112>>::swap_bytes(self)
    }

    fn from_be(x: Self) -> Self {
        <Aint<u128, 112>>::from_be(x)
    }

    fn from_le(x: Self) -> Self {
        <Aint<u128, 112>>::from_le(x)
    }

    fn to_be(self) -> Self {
        <Aint<u128, 112>>::to_be(self)
    }

    fn to_le(self) -> Self {
        <Aint<u128, 112>>::to_be(self)
    }

    fn pow(self, exp: u32) -> Self {
        <Aint<u128, 112>>::pow(self, exp)
    }
}

impl num_traits::PrimInt for Aint<u128, 120> {
    fn count_ones(self) -> u32 {
        <Aint<u128, 120>>::count_ones(self)
    }

    fn count_zeros(self) -> u32 {
        <Aint<u128, 120>>::count_zeros(self)
    }

    fn leading_zeros(self) -> u32 {
        <Aint<u128, 120>>::leading_zeros(self)
    }

    fn trailing_zeros(self) -> u32 {
        <Aint<u128, 120>>::trailing_zeros(self)
    }

    fn leading_ones(self) -> u32 {
        <Aint<u128, 120>>::leading_ones(self)
    }

    fn trailing_ones(self) -> u32 {
        <Aint<u128, 120>>::trailing_ones(self)
    }

    fn reverse_bits(self) -> Self {
        <Aint<u128, 120>>::reverse_bits(self)
    }

    fn rotate_left(self, n: u32) -> Self {
        <Aint<u128, 120>>::rotate_left(self, n)
    }

    fn rotate_right(self, n: u32) -> Self {
        <Aint<u128, 120>>::rotate_right(self, n)
    }

    fn signed_shl(self, n: u32) -> Self {
        <Aint<u128, 120>>::shl(self, n)
    }

    fn signed_shr(self, n: u32) -> Self {
        if (self.0 & (1u128 << (Self::BITS - 1))) == 0 {
            <Aint<u128, 120>>::shr(self, n)
        } else {
            Self((self.0 >> n) | (!(Self::MAX.0 >> n) & Self::MAX.0))
        }
    }

    fn unsigned_shl(self, n: u32) -> Self {
        <Aint<u128, 120>>::shl(self, n)
    }

    fn unsigned_shr(self, n: u32) -> Self {
        <Aint<u128, 120>>::shr(self, n)
    }

    fn swap_bytes(self) -> Self {
        <Aint<u128, 120>>::swap_bytes(self)
    }

    fn from_be(x: Self) -> Self {
        <Aint<u128, 120>>::from_be(x)
    }

    fn from_le(x: Self) -> Self {
        <Aint<u128, 120>>::from_le(x)
    }

    fn to_be(self) -> Self {
        <Aint<u128, 120>>::to_be(self)
    }

    fn to_le(self) -> Self {
        <Aint<u128, 120>>::to_be(self)
    }

    fn pow(self, exp: u32) -> Self {
        <Aint<u128, 120>>::pow(self, exp)
    }
}

impl num_traits::PrimInt for Aint<i32, 24> {
    fn count_ones(self) -> u32 {
        <Aint<i32, 24>>::count_ones(self)
    }

    fn count_zeros(self) -> u32 {
        <Aint<i32, 24>>::count_zeros(self)
    }

    fn leading_zeros(self) -> u32 {
        <Aint<i32, 24>>::leading_zeros(self)
    }

    fn trailing_zeros(self) -> u32 {
        <Aint<i32, 24>>::trailing_zeros(self)
    }

    fn leading_ones(self) -> u32 {
        <Aint<i32, 24>>::leading_ones(self)
    }

    fn trailing_ones(self) -> u32 {
        <Aint<i32, 24>>::trailing_ones(self)
    }

    fn reverse_bits(self) -> Self {
        <Aint<i32, 24>>::reverse_bits(self)
    }

    fn rotate_left(self, n: u32) -> Self {
        <Aint<i32, 24>>::rotate_left(self, n)
    }

    fn rotate_right(self, n: u32) -> Self {
        <Aint<i32, 24>>::rotate_right(self, n)
    }

    fn signed_shl(self, n: u32) -> Self {
        <Aint<i32, 24>>::shl(self, n)
    }

    fn signed_shr(self, n: u32) -> Self {
        <Aint<i32, 24>>::shr(self, n)
    }

    fn unsigned_shl(self, n: u32) -> Self {
        <Aint<i32, 24>>::shl(self, n)
    }

    fn unsigned_shr(self, n: u32) -> Self {
        Self((self.0 >> n) & (Self::MASK >> n))
    }

    fn swap_bytes(self) -> Self {
        <Aint<i32, 24>>::swap_bytes(self)
    }

    fn from_be(x: Self) -> Self {
        <Aint<i32, 24>>::from_be(x)
    }

    fn from_le(x: Self) -> Self {
        <Aint<i32, 24>>::from_le(x)
    }

    fn to_be(self) -> Self {
        <Aint<i32, 24>>::to_be(self)
    }

    fn to_le(self) -> Self {
        <Aint<i32, 24>>::to_be(self)
    }

    fn pow(self, exp: u32) -> Self {
        <Aint<i32, 24>>::pow(self, exp)
    }
}

impl num_traits::PrimInt for Aint<i64, 40> {
    fn count_ones(self) -> u32 {
        <Aint<i64, 40>>::count_ones(self)
    }

    fn count_zeros(self) -> u32 {
        <Aint<i64, 40>>::count_zeros(self)
    }

    fn leading_zeros(self) -> u32 {
        <Aint<i64, 40>>::leading_zeros(self)
    }

    fn trailing_zeros(self) -> u32 {
        <Aint<i64, 40>>::trailing_zeros(self)
    }

    fn leading_ones(self) -> u32 {
        <Aint<i64, 40>>::leading_ones(self)
    }

    fn trailing_ones(self) -> u32 {
        <Aint<i64, 40>>::trailing_ones(self)
    }

    fn reverse_bits(self) -> Self {
        <Aint<i64, 40>>::reverse_bits(self)
    }

    fn rotate_left(self, n: u32) -> Self {
        <Aint<i64, 40>>::rotate_left(self, n)
    }

    fn rotate_right(self, n: u32) -> Self {
        <Aint<i64, 40>>::rotate_right(self, n)
    }

    fn signed_shl(self, n: u32) -> Self {
        <Aint<i64, 40>>::shl(self, n)
    }

    fn signed_shr(self, n: u32) -> Self {
        <Aint<i64, 40>>::shr(self, n)
    }

    fn unsigned_shl(self, n: u32) -> Self {
        <Aint<i64, 40>>::shl(self, n)
    }

    fn unsigned_shr(self, n: u32) -> Self {
        Self((self.0 >> n) & (Self::MASK >> n))
    }

    fn swap_bytes(self) -> Self {
        <Aint<i64, 40>>::swap_bytes(self)
    }

    fn from_be(x: Self) -> Self {
        <Aint<i64, 40>>::from_be(x)
    }

    fn from_le(x: Self) -> Self {
        <Aint<i64, 40>>::from_le(x)
    }

    fn to_be(self) -> Self {
        <Aint<i64, 40>>::to_be(self)
    }

    fn to_le(self) -> Self {
        <Aint<i64, 40>>::to_be(self)
    }

    fn pow(self, exp: u32) -> Self {
        <Aint<i64, 40>>::pow(self, exp)
    }
}

impl num_traits::PrimInt for Aint<i64, 48> {
    fn count_ones(self) -> u32 {
        <Aint<i64, 48>>::count_ones(self)
    }

    fn count_zeros(self) -> u32 {
        <Aint<i64, 48>>::count_zeros(self)
    }

    fn leading_zeros(self) -> u32 {
        <Aint<i64, 48>>::leading_zeros(self)
    }

    fn trailing_zeros(self) -> u32 {
        <Aint<i64, 48>>::trailing_zeros(self)
    }

    fn leading_ones(self) -> u32 {
        <Aint<i64, 48>>::leading_ones(self)
    }

    fn trailing_ones(self) -> u32 {
        <Aint<i64, 48>>::trailing_ones(self)
    }

    fn reverse_bits(self) -> Self {
        <Aint<i64, 48>>::reverse_bits(self)
    }

    fn rotate_left(self, n: u32) -> Self {
        <Aint<i64, 48>>::rotate_left(self, n)
    }

    fn rotate_right(self, n: u32) -> Self {
        <Aint<i64, 48>>::rotate_right(self, n)
    }

    fn signed_shl(self, n: u32) -> Self {
        <Aint<i64, 48>>::shl(self, n)
    }

    fn signed_shr(self, n: u32) -> Self {
        <Aint<i64, 48>>::shr(self, n)
    }

    fn unsigned_shl(self, n: u32) -> Self {
        <Aint<i64, 48>>::shl(self, n)
    }

    fn unsigned_shr(self, n: u32) -> Self {
        Self((self.0 >> n) & (Self::MASK >> n))
    }

    fn swap_bytes(self) -> Self {
        <Aint<i64, 48>>::swap_bytes(self)
    }

    fn from_be(x: Self) -> Self {
        <Aint<i64, 48>>::from_be(x)
    }

    fn from_le(x: Self) -> Self {
        <Aint<i64, 48>>::from_le(x)
    }

    fn to_be(self) -> Self {
        <Aint<i64, 48>>::to_be(self)
    }

    fn to_le(self) -> Self {
        <Aint<i64, 48>>::to_be(self)
    }

    fn pow(self, exp: u32) -> Self {
        <Aint<i64, 48>>::pow(self, exp)
    }
}

impl num_traits::PrimInt for Aint<i64, 56> {
    fn count_ones(self) -> u32 {
        <Aint<i64, 56>>::count_ones(self)
    }

    fn count_zeros(self) -> u32 {
        <Aint<i64, 56>>::count_zeros(self)
    }

    fn leading_zeros(self) -> u32 {
        <Aint<i64, 56>>::leading_zeros(self)
    }

    fn trailing_zeros(self) -> u32 {
        <Aint<i64, 56>>::trailing_zeros(self)
    }

    fn leading_ones(self) -> u32 {
        <Aint<i64, 56>>::leading_ones(self)
    }

    fn trailing_ones(self) -> u32 {
        <Aint<i64, 56>>::trailing_ones(self)
    }

    fn reverse_bits(self) -> Self {
        <Aint<i64, 56>>::reverse_bits(self)
    }

    fn rotate_left(self, n: u32) -> Self {
        <Aint<i64, 56>>::rotate_left(self, n)
    }

    fn rotate_right(self, n: u32) -> Self {
        <Aint<i64, 56>>::rotate_right(self, n)
    }

    fn signed_shl(self, n: u32) -> Self {
        <Aint<i64, 56>>::shl(self, n)
    }

    fn signed_shr(self, n: u32) -> Self {
        <Aint<i64, 56>>::shr(self, n)
    }

    fn unsigned_shl(self, n: u32) -> Self {
        <Aint<i64, 56>>::shl(self, n)
    }

    fn unsigned_shr(self, n: u32) -> Self {
        Self((self.0 >> n) & (Self::MASK >> n))
    }

    fn swap_bytes(self) -> Self {
        <Aint<i64, 56>>::swap_bytes(self)
    }

    fn from_be(x: Self) -> Self {
        <Aint<i64, 56>>::from_be(x)
    }

    fn from_le(x: Self) -> Self {
        <Aint<i64, 56>>::from_le(x)
    }

    fn to_be(self) -> Self {
        <Aint<i64, 56>>::to_be(self)
    }

    fn to_le(self) -> Self {
        <Aint<i64, 56>>::to_be(self)
    }

    fn pow(self, exp: u32) -> Self {
        <Aint<i64, 56>>::pow(self, exp)
    }
}

impl num_traits::PrimInt for Aint<i128, 72> {
    fn count_ones(self) -> u32 {
        <Aint<i128, 72>>::count_ones(self)
    }

    fn count_zeros(self) -> u32 {
        <Aint<i128, 72>>::count_zeros(self)
    }

    fn leading_zeros(self) -> u32 {
        <Aint<i128, 72>>::leading_zeros(self)
    }

    fn trailing_zeros(self) -> u32 {
        <Aint<i128, 72>>::trailing_zeros(self)
    }

    fn leading_ones(self) -> u32 {
        <Aint<i128, 72>>::leading_ones(self)
    }

    fn trailing_ones(self) -> u32 {
        <Aint<i128, 72>>::trailing_ones(self)
    }

    fn reverse_bits(self) -> Self {
        <Aint<i128, 72>>::reverse_bits(self)
    }

    fn rotate_left(self, n: u32) -> Self {
        <Aint<i128, 72>>::rotate_left(self, n)
    }

    fn rotate_right(self, n: u32) -> Self {
        <Aint<i128, 72>>::rotate_right(self, n)
    }

    fn signed_shl(self, n: u32) -> Self {
        <Aint<i128, 72>>::shl(self, n)
    }

    fn signed_shr(self, n: u32) -> Self {
        <Aint<i128, 72>>::shr(self, n)
    }

    fn unsigned_shl(self, n: u32) -> Self {
        <Aint<i128, 72>>::shl(self, n)
    }

    fn unsigned_shr(self, n: u32) -> Self {
        Self((self.0 >> n) & (Self::MASK >> n))
    }

    fn swap_bytes(self) -> Self {
        <Aint<i128, 72>>::swap_bytes(self)
    }

    fn from_be(x: Self) -> Self {
        <Aint<i128, 72>>::from_be(x)
    }

    fn from_le(x: Self) -> Self {
        <Aint<i128, 72>>::from_le(x)
    }

    fn to_be(self) -> Self {
        <Aint<i128, 72>>::to_be(self)
    }

    fn to_le(self) -> Self {
        <Aint<i128, 72>>::to_be(self)
    }

    fn pow(self, exp: u32) -> Self {
        <Aint<i128, 72>>::pow(self, exp)
    }
}

impl num_traits::PrimInt for Aint<i128, 80> {
    fn count_ones(self) -> u32 {
        <Aint<i128, 80>>::count_ones(self)
    }

    fn count_zeros(self) -> u32 {
        <Aint<i128, 80>>::count_zeros(self)
    }

    fn leading_zeros(self) -> u32 {
        <Aint<i128, 80>>::leading_zeros(self)
    }

    fn trailing_zeros(self) -> u32 {
        <Aint<i128, 80>>::trailing_zeros(self)
    }

    fn leading_ones(self) -> u32 {
        <Aint<i128, 80>>::leading_ones(self)
    }

    fn trailing_ones(self) -> u32 {
        <Aint<i128, 80>>::trailing_ones(self)
    }

    fn reverse_bits(self) -> Self {
        <Aint<i128, 80>>::reverse_bits(self)
    }

    fn rotate_left(self, n: u32) -> Self {
        <Aint<i128, 80>>::rotate_left(self, n)
    }

    fn rotate_right(self, n: u32) -> Self {
        <Aint<i128, 80>>::rotate_right(self, n)
    }

    fn signed_shl(self, n: u32) -> Self {
        <Aint<i128, 80>>::shl(self, n)
    }

    fn signed_shr(self, n: u32) -> Self {
        <Aint<i128, 80>>::shr(self, n)
    }

    fn unsigned_shl(self, n: u32) -> Self {
        <Aint<i128, 80>>::shl(self, n)
    }

    fn unsigned_shr(self, n: u32) -> Self {
        Self((self.0 >> n) & (Self::MASK >> n))
    }

    fn swap_bytes(self) -> Self {
        <Aint<i128, 80>>::swap_bytes(self)
    }

    fn from_be(x: Self) -> Self {
        <Aint<i128, 80>>::from_be(x)
    }

    fn from_le(x: Self) -> Self {
        <Aint<i128, 80>>::from_le(x)
    }

    fn to_be(self) -> Self {
        <Aint<i128, 80>>::to_be(self)
    }

    fn to_le(self) -> Self {
        <Aint<i128, 80>>::to_be(self)
    }

    fn pow(self, exp: u32) -> Self {
        <Aint<i128, 80>>::pow(self, exp)
    }
}

impl num_traits::PrimInt for Aint<i128, 88> {
    fn count_ones(self) -> u32 {
        <Aint<i128, 88>>::count_ones(self)
    }

    fn count_zeros(self) -> u32 {
        <Aint<i128, 88>>::count_zeros(self)
    }

    fn leading_zeros(self) -> u32 {
        <Aint<i128, 88>>::leading_zeros(self)
    }

    fn trailing_zeros(self) -> u32 {
        <Aint<i128, 88>>::trailing_zeros(self)
    }

    fn leading_ones(self) -> u32 {
        <Aint<i128, 88>>::leading_ones(self)
    }

    fn trailing_ones(self) -> u32 {
        <Aint<i128, 88>>::trailing_ones(self)
    }

    fn reverse_bits(self) -> Self {
        <Aint<i128, 88>>::reverse_bits(self)
    }

    fn rotate_left(self, n: u32) -> Self {
        <Aint<i128, 88>>::rotate_left(self, n)
    }

    fn rotate_right(self, n: u32) -> Self {
        <Aint<i128, 88>>::rotate_right(self, n)
    }

    fn signed_shl(self, n: u32) -> Self {
        <Aint<i128, 88>>::shl(self, n)
    }

    fn signed_shr(self, n: u32) -> Self {
        <Aint<i128, 88>>::shr(self, n)
    }

    fn unsigned_shl(self, n: u32) -> Self {
        <Aint<i128, 88>>::shl(self, n)
    }

    fn unsigned_shr(self, n: u32) -> Self {
        Self((self.0 >> n) & (Self::MASK >> n))
    }

    fn swap_bytes(self) -> Self {
        <Aint<i128, 88>>::swap_bytes(self)
    }

    fn from_be(x: Self) -> Self {
        <Aint<i128, 88>>::from_be(x)
    }

    fn from_le(x: Self) -> Self {
        <Aint<i128, 88>>::from_le(x)
    }

    fn to_be(self) -> Self {
        <Aint<i128, 88>>::to_be(self)
    }

    fn to_le(self) -> Self {
        <Aint<i128, 88>>::to_be(self)
    }

    fn pow(self, exp: u32) -> Self {
        <Aint<i128, 88>>::pow(self, exp)
    }
}

impl num_traits::PrimInt for Aint<i128, 96> {
    fn count_ones(self) -> u32 {
        <Aint<i128, 96>>::count_ones(self)
    }

    fn count_zeros(self) -> u32 {
        <Aint<i128, 96>>::count_zeros(self)
    }

    fn leading_zeros(self) -> u32 {
        <Aint<i128, 96>>::leading_zeros(self)
    }

    fn trailing_zeros(self) -> u32 {
        <Aint<i128, 96>>::trailing_zeros(self)
    }

    fn leading_ones(self) -> u32 {
        <Aint<i128, 96>>::leading_ones(self)
    }

    fn trailing_ones(self) -> u32 {
        <Aint<i128, 96>>::trailing_ones(self)
    }

    fn reverse_bits(self) -> Self {
        <Aint<i128, 96>>::reverse_bits(self)
    }

    fn rotate_left(self, n: u32) -> Self {
        <Aint<i128, 96>>::rotate_left(self, n)
    }

    fn rotate_right(self, n: u32) -> Self {
        <Aint<i128, 96>>::rotate_right(self, n)
    }

    fn signed_shl(self, n: u32) -> Self {
        <Aint<i128, 96>>::shl(self, n)
    }

    fn signed_shr(self, n: u32) -> Self {
        <Aint<i128, 96>>::shr(self, n)
    }

    fn unsigned_shl(self, n: u32) -> Self {
        <Aint<i128, 96>>::shl(self, n)
    }

    fn unsigned_shr(self, n: u32) -> Self {
        Self((self.0 >> n) & (Self::MASK >> n))
    }

    fn swap_bytes(self) -> Self {
        <Aint<i128, 96>>::swap_bytes(self)
    }

    fn from_be(x: Self) -> Self {
        <Aint<i128, 96>>::from_be(x)
    }

    fn from_le(x: Self) -> Self {
        <Aint<i128, 96>>::from_le(x)
    }

    fn to_be(self) -> Self {
        <Aint<i128, 96>>::to_be(self)
    }

    fn to_le(self) -> Self {
        <Aint<i128, 96>>::to_be(self)
    }

    fn pow(self, exp: u32) -> Self {
        <Aint<i128, 96>>::pow(self, exp)
    }
}

impl num_traits::PrimInt for Aint<i128, 104> {
    fn count_ones(self) -> u32 {
        <Aint<i128, 104>>::count_ones(self)
    }

    fn count_zeros(self) -> u32 {
        <Aint<i128, 104>>::count_zeros(self)
    }

    fn leading_zeros(self) -> u32 {
        <Aint<i128, 104>>::leading_zeros(self)
    }

    fn trailing_zeros(self) -> u32 {
        <Aint<i128, 104>>::trailing_zeros(self)
    }

    fn leading_ones(self) -> u32 {
        <Aint<i128, 104>>::leading_ones(self)
    }

    fn trailing_ones(self) -> u32 {
        <Aint<i128, 104>>::trailing_ones(self)
    }

    fn reverse_bits(self) -> Self {
        <Aint<i128, 104>>::reverse_bits(self)
    }

    fn rotate_left(self, n: u32) -> Self {
        <Aint<i128, 104>>::rotate_left(self, n)
    }

    fn rotate_right(self, n: u32) -> Self {
        <Aint<i128, 104>>::rotate_right(self, n)
    }

    fn signed_shl(self, n: u32) -> Self {
        <Aint<i128, 104>>::shl(self, n)
    }

    fn signed_shr(self, n: u32) -> Self {
        <Aint<i128, 104>>::shr(self, n)
    }

    fn unsigned_shl(self, n: u32) -> Self {
        <Aint<i128, 104>>::shl(self, n)
    }

    fn unsigned_shr(self, n: u32) -> Self {
        Self((self.0 >> n) & (Self::MASK >> n))
    }

    fn swap_bytes(self) -> Self {
        <Aint<i128, 104>>::swap_bytes(self)
    }

    fn from_be(x: Self) -> Self {
        <Aint<i128, 104>>::from_be(x)
    }

    fn from_le(x: Self) -> Self {
        <Aint<i128, 104>>::from_le(x)
    }

    fn to_be(self) -> Self {
        <Aint<i128, 104>>::to_be(self)
    }

    fn to_le(self) -> Self {
        <Aint<i128, 104>>::to_be(self)
    }

    fn pow(self, exp: u32) -> Self {
        <Aint<i128, 104>>::pow(self, exp)
    }
}

impl num_traits::PrimInt for Aint<i128, 112> {
    fn count_ones(self) -> u32 {
        <Aint<i128, 112>>::count_ones(self)
    }

    fn count_zeros(self) -> u32 {
        <Aint<i128, 112>>::count_zeros(self)
    }

    fn leading_zeros(self) -> u32 {
        <Aint<i128, 112>>::leading_zeros(self)
    }

    fn trailing_zeros(self) -> u32 {
        <Aint<i128, 112>>::trailing_zeros(self)
    }

    fn leading_ones(self) -> u32 {
        <Aint<i128, 112>>::leading_ones(self)
    }

    fn trailing_ones(self) -> u32 {
        <Aint<i128, 112>>::trailing_ones(self)
    }

    fn reverse_bits(self) -> Self {
        <Aint<i128, 112>>::reverse_bits(self)
    }

    fn rotate_left(self, n: u32) -> Self {
        <Aint<i128, 112>>::rotate_left(self, n)
    }

    fn rotate_right(self, n: u32) -> Self {
        <Aint<i128, 112>>::rotate_right(self, n)
    }

    fn signed_shl(self, n: u32) -> Self {
        <Aint<i128, 112>>::shl(self, n)
    }

    fn signed_shr(self, n: u32) -> Self {
        <Aint<i128, 112>>::shr(self, n)
    }

    fn unsigned_shl(self, n: u32) -> Self {
        <Aint<i128, 112>>::shl(self, n)
    }

    fn unsigned_shr(self, n: u32) -> Self {
        Self((self.0 >> n) & (Self::MASK >> n))
    }

    fn swap_bytes(self) -> Self {
        <Aint<i128, 112>>::swap_bytes(self)
    }

    fn from_be(x: Self) -> Self {
        <Aint<i128, 112>>::from_be(x)
    }

    fn from_le(x: Self) -> Self {
        <Aint<i128, 112>>::from_le(x)
    }

    fn to_be(self) -> Self {
        <Aint<i128, 112>>::to_be(self)
    }

    fn to_le(self) -> Self {
        <Aint<i128, 112>>::to_be(self)
    }

    fn pow(self, exp: u32) -> Self {
        <Aint<i128, 112>>::pow(self, exp)
    }
}

impl num_traits::PrimInt for Aint<i128, 120> {
    fn count_ones(self) -> u32 {
        <Aint<i128, 120>>::count_ones(self)
    }

    fn count_zeros(self) -> u32 {
        <Aint<i128, 120>>::count_zeros(self)
    }

    fn leading_zeros(self) -> u32 {
        <Aint<i128, 120>>::leading_zeros(self)
    }

    fn trailing_zeros(self) -> u32 {
        <Aint<i128, 120>>::trailing_zeros(self)
    }

    fn leading_ones(self) -> u32 {
        <Aint<i128, 120>>::leading_ones(self)
    }

    fn trailing_ones(self) -> u32 {
        <Aint<i128, 120>>::trailing_ones(self)
    }

    fn reverse_bits(self) -> Self {
        <Aint<i128, 120>>::reverse_bits(self)
    }

    fn rotate_left(self, n: u32) -> Self {
        <Aint<i128, 120>>::rotate_left(self, n)
    }

    fn rotate_right(self, n: u32) -> Self {
        <Aint<i128, 120>>::rotate_right(self, n)
    }

    fn signed_shl(self, n: u32) -> Self {
        <Aint<i128, 120>>::shl(self, n)
    }

    fn signed_shr(self, n: u32) -> Self {
        <Aint<i128, 120>>::shr(self, n)
    }

    fn unsigned_shl(self, n: u32) -> Self {
        <Aint<i128, 120>>::shl(self, n)
    }

    fn unsigned_shr(self, n: u32) -> Self {
        Self((self.0 >> n) & (Self::MASK >> n))
    }

    fn swap_bytes(self) -> Self {
        <Aint<i128, 120>>::swap_bytes(self)
    }

    fn from_be(x: Self) -> Self {
        <Aint<i128, 120>>::from_be(x)
    }

    fn from_le(x: Self) -> Self {
        <Aint<i128, 120>>::from_le(x)
    }

    fn to_be(self) -> Self {
        <Aint<i128, 120>>::to_be(self)
    }

    fn to_le(self) -> Self {
        <Aint<i128, 120>>::to_be(self)
    }

    fn pow(self, exp: u32) -> Self {
        <Aint<i128, 120>>::pow(self, exp)
    }
}
