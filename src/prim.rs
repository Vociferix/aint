use crate::tybit;
use crate::width;

pub trait Prim:
    core::fmt::Debug
    + core::fmt::Display
    + core::fmt::Octal
    + core::fmt::Binary
    + core::fmt::UpperHex
    + core::fmt::LowerHex
    + core::fmt::UpperExp
    + core::fmt::LowerExp
    + Clone
    + Copy
    + PartialEq
    + Eq
    + PartialOrd
    + Ord
    + core::hash::Hash
    + core::ops::Add<Output = Self>
    + core::ops::Sub<Output = Self>
    + core::ops::Mul<Output = Self>
    + core::ops::Div<Output = Self>
    + core::ops::Rem<Output = Self>
    + core::ops::BitAnd<Output = Self>
    + core::ops::BitOr<Output = Self>
    + core::ops::BitXor<Output = Self>
    + core::ops::Not<Output = Self>
    + core::ops::Shl<u8, Output = Self>
    + core::ops::Shl<u16, Output = Self>
    + core::ops::Shl<u32, Output = Self>
    + core::ops::Shl<u64, Output = Self>
    + core::ops::Shl<u128, Output = Self>
    + core::ops::Shl<usize, Output = Self>
    + core::ops::Shl<i8, Output = Self>
    + core::ops::Shl<i16, Output = Self>
    + core::ops::Shl<i32, Output = Self>
    + core::ops::Shl<i64, Output = Self>
    + core::ops::Shl<i128, Output = Self>
    + core::ops::Shl<isize, Output = Self>
    + core::ops::Shr<u8, Output = Self>
    + core::ops::Shr<u16, Output = Self>
    + core::ops::Shr<u32, Output = Self>
    + core::ops::Shr<u64, Output = Self>
    + core::ops::Shr<u128, Output = Self>
    + core::ops::Shr<usize, Output = Self>
    + core::ops::Shr<i8, Output = Self>
    + core::ops::Shr<i16, Output = Self>
    + core::ops::Shr<i32, Output = Self>
    + core::ops::Shr<i64, Output = Self>
    + core::ops::Shr<i128, Output = Self>
    + core::ops::Shr<isize, Output = Self>
    + core::ops::AddAssign
    + core::ops::SubAssign
    + core::ops::MulAssign
    + core::ops::DivAssign
    + core::ops::RemAssign
    + core::ops::BitAndAssign
    + core::ops::BitOrAssign
    + core::ops::BitXorAssign
    + core::ops::ShlAssign<u8>
    + core::ops::ShlAssign<u16>
    + core::ops::ShlAssign<u32>
    + core::ops::ShlAssign<u64>
    + core::ops::ShlAssign<u128>
    + core::ops::ShlAssign<usize>
    + core::ops::ShlAssign<i8>
    + core::ops::ShlAssign<i16>
    + core::ops::ShlAssign<i32>
    + core::ops::ShlAssign<i64>
    + core::ops::ShlAssign<i128>
    + core::ops::ShlAssign<isize>
    + core::ops::ShrAssign<u8>
    + core::ops::ShrAssign<u16>
    + core::ops::ShrAssign<u32>
    + core::ops::ShrAssign<u64>
    + core::ops::ShrAssign<u128>
    + core::ops::ShrAssign<usize>
    + core::ops::ShrAssign<i8>
    + core::ops::ShrAssign<i16>
    + core::ops::ShrAssign<i32>
    + core::ops::ShrAssign<i64>
    + core::ops::ShrAssign<i128>
    + core::ops::ShrAssign<isize>
    + core::convert::TryFrom<i8>
    + core::convert::TryFrom<u8>
    + core::convert::TryFrom<u16>
    + core::convert::TryFrom<u32>
    + core::convert::TryFrom<u64>
    + core::convert::TryFrom<u128>
    + core::convert::TryFrom<usize>
    + core::convert::TryFrom<i8>
    + core::convert::TryFrom<i16>
    + core::convert::TryFrom<i32>
    + core::convert::TryFrom<i64>
    + core::convert::TryFrom<i128>
    + core::convert::TryFrom<isize>
    + core::convert::TryInto<u8>
    + core::convert::TryInto<u16>
    + core::convert::TryInto<u32>
    + core::convert::TryInto<u64>
    + core::convert::TryInto<u128>
    + core::convert::TryInto<usize>
    + core::convert::TryInto<i8>
    + core::convert::TryInto<i16>
    + core::convert::TryInto<i32>
    + core::convert::TryInto<i64>
    + core::convert::TryInto<i128>
    + core::convert::TryInto<isize>
    + Send
    + Sync
    + Unpin
    + 'static
{
    const SIGNED: bool;
    const BITS: u32;
    const ZERO: Self;
    const ONE: Self;
    const MIN: Self;
    const MAX: Self;
    type Signed: tybit::Bit;
    type Width: width::Width;
}

impl Prim for u8 {
    const SIGNED: bool = false;
    const BITS: u32 = Self::BITS;
    const ZERO: Self = 0;
    const ONE: Self = 1;
    const MIN: Self = Self::MIN;
    const MAX: Self = Self::MAX;
    type Signed = tybit::_0;
    type Width = width::W<8>;
}

impl Prim for u16 {
    const SIGNED: bool = false;
    const BITS: u32 = Self::BITS;
    const ZERO: Self = 0;
    const ONE: Self = 1;
    const MIN: Self = Self::MIN;
    const MAX: Self = Self::MAX;
    type Signed = tybit::_0;
    type Width = width::W<16>;
}

impl Prim for u32 {
    const SIGNED: bool = false;
    const BITS: u32 = Self::BITS;
    const ZERO: Self = 0;
    const ONE: Self = 1;
    const MIN: Self = Self::MIN;
    const MAX: Self = Self::MAX;
    type Signed = tybit::_0;
    type Width = width::W<32>;
}

impl Prim for u64 {
    const SIGNED: bool = false;
    const BITS: u32 = Self::BITS;
    const ZERO: Self = 0;
    const ONE: Self = 1;
    const MIN: Self = Self::MIN;
    const MAX: Self = Self::MAX;
    type Signed = tybit::_0;
    type Width = width::W<64>;
}

impl Prim for u128 {
    const SIGNED: bool = false;
    const BITS: u32 = Self::BITS;
    const ZERO: Self = 0;
    const ONE: Self = 1;
    const MIN: Self = Self::MIN;
    const MAX: Self = Self::MAX;
    type Signed = tybit::_0;
    type Width = width::W<128>;
}

impl Prim for usize {
    const SIGNED: bool = false;
    const BITS: u32 = Self::BITS;
    const ZERO: Self = 0;
    const ONE: Self = 1;
    const MIN: Self = Self::MIN;
    const MAX: Self = Self::MAX;
    type Signed = tybit::_0;
    #[cfg(target_pointer_width = "8")]
    type Width = width::W<8>;
    #[cfg(target_pointer_width = "16")]
    type Width = width::W<16>;
    #[cfg(target_pointer_width = "32")]
    type Width = width::W<32>;
    #[cfg(target_pointer_width = "64")]
    type Width = width::W<64>;
    #[cfg(target_pointer_width = "128")]
    type Width = width::W<128>;
}

impl Prim for i8 {
    const SIGNED: bool = true;
    const BITS: u32 = Self::BITS;
    const ZERO: Self = 0;
    const ONE: Self = 1;
    const MIN: Self = Self::MIN;
    const MAX: Self = Self::MAX;
    type Signed = tybit::_1;
    type Width = width::W<8>;
}

impl Prim for i16 {
    const SIGNED: bool = true;
    const BITS: u32 = Self::BITS;
    const ZERO: Self = 0;
    const ONE: Self = 1;
    const MIN: Self = Self::MIN;
    const MAX: Self = Self::MAX;
    type Signed = tybit::_1;
    type Width = width::W<16>;
}

impl Prim for i32 {
    const SIGNED: bool = true;
    const BITS: u32 = Self::BITS;
    const ZERO: Self = 0;
    const ONE: Self = 1;
    const MIN: Self = Self::MIN;
    const MAX: Self = Self::MAX;
    type Signed = tybit::_1;
    type Width = width::W<32>;
}

impl Prim for i64 {
    const SIGNED: bool = true;
    const BITS: u32 = Self::BITS;
    const ZERO: Self = 0;
    const ONE: Self = 1;
    const MIN: Self = Self::MIN;
    const MAX: Self = Self::MAX;
    type Signed = tybit::_1;
    type Width = width::W<64>;
}

impl Prim for i128 {
    const SIGNED: bool = true;
    const BITS: u32 = Self::BITS;
    const ZERO: Self = 0;
    const ONE: Self = 1;
    const MIN: Self = Self::MIN;
    const MAX: Self = Self::MAX;
    type Signed = tybit::_1;
    type Width = width::W<128>;
}

impl Prim for isize {
    const SIGNED: bool = true;
    const BITS: u32 = Self::BITS;
    const ZERO: Self = 0;
    const ONE: Self = 1;
    const MIN: Self = Self::MIN;
    const MAX: Self = Self::MAX;
    type Signed = tybit::_1;
    #[cfg(target_pointer_width = "8")]
    type Width = width::W<8>;
    #[cfg(target_pointer_width = "16")]
    type Width = width::W<16>;
    #[cfg(target_pointer_width = "32")]
    type Width = width::W<32>;
    #[cfg(target_pointer_width = "64")]
    type Width = width::W<64>;
    #[cfg(target_pointer_width = "128")]
    type Width = width::W<128>;
}
