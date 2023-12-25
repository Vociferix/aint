use crate::prim::Prim;
use crate::tybit;
use crate::width;
use crate::Aint;

pub trait NonPrim:
    core::fmt::Debug
    + core::fmt::Display
    + core::fmt::Octal
    + core::fmt::Binary
    + core::fmt::UpperHex
    + core::fmt::LowerHex
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

macro_rules! impl_aint {
    ($repr:ident, $width:literal) => {
        impl NonPrim for Aint<$repr, $width> {
            const SIGNED: bool = <$repr as Prim>::SIGNED;
            const BITS: u32 = $width;
            const ZERO: Self = Self::new_wrapping(0);
            const ONE: Self = Self::new_wrapping(1);
            const MIN: Self = Self::MIN;
            const MAX: Self = Self::MAX;
            type Signed = <$repr as Prim>::Signed;
            type Width = width::W<$width>;
        }
    };
}

impl_aint!(u8, 1);
impl_aint!(u8, 2);
impl_aint!(u8, 3);
impl_aint!(u8, 4);
impl_aint!(u8, 5);
impl_aint!(u8, 6);
impl_aint!(u8, 7);

impl_aint!(u16, 9);
impl_aint!(u16, 10);
impl_aint!(u16, 11);
impl_aint!(u16, 12);
impl_aint!(u16, 13);
impl_aint!(u16, 14);
impl_aint!(u16, 15);

impl_aint!(u32, 17);
impl_aint!(u32, 18);
impl_aint!(u32, 19);
impl_aint!(u32, 20);
impl_aint!(u32, 21);
impl_aint!(u32, 22);
impl_aint!(u32, 23);
impl_aint!(u32, 24);
impl_aint!(u32, 25);
impl_aint!(u32, 26);
impl_aint!(u32, 27);
impl_aint!(u32, 28);
impl_aint!(u32, 29);
impl_aint!(u32, 30);
impl_aint!(u32, 31);

impl_aint!(u64, 33);
impl_aint!(u64, 34);
impl_aint!(u64, 35);
impl_aint!(u64, 36);
impl_aint!(u64, 37);
impl_aint!(u64, 38);
impl_aint!(u64, 39);
impl_aint!(u64, 40);
impl_aint!(u64, 41);
impl_aint!(u64, 42);
impl_aint!(u64, 43);
impl_aint!(u64, 44);
impl_aint!(u64, 45);
impl_aint!(u64, 46);
impl_aint!(u64, 47);
impl_aint!(u64, 48);
impl_aint!(u64, 49);
impl_aint!(u64, 50);
impl_aint!(u64, 51);
impl_aint!(u64, 52);
impl_aint!(u64, 53);
impl_aint!(u64, 54);
impl_aint!(u64, 55);
impl_aint!(u64, 56);
impl_aint!(u64, 57);
impl_aint!(u64, 58);
impl_aint!(u64, 59);
impl_aint!(u64, 60);
impl_aint!(u64, 61);
impl_aint!(u64, 62);
impl_aint!(u64, 63);

impl_aint!(u128, 65);
impl_aint!(u128, 66);
impl_aint!(u128, 67);
impl_aint!(u128, 68);
impl_aint!(u128, 69);
impl_aint!(u128, 70);
impl_aint!(u128, 71);
impl_aint!(u128, 72);
impl_aint!(u128, 73);
impl_aint!(u128, 74);
impl_aint!(u128, 75);
impl_aint!(u128, 76);
impl_aint!(u128, 77);
impl_aint!(u128, 78);
impl_aint!(u128, 79);
impl_aint!(u128, 80);
impl_aint!(u128, 81);
impl_aint!(u128, 82);
impl_aint!(u128, 83);
impl_aint!(u128, 84);
impl_aint!(u128, 85);
impl_aint!(u128, 86);
impl_aint!(u128, 87);
impl_aint!(u128, 88);
impl_aint!(u128, 89);
impl_aint!(u128, 90);
impl_aint!(u128, 91);
impl_aint!(u128, 92);
impl_aint!(u128, 93);
impl_aint!(u128, 94);
impl_aint!(u128, 95);
impl_aint!(u128, 96);
impl_aint!(u128, 97);
impl_aint!(u128, 98);
impl_aint!(u128, 99);
impl_aint!(u128, 100);
impl_aint!(u128, 101);
impl_aint!(u128, 102);
impl_aint!(u128, 103);
impl_aint!(u128, 104);
impl_aint!(u128, 105);
impl_aint!(u128, 106);
impl_aint!(u128, 107);
impl_aint!(u128, 108);
impl_aint!(u128, 109);
impl_aint!(u128, 110);
impl_aint!(u128, 111);
impl_aint!(u128, 112);
impl_aint!(u128, 113);
impl_aint!(u128, 114);
impl_aint!(u128, 115);
impl_aint!(u128, 116);
impl_aint!(u128, 117);
impl_aint!(u128, 118);
impl_aint!(u128, 119);
impl_aint!(u128, 120);
impl_aint!(u128, 121);
impl_aint!(u128, 122);
impl_aint!(u128, 123);
impl_aint!(u128, 124);
impl_aint!(u128, 125);
impl_aint!(u128, 126);
impl_aint!(u128, 127);

impl_aint!(i8, 1);
impl_aint!(i8, 2);
impl_aint!(i8, 3);
impl_aint!(i8, 4);
impl_aint!(i8, 5);
impl_aint!(i8, 6);
impl_aint!(i8, 7);

impl_aint!(i16, 9);
impl_aint!(i16, 10);
impl_aint!(i16, 11);
impl_aint!(i16, 12);
impl_aint!(i16, 13);
impl_aint!(i16, 14);
impl_aint!(i16, 15);

impl_aint!(i32, 17);
impl_aint!(i32, 18);
impl_aint!(i32, 19);
impl_aint!(i32, 20);
impl_aint!(i32, 21);
impl_aint!(i32, 22);
impl_aint!(i32, 23);
impl_aint!(i32, 24);
impl_aint!(i32, 25);
impl_aint!(i32, 26);
impl_aint!(i32, 27);
impl_aint!(i32, 28);
impl_aint!(i32, 29);
impl_aint!(i32, 30);
impl_aint!(i32, 31);

impl_aint!(i64, 33);
impl_aint!(i64, 34);
impl_aint!(i64, 35);
impl_aint!(i64, 36);
impl_aint!(i64, 37);
impl_aint!(i64, 38);
impl_aint!(i64, 39);
impl_aint!(i64, 40);
impl_aint!(i64, 41);
impl_aint!(i64, 42);
impl_aint!(i64, 43);
impl_aint!(i64, 44);
impl_aint!(i64, 45);
impl_aint!(i64, 46);
impl_aint!(i64, 47);
impl_aint!(i64, 48);
impl_aint!(i64, 49);
impl_aint!(i64, 50);
impl_aint!(i64, 51);
impl_aint!(i64, 52);
impl_aint!(i64, 53);
impl_aint!(i64, 54);
impl_aint!(i64, 55);
impl_aint!(i64, 56);
impl_aint!(i64, 57);
impl_aint!(i64, 58);
impl_aint!(i64, 59);
impl_aint!(i64, 60);
impl_aint!(i64, 61);
impl_aint!(i64, 62);
impl_aint!(i64, 63);

impl_aint!(i128, 65);
impl_aint!(i128, 66);
impl_aint!(i128, 67);
impl_aint!(i128, 68);
impl_aint!(i128, 69);
impl_aint!(i128, 70);
impl_aint!(i128, 71);
impl_aint!(i128, 72);
impl_aint!(i128, 73);
impl_aint!(i128, 74);
impl_aint!(i128, 75);
impl_aint!(i128, 76);
impl_aint!(i128, 77);
impl_aint!(i128, 78);
impl_aint!(i128, 79);
impl_aint!(i128, 80);
impl_aint!(i128, 81);
impl_aint!(i128, 82);
impl_aint!(i128, 83);
impl_aint!(i128, 84);
impl_aint!(i128, 85);
impl_aint!(i128, 86);
impl_aint!(i128, 87);
impl_aint!(i128, 88);
impl_aint!(i128, 89);
impl_aint!(i128, 90);
impl_aint!(i128, 91);
impl_aint!(i128, 92);
impl_aint!(i128, 93);
impl_aint!(i128, 94);
impl_aint!(i128, 95);
impl_aint!(i128, 96);
impl_aint!(i128, 97);
impl_aint!(i128, 98);
impl_aint!(i128, 99);
impl_aint!(i128, 100);
impl_aint!(i128, 101);
impl_aint!(i128, 102);
impl_aint!(i128, 103);
impl_aint!(i128, 104);
impl_aint!(i128, 105);
impl_aint!(i128, 106);
impl_aint!(i128, 107);
impl_aint!(i128, 108);
impl_aint!(i128, 109);
impl_aint!(i128, 110);
impl_aint!(i128, 111);
impl_aint!(i128, 112);
impl_aint!(i128, 113);
impl_aint!(i128, 114);
impl_aint!(i128, 115);
impl_aint!(i128, 116);
impl_aint!(i128, 117);
impl_aint!(i128, 118);
impl_aint!(i128, 119);
impl_aint!(i128, 120);
impl_aint!(i128, 121);
impl_aint!(i128, 122);
impl_aint!(i128, 123);
impl_aint!(i128, 124);
impl_aint!(i128, 125);
impl_aint!(i128, 126);
impl_aint!(i128, 127);
