use crate::sealed::Sealed;
use crate::Aint;

impl<R: Sealed, const WIDTH: u32> core::ops::Shl<u8> for Aint<R, WIDTH> {
    type Output = Self;

    fn shl(self, rhs: u8) -> Self {
        debug_assert!((rhs as u32) < WIDTH, "attempt to shift left with overflow");
        Self::_new_wrapping(self.0 << rhs)
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shl<&u8> for Aint<R, WIDTH> {
    type Output = Self;

    fn shl(self, rhs: &u8) -> Self {
        self << *rhs
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shl<u8> for &Aint<R, WIDTH> {
    type Output = Aint<R, WIDTH>;

    fn shl(self, rhs: u8) -> Self::Output {
        *self << rhs
    }
}

impl<'a, 'b, R: Sealed, const WIDTH: u32> core::ops::Shl<&'b u8> for &'a Aint<R, WIDTH> {
    type Output = Aint<R, WIDTH>;

    fn shl(self, rhs: &'b u8) -> Self::Output {
        *self << *rhs
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShlAssign<u8> for Aint<R, WIDTH> {
    fn shl_assign(&mut self, rhs: u8) {
        *self = *self << rhs;
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShlAssign<&u8> for Aint<R, WIDTH> {
    fn shl_assign(&mut self, rhs: &u8) {
        *self = *self << *rhs;
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shl<u16> for Aint<R, WIDTH> {
    type Output = Self;

    fn shl(self, rhs: u16) -> Self {
        debug_assert!((rhs as u32) < WIDTH, "attempt to shift left with overflow");
        Self::_new_wrapping(self.0 << rhs)
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shl<&u16> for Aint<R, WIDTH> {
    type Output = Self;

    fn shl(self, rhs: &u16) -> Self {
        self << *rhs
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shl<u16> for &Aint<R, WIDTH> {
    type Output = Aint<R, WIDTH>;

    fn shl(self, rhs: u16) -> Self::Output {
        *self << rhs
    }
}

impl<'a, 'b, R: Sealed, const WIDTH: u32> core::ops::Shl<&'b u16> for &'a Aint<R, WIDTH> {
    type Output = Aint<R, WIDTH>;

    fn shl(self, rhs: &'b u16) -> Self::Output {
        *self << *rhs
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShlAssign<u16> for Aint<R, WIDTH> {
    fn shl_assign(&mut self, rhs: u16) {
        *self = *self << rhs;
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShlAssign<&u16> for Aint<R, WIDTH> {
    fn shl_assign(&mut self, rhs: &u16) {
        *self = *self << *rhs;
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shl<u32> for Aint<R, WIDTH> {
    type Output = Self;

    fn shl(self, rhs: u32) -> Self {
        debug_assert!(rhs < WIDTH, "attempt to shift left with overflow");
        Self::_new_wrapping(self.0 << rhs)
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shl<&u32> for Aint<R, WIDTH> {
    type Output = Self;

    fn shl(self, rhs: &u32) -> Self {
        self << *rhs
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shl<u32> for &Aint<R, WIDTH> {
    type Output = Aint<R, WIDTH>;

    fn shl(self, rhs: u32) -> Self::Output {
        *self << rhs
    }
}

impl<'a, 'b, R: Sealed, const WIDTH: u32> core::ops::Shl<&'b u32> for &'a Aint<R, WIDTH> {
    type Output = Aint<R, WIDTH>;

    fn shl(self, rhs: &'b u32) -> Self::Output {
        *self << *rhs
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShlAssign<u32> for Aint<R, WIDTH> {
    fn shl_assign(&mut self, rhs: u32) {
        *self = *self << rhs;
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShlAssign<&u32> for Aint<R, WIDTH> {
    fn shl_assign(&mut self, rhs: &u32) {
        *self = *self << *rhs;
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shl<u64> for Aint<R, WIDTH> {
    type Output = Self;

    fn shl(self, rhs: u64) -> Self {
        debug_assert!(rhs < WIDTH as u64, "attempt to shift left with overflow");
        Self::_new_wrapping(self.0 << rhs)
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shl<&u64> for Aint<R, WIDTH> {
    type Output = Self;

    fn shl(self, rhs: &u64) -> Self {
        self << *rhs
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shl<u64> for &Aint<R, WIDTH> {
    type Output = Aint<R, WIDTH>;

    fn shl(self, rhs: u64) -> Self::Output {
        *self << rhs
    }
}

impl<'a, 'b, R: Sealed, const WIDTH: u32> core::ops::Shl<&'b u64> for &'a Aint<R, WIDTH> {
    type Output = Aint<R, WIDTH>;

    fn shl(self, rhs: &'b u64) -> Self::Output {
        *self << *rhs
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShlAssign<u64> for Aint<R, WIDTH> {
    fn shl_assign(&mut self, rhs: u64) {
        *self = *self << rhs;
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShlAssign<&u64> for Aint<R, WIDTH> {
    fn shl_assign(&mut self, rhs: &u64) {
        *self = *self << *rhs;
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shl<u128> for Aint<R, WIDTH> {
    type Output = Self;

    fn shl(self, rhs: u128) -> Self {
        debug_assert!(rhs < WIDTH as u128, "attempt to shift left with overflow");
        Self::_new_wrapping(self.0 << rhs)
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shl<&u128> for Aint<R, WIDTH> {
    type Output = Self;

    fn shl(self, rhs: &u128) -> Self {
        self << *rhs
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shl<u128> for &Aint<R, WIDTH> {
    type Output = Aint<R, WIDTH>;

    fn shl(self, rhs: u128) -> Self::Output {
        *self << rhs
    }
}

impl<'a, 'b, R: Sealed, const WIDTH: u32> core::ops::Shl<&'b u128> for &'a Aint<R, WIDTH> {
    type Output = Aint<R, WIDTH>;

    fn shl(self, rhs: &'b u128) -> Self::Output {
        *self << *rhs
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShlAssign<u128> for Aint<R, WIDTH> {
    fn shl_assign(&mut self, rhs: u128) {
        *self = *self << rhs;
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShlAssign<&u128> for Aint<R, WIDTH> {
    fn shl_assign(&mut self, rhs: &u128) {
        *self = *self << *rhs;
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shl<usize> for Aint<R, WIDTH> {
    type Output = Self;

    fn shl(self, rhs: usize) -> Self {
        debug_assert!(rhs < WIDTH as usize, "attempt to shift left with overflow");
        Self::_new_wrapping(self.0 << rhs)
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shl<&usize> for Aint<R, WIDTH> {
    type Output = Self;

    fn shl(self, rhs: &usize) -> Self {
        self << *rhs
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shl<usize> for &Aint<R, WIDTH> {
    type Output = Aint<R, WIDTH>;

    fn shl(self, rhs: usize) -> Self::Output {
        *self << rhs
    }
}

impl<'a, 'b, R: Sealed, const WIDTH: u32> core::ops::Shl<&'b usize> for &'a Aint<R, WIDTH> {
    type Output = Aint<R, WIDTH>;

    fn shl(self, rhs: &'b usize) -> Self::Output {
        *self << *rhs
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShlAssign<usize> for Aint<R, WIDTH> {
    fn shl_assign(&mut self, rhs: usize) {
        *self = *self << rhs;
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShlAssign<&usize> for Aint<R, WIDTH> {
    fn shl_assign(&mut self, rhs: &usize) {
        *self = *self << *rhs;
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shl<i8> for Aint<R, WIDTH> {
    type Output = Self;

    fn shl(self, rhs: i8) -> Self {
        debug_assert!(
            (rhs as i64) < (WIDTH as i64) && (rhs as i64) > -(WIDTH as i64),
            "attempt to shift left with overflow"
        );
        Self::_new_wrapping(self.0 << rhs)
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shl<&i8> for Aint<R, WIDTH> {
    type Output = Self;

    fn shl(self, rhs: &i8) -> Self {
        self << *rhs
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shl<i8> for &Aint<R, WIDTH> {
    type Output = Aint<R, WIDTH>;

    fn shl(self, rhs: i8) -> Self::Output {
        *self << rhs
    }
}

impl<'a, 'b, R: Sealed, const WIDTH: u32> core::ops::Shl<&'b i8> for &'a Aint<R, WIDTH> {
    type Output = Aint<R, WIDTH>;

    fn shl(self, rhs: &'b i8) -> Self::Output {
        *self << *rhs
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShlAssign<i8> for Aint<R, WIDTH> {
    fn shl_assign(&mut self, rhs: i8) {
        *self = *self << rhs;
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShlAssign<&i8> for Aint<R, WIDTH> {
    fn shl_assign(&mut self, rhs: &i8) {
        *self = *self << *rhs;
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shl<i16> for Aint<R, WIDTH> {
    type Output = Self;

    fn shl(self, rhs: i16) -> Self {
        debug_assert!(
            (rhs as i64) < (WIDTH as i64) && (rhs as i64) > -(WIDTH as i64),
            "attempt to shift left with overflow"
        );
        Self::_new_wrapping(self.0 << rhs)
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shl<&i16> for Aint<R, WIDTH> {
    type Output = Self;

    fn shl(self, rhs: &i16) -> Self {
        self << *rhs
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shl<i16> for &Aint<R, WIDTH> {
    type Output = Aint<R, WIDTH>;

    fn shl(self, rhs: i16) -> Self::Output {
        *self << rhs
    }
}

impl<'a, 'b, R: Sealed, const WIDTH: u32> core::ops::Shl<&'b i16> for &'a Aint<R, WIDTH> {
    type Output = Aint<R, WIDTH>;

    fn shl(self, rhs: &'b i16) -> Self::Output {
        *self << *rhs
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShlAssign<i16> for Aint<R, WIDTH> {
    fn shl_assign(&mut self, rhs: i16) {
        *self = *self << rhs;
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShlAssign<&i16> for Aint<R, WIDTH> {
    fn shl_assign(&mut self, rhs: &i16) {
        *self = *self << *rhs;
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shl<i32> for Aint<R, WIDTH> {
    type Output = Self;

    fn shl(self, rhs: i32) -> Self {
        debug_assert!(
            (rhs as i64) < (WIDTH as i64) && (rhs as i64) > -(WIDTH as i64),
            "attempt to shift left with overflow"
        );
        Self::_new_wrapping(self.0 << rhs)
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shl<&i32> for Aint<R, WIDTH> {
    type Output = Self;

    fn shl(self, rhs: &i32) -> Self {
        self << *rhs
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shl<i32> for &Aint<R, WIDTH> {
    type Output = Aint<R, WIDTH>;

    fn shl(self, rhs: i32) -> Self::Output {
        *self << rhs
    }
}

impl<'a, 'b, R: Sealed, const WIDTH: u32> core::ops::Shl<&'b i32> for &'a Aint<R, WIDTH> {
    type Output = Aint<R, WIDTH>;

    fn shl(self, rhs: &'b i32) -> Self::Output {
        *self << *rhs
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShlAssign<i32> for Aint<R, WIDTH> {
    fn shl_assign(&mut self, rhs: i32) {
        *self = *self << rhs;
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShlAssign<&i32> for Aint<R, WIDTH> {
    fn shl_assign(&mut self, rhs: &i32) {
        *self = *self << *rhs;
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shl<i64> for Aint<R, WIDTH> {
    type Output = Self;

    fn shl(self, rhs: i64) -> Self {
        debug_assert!(
            rhs < WIDTH as i64 && rhs > -(WIDTH as i64),
            "attempt to shift left with overflow"
        );
        Self::_new_wrapping(self.0 << rhs)
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shl<&i64> for Aint<R, WIDTH> {
    type Output = Self;

    fn shl(self, rhs: &i64) -> Self {
        self << *rhs
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shl<i64> for &Aint<R, WIDTH> {
    type Output = Aint<R, WIDTH>;

    fn shl(self, rhs: i64) -> Self::Output {
        *self << rhs
    }
}

impl<'a, 'b, R: Sealed, const WIDTH: u32> core::ops::Shl<&'b i64> for &'a Aint<R, WIDTH> {
    type Output = Aint<R, WIDTH>;

    fn shl(self, rhs: &'b i64) -> Self::Output {
        *self << *rhs
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShlAssign<i64> for Aint<R, WIDTH> {
    fn shl_assign(&mut self, rhs: i64) {
        *self = *self << rhs;
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShlAssign<&i64> for Aint<R, WIDTH> {
    fn shl_assign(&mut self, rhs: &i64) {
        *self = *self << *rhs;
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shl<i128> for Aint<R, WIDTH> {
    type Output = Self;

    fn shl(self, rhs: i128) -> Self {
        debug_assert!(
            rhs < WIDTH as i128 && rhs > -(WIDTH as i128),
            "attempt to shift left with overflow"
        );
        Self::_new_wrapping(self.0 << rhs)
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shl<&i128> for Aint<R, WIDTH> {
    type Output = Self;

    fn shl(self, rhs: &i128) -> Self {
        self << *rhs
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shl<i128> for &Aint<R, WIDTH> {
    type Output = Aint<R, WIDTH>;

    fn shl(self, rhs: i128) -> Self::Output {
        *self << rhs
    }
}

impl<'a, 'b, R: Sealed, const WIDTH: u32> core::ops::Shl<&'b i128> for &'a Aint<R, WIDTH> {
    type Output = Aint<R, WIDTH>;

    fn shl(self, rhs: &'b i128) -> Self::Output {
        *self << *rhs
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShlAssign<i128> for Aint<R, WIDTH> {
    fn shl_assign(&mut self, rhs: i128) {
        *self = *self << rhs;
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShlAssign<&i128> for Aint<R, WIDTH> {
    fn shl_assign(&mut self, rhs: &i128) {
        *self = *self << *rhs;
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shl<isize> for Aint<R, WIDTH> {
    type Output = Self;

    fn shl(self, rhs: isize) -> Self {
        debug_assert!(
            rhs < WIDTH as isize && rhs > -(WIDTH as isize),
            "attempt to shift left with overflow"
        );
        Self::_new_wrapping(self.0 << rhs)
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shl<&isize> for Aint<R, WIDTH> {
    type Output = Self;

    fn shl(self, rhs: &isize) -> Self {
        self << *rhs
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shl<isize> for &Aint<R, WIDTH> {
    type Output = Aint<R, WIDTH>;

    fn shl(self, rhs: isize) -> Self::Output {
        *self << rhs
    }
}

impl<'a, 'b, R: Sealed, const WIDTH: u32> core::ops::Shl<&'b isize> for &'a Aint<R, WIDTH> {
    type Output = Aint<R, WIDTH>;

    fn shl(self, rhs: &'b isize) -> Self::Output {
        *self << *rhs
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShlAssign<isize> for Aint<R, WIDTH> {
    fn shl_assign(&mut self, rhs: isize) {
        *self = *self << rhs;
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShlAssign<&isize> for Aint<R, WIDTH> {
    fn shl_assign(&mut self, rhs: &isize) {
        *self = *self << *rhs;
    }
}

impl<R1: Sealed, R2: Sealed, const WIDTH1: u32, const WIDTH2: u32> core::ops::Shl<Aint<R2, WIDTH2>>
    for Aint<R1, WIDTH1>
{
    type Output = Self;

    fn shl(self, rhs: Aint<R2, WIDTH2>) -> Self {
        let rhs: i32 = match rhs.0.try_into() {
            Ok(rhs) => rhs,
            Err(_) => i32::MAX,
        };
        self << rhs
    }
}

impl<R1: Sealed, R2: Sealed, const WIDTH1: u32, const WIDTH2: u32> core::ops::Shl<Aint<R2, WIDTH2>>
    for &Aint<R1, WIDTH1>
{
    type Output = Aint<R1, WIDTH1>;

    fn shl(self, rhs: Aint<R2, WIDTH2>) -> Self::Output {
        *self << rhs
    }
}

impl<R1: Sealed, R2: Sealed, const WIDTH1: u32, const WIDTH2: u32> core::ops::Shl<&Aint<R2, WIDTH2>>
    for Aint<R1, WIDTH1>
{
    type Output = Self;

    fn shl(self, rhs: &Aint<R2, WIDTH2>) -> Self {
        self << *rhs
    }
}

impl<'a, 'b, R1: Sealed, R2: Sealed, const WIDTH1: u32, const WIDTH2: u32>
    core::ops::Shl<&'b Aint<R2, WIDTH2>> for &'a Aint<R1, WIDTH1>
{
    type Output = Aint<R1, WIDTH1>;

    fn shl(self, rhs: &'b Aint<R2, WIDTH2>) -> Self::Output {
        *self << *rhs
    }
}

impl<R1: Sealed, R2: Sealed, const WIDTH1: u32, const WIDTH2: u32>
    core::ops::ShlAssign<Aint<R2, WIDTH2>> for Aint<R1, WIDTH1>
{
    fn shl_assign(&mut self, rhs: Aint<R2, WIDTH2>) {
        *self = *self << rhs;
    }
}

impl<R1: Sealed, R2: Sealed, const WIDTH1: u32, const WIDTH2: u32>
    core::ops::ShlAssign<&Aint<R2, WIDTH2>> for Aint<R1, WIDTH1>
{
    fn shl_assign(&mut self, rhs: &Aint<R2, WIDTH2>) {
        *self = *self << *rhs;
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shr<u8> for Aint<R, WIDTH> {
    type Output = Self;

    fn shr(self, rhs: u8) -> Self {
        debug_assert!((rhs as u32) < WIDTH, "attempt to shift right with overflow");
        Self::_new_wrapping(self.0 >> rhs)
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shr<&u8> for Aint<R, WIDTH> {
    type Output = Self;

    fn shr(self, rhs: &u8) -> Self {
        self >> *rhs
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shr<u8> for &Aint<R, WIDTH> {
    type Output = Aint<R, WIDTH>;

    fn shr(self, rhs: u8) -> Self::Output {
        *self >> rhs
    }
}

impl<'a, 'b, R: Sealed, const WIDTH: u32> core::ops::Shr<&'b u8> for &'a Aint<R, WIDTH> {
    type Output = Aint<R, WIDTH>;

    fn shr(self, rhs: &'b u8) -> Self::Output {
        *self >> *rhs
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShrAssign<u8> for Aint<R, WIDTH> {
    fn shr_assign(&mut self, rhs: u8) {
        *self = *self >> rhs;
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShrAssign<&u8> for Aint<R, WIDTH> {
    fn shr_assign(&mut self, rhs: &u8) {
        *self = *self >> *rhs;
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shr<u16> for Aint<R, WIDTH> {
    type Output = Self;

    fn shr(self, rhs: u16) -> Self {
        debug_assert!((rhs as u32) < WIDTH, "attempt to shift right with overflow");
        Self::_new_wrapping(self.0 >> rhs)
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shr<&u16> for Aint<R, WIDTH> {
    type Output = Self;

    fn shr(self, rhs: &u16) -> Self {
        self >> *rhs
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shr<u16> for &Aint<R, WIDTH> {
    type Output = Aint<R, WIDTH>;

    fn shr(self, rhs: u16) -> Self::Output {
        *self >> rhs
    }
}

impl<'a, 'b, R: Sealed, const WIDTH: u32> core::ops::Shr<&'b u16> for &'a Aint<R, WIDTH> {
    type Output = Aint<R, WIDTH>;

    fn shr(self, rhs: &'b u16) -> Self::Output {
        *self >> *rhs
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShrAssign<u16> for Aint<R, WIDTH> {
    fn shr_assign(&mut self, rhs: u16) {
        *self = *self >> rhs;
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShrAssign<&u16> for Aint<R, WIDTH> {
    fn shr_assign(&mut self, rhs: &u16) {
        *self = *self >> *rhs;
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shr<u32> for Aint<R, WIDTH> {
    type Output = Self;

    fn shr(self, rhs: u32) -> Self {
        debug_assert!(rhs < WIDTH, "attempt to shift right with overflow");
        Self::_new_wrapping(self.0 >> rhs)
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shr<&u32> for Aint<R, WIDTH> {
    type Output = Self;

    fn shr(self, rhs: &u32) -> Self {
        self >> *rhs
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shr<u32> for &Aint<R, WIDTH> {
    type Output = Aint<R, WIDTH>;

    fn shr(self, rhs: u32) -> Self::Output {
        *self >> rhs
    }
}

impl<'a, 'b, R: Sealed, const WIDTH: u32> core::ops::Shr<&'b u32> for &'a Aint<R, WIDTH> {
    type Output = Aint<R, WIDTH>;

    fn shr(self, rhs: &'b u32) -> Self::Output {
        *self >> *rhs
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShrAssign<u32> for Aint<R, WIDTH> {
    fn shr_assign(&mut self, rhs: u32) {
        *self = *self >> rhs;
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShrAssign<&u32> for Aint<R, WIDTH> {
    fn shr_assign(&mut self, rhs: &u32) {
        *self = *self >> *rhs;
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shr<u64> for Aint<R, WIDTH> {
    type Output = Self;

    fn shr(self, rhs: u64) -> Self {
        debug_assert!(rhs < WIDTH as u64, "attempt to shift right with overflow");
        Self::_new_wrapping(self.0 >> rhs)
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shr<&u64> for Aint<R, WIDTH> {
    type Output = Self;

    fn shr(self, rhs: &u64) -> Self {
        self >> *rhs
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shr<u64> for &Aint<R, WIDTH> {
    type Output = Aint<R, WIDTH>;

    fn shr(self, rhs: u64) -> Self::Output {
        *self >> rhs
    }
}

impl<'a, 'b, R: Sealed, const WIDTH: u32> core::ops::Shr<&'b u64> for &'a Aint<R, WIDTH> {
    type Output = Aint<R, WIDTH>;

    fn shr(self, rhs: &'b u64) -> Self::Output {
        *self >> *rhs
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShrAssign<u64> for Aint<R, WIDTH> {
    fn shr_assign(&mut self, rhs: u64) {
        *self = *self >> rhs;
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShrAssign<&u64> for Aint<R, WIDTH> {
    fn shr_assign(&mut self, rhs: &u64) {
        *self = *self >> *rhs;
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shr<u128> for Aint<R, WIDTH> {
    type Output = Self;

    fn shr(self, rhs: u128) -> Self {
        debug_assert!(rhs < WIDTH as u128, "attempt to shift right with overflow");
        Self::_new_wrapping(self.0 >> rhs)
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shr<&u128> for Aint<R, WIDTH> {
    type Output = Self;

    fn shr(self, rhs: &u128) -> Self {
        self >> *rhs
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shr<u128> for &Aint<R, WIDTH> {
    type Output = Aint<R, WIDTH>;

    fn shr(self, rhs: u128) -> Self::Output {
        *self >> rhs
    }
}

impl<'a, 'b, R: Sealed, const WIDTH: u32> core::ops::Shr<&'b u128> for &'a Aint<R, WIDTH> {
    type Output = Aint<R, WIDTH>;

    fn shr(self, rhs: &'b u128) -> Self::Output {
        *self >> *rhs
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShrAssign<u128> for Aint<R, WIDTH> {
    fn shr_assign(&mut self, rhs: u128) {
        *self = *self >> rhs;
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShrAssign<&u128> for Aint<R, WIDTH> {
    fn shr_assign(&mut self, rhs: &u128) {
        *self = *self >> *rhs;
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shr<usize> for Aint<R, WIDTH> {
    type Output = Self;

    fn shr(self, rhs: usize) -> Self {
        debug_assert!(rhs < WIDTH as usize, "attempt to shift right with overflow");
        Self::_new_wrapping(self.0 >> rhs)
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shr<&usize> for Aint<R, WIDTH> {
    type Output = Self;

    fn shr(self, rhs: &usize) -> Self {
        self >> *rhs
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shr<usize> for &Aint<R, WIDTH> {
    type Output = Aint<R, WIDTH>;

    fn shr(self, rhs: usize) -> Self::Output {
        *self >> rhs
    }
}

impl<'a, 'b, R: Sealed, const WIDTH: u32> core::ops::Shr<&'b usize> for &'a Aint<R, WIDTH> {
    type Output = Aint<R, WIDTH>;

    fn shr(self, rhs: &'b usize) -> Self::Output {
        *self >> *rhs
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShrAssign<usize> for Aint<R, WIDTH> {
    fn shr_assign(&mut self, rhs: usize) {
        *self = *self >> rhs;
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShrAssign<&usize> for Aint<R, WIDTH> {
    fn shr_assign(&mut self, rhs: &usize) {
        *self = *self >> *rhs;
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shr<i8> for Aint<R, WIDTH> {
    type Output = Self;

    fn shr(self, rhs: i8) -> Self {
        debug_assert!(
            (rhs as i64) < (WIDTH as i64) && (rhs as i64) > -(WIDTH as i64),
            "attempt to shift right with overflow"
        );
        Self::_new_wrapping(self.0 >> rhs)
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shr<&i8> for Aint<R, WIDTH> {
    type Output = Self;

    fn shr(self, rhs: &i8) -> Self {
        self >> *rhs
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shr<i8> for &Aint<R, WIDTH> {
    type Output = Aint<R, WIDTH>;

    fn shr(self, rhs: i8) -> Self::Output {
        *self >> rhs
    }
}

impl<'a, 'b, R: Sealed, const WIDTH: u32> core::ops::Shr<&'b i8> for &'a Aint<R, WIDTH> {
    type Output = Aint<R, WIDTH>;

    fn shr(self, rhs: &'b i8) -> Self::Output {
        *self >> *rhs
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShrAssign<i8> for Aint<R, WIDTH> {
    fn shr_assign(&mut self, rhs: i8) {
        *self = *self >> rhs;
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShrAssign<&i8> for Aint<R, WIDTH> {
    fn shr_assign(&mut self, rhs: &i8) {
        *self = *self >> *rhs;
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shr<i16> for Aint<R, WIDTH> {
    type Output = Self;

    fn shr(self, rhs: i16) -> Self {
        debug_assert!(
            (rhs as i64) < (WIDTH as i64) && (rhs as i64) > -(WIDTH as i64),
            "attempt to shift right with overflow"
        );
        Self::_new_wrapping(self.0 >> rhs)
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shr<&i16> for Aint<R, WIDTH> {
    type Output = Self;

    fn shr(self, rhs: &i16) -> Self {
        self >> *rhs
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shr<i16> for &Aint<R, WIDTH> {
    type Output = Aint<R, WIDTH>;

    fn shr(self, rhs: i16) -> Self::Output {
        *self >> rhs
    }
}

impl<'a, 'b, R: Sealed, const WIDTH: u32> core::ops::Shr<&'b i16> for &'a Aint<R, WIDTH> {
    type Output = Aint<R, WIDTH>;

    fn shr(self, rhs: &'b i16) -> Self::Output {
        *self >> *rhs
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShrAssign<i16> for Aint<R, WIDTH> {
    fn shr_assign(&mut self, rhs: i16) {
        *self = *self >> rhs;
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShrAssign<&i16> for Aint<R, WIDTH> {
    fn shr_assign(&mut self, rhs: &i16) {
        *self = *self >> *rhs;
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shr<i32> for Aint<R, WIDTH> {
    type Output = Self;

    fn shr(self, rhs: i32) -> Self {
        debug_assert!(
            (rhs as i64) < (WIDTH as i64) && (rhs as i64) > -(WIDTH as i64),
            "attempt to shift right with overflow"
        );
        Self::_new_wrapping(self.0 >> rhs)
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shr<&i32> for Aint<R, WIDTH> {
    type Output = Self;

    fn shr(self, rhs: &i32) -> Self {
        self >> *rhs
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shr<i32> for &Aint<R, WIDTH> {
    type Output = Aint<R, WIDTH>;

    fn shr(self, rhs: i32) -> Self::Output {
        *self >> rhs
    }
}

impl<'a, 'b, R: Sealed, const WIDTH: u32> core::ops::Shr<&'b i32> for &'a Aint<R, WIDTH> {
    type Output = Aint<R, WIDTH>;

    fn shr(self, rhs: &'b i32) -> Self::Output {
        *self >> *rhs
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShrAssign<i32> for Aint<R, WIDTH> {
    fn shr_assign(&mut self, rhs: i32) {
        *self = *self >> rhs;
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShrAssign<&i32> for Aint<R, WIDTH> {
    fn shr_assign(&mut self, rhs: &i32) {
        *self = *self >> *rhs;
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shr<i64> for Aint<R, WIDTH> {
    type Output = Self;

    fn shr(self, rhs: i64) -> Self {
        debug_assert!(
            rhs < WIDTH as i64 && rhs > -(WIDTH as i64),
            "attempt to shift right with overflow"
        );
        Self::_new_wrapping(self.0 >> rhs)
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shr<&i64> for Aint<R, WIDTH> {
    type Output = Self;

    fn shr(self, rhs: &i64) -> Self {
        self >> *rhs
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shr<i64> for &Aint<R, WIDTH> {
    type Output = Aint<R, WIDTH>;

    fn shr(self, rhs: i64) -> Self::Output {
        *self >> rhs
    }
}

impl<'a, 'b, R: Sealed, const WIDTH: u32> core::ops::Shr<&'b i64> for &'a Aint<R, WIDTH> {
    type Output = Aint<R, WIDTH>;

    fn shr(self, rhs: &'b i64) -> Self::Output {
        *self >> *rhs
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShrAssign<i64> for Aint<R, WIDTH> {
    fn shr_assign(&mut self, rhs: i64) {
        *self = *self >> rhs;
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShrAssign<&i64> for Aint<R, WIDTH> {
    fn shr_assign(&mut self, rhs: &i64) {
        *self = *self >> *rhs;
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shr<i128> for Aint<R, WIDTH> {
    type Output = Self;

    fn shr(self, rhs: i128) -> Self {
        debug_assert!(
            rhs < WIDTH as i128 && rhs > -(WIDTH as i128),
            "attempt to shift right with overflow"
        );
        Self::_new_wrapping(self.0 >> rhs)
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shr<&i128> for Aint<R, WIDTH> {
    type Output = Self;

    fn shr(self, rhs: &i128) -> Self {
        self >> *rhs
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shr<i128> for &Aint<R, WIDTH> {
    type Output = Aint<R, WIDTH>;

    fn shr(self, rhs: i128) -> Self::Output {
        *self >> rhs
    }
}

impl<'a, 'b, R: Sealed, const WIDTH: u32> core::ops::Shr<&'b i128> for &'a Aint<R, WIDTH> {
    type Output = Aint<R, WIDTH>;

    fn shr(self, rhs: &'b i128) -> Self::Output {
        *self >> *rhs
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShrAssign<i128> for Aint<R, WIDTH> {
    fn shr_assign(&mut self, rhs: i128) {
        *self = *self >> rhs;
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShrAssign<&i128> for Aint<R, WIDTH> {
    fn shr_assign(&mut self, rhs: &i128) {
        *self = *self >> *rhs;
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shr<isize> for Aint<R, WIDTH> {
    type Output = Self;

    fn shr(self, rhs: isize) -> Self {
        debug_assert!(
            rhs < WIDTH as isize && rhs > -(WIDTH as isize),
            "attempt to shift right with overflow"
        );
        Self::_new_wrapping(self.0 >> rhs)
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shr<&isize> for Aint<R, WIDTH> {
    type Output = Self;

    fn shr(self, rhs: &isize) -> Self {
        self >> *rhs
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shr<isize> for &Aint<R, WIDTH> {
    type Output = Aint<R, WIDTH>;

    fn shr(self, rhs: isize) -> Self::Output {
        *self >> rhs
    }
}

impl<'a, 'b, R: Sealed, const WIDTH: u32> core::ops::Shr<&'b isize> for &'a Aint<R, WIDTH> {
    type Output = Aint<R, WIDTH>;

    fn shr(self, rhs: &'b isize) -> Self::Output {
        *self >> *rhs
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShrAssign<isize> for Aint<R, WIDTH> {
    fn shr_assign(&mut self, rhs: isize) {
        *self = *self >> rhs;
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShrAssign<&isize> for Aint<R, WIDTH> {
    fn shr_assign(&mut self, rhs: &isize) {
        *self = *self >> *rhs;
    }
}

impl<R1: Sealed, R2: Sealed, const WIDTH1: u32, const WIDTH2: u32> core::ops::Shr<Aint<R2, WIDTH2>>
    for Aint<R1, WIDTH1>
{
    type Output = Self;

    fn shr(self, rhs: Aint<R2, WIDTH2>) -> Self {
        let rhs: i32 = match rhs.0.try_into() {
            Ok(rhs) => rhs,
            Err(_) => i32::MAX,
        };
        self >> rhs
    }
}

impl<R1: Sealed, R2: Sealed, const WIDTH1: u32, const WIDTH2: u32> core::ops::Shr<Aint<R2, WIDTH2>>
    for &Aint<R1, WIDTH1>
{
    type Output = Aint<R1, WIDTH1>;

    fn shr(self, rhs: Aint<R2, WIDTH2>) -> Self::Output {
        *self >> rhs
    }
}

impl<R1: Sealed, R2: Sealed, const WIDTH1: u32, const WIDTH2: u32> core::ops::Shr<&Aint<R2, WIDTH2>>
    for Aint<R1, WIDTH1>
{
    type Output = Self;

    fn shr(self, rhs: &Aint<R2, WIDTH2>) -> Self {
        self >> *rhs
    }
}

impl<'a, 'b, R1: Sealed, R2: Sealed, const WIDTH1: u32, const WIDTH2: u32>
    core::ops::Shr<&'b Aint<R2, WIDTH2>> for &'a Aint<R1, WIDTH1>
{
    type Output = Aint<R1, WIDTH1>;

    fn shr(self, rhs: &'b Aint<R2, WIDTH2>) -> Self::Output {
        *self >> *rhs
    }
}

impl<R1: Sealed, R2: Sealed, const WIDTH1: u32, const WIDTH2: u32>
    core::ops::ShrAssign<Aint<R2, WIDTH2>> for Aint<R1, WIDTH1>
{
    fn shr_assign(&mut self, rhs: Aint<R2, WIDTH2>) {
        *self = *self >> rhs;
    }
}

impl<R1: Sealed, R2: Sealed, const WIDTH1: u32, const WIDTH2: u32>
    core::ops::ShrAssign<&Aint<R2, WIDTH2>> for Aint<R1, WIDTH1>
{
    fn shr_assign(&mut self, rhs: &Aint<R2, WIDTH2>) {
        *self = *self >> *rhs;
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shl<Aint<R, WIDTH>> for u8
where
    u8: core::ops::Shl<R, Output = u8>,
{
    type Output = Self;

    fn shl(self, rhs: Aint<R, WIDTH>) -> Self {
        self << rhs.0
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shl<Aint<R, WIDTH>> for &u8
where
    u8: core::ops::Shl<R, Output = u8>,
{
    type Output = u8;

    fn shl(self, rhs: Aint<R, WIDTH>) -> u8 {
        *self << rhs.0
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shl<&Aint<R, WIDTH>> for u8
where
    u8: core::ops::Shl<R, Output = u8>,
{
    type Output = Self;

    fn shl(self, rhs: &Aint<R, WIDTH>) -> Self {
        self << rhs.0
    }
}

impl<'a, 'b, R: Sealed, const WIDTH: u32> core::ops::Shl<&'b Aint<R, WIDTH>> for &'a u8
where
    u8: core::ops::Shl<R, Output = u8>,
{
    type Output = u8;

    fn shl(self, rhs: &'b Aint<R, WIDTH>) -> u8 {
        *self << rhs.0
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShlAssign<Aint<R, WIDTH>> for u8
where
    u8: core::ops::ShlAssign<R>,
{
    fn shl_assign(&mut self, rhs: Aint<R, WIDTH>) {
        self.shl_assign(rhs.0)
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShlAssign<&Aint<R, WIDTH>> for u8
where
    u8: core::ops::ShlAssign<R>,
{
    fn shl_assign(&mut self, rhs: &Aint<R, WIDTH>) {
        self.shl_assign(rhs.0)
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shl<Aint<R, WIDTH>> for u16
where
    u16: core::ops::Shl<R, Output = u16>,
{
    type Output = Self;

    fn shl(self, rhs: Aint<R, WIDTH>) -> Self {
        self << rhs.0
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shl<Aint<R, WIDTH>> for &u16
where
    u16: core::ops::Shl<R, Output = u16>,
{
    type Output = u16;

    fn shl(self, rhs: Aint<R, WIDTH>) -> u16 {
        *self << rhs.0
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shl<&Aint<R, WIDTH>> for u16
where
    u16: core::ops::Shl<R, Output = u16>,
{
    type Output = Self;

    fn shl(self, rhs: &Aint<R, WIDTH>) -> Self {
        self << rhs.0
    }
}

impl<'a, 'b, R: Sealed, const WIDTH: u32> core::ops::Shl<&'b Aint<R, WIDTH>> for &'a u16
where
    u16: core::ops::Shl<R, Output = u16>,
{
    type Output = u16;

    fn shl(self, rhs: &'b Aint<R, WIDTH>) -> u16 {
        *self << rhs.0
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShlAssign<Aint<R, WIDTH>> for u16
where
    u16: core::ops::ShlAssign<R>,
{
    fn shl_assign(&mut self, rhs: Aint<R, WIDTH>) {
        self.shl_assign(rhs.0)
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShlAssign<&Aint<R, WIDTH>> for u16
where
    u16: core::ops::ShlAssign<R>,
{
    fn shl_assign(&mut self, rhs: &Aint<R, WIDTH>) {
        self.shl_assign(rhs.0)
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shl<Aint<R, WIDTH>> for u32
where
    u32: core::ops::Shl<R, Output = u32>,
{
    type Output = Self;

    fn shl(self, rhs: Aint<R, WIDTH>) -> Self {
        self << rhs.0
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shl<Aint<R, WIDTH>> for &u32
where
    u32: core::ops::Shl<R, Output = u32>,
{
    type Output = u32;

    fn shl(self, rhs: Aint<R, WIDTH>) -> u32 {
        *self << rhs.0
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shl<&Aint<R, WIDTH>> for u32
where
    u32: core::ops::Shl<R, Output = u32>,
{
    type Output = Self;

    fn shl(self, rhs: &Aint<R, WIDTH>) -> Self {
        self << rhs.0
    }
}

impl<'a, 'b, R: Sealed, const WIDTH: u32> core::ops::Shl<&'b Aint<R, WIDTH>> for &'a u32
where
    u32: core::ops::Shl<R, Output = u32>,
{
    type Output = u32;

    fn shl(self, rhs: &'b Aint<R, WIDTH>) -> u32 {
        *self << rhs.0
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShlAssign<Aint<R, WIDTH>> for u32
where
    u32: core::ops::ShlAssign<R>,
{
    fn shl_assign(&mut self, rhs: Aint<R, WIDTH>) {
        self.shl_assign(rhs.0)
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShlAssign<&Aint<R, WIDTH>> for u32
where
    u32: core::ops::ShlAssign<R>,
{
    fn shl_assign(&mut self, rhs: &Aint<R, WIDTH>) {
        self.shl_assign(rhs.0)
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shl<Aint<R, WIDTH>> for u64
where
    u64: core::ops::Shl<R, Output = u64>,
{
    type Output = Self;

    fn shl(self, rhs: Aint<R, WIDTH>) -> Self {
        self << rhs.0
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shl<Aint<R, WIDTH>> for &u64
where
    u64: core::ops::Shl<R, Output = u64>,
{
    type Output = u64;

    fn shl(self, rhs: Aint<R, WIDTH>) -> u64 {
        *self << rhs.0
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shl<&Aint<R, WIDTH>> for u64
where
    u64: core::ops::Shl<R, Output = u64>,
{
    type Output = Self;

    fn shl(self, rhs: &Aint<R, WIDTH>) -> Self {
        self << rhs.0
    }
}

impl<'a, 'b, R: Sealed, const WIDTH: u32> core::ops::Shl<&'b Aint<R, WIDTH>> for &'a u64
where
    u64: core::ops::Shl<R, Output = u64>,
{
    type Output = u64;

    fn shl(self, rhs: &'b Aint<R, WIDTH>) -> u64 {
        *self << rhs.0
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShlAssign<Aint<R, WIDTH>> for u64
where
    u64: core::ops::ShlAssign<R>,
{
    fn shl_assign(&mut self, rhs: Aint<R, WIDTH>) {
        self.shl_assign(rhs.0)
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShlAssign<&Aint<R, WIDTH>> for u64
where
    u64: core::ops::ShlAssign<R>,
{
    fn shl_assign(&mut self, rhs: &Aint<R, WIDTH>) {
        self.shl_assign(rhs.0)
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shl<Aint<R, WIDTH>> for u128
where
    u128: core::ops::Shl<R, Output = u128>,
{
    type Output = Self;

    fn shl(self, rhs: Aint<R, WIDTH>) -> Self {
        self << rhs.0
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shl<Aint<R, WIDTH>> for &u128
where
    u128: core::ops::Shl<R, Output = u128>,
{
    type Output = u128;

    fn shl(self, rhs: Aint<R, WIDTH>) -> u128 {
        *self << rhs.0
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shl<&Aint<R, WIDTH>> for u128
where
    u128: core::ops::Shl<R, Output = u128>,
{
    type Output = Self;

    fn shl(self, rhs: &Aint<R, WIDTH>) -> Self {
        self << rhs.0
    }
}

impl<'a, 'b, R: Sealed, const WIDTH: u32> core::ops::Shl<&'b Aint<R, WIDTH>> for &'a u128
where
    u128: core::ops::Shl<R, Output = u128>,
{
    type Output = u128;

    fn shl(self, rhs: &'b Aint<R, WIDTH>) -> u128 {
        *self << rhs.0
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShlAssign<Aint<R, WIDTH>> for u128
where
    u128: core::ops::ShlAssign<R>,
{
    fn shl_assign(&mut self, rhs: Aint<R, WIDTH>) {
        self.shl_assign(rhs.0)
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShlAssign<&Aint<R, WIDTH>> for u128
where
    u128: core::ops::ShlAssign<R>,
{
    fn shl_assign(&mut self, rhs: &Aint<R, WIDTH>) {
        self.shl_assign(rhs.0)
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shl<Aint<R, WIDTH>> for usize
where
    usize: core::ops::Shl<R, Output = usize>,
{
    type Output = Self;

    fn shl(self, rhs: Aint<R, WIDTH>) -> Self {
        self << rhs.0
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shl<Aint<R, WIDTH>> for &usize
where
    usize: core::ops::Shl<R, Output = usize>,
{
    type Output = usize;

    fn shl(self, rhs: Aint<R, WIDTH>) -> usize {
        *self << rhs.0
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shl<&Aint<R, WIDTH>> for usize
where
    usize: core::ops::Shl<R, Output = usize>,
{
    type Output = Self;

    fn shl(self, rhs: &Aint<R, WIDTH>) -> Self {
        self << rhs.0
    }
}

impl<'a, 'b, R: Sealed, const WIDTH: u32> core::ops::Shl<&'b Aint<R, WIDTH>> for &'a usize
where
    usize: core::ops::Shl<R, Output = usize>,
{
    type Output = usize;

    fn shl(self, rhs: &'b Aint<R, WIDTH>) -> usize {
        *self << rhs.0
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShlAssign<Aint<R, WIDTH>> for usize
where
    usize: core::ops::ShlAssign<R>,
{
    fn shl_assign(&mut self, rhs: Aint<R, WIDTH>) {
        self.shl_assign(rhs.0)
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShlAssign<&Aint<R, WIDTH>> for usize
where
    usize: core::ops::ShlAssign<R>,
{
    fn shl_assign(&mut self, rhs: &Aint<R, WIDTH>) {
        self.shl_assign(rhs.0)
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shl<Aint<R, WIDTH>> for i8
where
    i8: core::ops::Shl<R, Output = i8>,
{
    type Output = Self;

    fn shl(self, rhs: Aint<R, WIDTH>) -> Self {
        self << rhs.0
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shl<Aint<R, WIDTH>> for &i8
where
    i8: core::ops::Shl<R, Output = i8>,
{
    type Output = i8;

    fn shl(self, rhs: Aint<R, WIDTH>) -> i8 {
        *self << rhs.0
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shl<&Aint<R, WIDTH>> for i8
where
    i8: core::ops::Shl<R, Output = i8>,
{
    type Output = Self;

    fn shl(self, rhs: &Aint<R, WIDTH>) -> Self {
        self << rhs.0
    }
}

impl<'a, 'b, R: Sealed, const WIDTH: u32> core::ops::Shl<&'b Aint<R, WIDTH>> for &'a i8
where
    i8: core::ops::Shl<R, Output = i8>,
{
    type Output = i8;

    fn shl(self, rhs: &'b Aint<R, WIDTH>) -> i8 {
        *self << rhs.0
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShlAssign<Aint<R, WIDTH>> for i8
where
    i8: core::ops::ShlAssign<R>,
{
    fn shl_assign(&mut self, rhs: Aint<R, WIDTH>) {
        self.shl_assign(rhs.0)
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShlAssign<&Aint<R, WIDTH>> for i8
where
    i8: core::ops::ShlAssign<R>,
{
    fn shl_assign(&mut self, rhs: &Aint<R, WIDTH>) {
        self.shl_assign(rhs.0)
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shl<Aint<R, WIDTH>> for i16
where
    i16: core::ops::Shl<R, Output = i16>,
{
    type Output = Self;

    fn shl(self, rhs: Aint<R, WIDTH>) -> Self {
        self << rhs.0
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shl<Aint<R, WIDTH>> for &i16
where
    i16: core::ops::Shl<R, Output = i16>,
{
    type Output = i16;

    fn shl(self, rhs: Aint<R, WIDTH>) -> i16 {
        *self << rhs.0
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shl<&Aint<R, WIDTH>> for i16
where
    i16: core::ops::Shl<R, Output = i16>,
{
    type Output = Self;

    fn shl(self, rhs: &Aint<R, WIDTH>) -> Self {
        self << rhs.0
    }
}

impl<'a, 'b, R: Sealed, const WIDTH: u32> core::ops::Shl<&'b Aint<R, WIDTH>> for &'a i16
where
    i16: core::ops::Shl<R, Output = i16>,
{
    type Output = i16;

    fn shl(self, rhs: &'b Aint<R, WIDTH>) -> i16 {
        *self << rhs.0
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShlAssign<Aint<R, WIDTH>> for i16
where
    i16: core::ops::ShlAssign<R>,
{
    fn shl_assign(&mut self, rhs: Aint<R, WIDTH>) {
        self.shl_assign(rhs.0)
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShlAssign<&Aint<R, WIDTH>> for i16
where
    i16: core::ops::ShlAssign<R>,
{
    fn shl_assign(&mut self, rhs: &Aint<R, WIDTH>) {
        self.shl_assign(rhs.0)
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shl<Aint<R, WIDTH>> for i32
where
    i32: core::ops::Shl<R, Output = i32>,
{
    type Output = Self;

    fn shl(self, rhs: Aint<R, WIDTH>) -> Self {
        self << rhs.0
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shl<Aint<R, WIDTH>> for &i32
where
    i32: core::ops::Shl<R, Output = i32>,
{
    type Output = i32;

    fn shl(self, rhs: Aint<R, WIDTH>) -> i32 {
        *self << rhs.0
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shl<&Aint<R, WIDTH>> for i32
where
    i32: core::ops::Shl<R, Output = i32>,
{
    type Output = Self;

    fn shl(self, rhs: &Aint<R, WIDTH>) -> Self {
        self << rhs.0
    }
}

impl<'a, 'b, R: Sealed, const WIDTH: u32> core::ops::Shl<&'b Aint<R, WIDTH>> for &'a i32
where
    i32: core::ops::Shl<R, Output = i32>,
{
    type Output = i32;

    fn shl(self, rhs: &'b Aint<R, WIDTH>) -> i32 {
        *self << rhs.0
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShlAssign<Aint<R, WIDTH>> for i32
where
    i32: core::ops::ShlAssign<R>,
{
    fn shl_assign(&mut self, rhs: Aint<R, WIDTH>) {
        self.shl_assign(rhs.0)
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShlAssign<&Aint<R, WIDTH>> for i32
where
    i32: core::ops::ShlAssign<R>,
{
    fn shl_assign(&mut self, rhs: &Aint<R, WIDTH>) {
        self.shl_assign(rhs.0)
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shl<Aint<R, WIDTH>> for i64
where
    i64: core::ops::Shl<R, Output = i64>,
{
    type Output = Self;

    fn shl(self, rhs: Aint<R, WIDTH>) -> Self {
        self << rhs.0
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shl<Aint<R, WIDTH>> for &i64
where
    i64: core::ops::Shl<R, Output = i64>,
{
    type Output = i64;

    fn shl(self, rhs: Aint<R, WIDTH>) -> i64 {
        *self << rhs.0
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shl<&Aint<R, WIDTH>> for i64
where
    i64: core::ops::Shl<R, Output = i64>,
{
    type Output = Self;

    fn shl(self, rhs: &Aint<R, WIDTH>) -> Self {
        self << rhs.0
    }
}

impl<'a, 'b, R: Sealed, const WIDTH: u32> core::ops::Shl<&'b Aint<R, WIDTH>> for &'a i64
where
    i64: core::ops::Shl<R, Output = i64>,
{
    type Output = i64;

    fn shl(self, rhs: &'b Aint<R, WIDTH>) -> i64 {
        *self << rhs.0
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShlAssign<Aint<R, WIDTH>> for i64
where
    i64: core::ops::ShlAssign<R>,
{
    fn shl_assign(&mut self, rhs: Aint<R, WIDTH>) {
        self.shl_assign(rhs.0)
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShlAssign<&Aint<R, WIDTH>> for i64
where
    i64: core::ops::ShlAssign<R>,
{
    fn shl_assign(&mut self, rhs: &Aint<R, WIDTH>) {
        self.shl_assign(rhs.0)
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shl<Aint<R, WIDTH>> for i128
where
    i128: core::ops::Shl<R, Output = i128>,
{
    type Output = Self;

    fn shl(self, rhs: Aint<R, WIDTH>) -> Self {
        self << rhs.0
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shl<Aint<R, WIDTH>> for &i128
where
    i128: core::ops::Shl<R, Output = i128>,
{
    type Output = i128;

    fn shl(self, rhs: Aint<R, WIDTH>) -> i128 {
        *self << rhs.0
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shl<&Aint<R, WIDTH>> for i128
where
    i128: core::ops::Shl<R, Output = i128>,
{
    type Output = Self;

    fn shl(self, rhs: &Aint<R, WIDTH>) -> Self {
        self << rhs.0
    }
}

impl<'a, 'b, R: Sealed, const WIDTH: u32> core::ops::Shl<&'b Aint<R, WIDTH>> for &'a i128
where
    i128: core::ops::Shl<R, Output = i128>,
{
    type Output = i128;

    fn shl(self, rhs: &'b Aint<R, WIDTH>) -> i128 {
        *self << rhs.0
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShlAssign<Aint<R, WIDTH>> for i128
where
    i128: core::ops::ShlAssign<R>,
{
    fn shl_assign(&mut self, rhs: Aint<R, WIDTH>) {
        self.shl_assign(rhs.0)
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShlAssign<&Aint<R, WIDTH>> for i128
where
    i128: core::ops::ShlAssign<R>,
{
    fn shl_assign(&mut self, rhs: &Aint<R, WIDTH>) {
        self.shl_assign(rhs.0)
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shl<Aint<R, WIDTH>> for isize
where
    isize: core::ops::Shl<R, Output = isize>,
{
    type Output = Self;

    fn shl(self, rhs: Aint<R, WIDTH>) -> Self {
        self << rhs.0
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shl<Aint<R, WIDTH>> for &isize
where
    isize: core::ops::Shl<R, Output = isize>,
{
    type Output = isize;

    fn shl(self, rhs: Aint<R, WIDTH>) -> isize {
        *self << rhs.0
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shl<&Aint<R, WIDTH>> for isize
where
    isize: core::ops::Shl<R, Output = isize>,
{
    type Output = Self;

    fn shl(self, rhs: &Aint<R, WIDTH>) -> Self {
        self << rhs.0
    }
}

impl<'a, 'b, R: Sealed, const WIDTH: u32> core::ops::Shl<&'b Aint<R, WIDTH>> for &'a isize
where
    isize: core::ops::Shl<R, Output = isize>,
{
    type Output = isize;

    fn shl(self, rhs: &'b Aint<R, WIDTH>) -> isize {
        *self << rhs.0
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShlAssign<Aint<R, WIDTH>> for isize
where
    isize: core::ops::ShlAssign<R>,
{
    fn shl_assign(&mut self, rhs: Aint<R, WIDTH>) {
        self.shl_assign(rhs.0)
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShlAssign<&Aint<R, WIDTH>> for isize
where
    isize: core::ops::ShlAssign<R>,
{
    fn shl_assign(&mut self, rhs: &Aint<R, WIDTH>) {
        self.shl_assign(rhs.0)
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shr<Aint<R, WIDTH>> for u8
where
    u8: core::ops::Shr<R, Output = u8>,
{
    type Output = Self;

    fn shr(self, rhs: Aint<R, WIDTH>) -> Self {
        self >> rhs.0
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shr<Aint<R, WIDTH>> for &u8
where
    u8: core::ops::Shr<R, Output = u8>,
{
    type Output = u8;

    fn shr(self, rhs: Aint<R, WIDTH>) -> u8 {
        *self >> rhs.0
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shr<&Aint<R, WIDTH>> for u8
where
    u8: core::ops::Shr<R, Output = u8>,
{
    type Output = Self;

    fn shr(self, rhs: &Aint<R, WIDTH>) -> Self {
        self >> rhs.0
    }
}

impl<'a, 'b, R: Sealed, const WIDTH: u32> core::ops::Shr<&'b Aint<R, WIDTH>> for &'a u8
where
    u8: core::ops::Shr<R, Output = u8>,
{
    type Output = u8;

    fn shr(self, rhs: &'b Aint<R, WIDTH>) -> u8 {
        *self >> rhs.0
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShrAssign<Aint<R, WIDTH>> for u8
where
    u8: core::ops::ShrAssign<R>,
{
    fn shr_assign(&mut self, rhs: Aint<R, WIDTH>) {
        self.shr_assign(rhs.0)
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShrAssign<&Aint<R, WIDTH>> for u8
where
    u8: core::ops::ShrAssign<R>,
{
    fn shr_assign(&mut self, rhs: &Aint<R, WIDTH>) {
        self.shr_assign(rhs.0)
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shr<Aint<R, WIDTH>> for u16
where
    u16: core::ops::Shr<R, Output = u16>,
{
    type Output = Self;

    fn shr(self, rhs: Aint<R, WIDTH>) -> Self {
        self >> rhs.0
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shr<Aint<R, WIDTH>> for &u16
where
    u16: core::ops::Shr<R, Output = u16>,
{
    type Output = u16;

    fn shr(self, rhs: Aint<R, WIDTH>) -> u16 {
        *self >> rhs.0
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shr<&Aint<R, WIDTH>> for u16
where
    u16: core::ops::Shr<R, Output = u16>,
{
    type Output = Self;

    fn shr(self, rhs: &Aint<R, WIDTH>) -> Self {
        self >> rhs.0
    }
}

impl<'a, 'b, R: Sealed, const WIDTH: u32> core::ops::Shr<&'b Aint<R, WIDTH>> for &'a u16
where
    u16: core::ops::Shr<R, Output = u16>,
{
    type Output = u16;

    fn shr(self, rhs: &'b Aint<R, WIDTH>) -> u16 {
        *self >> rhs.0
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShrAssign<Aint<R, WIDTH>> for u16
where
    u16: core::ops::ShrAssign<R>,
{
    fn shr_assign(&mut self, rhs: Aint<R, WIDTH>) {
        self.shr_assign(rhs.0)
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShrAssign<&Aint<R, WIDTH>> for u16
where
    u16: core::ops::ShrAssign<R>,
{
    fn shr_assign(&mut self, rhs: &Aint<R, WIDTH>) {
        self.shr_assign(rhs.0)
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shr<Aint<R, WIDTH>> for u32
where
    u32: core::ops::Shr<R, Output = u32>,
{
    type Output = Self;

    fn shr(self, rhs: Aint<R, WIDTH>) -> Self {
        self >> rhs.0
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shr<Aint<R, WIDTH>> for &u32
where
    u32: core::ops::Shr<R, Output = u32>,
{
    type Output = u32;

    fn shr(self, rhs: Aint<R, WIDTH>) -> u32 {
        *self >> rhs.0
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shr<&Aint<R, WIDTH>> for u32
where
    u32: core::ops::Shr<R, Output = u32>,
{
    type Output = Self;

    fn shr(self, rhs: &Aint<R, WIDTH>) -> Self {
        self >> rhs.0
    }
}

impl<'a, 'b, R: Sealed, const WIDTH: u32> core::ops::Shr<&'b Aint<R, WIDTH>> for &'a u32
where
    u32: core::ops::Shr<R, Output = u32>,
{
    type Output = u32;

    fn shr(self, rhs: &'b Aint<R, WIDTH>) -> u32 {
        *self >> rhs.0
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShrAssign<Aint<R, WIDTH>> for u32
where
    u32: core::ops::ShrAssign<R>,
{
    fn shr_assign(&mut self, rhs: Aint<R, WIDTH>) {
        self.shr_assign(rhs.0)
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShrAssign<&Aint<R, WIDTH>> for u32
where
    u32: core::ops::ShrAssign<R>,
{
    fn shr_assign(&mut self, rhs: &Aint<R, WIDTH>) {
        self.shr_assign(rhs.0)
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shr<Aint<R, WIDTH>> for u64
where
    u64: core::ops::Shr<R, Output = u64>,
{
    type Output = Self;

    fn shr(self, rhs: Aint<R, WIDTH>) -> Self {
        self >> rhs.0
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shr<Aint<R, WIDTH>> for &u64
where
    u64: core::ops::Shr<R, Output = u64>,
{
    type Output = u64;

    fn shr(self, rhs: Aint<R, WIDTH>) -> u64 {
        *self >> rhs.0
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shr<&Aint<R, WIDTH>> for u64
where
    u64: core::ops::Shr<R, Output = u64>,
{
    type Output = Self;

    fn shr(self, rhs: &Aint<R, WIDTH>) -> Self {
        self >> rhs.0
    }
}

impl<'a, 'b, R: Sealed, const WIDTH: u32> core::ops::Shr<&'b Aint<R, WIDTH>> for &'a u64
where
    u64: core::ops::Shr<R, Output = u64>,
{
    type Output = u64;

    fn shr(self, rhs: &'b Aint<R, WIDTH>) -> u64 {
        *self >> rhs.0
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShrAssign<Aint<R, WIDTH>> for u64
where
    u64: core::ops::ShrAssign<R>,
{
    fn shr_assign(&mut self, rhs: Aint<R, WIDTH>) {
        self.shr_assign(rhs.0)
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShrAssign<&Aint<R, WIDTH>> for u64
where
    u64: core::ops::ShrAssign<R>,
{
    fn shr_assign(&mut self, rhs: &Aint<R, WIDTH>) {
        self.shr_assign(rhs.0)
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shr<Aint<R, WIDTH>> for u128
where
    u128: core::ops::Shr<R, Output = u128>,
{
    type Output = Self;

    fn shr(self, rhs: Aint<R, WIDTH>) -> Self {
        self >> rhs.0
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shr<Aint<R, WIDTH>> for &u128
where
    u128: core::ops::Shr<R, Output = u128>,
{
    type Output = u128;

    fn shr(self, rhs: Aint<R, WIDTH>) -> u128 {
        *self >> rhs.0
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shr<&Aint<R, WIDTH>> for u128
where
    u128: core::ops::Shr<R, Output = u128>,
{
    type Output = Self;

    fn shr(self, rhs: &Aint<R, WIDTH>) -> Self {
        self >> rhs.0
    }
}

impl<'a, 'b, R: Sealed, const WIDTH: u32> core::ops::Shr<&'b Aint<R, WIDTH>> for &'a u128
where
    u128: core::ops::Shr<R, Output = u128>,
{
    type Output = u128;

    fn shr(self, rhs: &'b Aint<R, WIDTH>) -> u128 {
        *self >> rhs.0
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShrAssign<Aint<R, WIDTH>> for u128
where
    u128: core::ops::ShrAssign<R>,
{
    fn shr_assign(&mut self, rhs: Aint<R, WIDTH>) {
        self.shr_assign(rhs.0)
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShrAssign<&Aint<R, WIDTH>> for u128
where
    u128: core::ops::ShrAssign<R>,
{
    fn shr_assign(&mut self, rhs: &Aint<R, WIDTH>) {
        self.shr_assign(rhs.0)
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shr<Aint<R, WIDTH>> for usize
where
    usize: core::ops::Shr<R, Output = usize>,
{
    type Output = Self;

    fn shr(self, rhs: Aint<R, WIDTH>) -> Self {
        self >> rhs.0
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shr<Aint<R, WIDTH>> for &usize
where
    usize: core::ops::Shr<R, Output = usize>,
{
    type Output = usize;

    fn shr(self, rhs: Aint<R, WIDTH>) -> usize {
        *self >> rhs.0
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shr<&Aint<R, WIDTH>> for usize
where
    usize: core::ops::Shr<R, Output = usize>,
{
    type Output = Self;

    fn shr(self, rhs: &Aint<R, WIDTH>) -> Self {
        self >> rhs.0
    }
}

impl<'a, 'b, R: Sealed, const WIDTH: u32> core::ops::Shr<&'b Aint<R, WIDTH>> for &'a usize
where
    usize: core::ops::Shr<R, Output = usize>,
{
    type Output = usize;

    fn shr(self, rhs: &'b Aint<R, WIDTH>) -> usize {
        *self >> rhs.0
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShrAssign<Aint<R, WIDTH>> for usize
where
    usize: core::ops::ShrAssign<R>,
{
    fn shr_assign(&mut self, rhs: Aint<R, WIDTH>) {
        self.shr_assign(rhs.0)
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShrAssign<&Aint<R, WIDTH>> for usize
where
    usize: core::ops::ShrAssign<R>,
{
    fn shr_assign(&mut self, rhs: &Aint<R, WIDTH>) {
        self.shr_assign(rhs.0)
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shr<Aint<R, WIDTH>> for i8
where
    i8: core::ops::Shr<R, Output = i8>,
{
    type Output = Self;

    fn shr(self, rhs: Aint<R, WIDTH>) -> Self {
        self >> rhs.0
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shr<Aint<R, WIDTH>> for &i8
where
    i8: core::ops::Shr<R, Output = i8>,
{
    type Output = i8;

    fn shr(self, rhs: Aint<R, WIDTH>) -> i8 {
        *self >> rhs.0
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shr<&Aint<R, WIDTH>> for i8
where
    i8: core::ops::Shr<R, Output = i8>,
{
    type Output = Self;

    fn shr(self, rhs: &Aint<R, WIDTH>) -> Self {
        self >> rhs.0
    }
}

impl<'a, 'b, R: Sealed, const WIDTH: u32> core::ops::Shr<&'b Aint<R, WIDTH>> for &'a i8
where
    i8: core::ops::Shr<R, Output = i8>,
{
    type Output = i8;

    fn shr(self, rhs: &'b Aint<R, WIDTH>) -> i8 {
        *self >> rhs.0
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShrAssign<Aint<R, WIDTH>> for i8
where
    i8: core::ops::ShrAssign<R>,
{
    fn shr_assign(&mut self, rhs: Aint<R, WIDTH>) {
        self.shr_assign(rhs.0)
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShrAssign<&Aint<R, WIDTH>> for i8
where
    i8: core::ops::ShrAssign<R>,
{
    fn shr_assign(&mut self, rhs: &Aint<R, WIDTH>) {
        self.shr_assign(rhs.0)
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shr<Aint<R, WIDTH>> for i16
where
    i16: core::ops::Shr<R, Output = i16>,
{
    type Output = Self;

    fn shr(self, rhs: Aint<R, WIDTH>) -> Self {
        self >> rhs.0
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shr<Aint<R, WIDTH>> for &i16
where
    i16: core::ops::Shr<R, Output = i16>,
{
    type Output = i16;

    fn shr(self, rhs: Aint<R, WIDTH>) -> i16 {
        *self >> rhs.0
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shr<&Aint<R, WIDTH>> for i16
where
    i16: core::ops::Shr<R, Output = i16>,
{
    type Output = Self;

    fn shr(self, rhs: &Aint<R, WIDTH>) -> Self {
        self >> rhs.0
    }
}

impl<'a, 'b, R: Sealed, const WIDTH: u32> core::ops::Shr<&'b Aint<R, WIDTH>> for &'a i16
where
    i16: core::ops::Shr<R, Output = i16>,
{
    type Output = i16;

    fn shr(self, rhs: &'b Aint<R, WIDTH>) -> i16 {
        *self >> rhs.0
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShrAssign<Aint<R, WIDTH>> for i16
where
    i16: core::ops::ShrAssign<R>,
{
    fn shr_assign(&mut self, rhs: Aint<R, WIDTH>) {
        self.shr_assign(rhs.0)
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShrAssign<&Aint<R, WIDTH>> for i16
where
    i16: core::ops::ShrAssign<R>,
{
    fn shr_assign(&mut self, rhs: &Aint<R, WIDTH>) {
        self.shr_assign(rhs.0)
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shr<Aint<R, WIDTH>> for i32
where
    i32: core::ops::Shr<R, Output = i32>,
{
    type Output = Self;

    fn shr(self, rhs: Aint<R, WIDTH>) -> Self {
        self >> rhs.0
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shr<Aint<R, WIDTH>> for &i32
where
    i32: core::ops::Shr<R, Output = i32>,
{
    type Output = i32;

    fn shr(self, rhs: Aint<R, WIDTH>) -> i32 {
        *self >> rhs.0
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shr<&Aint<R, WIDTH>> for i32
where
    i32: core::ops::Shr<R, Output = i32>,
{
    type Output = Self;

    fn shr(self, rhs: &Aint<R, WIDTH>) -> Self {
        self >> rhs.0
    }
}

impl<'a, 'b, R: Sealed, const WIDTH: u32> core::ops::Shr<&'b Aint<R, WIDTH>> for &'a i32
where
    i32: core::ops::Shr<R, Output = i32>,
{
    type Output = i32;

    fn shr(self, rhs: &'b Aint<R, WIDTH>) -> i32 {
        *self >> rhs.0
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShrAssign<Aint<R, WIDTH>> for i32
where
    i32: core::ops::ShrAssign<R>,
{
    fn shr_assign(&mut self, rhs: Aint<R, WIDTH>) {
        self.shr_assign(rhs.0)
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShrAssign<&Aint<R, WIDTH>> for i32
where
    i32: core::ops::ShrAssign<R>,
{
    fn shr_assign(&mut self, rhs: &Aint<R, WIDTH>) {
        self.shr_assign(rhs.0)
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shr<Aint<R, WIDTH>> for i64
where
    i64: core::ops::Shr<R, Output = i64>,
{
    type Output = Self;

    fn shr(self, rhs: Aint<R, WIDTH>) -> Self {
        self >> rhs.0
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shr<Aint<R, WIDTH>> for &i64
where
    i64: core::ops::Shr<R, Output = i64>,
{
    type Output = i64;

    fn shr(self, rhs: Aint<R, WIDTH>) -> i64 {
        *self >> rhs.0
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shr<&Aint<R, WIDTH>> for i64
where
    i64: core::ops::Shr<R, Output = i64>,
{
    type Output = Self;

    fn shr(self, rhs: &Aint<R, WIDTH>) -> Self {
        self >> rhs.0
    }
}

impl<'a, 'b, R: Sealed, const WIDTH: u32> core::ops::Shr<&'b Aint<R, WIDTH>> for &'a i64
where
    i64: core::ops::Shr<R, Output = i64>,
{
    type Output = i64;

    fn shr(self, rhs: &'b Aint<R, WIDTH>) -> i64 {
        *self >> rhs.0
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShrAssign<Aint<R, WIDTH>> for i64
where
    i64: core::ops::ShrAssign<R>,
{
    fn shr_assign(&mut self, rhs: Aint<R, WIDTH>) {
        self.shr_assign(rhs.0)
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShrAssign<&Aint<R, WIDTH>> for i64
where
    i64: core::ops::ShrAssign<R>,
{
    fn shr_assign(&mut self, rhs: &Aint<R, WIDTH>) {
        self.shr_assign(rhs.0)
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shr<Aint<R, WIDTH>> for i128
where
    i128: core::ops::Shr<R, Output = i128>,
{
    type Output = Self;

    fn shr(self, rhs: Aint<R, WIDTH>) -> Self {
        self >> rhs.0
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shr<Aint<R, WIDTH>> for &i128
where
    i128: core::ops::Shr<R, Output = i128>,
{
    type Output = i128;

    fn shr(self, rhs: Aint<R, WIDTH>) -> i128 {
        *self >> rhs.0
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shr<&Aint<R, WIDTH>> for i128
where
    i128: core::ops::Shr<R, Output = i128>,
{
    type Output = Self;

    fn shr(self, rhs: &Aint<R, WIDTH>) -> Self {
        self >> rhs.0
    }
}

impl<'a, 'b, R: Sealed, const WIDTH: u32> core::ops::Shr<&'b Aint<R, WIDTH>> for &'a i128
where
    i128: core::ops::Shr<R, Output = i128>,
{
    type Output = i128;

    fn shr(self, rhs: &'b Aint<R, WIDTH>) -> i128 {
        *self >> rhs.0
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShrAssign<Aint<R, WIDTH>> for i128
where
    i128: core::ops::ShrAssign<R>,
{
    fn shr_assign(&mut self, rhs: Aint<R, WIDTH>) {
        self.shr_assign(rhs.0)
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShrAssign<&Aint<R, WIDTH>> for i128
where
    i128: core::ops::ShrAssign<R>,
{
    fn shr_assign(&mut self, rhs: &Aint<R, WIDTH>) {
        self.shr_assign(rhs.0)
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shr<Aint<R, WIDTH>> for isize
where
    isize: core::ops::Shr<R, Output = isize>,
{
    type Output = Self;

    fn shr(self, rhs: Aint<R, WIDTH>) -> Self {
        self >> rhs.0
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shr<Aint<R, WIDTH>> for &isize
where
    isize: core::ops::Shr<R, Output = isize>,
{
    type Output = isize;

    fn shr(self, rhs: Aint<R, WIDTH>) -> isize {
        *self >> rhs.0
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Shr<&Aint<R, WIDTH>> for isize
where
    isize: core::ops::Shr<R, Output = isize>,
{
    type Output = Self;

    fn shr(self, rhs: &Aint<R, WIDTH>) -> Self {
        self >> rhs.0
    }
}

impl<'a, 'b, R: Sealed, const WIDTH: u32> core::ops::Shr<&'b Aint<R, WIDTH>> for &'a isize
where
    isize: core::ops::Shr<R, Output = isize>,
{
    type Output = isize;

    fn shr(self, rhs: &'b Aint<R, WIDTH>) -> isize {
        *self >> rhs.0
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShrAssign<Aint<R, WIDTH>> for isize
where
    isize: core::ops::ShrAssign<R>,
{
    fn shr_assign(&mut self, rhs: Aint<R, WIDTH>) {
        self.shr_assign(rhs.0)
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::ShrAssign<&Aint<R, WIDTH>> for isize
where
    isize: core::ops::ShrAssign<R>,
{
    fn shr_assign(&mut self, rhs: &Aint<R, WIDTH>) {
        self.shr_assign(rhs.0)
    }
}
