use crate::sealed::Sealed;
use crate::Aint;

impl<R: Sealed + core::ops::Neg<Output = R>, const WIDTH: u32> core::ops::Neg for Aint<R, WIDTH> {
    type Output = Self;

    fn neg(self) -> Self {
        let val = -self.0;
        debug_assert!(
            val <= Self::_max().0 && val >= Self::_min().0,
            "attempt to negate with overflow"
        );
        Self::_new_wrapping(val)
    }
}

impl<R: Sealed + core::ops::Neg<Output = R>, const WIDTH: u32> core::ops::Neg for &Aint<R, WIDTH> {
    type Output = Aint<R, WIDTH>;

    fn neg(self) -> Self::Output {
        -*self
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Add for Aint<R, WIDTH> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let val = self.0 + rhs.0;
        debug_assert!(
            val <= Self::_max().0 && val >= Self::_min().0,
            "attempt to add with overflow"
        );
        Self::_new_wrapping(val)
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Add<&Aint<R, WIDTH>> for Aint<R, WIDTH> {
    type Output = Self;

    fn add(self, rhs: &Aint<R, WIDTH>) -> Self {
        self + *rhs
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Add<Aint<R, WIDTH>> for &Aint<R, WIDTH> {
    type Output = Aint<R, WIDTH>;

    fn add(self, rhs: Aint<R, WIDTH>) -> Self::Output {
        *self + rhs
    }
}

impl<'a, 'b, R: Sealed, const WIDTH: u32> core::ops::Add<&'b Aint<R, WIDTH>>
    for &'a Aint<R, WIDTH>
{
    type Output = Aint<R, WIDTH>;

    fn add(self, rhs: &'b Aint<R, WIDTH>) -> Self::Output {
        *self + *rhs
    }
}

impl<T, R: Sealed, const WIDTH: u32> core::ops::AddAssign<T> for Aint<R, WIDTH>
where
    Self: core::ops::Add<T, Output = Self>,
{
    fn add_assign(&mut self, rhs: T) {
        *self = *self + rhs
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Sub for Aint<R, WIDTH> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let val = self.0 - rhs.0;
        debug_assert!(
            val <= Self::_max().0 && val >= Self::_min().0,
            "attempt to subtract with overflow"
        );
        Self::_new_wrapping(val)
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Sub<&Aint<R, WIDTH>> for Aint<R, WIDTH> {
    type Output = Self;

    fn sub(self, rhs: &Aint<R, WIDTH>) -> Self {
        self - *rhs
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Sub<Aint<R, WIDTH>> for &Aint<R, WIDTH> {
    type Output = Aint<R, WIDTH>;

    fn sub(self, rhs: Aint<R, WIDTH>) -> Self::Output {
        *self - rhs
    }
}

impl<'a, 'b, R: Sealed, const WIDTH: u32> core::ops::Sub<&'b Aint<R, WIDTH>>
    for &'a Aint<R, WIDTH>
{
    type Output = Aint<R, WIDTH>;

    fn sub(self, rhs: &'b Aint<R, WIDTH>) -> Self::Output {
        *self - *rhs
    }
}

impl<T, R: Sealed, const WIDTH: u32> core::ops::SubAssign<T> for Aint<R, WIDTH>
where
    Self: core::ops::Sub<T, Output = Self>,
{
    fn sub_assign(&mut self, rhs: T) {
        *self = *self - rhs
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Mul for Aint<R, WIDTH> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let val = self.0 * rhs.0;
        debug_assert!(
            val <= Self::_max().0 && val >= Self::_min().0,
            "attempt to multiply with overflow"
        );
        Self::_new_wrapping(val)
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Mul<&Aint<R, WIDTH>> for Aint<R, WIDTH> {
    type Output = Self;

    fn mul(self, rhs: &Aint<R, WIDTH>) -> Self {
        self * *rhs
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Mul<Aint<R, WIDTH>> for &Aint<R, WIDTH> {
    type Output = Aint<R, WIDTH>;

    fn mul(self, rhs: Aint<R, WIDTH>) -> Self::Output {
        *self * rhs
    }
}

impl<'a, 'b, R: Sealed, const WIDTH: u32> core::ops::Mul<&'b Aint<R, WIDTH>>
    for &'a Aint<R, WIDTH>
{
    type Output = Aint<R, WIDTH>;

    fn mul(self, rhs: &'b Aint<R, WIDTH>) -> Self::Output {
        *self * *rhs
    }
}

impl<T, R: Sealed, const WIDTH: u32> core::ops::MulAssign<T> for Aint<R, WIDTH>
where
    Self: core::ops::Mul<T, Output = Self>,
{
    fn mul_assign(&mut self, rhs: T) {
        *self = *self * rhs
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Div for Aint<R, WIDTH> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        let val = self.0 / rhs.0;
        debug_assert!(
            val <= Self::_max().0 && val >= Self::_min().0,
            "attempt to divide with overflow"
        );
        Self::_new_wrapping(val)
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Div<&Aint<R, WIDTH>> for Aint<R, WIDTH> {
    type Output = Self;

    fn div(self, rhs: &Aint<R, WIDTH>) -> Self {
        self / *rhs
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Div<Aint<R, WIDTH>> for &Aint<R, WIDTH> {
    type Output = Aint<R, WIDTH>;

    fn div(self, rhs: Aint<R, WIDTH>) -> Self::Output {
        *self / rhs
    }
}

impl<'a, 'b, R: Sealed, const WIDTH: u32> core::ops::Div<&'b Aint<R, WIDTH>>
    for &'a Aint<R, WIDTH>
{
    type Output = Aint<R, WIDTH>;

    fn div(self, rhs: &'b Aint<R, WIDTH>) -> Self::Output {
        *self / *rhs
    }
}

impl<T, R: Sealed, const WIDTH: u32> core::ops::DivAssign<T> for Aint<R, WIDTH>
where
    Self: core::ops::Div<T, Output = Self>,
{
    fn div_assign(&mut self, rhs: T) {
        *self = *self / rhs
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Rem for Aint<R, WIDTH> {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self {
        let val = self.0 % rhs.0;
        debug_assert!(
            val <= Self::_max().0 && val >= Self::_min().0,
            "attempt to compute modulus with overflow"
        );
        Self::_new_wrapping(val)
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Rem<&Aint<R, WIDTH>> for Aint<R, WIDTH> {
    type Output = Self;

    fn rem(self, rhs: &Aint<R, WIDTH>) -> Self {
        self % *rhs
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Rem<Aint<R, WIDTH>> for &Aint<R, WIDTH> {
    type Output = Aint<R, WIDTH>;

    fn rem(self, rhs: Aint<R, WIDTH>) -> Self::Output {
        *self % rhs
    }
}

impl<'a, 'b, R: Sealed, const WIDTH: u32> core::ops::Rem<&'b Aint<R, WIDTH>>
    for &'a Aint<R, WIDTH>
{
    type Output = Aint<R, WIDTH>;

    fn rem(self, rhs: &'b Aint<R, WIDTH>) -> Self::Output {
        *self % *rhs
    }
}

impl<T, R: Sealed, const WIDTH: u32> core::ops::RemAssign<T> for Aint<R, WIDTH>
where
    Self: core::ops::Rem<T, Output = Self>,
{
    fn rem_assign(&mut self, rhs: T) {
        *self = *self % rhs
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Not for Aint<R, WIDTH> {
    type Output = Self;

    fn not(self) -> Self {
        if R::SIGNED {
            Self(!self.0)
        } else {
            Self(!self.0 & Self::_mask())
        }
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::Not for &Aint<R, WIDTH> {
    type Output = Aint<R, WIDTH>;

    fn not(self) -> Self::Output {
        !*self
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::BitAnd for Aint<R, WIDTH> {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::BitAnd<&Aint<R, WIDTH>> for Aint<R, WIDTH> {
    type Output = Self;

    fn bitand(self, rhs: &Aint<R, WIDTH>) -> Self {
        self & *rhs
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::BitAnd<Aint<R, WIDTH>> for &Aint<R, WIDTH> {
    type Output = Aint<R, WIDTH>;

    fn bitand(self, rhs: Aint<R, WIDTH>) -> Self::Output {
        *self & rhs
    }
}

impl<'a, 'b, R: Sealed, const WIDTH: u32> core::ops::BitAnd<&'b Aint<R, WIDTH>>
    for &'a Aint<R, WIDTH>
{
    type Output = Aint<R, WIDTH>;

    fn bitand(self, rhs: &'b Aint<R, WIDTH>) -> Self::Output {
        *self & *rhs
    }
}

impl<T, R: Sealed, const WIDTH: u32> core::ops::BitAndAssign<T> for Aint<R, WIDTH>
where
    Self: core::ops::BitAnd<T, Output = Self>,
{
    fn bitand_assign(&mut self, rhs: T) {
        *self = *self & rhs
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::BitOr for Aint<R, WIDTH> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::BitOr<&Aint<R, WIDTH>> for Aint<R, WIDTH> {
    type Output = Self;

    fn bitor(self, rhs: &Aint<R, WIDTH>) -> Self {
        self | *rhs
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::BitOr<Aint<R, WIDTH>> for &Aint<R, WIDTH> {
    type Output = Aint<R, WIDTH>;

    fn bitor(self, rhs: Aint<R, WIDTH>) -> Self::Output {
        *self | rhs
    }
}

impl<'a, 'b, R: Sealed, const WIDTH: u32> core::ops::BitOr<&'b Aint<R, WIDTH>>
    for &'a Aint<R, WIDTH>
{
    type Output = Aint<R, WIDTH>;

    fn bitor(self, rhs: &'b Aint<R, WIDTH>) -> Self::Output {
        *self | *rhs
    }
}

impl<T, R: Sealed, const WIDTH: u32> core::ops::BitOrAssign<T> for Aint<R, WIDTH>
where
    Self: core::ops::BitOr<T, Output = Self>,
{
    fn bitor_assign(&mut self, rhs: T) {
        *self = *self | rhs
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::BitXor for Aint<R, WIDTH> {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self {
        Self(self.0 ^ rhs.0)
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::BitXor<&Aint<R, WIDTH>> for Aint<R, WIDTH> {
    type Output = Self;

    fn bitxor(self, rhs: &Aint<R, WIDTH>) -> Self {
        self ^ *rhs
    }
}

impl<R: Sealed, const WIDTH: u32> core::ops::BitXor<Aint<R, WIDTH>> for &Aint<R, WIDTH> {
    type Output = Aint<R, WIDTH>;

    fn bitxor(self, rhs: Aint<R, WIDTH>) -> Self::Output {
        *self ^ rhs
    }
}

impl<'a, 'b, R: Sealed, const WIDTH: u32> core::ops::BitXor<&'b Aint<R, WIDTH>>
    for &'a Aint<R, WIDTH>
{
    type Output = Aint<R, WIDTH>;

    fn bitxor(self, rhs: &'b Aint<R, WIDTH>) -> Self::Output {
        *self ^ *rhs
    }
}

impl<T, R: Sealed, const WIDTH: u32> core::ops::BitXorAssign<T> for Aint<R, WIDTH>
where
    Self: core::ops::BitXor<T, Output = Self>,
{
    fn bitxor_assign(&mut self, rhs: T) {
        *self = *self ^ rhs
    }
}
