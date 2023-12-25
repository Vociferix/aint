use crate::sealed::Sealed;

/// An integer with a given representation, `R`, and bit width, `WIDTH`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Aint<R: Sealed, const WIDTH: u32>(pub(crate) R);

fn make_parse_int_err() -> core::num::ParseIntError {
    let Err(e) = u8::from_str_radix("256", 10) else {
        unreachable!()
    };
    e
}

impl<R: Sealed, const WIDTH: u32> Aint<R, WIDTH> {
    pub(crate) fn _min() -> Self {
        if R::SIGNED {
            Self(!R::ZERO << (WIDTH - 1))
        } else {
            Self(R::ZERO)
        }
    }

    pub(crate) fn _max() -> Self {
        if R::SIGNED {
            Self(!Self::_min().0)
        } else {
            Self(!(!R::ZERO << WIDTH))
        }
    }

    pub(crate) fn _mask() -> R {
        !(!R::ZERO << WIDTH)
    }

    pub(crate) fn _sign_bit() -> R {
        if R::SIGNED {
            R::ONE << (WIDTH - 1)
        } else {
            R::ZERO
        }
    }

    pub(crate) fn _new(repr: R) -> Option<Self> {
        if repr <= Self::_max().0 && repr >= Self::_min().0 {
            Some(Self(repr))
        } else {
            None
        }
    }

    pub(crate) fn _new_wrapping(repr: R) -> Self {
        if (repr & Self::_sign_bit()) == R::ZERO {
            Self(repr & Self::_max().0)
        } else {
            Self(repr | !Self::_max().0)
        }
    }

    pub(crate) fn _new_saturating(repr: R) -> Self {
        if repr >= Self::_max().0 {
            Self::_max()
        } else if repr <= Self::_min().0 {
            Self::_min()
        } else {
            Self(repr)
        }
    }
}

impl<const WIDTH: u32> Aint<u8, WIDTH> {
    const ASSERT: () = if WIDTH > 0 && WIDTH < u8::BITS {
        ()
    } else {
        panic!("Invalid Aint width with repr type u8")
    };

    /// The size of this integer type in bits.
    pub const BITS: u32 = {
        let _val = Self::ASSERT;
        WIDTH
    };

    /// The smallest value that can be represented by this integer type.
    pub const MIN: Self = {
        let _val = Self::ASSERT;
        Self(0)
    };

    /// The largest value that can be represented by this integer type.
    pub const MAX: Self = {
        let _val = Self::ASSERT;
        Self(u8::MAX >> (u8::BITS - WIDTH))
    };

    pub(crate) const MASK: u8 = {
        let _val = Self::ASSERT;
        u8::MAX >> (u8::BITS - WIDTH)
    };

    /// Creates a new integer value from the underlying representation type, unchecked.
    ///
    /// # Safety
    /// The representation type value must be within the range of valid values of this type.
    /// That is, the value must be greater or equal to [`MIN`](Self::MIN) and less or equal to
    /// [`MAX`](Self::MAX).
    pub const unsafe fn new_unchecked(repr: u8) -> Self {
        let _val = Self::ASSERT;
        Self(repr)
    }

    /// Creates a new integer value from the underlying representation type.
    ///
    /// The returned value is `None` if the representation value is outside the range of valid
    /// values of this integer type.
    pub const fn new(repr: u8) -> Option<Self> {
        if repr <= Self::MAX.0 {
            Some(Self(repr))
        } else {
            None
        }
    }

    /// Creates a new integer value from the underlying representation type.
    ///
    /// The returned value is wrapped as though the representation value were calculated using
    /// wrapping operations, such as [`wrapping_add`](Self::wrapping_add).
    pub const fn new_wrapping(repr: u8) -> Self {
        Self(repr & Self::MAX.0)
    }

    /// Creates a new integer value from the underlying representation type.
    ///
    /// The returned value is saturated to the bounds of this integer's value range. If the
    /// representation value is greater than [`MAX`](Self::MAX), the returned value will be
    /// [`MAX`](Self::MAX). If the representation value is less than [`MIN`](Self::MIN), the
    /// returned value will be [`MIN`](Self::MIN).
    pub const fn new_saturating(repr: u8) -> Self {
        if repr <= Self::MAX.0 {
            Self(repr)
        } else {
            Self::MAX
        }
    }

    const fn new_overflowing_impl((repr, over): (u8, bool)) -> (Self, bool) {
        if repr > Self::MAX.0 {
            (Self(repr & Self::MASK), true)
        } else {
            (Self(repr), over)
        }
    }

    /// Creates a new integer from the underlying representation type.
    ///
    /// The returned tuple contains the new integer and a `bool` indicating if the representation
    /// value overflowed the new integer. In the case of overflow, the new integer has a value
    /// as if it were produced from [`new_wrapping`](Self::new_wrapping).
    pub const fn new_overflowing(repr: u8) -> (Self, bool) {
        Self::new_overflowing_impl((repr, false))
    }

    /// Returns the value of this integer as the underlying representation type.
    pub const fn repr(self) -> u8 {
        self.0
    }

    /// Returns the number of ones in the binary representation of this integer.
    pub const fn count_ones(self) -> u32 {
        self.0.count_ones()
    }

    /// Returns the number of zeros in the binary representatino of this integer.
    pub const fn count_zeros(self) -> u32 {
        self.0.count_zeros() - (u8::BITS - Self::BITS)
    }

    /// Returns the number of leading zeros in the binary represnetation of this integer.
    pub const fn leading_zeros(self) -> u32 {
        self.0.leading_zeros() - (u8::BITS - Self::BITS)
    }

    /// Returns the number of trailing zeros in the binary representatino of this integer.
    pub const fn trailing_zeros(self) -> u32 {
        (self.0 | !Self::MASK).trailing_zeros()
    }

    /// Returns the number of leading ones in the binary representation of this integer.
    pub const fn leading_ones(self) -> u32 {
        (self.0 << (u8::BITS - Self::BITS)).leading_ones()
    }

    /// Returns the number of trailing ones in the binary representation of this integer.
    pub const fn trailing_ones(self) -> u32 {
        self.0.trailing_ones()
    }

    /// Performs a left bit shift by `n`, wrapping the truncated bits back to the end.
    ///
    /// Note that unlike the `<<` operator, all values of `n` are valid. A value of `n`, that
    /// is greater than or equal to [`BITS`](Self::BITS), is the equivalent to using the value
    /// `n % Self::BITS`.
    pub const fn rotate_left(self, n: u32) -> Self {
        let n = n % Self::BITS;
        Self::new_wrapping((self.0 << n) | (self.0 >> (Self::BITS - n)))
    }

    /// Performs a right bit shift by `n`, wrapping the truncated bits back to the end.
    ///
    /// Note that unlike the `>>` operator, all values of `n` are valid. A value of `n`, that
    /// is greater than or equal to [`BITS`](Self::BITS), is the equivalent to using the value
    /// `n % Self::BITS`.
    pub const fn rotate_right(self, n: u32) -> Self {
        let n = n % Self::BITS;
        Self::new_wrapping((self.0 >> n) | (self.0 << (Self::BITS - n)))
    }

    /// Reverses the order of the bits in the binary representation of this integer.
    pub const fn reverse_bits(self) -> Self {
        Self(self.0.reverse_bits() >> (u8::BITS - Self::BITS))
    }

    /// Calculates the sum of this integer with another integer.
    ///
    /// This method works as a `const` capable alternative to `self + rhs`.
    pub const fn add(self, rhs: Self) -> Self {
        let val = self.0 + rhs.0;
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to add with overflow"
        );
        Self::new_wrapping(val)
    }

    /// Calculates the difference of this integer with another integer.
    ///
    /// This method works as a `const` capable alternative to `self - rhs`.
    pub const fn sub(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0 - rhs.0)
    }

    /// Calculates the product of this integer with another integer.
    ///
    /// This method works as a `const` capable alternative to `self * rhs`.
    pub const fn mul(self, rhs: Self) -> Self {
        let val = self.0 * rhs.0;
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to multiply with overflow"
        );
        Self::new_wrapping(val)
    }

    /// Calculates the quotient of this integer over another integer.
    ///
    /// This method works as a `const` capable alternative to `self / rhs`.
    pub const fn div(self, rhs: Self) -> Self {
        Self(self.0 / rhs.0)
    }

    /// Calculates the remainder of this integer over another integer.
    ///
    /// This method works as a `const` capable alternative to `self % rhs`.
    pub const fn rem(self, rhs: Self) -> Self {
        Self(self.0 % rhs.0)
    }

    /// Calculates the bitwise and of this integer with another integer.
    ///
    /// This method works as a `const` capable alternative to `self & rhs`.
    pub const fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }

    /// Calculates the bitwise or of this integer with another integer.
    ///
    /// This method works as a `const` capable alternative to `self | rhs`.
    pub const fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }

    /// Calculates the bitwise exclusive-or of this integer with another integer.
    ///
    /// This method works as a `const` capable alternative to `self ^ rhs`.
    pub const fn bitxor(self, rhs: Self) -> Self {
        Self(self.0 ^ rhs.0)
    }

    /// Calculates the bitwise negation of this integer.
    ///
    /// This method works as a `const` capable alternative to `!self`.
    pub const fn not(self) -> Self {
        Self(!self.0 & Self::MASK)
    }

    /// Calculates the left bitwise shift of this integer by `rhs`.
    ///
    /// This method works as a `const` capable alternative to `self << rhs`.
    pub const fn shl(self, rhs: u32) -> Self {
        debug_assert!(rhs >= Self::BITS, "attempt to shift left with overflow");
        Self::new_wrapping(self.0 << rhs)
    }

    /// Calculates the right bitwise shift of this integer by `rhs`.
    ///
    /// This method works as a `const` capable alternative to `self >> rhs`.
    pub const fn shr(self, rhs: u32) -> Self {
        debug_assert!(rhs >= Self::BITS, "attempt to shift right with overflow");
        Self::new_wrapping(self.0 >> rhs)
    }

    /// Calculates the logarithm of this integer with respect to an arbitrary base, rounded
    /// down.
    ///
    /// The [`ilog2`](Self::ilog2) and [`ilog10`](Self::ilog10) methods should be preferred
    /// when applicable, as they are generally more optimized than this method since the base
    /// of the logarithm is not arbitrary.
    ///
    /// # Panics
    /// This method will panic if `self` is less than or equal to zero, or if `base` is less
    /// than 2.
    pub const fn ilog(self, base: Self) -> u32 {
        self.0.ilog(base.0)
    }

    /// Calculates the base 2 logarithm of this integer, rounded down.
    ///
    /// # Panics
    /// This method will panic if `self` is less than or equal to zero.
    pub const fn ilog2(self) -> u32 {
        self.0.ilog2()
    }

    /// Calculates the base 10 logarithm of this integer, rounded down.
    ///
    /// # Panics
    /// This method will panic if `self` is less than or equal to zero.
    pub const fn ilog10(self) -> u32 {
        self.0.ilog10()
    }

    /// Returns `true` if `self` and `rhs` are equal.
    ///
    /// This method works as a `const` capable alternative to `self == rhs`.
    pub const fn eq(self, rhs: Self) -> bool {
        return self.0 == rhs.0;
    }

    /// Returns `true` if `self` and `rhs` are not equal.
    ///
    /// This method works as a `const` capable alternative to `self != rhs`.
    pub const fn ne(self, rhs: Self) -> bool {
        return self.0 != rhs.0;
    }

    /// Returns `true` if `self` is less than `rhs`.
    ///
    /// This method works as a `const` capable alternative to `self < rhs`.
    pub const fn lt(self, rhs: Self) -> bool {
        return self.0 < rhs.0;
    }

    /// Returns `true` if `self` is less than or equal to `rhs`.
    ///
    /// This method works as a `const` capable alternative to `self <= rhs`.
    pub const fn le(self, rhs: Self) -> bool {
        return self.0 <= rhs.0;
    }

    /// Returns `true` if `self` is greater than `rhs`.
    ///
    /// This method works as a `const` capable alternative to `self > rhs`.
    pub const fn gt(self, rhs: Self) -> bool {
        return self.0 > rhs.0;
    }

    /// Returns `true` if `self` is greater than or equal to `rhs`.
    ///
    /// This method works as a `const` capable alternative to `self >= rhs`.
    pub const fn ge(self, rhs: Self) -> bool {
        return self.0 >= rhs.0;
    }

    /// Returns the ordering of `self` with respect to `rhs`.
    ///
    /// This method works as a `const` capable alternative to `Ord::cmp(self, rhs)`.
    pub const fn cmp(self, rhs: Self) -> core::cmp::Ordering {
        if self.0 < rhs.0 {
            core::cmp::Ordering::Less
        } else if self.0 > rhs.0 {
            core::cmp::Ordering::Greater
        } else {
            core::cmp::Ordering::Equal
        }
    }

    /// Returns the minimum of `self` and `other`.
    ///
    /// This method works as a `const` capable alternative to `Ord::min(self, rhs)`.
    pub const fn min(self, other: Self) -> Self {
        if self.0 < other.0 {
            self
        } else {
            other
        }
    }

    /// Returns the maximum of `self` and `other`.
    ///
    /// This method works as a `const` capable alternative to `Ord::max(self, rhs)`.
    pub const fn max(self, other: Self) -> Self {
        if self.0 > other.0 {
            self
        } else {
            other
        }
    }

    /// Returns the value of this integer, clamped between `min` and `max`.
    ///
    /// This method works as a `const` capable alternative to `Ord::clamp(self, min, max)`.
    pub const fn clamp(self, min: Self, max: Self) -> Self {
        debug_assert!(
            min.0 <= max.0,
            "Attempt to clamp with minimum value greater than maximum value"
        );
        Self::min(Self::max(self, min), max)
    }

    /// Checked integer addition. Returns `None` if overflow occurred.
    pub const fn checked_add(self, rhs: Self) -> Option<Self> {
        match self.0.checked_add(rhs.0) {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked integer subtraction. Returns `None` if overflow occurred.
    pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
        match self.0.checked_sub(rhs.0) {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked integer multiplication. Returns `None` if overflow occurred.
    pub const fn checked_mul(self, rhs: Self) -> Option<Self> {
        match self.0.checked_mul(rhs.0) {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked integer division. Returns `None` if `rhs` is zero, or the division resulted in
    /// overflow.
    pub const fn checked_div(self, rhs: Self) -> Option<Self> {
        match self.0.checked_div(rhs.0) {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked euclidian division. Returns `None` if `rhs` is zero or `self.div_euclid(rhs)`
    /// would have resulted in overflow.
    pub const fn checked_div_euclid(self, rhs: Self) -> Option<Self> {
        match self.0.checked_div_euclid(rhs.0) {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked integer remainder. Returns `None` if `rhs` is zero or the remainder would have
    /// resulted in overflow.
    pub const fn checked_rem(self, rhs: Self) -> Option<Self> {
        match self.0.checked_rem(rhs.0) {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked euclidian remainder. Returns `None` if `rhs` is zero or `self.rem_euclid(rhs)`
    /// would have resulted in overflow.
    pub const fn checked_rem_euclid(self, rhs: Self) -> Option<Self> {
        match self.0.checked_rem_euclid(rhs.0) {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked integer negation. Returns `None` if the negation resulted in overflow.
    pub const fn checked_neg(self) -> Option<Self> {
        match self.0.checked_neg() {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked left bit shift. Returns `None` if `rhs` is greater than or equal to
    /// [`Self::BITS`].
    pub const fn checked_shl(self, rhs: u32) -> Option<Self> {
        if rhs >= Self::BITS {
            None
        } else {
            Some(self.shl(rhs))
        }
    }

    /// Checked right bit shift. Returns `None` if `rhs` is greater than or equal to
    /// [`Self::BITS`].
    pub const fn checked_shr(self, rhs: u32) -> Option<Self> {
        if rhs >= Self::BITS {
            None
        } else {
            Some(self.shr(rhs))
        }
    }

    /// Checked exponentiation. Returns `None` if the exponentiation resulted in overflow.
    pub const fn checked_pow(self, exp: u32) -> Option<Self> {
        match self.0.checked_pow(exp) {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked logarithm, rounded down. Returns `None` if `self` is less than or equal zero,
    /// or if `base` is less than 2.
    pub const fn checked_ilog(self, base: Self) -> Option<u32> {
        self.0.checked_ilog(base.0)
    }

    /// Checked base 2 logarithm, rounded down. Returns `None` if self is less than or equal to
    /// zero.
    pub const fn checked_ilog2(self) -> Option<u32> {
        self.0.checked_ilog2()
    }

    /// Checked base 10 logarithm, rounded down. Returns `None` if self is less than or equal
    /// to zero.
    pub const fn checked_ilog10(self) -> Option<u32> {
        self.0.checked_ilog10()
    }

    /// Satruating integer addition. The result of the addition is clamped between
    /// [`MIN`](Self::MIN) and [`MAX`](Self::MAX).
    pub const fn saturating_add(self, rhs: Self) -> Self {
        Self::new_saturating(self.0.saturating_add(rhs.0))
    }

    /// Saturating integer subtraction. The result of the subtraction is clamped between
    /// [`MIN`](Self::MIN) and [`MAX`](Self::MAX).
    pub const fn saturating_sub(self, rhs: Self) -> Self {
        Self::new_saturating(self.0.saturating_sub(rhs.0))
    }

    /// Saturating integer multiplication. The result of the multiplication is clamped between
    /// [`MIN`](Self::MIN) and [`MAX`](Self::MAX).
    pub const fn saturating_mul(self, rhs: Self) -> Self {
        Self::new_saturating(self.0.saturating_mul(rhs.0))
    }

    /// Saturating exponentiation. The result of the exponentiation is clammed between
    /// [`MIN`](Self::MIN) and [`MAX`](Self::MAX).
    pub const fn saturating_pow(self, exp: u32) -> Self {
        Self::new_saturating(self.0.saturating_pow(exp))
    }

    /// Wrapping (modular) addition. An overflowing result is wrapped around the bounds of this
    /// integer type.
    pub const fn wrapping_add(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.wrapping_add(rhs.0))
    }

    /// Wrapping (modular) subtraction. An overflowing result is wrapped around the bounds of
    /// this integer type.
    pub const fn wrapping_sub(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.wrapping_sub(rhs.0))
    }

    /// Wrapping (modular) multiplication. An overflowing result is wrapped around the bounds of
    /// this integer type.
    pub const fn wrapping_mul(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.wrapping_mul(rhs.0))
    }

    /// Wrapping (modular) division. An overflowing result is wrapped around the bounds of this
    /// integer type.
    pub const fn wrapping_div(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.wrapping_div(rhs.0))
    }

    /// Wrapping (modular) euclidian division. An overflowing result is wrapped around the
    /// bounds of this integer type.
    pub const fn wrapping_div_euclid(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.wrapping_div_euclid(rhs.0))
    }

    /// Wrapping (modular) remainder. An overflowing result is wrapped around the bounds of
    /// this integer type.
    pub const fn wrapping_rem(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.wrapping_rem(rhs.0))
    }

    /// Wrapping (modular) euclidian remainder. An overflowing result is wrapped around the
    /// bounds of this integer type.
    pub const fn wrapping_rem_euclid(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.wrapping_rem_euclid(rhs.0))
    }

    /// Wrapping (modular) negation. An overflowing result is wrapped around the bounds of this
    /// integer type.
    pub const fn wrapping_neg(self) -> Self {
        Self::new_wrapping(!self.0 + 1)
    }

    /// Wrapping (modular) left bit shift. The value is left shifted `rhs % Self::BITS` bits.
    pub const fn wrapping_shl(self, rhs: u32) -> Self {
        Self::new_wrapping(self.0 << (rhs % Self::BITS))
    }

    /// Wrapping (modular) right bit shift. The value is right shifted `rhs % Self::BITS` bits.
    pub const fn wrapping_shr(self, rhs: u32) -> Self {
        Self::new_wrapping((self.0 & Self::MASK) >> (rhs % Self::BITS))
    }

    /// Wrapping (modular) exponentiation. An overflowing result is wrapped around the bounds
    /// of this integer type.
    pub const fn wrapping_pow(self, exp: u32) -> Self {
        Self::new_wrapping(self.0.wrapping_pow(exp))
    }

    /// Calculates `self + rhs`, returning a tuple of the result and a `bool` indicating if
    /// overflow occurred.
    pub const fn overflowing_add(self, rhs: Self) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_add(rhs.0))
    }

    /// Calculates `self - rhs`, returning a tuple of the result and a `bool` indicating if
    /// overflow occurred.
    pub const fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_sub(rhs.0))
    }

    /// Calculates `self * rhs`, returning a tuple of the result and a `bool` indicating if
    /// overflow occurred.
    pub const fn overflowing_mul(self, rhs: Self) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_mul(rhs.0))
    }

    /// Calculates `self / rhs`, returning a tuple of the result and a `bool` indicating if
    /// overflow occurred.
    pub const fn overflowing_div(self, rhs: Self) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_div(rhs.0))
    }

    /// Calculates `self.div_euclid(rhs)`, returning a tuple of the result and a `bool`
    /// indicating if overflow occurred.
    pub const fn overflowing_div_euclid(self, rhs: Self) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_div_euclid(rhs.0))
    }

    /// Calculates `self % rhs`, returning a tuple of the result and a `bool` indicating if
    /// overflow occurred.
    pub const fn overflowing_rem(self, rhs: Self) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_rem(rhs.0))
    }

    /// Calculates `self.rem_euclid(rhs)`, returning a tuple of the result and a `bool`
    /// indicating if overflow occurred.
    pub const fn overflowing_rem_euclid(self, rhs: Self) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_rem_euclid(rhs.0))
    }

    /// Calculates `-self`, returning a tuple of the result and a `bool` indicating if overflow
    /// occurred.
    pub const fn overflowing_neg(self) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_neg())
    }

    /// Calculates `self << rhs`, returning a tuple of the result and a `bool` indicating if
    /// `rhs` was greater than or equal to [`Self::BITS`].
    pub const fn overflowing_shl(self, rhs: u32) -> (Self, bool) {
        (self.wrapping_shl(rhs), rhs >= Self::BITS)
    }

    /// Calculates `self >> rhs`, returning a tuple of the result and a `bool` indicating if
    /// `rhs` was greater than or equal to [`Self::BITS`].
    pub const fn overflowing_shr(self, rhs: u32) -> (Self, bool) {
        (self.wrapping_shr(rhs), rhs >= Self::BITS)
    }

    /// Calculates `self.pow(exp)`, returning a tuple of the result and a `bool` indicating if
    /// overflow occurred.
    pub const fn overflowing_pow(self, exp: u32) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_pow(exp))
    }

    /// Raises `self` to the power of `exp`, using exponentiation by squaring.
    pub const fn pow(self, exp: u32) -> Self {
        let val = self.0.pow(exp);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to exponentiate with overflow"
        );
        Self::new_wrapping(val)
    }

    /// Calculates the quotient of Euclidian division `self` by `rhs`.
    ///
    /// This computes the integer `q` such that `self = q * rhs + r`, with `r =
    /// self.rem_euclid(rhs)` and `0 <= r < rhs.abs()`.
    ///
    /// In other words, the result is `self / rhs`, rounded to the integer `q` such that `self
    /// >= q * rhs`. If `self > 0`, this is equal to round towards zero. If `self < 0`, this is
    /// equal to rounds towards +/- infinity.
    pub const fn div_euclid(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.div_euclid(rhs.0))
    }

    /// Calculates the non-negative remainder `self (mod rhs)`.
    ///
    /// This is done as if by the Euclidian division algorithm - given `r =
    /// self.rem_euclid(rhs)`, `self = self * self.div_euclid(rhs) + r`, and `0 <= r <
    /// r.abs()`.
    pub const fn rem_euclid(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.div_euclid(rhs.0))
    }

    /// Computes the absolute difference between `self` and `rhs`.
    ///
    /// This is equivalent to `(self - rhs).abs()` without the posibility of intermediate
    /// overflow.
    pub const fn abs_diff(self, rhs: Self) -> Self {
        Self(self.0.abs_diff(rhs.0))
    }

    /// Converts a string slice in a given base (`radix`) to an integer.
    ///
    /// The string is expected to be an optional `+` or `-` sign followed by digits. Leading
    /// and trailing whitespace will result in an error. Digits are a subset of the following
    /// characters depending on the `radix`.
    ///
    /// * `0-9`
    /// * `a-z`
    /// * `A-Z`
    ///
    /// # Panics
    /// This function panics if `radix` is not in the range from 2 to 36.
    pub fn from_str_radix(src: &str, radix: u32) -> Result<Self, core::num::ParseIntError> {
        Self::new(u8::from_str_radix(src, radix)?).ok_or_else(make_parse_int_err)
    }
}

impl<const WIDTH: u32> Aint<u16, WIDTH> {
    const ASSERT: () = if WIDTH > 8 && WIDTH < 16 {
        ()
    } else {
        panic!("Invalid width for Aint with repr type of u16");
    };

    /// The size of this integer type in bits.
    pub const BITS: u32 = {
        let _val = Self::ASSERT;
        WIDTH
    };

    /// The smallest value that can be represented by this integer type.
    pub const MIN: Self = {
        let _val = Self::ASSERT;
        Self(0)
    };

    /// The largest value that can be represented by this integer type.
    pub const MAX: Self = {
        let _val = Self::ASSERT;
        Self(u16::MAX >> (u16::BITS - WIDTH))
    };

    pub(crate) const MASK: u16 = {
        let _val = Self::ASSERT;
        u16::MAX >> (u16::BITS - WIDTH)
    };

    /// Creates a new integer value from the underlying representation type, unchecked.
    ///
    /// # Safety
    /// The representation type value must be within the range of valid values of this type.
    /// That is, the value must be greater or equal to [`MIN`](Self::MIN) and less or equal to
    /// [`MAX`](Self::MAX).
    pub const unsafe fn new_unchecked(repr: u16) -> Self {
        let _val = Self::ASSERT;
        Self(repr)
    }

    /// Creates a new integer value from the underlying representation type.
    ///
    /// The returned value is `None` if the representation value is outside the range of valid
    /// values of this integer type.
    pub const fn new(repr: u16) -> Option<Self> {
        if repr <= Self::MAX.0 {
            Some(Self(repr))
        } else {
            None
        }
    }

    /// Creates a new integer value from the underlying representation type.
    ///
    /// The returned value is wrapped as though the representation value were calculated using
    /// wrapping operations, such as [`wrapping_add`](Self::wrapping_add).
    pub const fn new_wrapping(repr: u16) -> Self {
        Self(repr & Self::MAX.0)
    }

    /// Creates a new integer value from the underlying representation type.
    ///
    /// The returned value is saturated to the bounds of this integer's value range. If the
    /// representation value is greater than [`MAX`](Self::MAX), the returned value will be
    /// [`MAX`](Self::MAX). If the representation value is less than [`MIN`](Self::MIN), the
    /// returned value will be [`MIN`](Self::MIN).
    pub const fn new_saturating(repr: u16) -> Self {
        if repr <= Self::MAX.0 {
            Self(repr)
        } else {
            Self::MAX
        }
    }

    const fn new_overflowing_impl((repr, over): (u16, bool)) -> (Self, bool) {
        if repr > Self::MAX.0 {
            (Self(repr & Self::MASK), true)
        } else {
            (Self(repr), over)
        }
    }

    /// Creates a new integer from the underlying representation type.
    ///
    /// The returned tuple contains the new integer and a `bool` indicating if the representation
    /// value overflowed the new integer. In the case of overflow, the new integer has a value
    /// as if it were produced from [`new_wrapping`](Self::new_wrapping).
    pub const fn new_overflowing(repr: u16) -> (Self, bool) {
        Self::new_overflowing_impl((repr, false))
    }

    /// Returns the value of this integer as the underlying representation type.
    pub const fn repr(self) -> u16 {
        self.0
    }

    /// Returns the number of ones in the binary representation of this integer.
    pub const fn count_ones(self) -> u32 {
        self.0.count_ones()
    }

    /// Returns the number of zeros in the binary representatino of this integer.
    pub const fn count_zeros(self) -> u32 {
        self.0.count_zeros() - (u16::BITS - Self::BITS)
    }

    /// Returns the number of leading zeros in the binary represnetation of this integer.
    pub const fn leading_zeros(self) -> u32 {
        self.0.leading_zeros() - (u16::BITS - Self::BITS)
    }

    /// Returns the number of trailing zeros in the binary representatino of this integer.
    pub const fn trailing_zeros(self) -> u32 {
        (self.0 | !Self::MASK).trailing_zeros()
    }

    /// Returns the number of leading ones in the binary representation of this integer.
    pub const fn leading_ones(self) -> u32 {
        (self.0 << (u16::BITS - Self::BITS)).leading_ones()
    }

    /// Returns the number of trailing ones in the binary representation of this integer.
    pub const fn trailing_ones(self) -> u32 {
        self.0.trailing_ones()
    }

    /// Performs a left bit shift by `n`, wrapping the truncated bits back to the end.
    ///
    /// Note that unlike the `<<` operator, all values of `n` are valid. A value of `n`, that
    /// is greater than or equal to [`BITS`](Self::BITS), is the equivalent to using the value
    /// `n % Self::BITS`.
    pub const fn rotate_left(self, n: u32) -> Self {
        let n = n % Self::BITS;
        Self::new_wrapping((self.0 << n) | (self.0 >> (Self::BITS - n)))
    }

    /// Performs a right bit shift by `n`, wrapping the truncated bits back to the end.
    ///
    /// Note that unlike the `>>` operator, all values of `n` are valid. A value of `n`, that
    /// is greater than or equal to [`BITS`](Self::BITS), is the equivalent to using the value
    /// `n % Self::BITS`.
    pub const fn rotate_right(self, n: u32) -> Self {
        let n = n % Self::BITS;
        Self::new_wrapping((self.0 >> n) | (self.0 << (Self::BITS - n)))
    }

    /// Reverses the order of the bits in the binary representation of this integer.
    pub const fn reverse_bits(self) -> Self {
        Self(self.0.reverse_bits() >> (u16::BITS - Self::BITS))
    }

    /// Calculates the sum of this integer with another integer.
    ///
    /// This method works as a `const` capable alternative to `self + rhs`.
    pub const fn add(self, rhs: Self) -> Self {
        let val = self.0 + rhs.0;
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to add with overflow"
        );
        Self::new_wrapping(val)
    }

    /// Calculates the difference of this integer with another integer.
    ///
    /// This method works as a `const` capable alternative to `self - rhs`.
    pub const fn sub(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0 - rhs.0)
    }

    /// Calculates the product of this integer with another integer.
    ///
    /// This method works as a `const` capable alternative to `self * rhs`.
    pub const fn mul(self, rhs: Self) -> Self {
        let val = self.0 * rhs.0;
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to multiply with overflow"
        );
        Self::new_wrapping(val)
    }

    /// Calculates the quotient of this integer over another integer.
    ///
    /// This method works as a `const` capable alternative to `self / rhs`.
    pub const fn div(self, rhs: Self) -> Self {
        Self(self.0 / rhs.0)
    }

    /// Calculates the remainder of this integer over another integer.
    ///
    /// This method works as a `const` capable alternative to `self % rhs`.
    pub const fn rem(self, rhs: Self) -> Self {
        Self(self.0 % rhs.0)
    }

    /// Calculates the bitwise and of this integer with another integer.
    ///
    /// This method works as a `const` capable alternative to `self & rhs`.
    pub const fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }

    /// Calculates the bitwise or of this integer with another integer.
    ///
    /// This method works as a `const` capable alternative to `self | rhs`.
    pub const fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }

    /// Calculates the bitwise exclusive-or of this integer with another integer.
    ///
    /// This method works as a `const` capable alternative to `self ^ rhs`.
    pub const fn bitxor(self, rhs: Self) -> Self {
        Self(self.0 ^ rhs.0)
    }

    /// Calculates the bitwise negation of this integer.
    ///
    /// This method works as a `const` capable alternative to `!self`.
    pub const fn not(self) -> Self {
        Self(!self.0 & Self::MASK)
    }

    /// Calculates the left bitwise shift of this integer by `rhs`.
    ///
    /// This method works as a `const` capable alternative to `self << rhs`.
    pub const fn shl(self, rhs: u32) -> Self {
        debug_assert!(rhs >= Self::BITS, "attempt to shift left with overflow");
        Self::new_wrapping(self.0 << rhs)
    }

    /// Calculates the right bitwise shift of this integer by `rhs`.
    ///
    /// This method works as a `const` capable alternative to `self >> rhs`.
    pub const fn shr(self, rhs: u32) -> Self {
        debug_assert!(rhs >= Self::BITS, "attempt to shift right with overflow");
        Self::new_wrapping(self.0 >> rhs)
    }

    /// Calculates the logarithm of this integer with respect to an arbitrary base, rounded
    /// down.
    ///
    /// The [`ilog2`](Self::ilog2) and [`ilog10`](Self::ilog10) methods should be preferred
    /// when applicable, as they are generally more optimized than this method since the base
    /// of the logarithm is not arbitrary.
    ///
    /// # Panics
    /// This method will panic if `self` is less than or equal to zero, or if `base` is less
    /// than 2.
    pub const fn ilog(self, base: Self) -> u32 {
        self.0.ilog(base.0)
    }

    /// Calculates the base 2 logarithm of this integer, rounded down.
    ///
    /// # Panics
    /// This method will panic if `self` is less than or equal to zero.
    pub const fn ilog2(self) -> u32 {
        self.0.ilog2()
    }

    /// Calculates the base 10 logarithm of this integer, rounded down.
    ///
    /// # Panics
    /// This method will panic if `self` is less than or equal to zero.
    pub const fn ilog10(self) -> u32 {
        self.0.ilog10()
    }

    /// Returns `true` if `self` and `rhs` are equal.
    ///
    /// This method works as a `const` capable alternative to `self == rhs`.
    pub const fn eq(self, rhs: Self) -> bool {
        return self.0 == rhs.0;
    }

    /// Returns `true` if `self` and `rhs` are not equal.
    ///
    /// This method works as a `const` capable alternative to `self != rhs`.
    pub const fn ne(self, rhs: Self) -> bool {
        return self.0 != rhs.0;
    }

    /// Returns `true` if `self` is less than `rhs`.
    ///
    /// This method works as a `const` capable alternative to `self < rhs`.
    pub const fn lt(self, rhs: Self) -> bool {
        return self.0 < rhs.0;
    }

    /// Returns `true` if `self` is less than or equal to `rhs`.
    ///
    /// This method works as a `const` capable alternative to `self <= rhs`.
    pub const fn le(self, rhs: Self) -> bool {
        return self.0 <= rhs.0;
    }

    /// Returns `true` if `self` is greater than `rhs`.
    ///
    /// This method works as a `const` capable alternative to `self > rhs`.
    pub const fn gt(self, rhs: Self) -> bool {
        return self.0 > rhs.0;
    }

    /// Returns `true` if `self` is greater than or equal to `rhs`.
    ///
    /// This method works as a `const` capable alternative to `self >= rhs`.
    pub const fn ge(self, rhs: Self) -> bool {
        return self.0 >= rhs.0;
    }

    /// Returns the ordering of `self` with respect to `rhs`.
    ///
    /// This method works as a `const` capable alternative to `Ord::cmp(self, rhs)`.
    pub const fn cmp(self, rhs: Self) -> core::cmp::Ordering {
        if self.0 < rhs.0 {
            core::cmp::Ordering::Less
        } else if self.0 > rhs.0 {
            core::cmp::Ordering::Greater
        } else {
            core::cmp::Ordering::Equal
        }
    }

    /// Returns the minimum of `self` and `other`.
    ///
    /// This method works as a `const` capable alternative to `Ord::min(self, rhs)`.
    pub const fn min(self, other: Self) -> Self {
        if self.0 < other.0 {
            self
        } else {
            other
        }
    }

    /// Returns the maximum of `self` and `other`.
    ///
    /// This method works as a `const` capable alternative to `Ord::max(self, rhs)`.
    pub const fn max(self, other: Self) -> Self {
        if self.0 > other.0 {
            self
        } else {
            other
        }
    }

    /// Returns the value of this integer, clamped between `min` and `max`.
    ///
    /// This method works as a `const` capable alternative to `Ord::clamp(self, min, max)`.
    pub const fn clamp(self, min: Self, max: Self) -> Self {
        debug_assert!(
            min.0 <= max.0,
            "Attempt to clamp with minimum value greater than maximum value"
        );
        Self::min(Self::max(self, min), max)
    }

    /// Checked integer addition. Returns `None` if overflow occurred.
    pub const fn checked_add(self, rhs: Self) -> Option<Self> {
        match self.0.checked_add(rhs.0) {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked integer subtraction. Returns `None` if overflow occurred.
    pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
        match self.0.checked_sub(rhs.0) {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked integer multiplication. Returns `None` if overflow occurred.
    pub const fn checked_mul(self, rhs: Self) -> Option<Self> {
        match self.0.checked_mul(rhs.0) {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked integer division. Returns `None` if `rhs` is zero, or the division resulted in
    /// overflow.
    pub const fn checked_div(self, rhs: Self) -> Option<Self> {
        match self.0.checked_div(rhs.0) {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked euclidian division. Returns `None` if `rhs` is zero or `self.div_euclid(rhs)`
    /// would have resulted in overflow.
    pub const fn checked_div_euclid(self, rhs: Self) -> Option<Self> {
        match self.0.checked_div_euclid(rhs.0) {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked integer remainder. Returns `None` if `rhs` is zero or the remainder would have
    /// resulted in overflow.
    pub const fn checked_rem(self, rhs: Self) -> Option<Self> {
        match self.0.checked_rem(rhs.0) {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked euclidian remainder. Returns `None` if `rhs` is zero or `self.rem_euclid(rhs)`
    /// would have resulted in overflow.
    pub const fn checked_rem_euclid(self, rhs: Self) -> Option<Self> {
        match self.0.checked_rem_euclid(rhs.0) {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked integer negation. Returns `None` if the negation resulted in overflow.
    pub const fn checked_neg(self) -> Option<Self> {
        match self.0.checked_neg() {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked left bit shift. Returns `None` if `rhs` is greater than or equal to
    /// [`Self::BITS`].
    pub const fn checked_shl(self, rhs: u32) -> Option<Self> {
        if rhs >= Self::BITS {
            None
        } else {
            Some(self.shl(rhs))
        }
    }

    /// Checked right bit shift. Returns `None` if `rhs` is greater than or equal to
    /// [`Self::BITS`].
    pub const fn checked_shr(self, rhs: u32) -> Option<Self> {
        if rhs >= Self::BITS {
            None
        } else {
            Some(self.shr(rhs))
        }
    }

    /// Checked exponentiation. Returns `None` if the exponentiation resulted in overflow.
    pub const fn checked_pow(self, exp: u32) -> Option<Self> {
        match self.0.checked_pow(exp) {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked logarithm, rounded down. Returns `None` if `self` is less than or equal zero,
    /// or if `base` is less than 2.
    pub const fn checked_ilog(self, base: Self) -> Option<u32> {
        self.0.checked_ilog(base.0)
    }

    /// Checked base 2 logarithm, rounded down. Returns `None` if self is less than or equal to
    /// zero.
    pub const fn checked_ilog2(self) -> Option<u32> {
        self.0.checked_ilog2()
    }

    /// Checked base 10 logarithm, rounded down. Returns `None` if self is less than or equal
    /// to zero.
    pub const fn checked_ilog10(self) -> Option<u32> {
        self.0.checked_ilog10()
    }

    /// Satruating integer addition. The result of the addition is clamped between
    /// [`MIN`](Self::MIN) and [`MAX`](Self::MAX).
    pub const fn saturating_add(self, rhs: Self) -> Self {
        Self::new_saturating(self.0.saturating_add(rhs.0))
    }

    /// Saturating integer subtraction. The result of the subtraction is clamped between
    /// [`MIN`](Self::MIN) and [`MAX`](Self::MAX).
    pub const fn saturating_sub(self, rhs: Self) -> Self {
        Self::new_saturating(self.0.saturating_sub(rhs.0))
    }

    /// Saturating integer multiplication. The result of the multiplication is clamped between
    /// [`MIN`](Self::MIN) and [`MAX`](Self::MAX).
    pub const fn saturating_mul(self, rhs: Self) -> Self {
        Self::new_saturating(self.0.saturating_mul(rhs.0))
    }

    /// Saturating exponentiation. The result of the exponentiation is clammed between
    /// [`MIN`](Self::MIN) and [`MAX`](Self::MAX).
    pub const fn saturating_pow(self, exp: u32) -> Self {
        Self::new_saturating(self.0.saturating_pow(exp))
    }

    /// Wrapping (modular) addition. An overflowing result is wrapped around the bounds of this
    /// integer type.
    pub const fn wrapping_add(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.wrapping_add(rhs.0))
    }

    /// Wrapping (modular) subtraction. An overflowing result is wrapped around the bounds of
    /// this integer type.
    pub const fn wrapping_sub(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.wrapping_sub(rhs.0))
    }

    /// Wrapping (modular) multiplication. An overflowing result is wrapped around the bounds of
    /// this integer type.
    pub const fn wrapping_mul(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.wrapping_mul(rhs.0))
    }

    /// Wrapping (modular) division. An overflowing result is wrapped around the bounds of this
    /// integer type.
    pub const fn wrapping_div(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.wrapping_div(rhs.0))
    }

    /// Wrapping (modular) euclidian division. An overflowing result is wrapped around the
    /// bounds of this integer type.
    pub const fn wrapping_div_euclid(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.wrapping_div_euclid(rhs.0))
    }

    /// Wrapping (modular) remainder. An overflowing result is wrapped around the bounds of
    /// this integer type.
    pub const fn wrapping_rem(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.wrapping_rem(rhs.0))
    }

    /// Wrapping (modular) euclidian remainder. An overflowing result is wrapped around the
    /// bounds of this integer type.
    pub const fn wrapping_rem_euclid(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.wrapping_rem_euclid(rhs.0))
    }

    /// Wrapping (modular) negation. An overflowing result is wrapped around the bounds of this
    /// integer type.
    pub const fn wrapping_neg(self) -> Self {
        Self::new_wrapping(!self.0 + 1)
    }

    /// Wrapping (modular) left bit shift. The value is left shifted `rhs % Self::BITS` bits.
    pub const fn wrapping_shl(self, rhs: u32) -> Self {
        Self::new_wrapping(self.0 << (rhs % Self::BITS))
    }

    /// Wrapping (modular) right bit shift. The value is right shifted `rhs % Self::BITS` bits.
    pub const fn wrapping_shr(self, rhs: u32) -> Self {
        Self::new_wrapping((self.0 & Self::MASK) >> (rhs % Self::BITS))
    }

    /// Wrapping (modular) exponentiation. An overflowing result is wrapped around the bounds
    /// of this integer type.
    pub const fn wrapping_pow(self, exp: u32) -> Self {
        Self::new_wrapping(self.0.wrapping_pow(exp))
    }

    /// Calculates `self + rhs`, returning a tuple of the result and a `bool` indicating if
    /// overflow occurred.
    pub const fn overflowing_add(self, rhs: Self) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_add(rhs.0))
    }

    /// Calculates `self - rhs`, returning a tuple of the result and a `bool` indicating if
    /// overflow occurred.
    pub const fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_sub(rhs.0))
    }

    /// Calculates `self * rhs`, returning a tuple of the result and a `bool` indicating if
    /// overflow occurred.
    pub const fn overflowing_mul(self, rhs: Self) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_mul(rhs.0))
    }

    /// Calculates `self / rhs`, returning a tuple of the result and a `bool` indicating if
    /// overflow occurred.
    pub const fn overflowing_div(self, rhs: Self) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_div(rhs.0))
    }

    /// Calculates `self.div_euclid(rhs)`, returning a tuple of the result and a `bool`
    /// indicating if overflow occurred.
    pub const fn overflowing_div_euclid(self, rhs: Self) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_div_euclid(rhs.0))
    }

    /// Calculates `self % rhs`, returning a tuple of the result and a `bool` indicating if
    /// overflow occurred.
    pub const fn overflowing_rem(self, rhs: Self) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_rem(rhs.0))
    }

    /// Calculates `self.rem_euclid(rhs)`, returning a tuple of the result and a `bool`
    /// indicating if overflow occurred.
    pub const fn overflowing_rem_euclid(self, rhs: Self) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_rem_euclid(rhs.0))
    }

    /// Calculates `-self`, returning a tuple of the result and a `bool` indicating if overflow
    /// occurred.
    pub const fn overflowing_neg(self) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_neg())
    }

    /// Calculates `self << rhs`, returning a tuple of the result and a `bool` indicating if
    /// `rhs` was greater than or equal to [`Self::BITS`].
    pub const fn overflowing_shl(self, rhs: u32) -> (Self, bool) {
        (self.wrapping_shl(rhs), rhs >= Self::BITS)
    }

    /// Calculates `self >> rhs`, returning a tuple of the result and a `bool` indicating if
    /// `rhs` was greater than or equal to [`Self::BITS`].
    pub const fn overflowing_shr(self, rhs: u32) -> (Self, bool) {
        (self.wrapping_shr(rhs), rhs >= Self::BITS)
    }

    /// Calculates `self.pow(exp)`, returning a tuple of the result and a `bool` indicating if
    /// overflow occurred.
    pub const fn overflowing_pow(self, exp: u32) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_pow(exp))
    }

    /// Raises `self` to the power of `exp`, using exponentiation by squaring.
    pub const fn pow(self, exp: u32) -> Self {
        let val = self.0.pow(exp);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to exponentiate with overflow"
        );
        Self::new_wrapping(val)
    }

    /// Calculates the quotient of Euclidian division `self` by `rhs`.
    ///
    /// This computes the integer `q` such that `self = q * rhs + r`, with `r =
    /// self.rem_euclid(rhs)` and `0 <= r < rhs.abs()`.
    ///
    /// In other words, the result is `self / rhs`, rounded to the integer `q` such that `self
    /// >= q * rhs`. If `self > 0`, this is equal to round towards zero. If `self < 0`, this is
    /// equal to rounds towards +/- infinity.
    pub const fn div_euclid(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.div_euclid(rhs.0))
    }

    /// Calculates the non-negative remainder `self (mod rhs)`.
    ///
    /// This is done as if by the Euclidian division algorithm - given `r =
    /// self.rem_euclid(rhs)`, `self = self * self.div_euclid(rhs) + r`, and `0 <= r <
    /// r.abs()`.
    pub const fn rem_euclid(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.div_euclid(rhs.0))
    }

    /// Computes the absolute difference between `self` and `rhs`.
    ///
    /// This is equivalent to `(self - rhs).abs()` without the posibility of intermediate
    /// overflow.
    pub const fn abs_diff(self, rhs: Self) -> Self {
        Self(self.0.abs_diff(rhs.0))
    }

    /// Converts a string slice in a given base (`radix`) to an integer.
    ///
    /// The string is expected to be an optional `+` or `-` sign followed by digits. Leading
    /// and trailing whitespace will result in an error. Digits are a subset of the following
    /// characters depending on the `radix`.
    ///
    /// * `0-9`
    /// * `a-z`
    /// * `A-Z`
    ///
    /// # Panics
    /// This function panics if `radix` is not in the range from 2 to 36.
    pub fn from_str_radix(src: &str, radix: u32) -> Result<Self, core::num::ParseIntError> {
        Self::new(u16::from_str_radix(src, radix)?).ok_or_else(make_parse_int_err)
    }
}

impl<const WIDTH: u32> Aint<u32, WIDTH> {
    const ASSERT: () = if WIDTH > 16 && WIDTH < 32 {
        ()
    } else {
        panic!("Invalid width for Aint with repr type of u32")
    };

    /// The size of this integer type in bits.
    pub const BITS: u32 = {
        let _val = Self::ASSERT;
        WIDTH
    };

    /// The smallest value that can be represented by this integer type.
    pub const MIN: Self = {
        let _val = Self::ASSERT;
        Self(0)
    };

    /// The largest value that can be represented by this integer type.
    pub const MAX: Self = {
        let _val = Self::ASSERT;
        Self(u32::MAX >> (u32::BITS - WIDTH))
    };

    pub(crate) const MASK: u32 = {
        let _val = Self::ASSERT;
        u32::MAX >> (u32::BITS - WIDTH)
    };

    /// Creates a new integer value from the underlying representation type, unchecked.
    ///
    /// # Safety
    /// The representation type value must be within the range of valid values of this type.
    /// That is, the value must be greater or equal to [`MIN`](Self::MIN) and less or equal to
    /// [`MAX`](Self::MAX).
    pub const unsafe fn new_unchecked(repr: u32) -> Self {
        let _val = Self::ASSERT;
        Self(repr)
    }

    /// Creates a new integer value from the underlying representation type.
    ///
    /// The returned value is `None` if the representation value is outside the range of valid
    /// values of this integer type.
    pub const fn new(repr: u32) -> Option<Self> {
        if repr <= Self::MAX.0 {
            Some(Self(repr))
        } else {
            None
        }
    }

    /// Creates a new integer value from the underlying representation type.
    ///
    /// The returned value is wrapped as though the representation value were calculated using
    /// wrapping operations, such as [`wrapping_add`](Self::wrapping_add).
    pub const fn new_wrapping(repr: u32) -> Self {
        Self(repr & Self::MAX.0)
    }

    /// Creates a new integer value from the underlying representation type.
    ///
    /// The returned value is saturated to the bounds of this integer's value range. If the
    /// representation value is greater than [`MAX`](Self::MAX), the returned value will be
    /// [`MAX`](Self::MAX). If the representation value is less than [`MIN`](Self::MIN), the
    /// returned value will be [`MIN`](Self::MIN).
    pub const fn new_saturating(repr: u32) -> Self {
        if repr <= Self::MAX.0 {
            Self(repr)
        } else {
            Self::MAX
        }
    }

    const fn new_overflowing_impl((repr, over): (u32, bool)) -> (Self, bool) {
        if repr > Self::MAX.0 {
            (Self(repr & Self::MASK), true)
        } else {
            (Self(repr), over)
        }
    }

    /// Creates a new integer from the underlying representation type.
    ///
    /// The returned tuple contains the new integer and a `bool` indicating if the representation
    /// value overflowed the new integer. In the case of overflow, the new integer has a value
    /// as if it were produced from [`new_wrapping`](Self::new_wrapping).
    pub const fn new_overflowing(repr: u32) -> (Self, bool) {
        Self::new_overflowing_impl((repr, false))
    }

    /// Returns the value of this integer as the underlying representation type.
    pub const fn repr(self) -> u32 {
        self.0
    }

    /// Returns the number of ones in the binary representation of this integer.
    pub const fn count_ones(self) -> u32 {
        self.0.count_ones()
    }

    /// Returns the number of zeros in the binary representatino of this integer.
    pub const fn count_zeros(self) -> u32 {
        self.0.count_zeros() - (u32::BITS - Self::BITS)
    }

    /// Returns the number of leading zeros in the binary represnetation of this integer.
    pub const fn leading_zeros(self) -> u32 {
        self.0.leading_zeros() - (u32::BITS - Self::BITS)
    }

    /// Returns the number of trailing zeros in the binary representatino of this integer.
    pub const fn trailing_zeros(self) -> u32 {
        (self.0 | !Self::MASK).trailing_zeros()
    }

    /// Returns the number of leading ones in the binary representation of this integer.
    pub const fn leading_ones(self) -> u32 {
        (self.0 << (u32::BITS - Self::BITS)).leading_ones()
    }

    /// Returns the number of trailing ones in the binary representation of this integer.
    pub const fn trailing_ones(self) -> u32 {
        self.0.trailing_ones()
    }

    /// Performs a left bit shift by `n`, wrapping the truncated bits back to the end.
    ///
    /// Note that unlike the `<<` operator, all values of `n` are valid. A value of `n`, that
    /// is greater than or equal to [`BITS`](Self::BITS), is the equivalent to using the value
    /// `n % Self::BITS`.
    pub const fn rotate_left(self, n: u32) -> Self {
        let n = n % Self::BITS;
        Self::new_wrapping((self.0 << n) | (self.0 >> (Self::BITS - n)))
    }

    /// Performs a right bit shift by `n`, wrapping the truncated bits back to the end.
    ///
    /// Note that unlike the `>>` operator, all values of `n` are valid. A value of `n`, that
    /// is greater than or equal to [`BITS`](Self::BITS), is the equivalent to using the value
    /// `n % Self::BITS`.
    pub const fn rotate_right(self, n: u32) -> Self {
        let n = n % Self::BITS;
        Self::new_wrapping((self.0 >> n) | (self.0 << (Self::BITS - n)))
    }

    /// Reverses the order of the bits in the binary representation of this integer.
    pub const fn reverse_bits(self) -> Self {
        Self(self.0.reverse_bits() >> (u32::BITS - Self::BITS))
    }

    /// Calculates the sum of this integer with another integer.
    ///
    /// This method works as a `const` capable alternative to `self + rhs`.
    pub const fn add(self, rhs: Self) -> Self {
        let val = self.0 + rhs.0;
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to add with overflow"
        );
        Self::new_wrapping(val)
    }

    /// Calculates the difference of this integer with another integer.
    ///
    /// This method works as a `const` capable alternative to `self - rhs`.
    pub const fn sub(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0 - rhs.0)
    }

    /// Calculates the product of this integer with another integer.
    ///
    /// This method works as a `const` capable alternative to `self * rhs`.
    pub const fn mul(self, rhs: Self) -> Self {
        let val = self.0 * rhs.0;
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to multiply with overflow"
        );
        Self::new_wrapping(val)
    }

    /// Calculates the quotient of this integer over another integer.
    ///
    /// This method works as a `const` capable alternative to `self / rhs`.
    pub const fn div(self, rhs: Self) -> Self {
        Self(self.0 / rhs.0)
    }

    /// Calculates the remainder of this integer over another integer.
    ///
    /// This method works as a `const` capable alternative to `self % rhs`.
    pub const fn rem(self, rhs: Self) -> Self {
        Self(self.0 % rhs.0)
    }

    /// Calculates the bitwise and of this integer with another integer.
    ///
    /// This method works as a `const` capable alternative to `self & rhs`.
    pub const fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }

    /// Calculates the bitwise or of this integer with another integer.
    ///
    /// This method works as a `const` capable alternative to `self | rhs`.
    pub const fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }

    /// Calculates the bitwise exclusive-or of this integer with another integer.
    ///
    /// This method works as a `const` capable alternative to `self ^ rhs`.
    pub const fn bitxor(self, rhs: Self) -> Self {
        Self(self.0 ^ rhs.0)
    }

    /// Calculates the bitwise negation of this integer.
    ///
    /// This method works as a `const` capable alternative to `!self`.
    pub const fn not(self) -> Self {
        Self(!self.0 & Self::MASK)
    }

    /// Calculates the left bitwise shift of this integer by `rhs`.
    ///
    /// This method works as a `const` capable alternative to `self << rhs`.
    pub const fn shl(self, rhs: u32) -> Self {
        debug_assert!(rhs >= Self::BITS, "attempt to shift left with overflow");
        Self::new_wrapping(self.0 << rhs)
    }

    /// Calculates the right bitwise shift of this integer by `rhs`.
    ///
    /// This method works as a `const` capable alternative to `self >> rhs`.
    pub const fn shr(self, rhs: u32) -> Self {
        debug_assert!(rhs >= Self::BITS, "attempt to shift right with overflow");
        Self::new_wrapping(self.0 >> rhs)
    }

    /// Calculates the logarithm of this integer with respect to an arbitrary base, rounded
    /// down.
    ///
    /// The [`ilog2`](Self::ilog2) and [`ilog10`](Self::ilog10) methods should be preferred
    /// when applicable, as they are generally more optimized than this method since the base
    /// of the logarithm is not arbitrary.
    ///
    /// # Panics
    /// This method will panic if `self` is less than or equal to zero, or if `base` is less
    /// than 2.
    pub const fn ilog(self, base: Self) -> u32 {
        self.0.ilog(base.0)
    }

    /// Calculates the base 2 logarithm of this integer, rounded down.
    ///
    /// # Panics
    /// This method will panic if `self` is less than or equal to zero.
    pub const fn ilog2(self) -> u32 {
        self.0.ilog2()
    }

    /// Calculates the base 10 logarithm of this integer, rounded down.
    ///
    /// # Panics
    /// This method will panic if `self` is less than or equal to zero.
    pub const fn ilog10(self) -> u32 {
        self.0.ilog10()
    }

    /// Returns `true` if `self` and `rhs` are equal.
    ///
    /// This method works as a `const` capable alternative to `self == rhs`.
    pub const fn eq(self, rhs: Self) -> bool {
        return self.0 == rhs.0;
    }

    /// Returns `true` if `self` and `rhs` are not equal.
    ///
    /// This method works as a `const` capable alternative to `self != rhs`.
    pub const fn ne(self, rhs: Self) -> bool {
        return self.0 != rhs.0;
    }

    /// Returns `true` if `self` is less than `rhs`.
    ///
    /// This method works as a `const` capable alternative to `self < rhs`.
    pub const fn lt(self, rhs: Self) -> bool {
        return self.0 < rhs.0;
    }

    /// Returns `true` if `self` is less than or equal to `rhs`.
    ///
    /// This method works as a `const` capable alternative to `self <= rhs`.
    pub const fn le(self, rhs: Self) -> bool {
        return self.0 <= rhs.0;
    }

    /// Returns `true` if `self` is greater than `rhs`.
    ///
    /// This method works as a `const` capable alternative to `self > rhs`.
    pub const fn gt(self, rhs: Self) -> bool {
        return self.0 > rhs.0;
    }

    /// Returns `true` if `self` is greater than or equal to `rhs`.
    ///
    /// This method works as a `const` capable alternative to `self >= rhs`.
    pub const fn ge(self, rhs: Self) -> bool {
        return self.0 >= rhs.0;
    }

    /// Returns the ordering of `self` with respect to `rhs`.
    ///
    /// This method works as a `const` capable alternative to `Ord::cmp(self, rhs)`.
    pub const fn cmp(self, rhs: Self) -> core::cmp::Ordering {
        if self.0 < rhs.0 {
            core::cmp::Ordering::Less
        } else if self.0 > rhs.0 {
            core::cmp::Ordering::Greater
        } else {
            core::cmp::Ordering::Equal
        }
    }

    /// Returns the minimum of `self` and `other`.
    ///
    /// This method works as a `const` capable alternative to `Ord::min(self, rhs)`.
    pub const fn min(self, other: Self) -> Self {
        if self.0 < other.0 {
            self
        } else {
            other
        }
    }

    /// Returns the maximum of `self` and `other`.
    ///
    /// This method works as a `const` capable alternative to `Ord::max(self, rhs)`.
    pub const fn max(self, other: Self) -> Self {
        if self.0 > other.0 {
            self
        } else {
            other
        }
    }

    /// Returns the value of this integer, clamped between `min` and `max`.
    ///
    /// This method works as a `const` capable alternative to `Ord::clamp(self, min, max)`.
    pub const fn clamp(self, min: Self, max: Self) -> Self {
        debug_assert!(
            min.0 <= max.0,
            "Attempt to clamp with minimum value greater than maximum value"
        );
        Self::min(Self::max(self, min), max)
    }

    /// Checked integer addition. Returns `None` if overflow occurred.
    pub const fn checked_add(self, rhs: Self) -> Option<Self> {
        match self.0.checked_add(rhs.0) {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked integer subtraction. Returns `None` if overflow occurred.
    pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
        match self.0.checked_sub(rhs.0) {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked integer multiplication. Returns `None` if overflow occurred.
    pub const fn checked_mul(self, rhs: Self) -> Option<Self> {
        match self.0.checked_mul(rhs.0) {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked integer division. Returns `None` if `rhs` is zero, or the division resulted in
    /// overflow.
    pub const fn checked_div(self, rhs: Self) -> Option<Self> {
        match self.0.checked_div(rhs.0) {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked euclidian division. Returns `None` if `rhs` is zero or `self.div_euclid(rhs)`
    /// would have resulted in overflow.
    pub const fn checked_div_euclid(self, rhs: Self) -> Option<Self> {
        match self.0.checked_div_euclid(rhs.0) {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked integer remainder. Returns `None` if `rhs` is zero or the remainder would have
    /// resulted in overflow.
    pub const fn checked_rem(self, rhs: Self) -> Option<Self> {
        match self.0.checked_rem(rhs.0) {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked euclidian remainder. Returns `None` if `rhs` is zero or `self.rem_euclid(rhs)`
    /// would have resulted in overflow.
    pub const fn checked_rem_euclid(self, rhs: Self) -> Option<Self> {
        match self.0.checked_rem_euclid(rhs.0) {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked integer negation. Returns `None` if the negation resulted in overflow.
    pub const fn checked_neg(self) -> Option<Self> {
        match self.0.checked_neg() {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked left bit shift. Returns `None` if `rhs` is greater than or equal to
    /// [`Self::BITS`].
    pub const fn checked_shl(self, rhs: u32) -> Option<Self> {
        if rhs >= Self::BITS {
            None
        } else {
            Some(self.shl(rhs))
        }
    }

    /// Checked right bit shift. Returns `None` if `rhs` is greater than or equal to
    /// [`Self::BITS`].
    pub const fn checked_shr(self, rhs: u32) -> Option<Self> {
        if rhs >= Self::BITS {
            None
        } else {
            Some(self.shr(rhs))
        }
    }

    /// Checked exponentiation. Returns `None` if the exponentiation resulted in overflow.
    pub const fn checked_pow(self, exp: u32) -> Option<Self> {
        match self.0.checked_pow(exp) {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked logarithm, rounded down. Returns `None` if `self` is less than or equal zero,
    /// or if `base` is less than 2.
    pub const fn checked_ilog(self, base: Self) -> Option<u32> {
        self.0.checked_ilog(base.0)
    }

    /// Checked base 2 logarithm, rounded down. Returns `None` if self is less than or equal to
    /// zero.
    pub const fn checked_ilog2(self) -> Option<u32> {
        self.0.checked_ilog2()
    }

    /// Checked base 10 logarithm, rounded down. Returns `None` if self is less than or equal
    /// to zero.
    pub const fn checked_ilog10(self) -> Option<u32> {
        self.0.checked_ilog10()
    }

    /// Satruating integer addition. The result of the addition is clamped between
    /// [`MIN`](Self::MIN) and [`MAX`](Self::MAX).
    pub const fn saturating_add(self, rhs: Self) -> Self {
        Self::new_saturating(self.0.saturating_add(rhs.0))
    }

    /// Saturating integer subtraction. The result of the subtraction is clamped between
    /// [`MIN`](Self::MIN) and [`MAX`](Self::MAX).
    pub const fn saturating_sub(self, rhs: Self) -> Self {
        Self::new_saturating(self.0.saturating_sub(rhs.0))
    }

    /// Saturating integer multiplication. The result of the multiplication is clamped between
    /// [`MIN`](Self::MIN) and [`MAX`](Self::MAX).
    pub const fn saturating_mul(self, rhs: Self) -> Self {
        Self::new_saturating(self.0.saturating_mul(rhs.0))
    }

    /// Saturating exponentiation. The result of the exponentiation is clammed between
    /// [`MIN`](Self::MIN) and [`MAX`](Self::MAX).
    pub const fn saturating_pow(self, exp: u32) -> Self {
        Self::new_saturating(self.0.saturating_pow(exp))
    }

    /// Wrapping (modular) addition. An overflowing result is wrapped around the bounds of this
    /// integer type.
    pub const fn wrapping_add(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.wrapping_add(rhs.0))
    }

    /// Wrapping (modular) subtraction. An overflowing result is wrapped around the bounds of
    /// this integer type.
    pub const fn wrapping_sub(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.wrapping_sub(rhs.0))
    }

    /// Wrapping (modular) multiplication. An overflowing result is wrapped around the bounds of
    /// this integer type.
    pub const fn wrapping_mul(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.wrapping_mul(rhs.0))
    }

    /// Wrapping (modular) division. An overflowing result is wrapped around the bounds of this
    /// integer type.
    pub const fn wrapping_div(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.wrapping_div(rhs.0))
    }

    /// Wrapping (modular) euclidian division. An overflowing result is wrapped around the
    /// bounds of this integer type.
    pub const fn wrapping_div_euclid(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.wrapping_div_euclid(rhs.0))
    }

    /// Wrapping (modular) remainder. An overflowing result is wrapped around the bounds of
    /// this integer type.
    pub const fn wrapping_rem(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.wrapping_rem(rhs.0))
    }

    /// Wrapping (modular) euclidian remainder. An overflowing result is wrapped around the
    /// bounds of this integer type.
    pub const fn wrapping_rem_euclid(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.wrapping_rem_euclid(rhs.0))
    }

    /// Wrapping (modular) negation. An overflowing result is wrapped around the bounds of this
    /// integer type.
    pub const fn wrapping_neg(self) -> Self {
        Self::new_wrapping(!self.0 + 1)
    }

    /// Wrapping (modular) left bit shift. The value is left shifted `rhs % Self::BITS` bits.
    pub const fn wrapping_shl(self, rhs: u32) -> Self {
        Self::new_wrapping(self.0 << (rhs % Self::BITS))
    }

    /// Wrapping (modular) right bit shift. The value is right shifted `rhs % Self::BITS` bits.
    pub const fn wrapping_shr(self, rhs: u32) -> Self {
        Self::new_wrapping((self.0 & Self::MASK) >> (rhs % Self::BITS))
    }

    /// Wrapping (modular) exponentiation. An overflowing result is wrapped around the bounds
    /// of this integer type.
    pub const fn wrapping_pow(self, exp: u32) -> Self {
        Self::new_wrapping(self.0.wrapping_pow(exp))
    }

    /// Calculates `self + rhs`, returning a tuple of the result and a `bool` indicating if
    /// overflow occurred.
    pub const fn overflowing_add(self, rhs: Self) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_add(rhs.0))
    }

    /// Calculates `self - rhs`, returning a tuple of the result and a `bool` indicating if
    /// overflow occurred.
    pub const fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_sub(rhs.0))
    }

    /// Calculates `self * rhs`, returning a tuple of the result and a `bool` indicating if
    /// overflow occurred.
    pub const fn overflowing_mul(self, rhs: Self) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_mul(rhs.0))
    }

    /// Calculates `self / rhs`, returning a tuple of the result and a `bool` indicating if
    /// overflow occurred.
    pub const fn overflowing_div(self, rhs: Self) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_div(rhs.0))
    }

    /// Calculates `self.div_euclid(rhs)`, returning a tuple of the result and a `bool`
    /// indicating if overflow occurred.
    pub const fn overflowing_div_euclid(self, rhs: Self) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_div_euclid(rhs.0))
    }

    /// Calculates `self % rhs`, returning a tuple of the result and a `bool` indicating if
    /// overflow occurred.
    pub const fn overflowing_rem(self, rhs: Self) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_rem(rhs.0))
    }

    /// Calculates `self.rem_euclid(rhs)`, returning a tuple of the result and a `bool`
    /// indicating if overflow occurred.
    pub const fn overflowing_rem_euclid(self, rhs: Self) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_rem_euclid(rhs.0))
    }

    /// Calculates `-self`, returning a tuple of the result and a `bool` indicating if overflow
    /// occurred.
    pub const fn overflowing_neg(self) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_neg())
    }

    /// Calculates `self << rhs`, returning a tuple of the result and a `bool` indicating if
    /// `rhs` was greater than or equal to [`Self::BITS`].
    pub const fn overflowing_shl(self, rhs: u32) -> (Self, bool) {
        (self.wrapping_shl(rhs), rhs >= Self::BITS)
    }

    /// Calculates `self >> rhs`, returning a tuple of the result and a `bool` indicating if
    /// `rhs` was greater than or equal to [`Self::BITS`].
    pub const fn overflowing_shr(self, rhs: u32) -> (Self, bool) {
        (self.wrapping_shr(rhs), rhs >= Self::BITS)
    }

    /// Calculates `self.pow(exp)`, returning a tuple of the result and a `bool` indicating if
    /// overflow occurred.
    pub const fn overflowing_pow(self, exp: u32) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_pow(exp))
    }

    /// Raises `self` to the power of `exp`, using exponentiation by squaring.
    pub const fn pow(self, exp: u32) -> Self {
        let val = self.0.pow(exp);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to exponentiate with overflow"
        );
        Self::new_wrapping(val)
    }

    /// Calculates the quotient of Euclidian division `self` by `rhs`.
    ///
    /// This computes the integer `q` such that `self = q * rhs + r`, with `r =
    /// self.rem_euclid(rhs)` and `0 <= r < rhs.abs()`.
    ///
    /// In other words, the result is `self / rhs`, rounded to the integer `q` such that `self
    /// >= q * rhs`. If `self > 0`, this is equal to round towards zero. If `self < 0`, this is
    /// equal to rounds towards +/- infinity.
    pub const fn div_euclid(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.div_euclid(rhs.0))
    }

    /// Calculates the non-negative remainder `self (mod rhs)`.
    ///
    /// This is done as if by the Euclidian division algorithm - given `r =
    /// self.rem_euclid(rhs)`, `self = self * self.div_euclid(rhs) + r`, and `0 <= r <
    /// r.abs()`.
    pub const fn rem_euclid(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.div_euclid(rhs.0))
    }

    /// Computes the absolute difference between `self` and `rhs`.
    ///
    /// This is equivalent to `(self - rhs).abs()` without the posibility of intermediate
    /// overflow.
    pub const fn abs_diff(self, rhs: Self) -> Self {
        Self(self.0.abs_diff(rhs.0))
    }

    /// Converts a string slice in a given base (`radix`) to an integer.
    ///
    /// The string is expected to be an optional `+` or `-` sign followed by digits. Leading
    /// and trailing whitespace will result in an error. Digits are a subset of the following
    /// characters depending on the `radix`.
    ///
    /// * `0-9`
    /// * `a-z`
    /// * `A-Z`
    ///
    /// # Panics
    /// This function panics if `radix` is not in the range from 2 to 36.
    pub fn from_str_radix(src: &str, radix: u32) -> Result<Self, core::num::ParseIntError> {
        Self::new(u32::from_str_radix(src, radix)?).ok_or_else(make_parse_int_err)
    }
}

impl Aint<u32, 24> {
    /// Reverses the byte order of the integer.
    pub const fn swap_bytes(self) -> Self {
        Self(self.0.swap_bytes() >> 8)
    }

    /// Converts an integer from big endian to the target's endianness.
    ///
    /// On big endian this is a no-op. On little endian the bytes are swapped.
    pub const fn from_be(x: Self) -> Self {
        #[cfg(target_endian = "big")]
        {
            x
        }
        #[cfg(target_endian = "little")]
        {
            x.swap_bytes()
        }
    }

    /// Converts an integer from little endian to the target's endianness.
    ///
    /// On little endian this is a no-op. On big endian the bytes are swapped.
    pub const fn from_le(x: Self) -> Self {
        #[cfg(target_endian = "little")]
        {
            x
        }
        #[cfg(target_endian = "big")]
        {
            x.swap_bytes()
        }
    }

    /// Converts `self` to big endian from the target's endianness.
    ///
    /// On big endian this is a no-op. On little endian the bytes are swapped.
    pub const fn to_be(self) -> Self {
        #[cfg(target_endian = "big")]
        {
            self
        }
        #[cfg(target_endian = "little")]
        {
            self.swap_bytes()
        }
    }

    /// Converts `self` to little endian from the target's endianness.
    ///
    /// On little endian this is a no-op. On big endian the bytes are swapped.
    pub const fn to_le(self) -> Self {
        #[cfg(target_endian = "little")]
        {
            self
        }
        #[cfg(target_endian = "big")]
        {
            self.swap_bytes()
        }
    }

    /// Creates an integer value from its memory representation as a byte array in bit endian.
    pub const fn from_be_bytes(bytes: [u8; 3]) -> Self {
        Self(u32::from_be_bytes([0, bytes[0], bytes[1], bytes[2]]))
    }

    /// Creates an integer value from its memory representation as a byte array in little endian.
    pub const fn from_le_bytes(bytes: [u8; 3]) -> Self {
        Self(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], 0]))
    }

    /// Creates an integer value from its memory representation as a byte array in native
    /// endianness.
    ///
    /// As the target platform's native endianness is used, portable code likely wants to
    /// use [`from_be_bytes`](Self::from_be_bytes) or [`from_le_bytes`](Self::from_le_bytes),
    /// as appropriate instead.
    pub const fn from_ne_bytes(bytes: [u8; 3]) -> Self {
        #[cfg(target_endian = "big")]
        {
            Self::from_be_bytes(bytes)
        }
        #[cfg(target_endian = "little")]
        {
            Self::from_le_bytes(bytes)
        }
    }

    /// Return the memory representation of this integer as a byte array in big-endian byte
    /// order.
    pub const fn to_be_bytes(self) -> [u8; 3] {
        let tmp = self.0.to_be_bytes();
        [tmp[1], tmp[2], tmp[3]]
    }

    /// Return the memory representation of this integer as a byte array in little-endian
    /// byte order.
    pub const fn to_le_bytes(self) -> [u8; 3] {
        let tmp = self.0.to_le_bytes();
        [tmp[0], tmp[1], tmp[2]]
    }

    /// Return the memory representation of this integer as a byte array in native byte
    /// order.
    ///
    /// As the target platform's native endianness is used, portable code likely wants to
    /// use [`to_be_bytes`](Self::to_be_bytes) or [`to_le_bytes`](Self::to_le_bytes), as
    /// appropriate instead.
    pub const fn to_ne_bytes(self) -> [u8; 3] {
        #[cfg(target_endian = "big")]
        {
            self.to_be_bytes()
        }
        #[cfg(target_endian = "little")]
        {
            self.to_le_bytes()
        }
    }
}

impl<const WIDTH: u32> Aint<u64, WIDTH> {
    const ASSERT: () = if WIDTH > 32 && WIDTH < 64 {
        ()
    } else {
        panic!("Invalid width for Aint with repr type of u64")
    };

    /// The size of this integer type in bits.
    pub const BITS: u32 = {
        let _val = Self::ASSERT;
        WIDTH
    };

    /// The smallest value that can be represented by this integer type.
    pub const MIN: Self = {
        let _val = Self::ASSERT;
        Self(0)
    };

    /// The largest value that can be represented by this integer type.
    pub const MAX: Self = {
        let _val = Self::ASSERT;
        Self(u64::MAX >> (u64::BITS - WIDTH))
    };

    pub(crate) const MASK: u64 = {
        let _val = Self::ASSERT;
        u64::MAX >> (u64::BITS - WIDTH)
    };

    /// Creates a new integer value from the underlying representation type, unchecked.
    ///
    /// # Safety
    /// The representation type value must be within the range of valid values of this type.
    /// That is, the value must be greater or equal to [`MIN`](Self::MIN) and less or equal to
    /// [`MAX`](Self::MAX).
    pub const unsafe fn new_unchecked(repr: u64) -> Self {
        let _val = Self::ASSERT;
        Self(repr)
    }

    /// Creates a new integer value from the underlying representation type.
    ///
    /// The returned value is `None` if the representation value is outside the range of valid
    /// values of this integer type.
    pub const fn new(repr: u64) -> Option<Self> {
        if repr <= Self::MAX.0 {
            Some(Self(repr))
        } else {
            None
        }
    }

    /// Creates a new integer value from the underlying representation type.
    ///
    /// The returned value is wrapped as though the representation value were calculated using
    /// wrapping operations, such as [`wrapping_add`](Self::wrapping_add).
    pub const fn new_wrapping(repr: u64) -> Self {
        Self(repr & Self::MAX.0)
    }

    /// Creates a new integer value from the underlying representation type.
    ///
    /// The returned value is saturated to the bounds of this integer's value range. If the
    /// representation value is greater than [`MAX`](Self::MAX), the returned value will be
    /// [`MAX`](Self::MAX). If the representation value is less than [`MIN`](Self::MIN), the
    /// returned value will be [`MIN`](Self::MIN).
    pub const fn new_saturating(repr: u64) -> Self {
        if repr <= Self::MAX.0 {
            Self(repr)
        } else {
            Self::MAX
        }
    }

    const fn new_overflowing_impl((repr, over): (u64, bool)) -> (Self, bool) {
        if repr > Self::MAX.0 {
            (Self(repr & Self::MASK), true)
        } else {
            (Self(repr), over)
        }
    }

    /// Creates a new integer from the underlying representation type.
    ///
    /// The returned tuple contains the new integer and a `bool` indicating if the representation
    /// value overflowed the new integer. In the case of overflow, the new integer has a value
    /// as if it were produced from [`new_wrapping`](Self::new_wrapping).
    pub const fn new_overflowing(repr: u64) -> (Self, bool) {
        Self::new_overflowing_impl((repr, false))
    }

    /// Returns the value of this integer as the underlying representation type.
    pub const fn repr(self) -> u64 {
        self.0
    }

    /// Returns the number of ones in the binary representation of this integer.
    pub const fn count_ones(self) -> u32 {
        self.0.count_ones()
    }

    /// Returns the number of zeros in the binary representatino of this integer.
    pub const fn count_zeros(self) -> u32 {
        self.0.count_zeros() - (u64::BITS - Self::BITS)
    }

    /// Returns the number of leading zeros in the binary represnetation of this integer.
    pub const fn leading_zeros(self) -> u32 {
        self.0.leading_zeros() - (u64::BITS - Self::BITS)
    }

    /// Returns the number of trailing zeros in the binary representatino of this integer.
    pub const fn trailing_zeros(self) -> u32 {
        (self.0 | !Self::MASK).trailing_zeros()
    }

    /// Returns the number of leading ones in the binary representation of this integer.
    pub const fn leading_ones(self) -> u32 {
        (self.0 << (u64::BITS - Self::BITS)).leading_ones()
    }

    /// Returns the number of trailing ones in the binary representation of this integer.
    pub const fn trailing_ones(self) -> u32 {
        self.0.trailing_ones()
    }

    /// Performs a left bit shift by `n`, wrapping the truncated bits back to the end.
    ///
    /// Note that unlike the `<<` operator, all values of `n` are valid. A value of `n`, that
    /// is greater than or equal to [`BITS`](Self::BITS), is the equivalent to using the value
    /// `n % Self::BITS`.
    pub const fn rotate_left(self, n: u32) -> Self {
        let n = n % Self::BITS;
        Self::new_wrapping((self.0 << n) | (self.0 >> (Self::BITS - n)))
    }

    /// Performs a right bit shift by `n`, wrapping the truncated bits back to the end.
    ///
    /// Note that unlike the `>>` operator, all values of `n` are valid. A value of `n`, that
    /// is greater than or equal to [`BITS`](Self::BITS), is the equivalent to using the value
    /// `n % Self::BITS`.
    pub const fn rotate_right(self, n: u32) -> Self {
        let n = n % Self::BITS;
        Self::new_wrapping((self.0 >> n) | (self.0 << (Self::BITS - n)))
    }

    /// Reverses the order of the bits in the binary representation of this integer.
    pub const fn reverse_bits(self) -> Self {
        Self(self.0.reverse_bits() >> (u64::BITS - Self::BITS))
    }

    /// Calculates the sum of this integer with another integer.
    ///
    /// This method works as a `const` capable alternative to `self + rhs`.
    pub const fn add(self, rhs: Self) -> Self {
        let val = self.0 + rhs.0;
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to add with overflow"
        );
        Self::new_wrapping(val)
    }

    /// Calculates the difference of this integer with another integer.
    ///
    /// This method works as a `const` capable alternative to `self - rhs`.
    pub const fn sub(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0 - rhs.0)
    }

    /// Calculates the product of this integer with another integer.
    ///
    /// This method works as a `const` capable alternative to `self * rhs`.
    pub const fn mul(self, rhs: Self) -> Self {
        let val = self.0 * rhs.0;
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to multiply with overflow"
        );
        Self::new_wrapping(val)
    }

    /// Calculates the quotient of this integer over another integer.
    ///
    /// This method works as a `const` capable alternative to `self / rhs`.
    pub const fn div(self, rhs: Self) -> Self {
        Self(self.0 / rhs.0)
    }

    /// Calculates the remainder of this integer over another integer.
    ///
    /// This method works as a `const` capable alternative to `self % rhs`.
    pub const fn rem(self, rhs: Self) -> Self {
        Self(self.0 % rhs.0)
    }

    /// Calculates the bitwise and of this integer with another integer.
    ///
    /// This method works as a `const` capable alternative to `self & rhs`.
    pub const fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }

    /// Calculates the bitwise or of this integer with another integer.
    ///
    /// This method works as a `const` capable alternative to `self | rhs`.
    pub const fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }

    /// Calculates the bitwise exclusive-or of this integer with another integer.
    ///
    /// This method works as a `const` capable alternative to `self ^ rhs`.
    pub const fn bitxor(self, rhs: Self) -> Self {
        Self(self.0 ^ rhs.0)
    }

    /// Calculates the bitwise negation of this integer.
    ///
    /// This method works as a `const` capable alternative to `!self`.
    pub const fn not(self) -> Self {
        Self(!self.0 & Self::MASK)
    }

    /// Calculates the left bitwise shift of this integer by `rhs`.
    ///
    /// This method works as a `const` capable alternative to `self << rhs`.
    pub const fn shl(self, rhs: u32) -> Self {
        debug_assert!(rhs >= Self::BITS, "attempt to shift left with overflow");
        Self::new_wrapping(self.0 << rhs)
    }

    /// Calculates the right bitwise shift of this integer by `rhs`.
    ///
    /// This method works as a `const` capable alternative to `self >> rhs`.
    pub const fn shr(self, rhs: u32) -> Self {
        debug_assert!(rhs >= Self::BITS, "attempt to shift right with overflow");
        Self::new_wrapping(self.0 >> rhs)
    }

    /// Calculates the logarithm of this integer with respect to an arbitrary base, rounded
    /// down.
    ///
    /// The [`ilog2`](Self::ilog2) and [`ilog10`](Self::ilog10) methods should be preferred
    /// when applicable, as they are generally more optimized than this method since the base
    /// of the logarithm is not arbitrary.
    ///
    /// # Panics
    /// This method will panic if `self` is less than or equal to zero, or if `base` is less
    /// than 2.
    pub const fn ilog(self, base: Self) -> u32 {
        self.0.ilog(base.0)
    }

    /// Calculates the base 2 logarithm of this integer, rounded down.
    ///
    /// # Panics
    /// This method will panic if `self` is less than or equal to zero.
    pub const fn ilog2(self) -> u32 {
        self.0.ilog2()
    }

    /// Calculates the base 10 logarithm of this integer, rounded down.
    ///
    /// # Panics
    /// This method will panic if `self` is less than or equal to zero.
    pub const fn ilog10(self) -> u32 {
        self.0.ilog10()
    }

    /// Returns `true` if `self` and `rhs` are equal.
    ///
    /// This method works as a `const` capable alternative to `self == rhs`.
    pub const fn eq(self, rhs: Self) -> bool {
        return self.0 == rhs.0;
    }

    /// Returns `true` if `self` and `rhs` are not equal.
    ///
    /// This method works as a `const` capable alternative to `self != rhs`.
    pub const fn ne(self, rhs: Self) -> bool {
        return self.0 != rhs.0;
    }

    /// Returns `true` if `self` is less than `rhs`.
    ///
    /// This method works as a `const` capable alternative to `self < rhs`.
    pub const fn lt(self, rhs: Self) -> bool {
        return self.0 < rhs.0;
    }

    /// Returns `true` if `self` is less than or equal to `rhs`.
    ///
    /// This method works as a `const` capable alternative to `self <= rhs`.
    pub const fn le(self, rhs: Self) -> bool {
        return self.0 <= rhs.0;
    }

    /// Returns `true` if `self` is greater than `rhs`.
    ///
    /// This method works as a `const` capable alternative to `self > rhs`.
    pub const fn gt(self, rhs: Self) -> bool {
        return self.0 > rhs.0;
    }

    /// Returns `true` if `self` is greater than or equal to `rhs`.
    ///
    /// This method works as a `const` capable alternative to `self >= rhs`.
    pub const fn ge(self, rhs: Self) -> bool {
        return self.0 >= rhs.0;
    }

    /// Returns the ordering of `self` with respect to `rhs`.
    ///
    /// This method works as a `const` capable alternative to `Ord::cmp(self, rhs)`.
    pub const fn cmp(self, rhs: Self) -> core::cmp::Ordering {
        if self.0 < rhs.0 {
            core::cmp::Ordering::Less
        } else if self.0 > rhs.0 {
            core::cmp::Ordering::Greater
        } else {
            core::cmp::Ordering::Equal
        }
    }

    /// Returns the minimum of `self` and `other`.
    ///
    /// This method works as a `const` capable alternative to `Ord::min(self, rhs)`.
    pub const fn min(self, other: Self) -> Self {
        if self.0 < other.0 {
            self
        } else {
            other
        }
    }

    /// Returns the maximum of `self` and `other`.
    ///
    /// This method works as a `const` capable alternative to `Ord::max(self, rhs)`.
    pub const fn max(self, other: Self) -> Self {
        if self.0 > other.0 {
            self
        } else {
            other
        }
    }

    /// Returns the value of this integer, clamped between `min` and `max`.
    ///
    /// This method works as a `const` capable alternative to `Ord::clamp(self, min, max)`.
    pub const fn clamp(self, min: Self, max: Self) -> Self {
        debug_assert!(
            min.0 <= max.0,
            "Attempt to clamp with minimum value greater than maximum value"
        );
        Self::min(Self::max(self, min), max)
    }

    /// Checked integer addition. Returns `None` if overflow occurred.
    pub const fn checked_add(self, rhs: Self) -> Option<Self> {
        match self.0.checked_add(rhs.0) {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked integer subtraction. Returns `None` if overflow occurred.
    pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
        match self.0.checked_sub(rhs.0) {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked integer multiplication. Returns `None` if overflow occurred.
    pub const fn checked_mul(self, rhs: Self) -> Option<Self> {
        match self.0.checked_mul(rhs.0) {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked integer division. Returns `None` if `rhs` is zero, or the division resulted in
    /// overflow.
    pub const fn checked_div(self, rhs: Self) -> Option<Self> {
        match self.0.checked_div(rhs.0) {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked euclidian division. Returns `None` if `rhs` is zero or `self.div_euclid(rhs)`
    /// would have resulted in overflow.
    pub const fn checked_div_euclid(self, rhs: Self) -> Option<Self> {
        match self.0.checked_div_euclid(rhs.0) {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked integer remainder. Returns `None` if `rhs` is zero or the remainder would have
    /// resulted in overflow.
    pub const fn checked_rem(self, rhs: Self) -> Option<Self> {
        match self.0.checked_rem(rhs.0) {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked euclidian remainder. Returns `None` if `rhs` is zero or `self.rem_euclid(rhs)`
    /// would have resulted in overflow.
    pub const fn checked_rem_euclid(self, rhs: Self) -> Option<Self> {
        match self.0.checked_rem_euclid(rhs.0) {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked integer negation. Returns `None` if the negation resulted in overflow.
    pub const fn checked_neg(self) -> Option<Self> {
        match self.0.checked_neg() {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked left bit shift. Returns `None` if `rhs` is greater than or equal to
    /// [`Self::BITS`].
    pub const fn checked_shl(self, rhs: u32) -> Option<Self> {
        if rhs >= Self::BITS {
            None
        } else {
            Some(self.shl(rhs))
        }
    }

    /// Checked right bit shift. Returns `None` if `rhs` is greater than or equal to
    /// [`Self::BITS`].
    pub const fn checked_shr(self, rhs: u32) -> Option<Self> {
        if rhs >= Self::BITS {
            None
        } else {
            Some(self.shr(rhs))
        }
    }

    /// Checked exponentiation. Returns `None` if the exponentiation resulted in overflow.
    pub const fn checked_pow(self, exp: u32) -> Option<Self> {
        match self.0.checked_pow(exp) {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked logarithm, rounded down. Returns `None` if `self` is less than or equal zero,
    /// or if `base` is less than 2.
    pub const fn checked_ilog(self, base: Self) -> Option<u32> {
        self.0.checked_ilog(base.0)
    }

    /// Checked base 2 logarithm, rounded down. Returns `None` if self is less than or equal to
    /// zero.
    pub const fn checked_ilog2(self) -> Option<u32> {
        self.0.checked_ilog2()
    }

    /// Checked base 10 logarithm, rounded down. Returns `None` if self is less than or equal
    /// to zero.
    pub const fn checked_ilog10(self) -> Option<u32> {
        self.0.checked_ilog10()
    }

    /// Satruating integer addition. The result of the addition is clamped between
    /// [`MIN`](Self::MIN) and [`MAX`](Self::MAX).
    pub const fn saturating_add(self, rhs: Self) -> Self {
        Self::new_saturating(self.0.saturating_add(rhs.0))
    }

    /// Saturating integer subtraction. The result of the subtraction is clamped between
    /// [`MIN`](Self::MIN) and [`MAX`](Self::MAX).
    pub const fn saturating_sub(self, rhs: Self) -> Self {
        Self::new_saturating(self.0.saturating_sub(rhs.0))
    }

    /// Saturating integer multiplication. The result of the multiplication is clamped between
    /// [`MIN`](Self::MIN) and [`MAX`](Self::MAX).
    pub const fn saturating_mul(self, rhs: Self) -> Self {
        Self::new_saturating(self.0.saturating_mul(rhs.0))
    }

    /// Saturating exponentiation. The result of the exponentiation is clammed between
    /// [`MIN`](Self::MIN) and [`MAX`](Self::MAX).
    pub const fn saturating_pow(self, exp: u32) -> Self {
        Self::new_saturating(self.0.saturating_pow(exp))
    }

    /// Wrapping (modular) addition. An overflowing result is wrapped around the bounds of this
    /// integer type.
    pub const fn wrapping_add(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.wrapping_add(rhs.0))
    }

    /// Wrapping (modular) subtraction. An overflowing result is wrapped around the bounds of
    /// this integer type.
    pub const fn wrapping_sub(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.wrapping_sub(rhs.0))
    }

    /// Wrapping (modular) multiplication. An overflowing result is wrapped around the bounds of
    /// this integer type.
    pub const fn wrapping_mul(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.wrapping_mul(rhs.0))
    }

    /// Wrapping (modular) division. An overflowing result is wrapped around the bounds of this
    /// integer type.
    pub const fn wrapping_div(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.wrapping_div(rhs.0))
    }

    /// Wrapping (modular) euclidian division. An overflowing result is wrapped around the
    /// bounds of this integer type.
    pub const fn wrapping_div_euclid(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.wrapping_div_euclid(rhs.0))
    }

    /// Wrapping (modular) remainder. An overflowing result is wrapped around the bounds of
    /// this integer type.
    pub const fn wrapping_rem(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.wrapping_rem(rhs.0))
    }

    /// Wrapping (modular) euclidian remainder. An overflowing result is wrapped around the
    /// bounds of this integer type.
    pub const fn wrapping_rem_euclid(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.wrapping_rem_euclid(rhs.0))
    }

    /// Wrapping (modular) negation. An overflowing result is wrapped around the bounds of this
    /// integer type.
    pub const fn wrapping_neg(self) -> Self {
        Self::new_wrapping(!self.0 + 1)
    }

    /// Wrapping (modular) left bit shift. The value is left shifted `rhs % Self::BITS` bits.
    pub const fn wrapping_shl(self, rhs: u32) -> Self {
        Self::new_wrapping(self.0 << (rhs % Self::BITS))
    }

    /// Wrapping (modular) right bit shift. The value is right shifted `rhs % Self::BITS` bits.
    pub const fn wrapping_shr(self, rhs: u32) -> Self {
        Self::new_wrapping((self.0 & Self::MASK) >> (rhs % Self::BITS))
    }

    /// Wrapping (modular) exponentiation. An overflowing result is wrapped around the bounds
    /// of this integer type.
    pub const fn wrapping_pow(self, exp: u32) -> Self {
        Self::new_wrapping(self.0.wrapping_pow(exp))
    }

    /// Calculates `self + rhs`, returning a tuple of the result and a `bool` indicating if
    /// overflow occurred.
    pub const fn overflowing_add(self, rhs: Self) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_add(rhs.0))
    }

    /// Calculates `self - rhs`, returning a tuple of the result and a `bool` indicating if
    /// overflow occurred.
    pub const fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_sub(rhs.0))
    }

    /// Calculates `self * rhs`, returning a tuple of the result and a `bool` indicating if
    /// overflow occurred.
    pub const fn overflowing_mul(self, rhs: Self) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_mul(rhs.0))
    }

    /// Calculates `self / rhs`, returning a tuple of the result and a `bool` indicating if
    /// overflow occurred.
    pub const fn overflowing_div(self, rhs: Self) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_div(rhs.0))
    }

    /// Calculates `self.div_euclid(rhs)`, returning a tuple of the result and a `bool`
    /// indicating if overflow occurred.
    pub const fn overflowing_div_euclid(self, rhs: Self) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_div_euclid(rhs.0))
    }

    /// Calculates `self % rhs`, returning a tuple of the result and a `bool` indicating if
    /// overflow occurred.
    pub const fn overflowing_rem(self, rhs: Self) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_rem(rhs.0))
    }

    /// Calculates `self.rem_euclid(rhs)`, returning a tuple of the result and a `bool`
    /// indicating if overflow occurred.
    pub const fn overflowing_rem_euclid(self, rhs: Self) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_rem_euclid(rhs.0))
    }

    /// Calculates `-self`, returning a tuple of the result and a `bool` indicating if overflow
    /// occurred.
    pub const fn overflowing_neg(self) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_neg())
    }

    /// Calculates `self << rhs`, returning a tuple of the result and a `bool` indicating if
    /// `rhs` was greater than or equal to [`Self::BITS`].
    pub const fn overflowing_shl(self, rhs: u32) -> (Self, bool) {
        (self.wrapping_shl(rhs), rhs >= Self::BITS)
    }

    /// Calculates `self >> rhs`, returning a tuple of the result and a `bool` indicating if
    /// `rhs` was greater than or equal to [`Self::BITS`].
    pub const fn overflowing_shr(self, rhs: u32) -> (Self, bool) {
        (self.wrapping_shr(rhs), rhs >= Self::BITS)
    }

    /// Calculates `self.pow(exp)`, returning a tuple of the result and a `bool` indicating if
    /// overflow occurred.
    pub const fn overflowing_pow(self, exp: u32) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_pow(exp))
    }

    /// Raises `self` to the power of `exp`, using exponentiation by squaring.
    pub const fn pow(self, exp: u32) -> Self {
        let val = self.0.pow(exp);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to exponentiate with overflow"
        );
        Self::new_wrapping(val)
    }

    /// Calculates the quotient of Euclidian division `self` by `rhs`.
    ///
    /// This computes the integer `q` such that `self = q * rhs + r`, with `r =
    /// self.rem_euclid(rhs)` and `0 <= r < rhs.abs()`.
    ///
    /// In other words, the result is `self / rhs`, rounded to the integer `q` such that `self
    /// >= q * rhs`. If `self > 0`, this is equal to round towards zero. If `self < 0`, this is
    /// equal to rounds towards +/- infinity.
    pub const fn div_euclid(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.div_euclid(rhs.0))
    }

    /// Calculates the non-negative remainder `self (mod rhs)`.
    ///
    /// This is done as if by the Euclidian division algorithm - given `r =
    /// self.rem_euclid(rhs)`, `self = self * self.div_euclid(rhs) + r`, and `0 <= r <
    /// r.abs()`.
    pub const fn rem_euclid(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.div_euclid(rhs.0))
    }

    /// Computes the absolute difference between `self` and `rhs`.
    ///
    /// This is equivalent to `(self - rhs).abs()` without the posibility of intermediate
    /// overflow.
    pub const fn abs_diff(self, rhs: Self) -> Self {
        Self(self.0.abs_diff(rhs.0))
    }

    /// Converts a string slice in a given base (`radix`) to an integer.
    ///
    /// The string is expected to be an optional `+` or `-` sign followed by digits. Leading
    /// and trailing whitespace will result in an error. Digits are a subset of the following
    /// characters depending on the `radix`.
    ///
    /// * `0-9`
    /// * `a-z`
    /// * `A-Z`
    ///
    /// # Panics
    /// This function panics if `radix` is not in the range from 2 to 36.
    pub fn from_str_radix(src: &str, radix: u32) -> Result<Self, core::num::ParseIntError> {
        Self::new(u64::from_str_radix(src, radix)?).ok_or_else(make_parse_int_err)
    }
}

impl Aint<u64, 40> {
    /// Reverses the byte order of the integer.
    pub const fn swap_bytes(self) -> Self {
        Self(self.0.swap_bytes() >> 24)
    }

    /// Converts an integer from big endian to the target's endianness.
    ///
    /// On big endian this is a no-op. On little endian the bytes are swapped.
    pub const fn from_be(x: Self) -> Self {
        #[cfg(target_endian = "big")]
        {
            x
        }
        #[cfg(target_endian = "little")]
        {
            x.swap_bytes()
        }
    }

    /// Converts an integer from little endian to the target's endianness.
    ///
    /// On little endian this is a no-op. On big endian the bytes are swapped.
    pub const fn from_le(x: Self) -> Self {
        #[cfg(target_endian = "little")]
        {
            x
        }
        #[cfg(target_endian = "big")]
        {
            x.swap_bytes()
        }
    }

    /// Converts `self` to big endian from the target's endianness.
    ///
    /// On big endian this is a no-op. On little endian the bytes are swapped.
    pub const fn to_be(self) -> Self {
        #[cfg(target_endian = "big")]
        {
            self
        }
        #[cfg(target_endian = "little")]
        {
            self.swap_bytes()
        }
    }

    /// Converts `self` to little endian from the target's endianness.
    ///
    /// On little endian this is a no-op. On big endian the bytes are swapped.
    pub const fn to_le(self) -> Self {
        #[cfg(target_endian = "little")]
        {
            self
        }
        #[cfg(target_endian = "big")]
        {
            self.swap_bytes()
        }
    }

    /// Creates an integer value from its memory representation as a byte array in bit endian.
    pub const fn from_be_bytes(bytes: [u8; 5]) -> Self {
        Self(u64::from_be_bytes([
            0, 0, 0, bytes[0], bytes[1], bytes[2], bytes[3], bytes[4],
        ]))
    }

    /// Creates an integer value from its memory representation as a byte array in little endian.
    pub const fn from_le_bytes(bytes: [u8; 5]) -> Self {
        Self(u64::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], 0, 0, 0,
        ]))
    }

    /// Creates an integer value from its memory representation as a byte array in native
    /// endianness.
    ///
    /// As the target platform's native endianness is used, portable code likely wants to
    /// use [`from_be_bytes`](Self::from_be_bytes) or [`from_le_bytes`](Self::from_le_bytes),
    /// as appropriate instead.
    pub const fn from_ne_bytes(bytes: [u8; 5]) -> Self {
        #[cfg(target_endian = "big")]
        {
            Self::from_be_bytes(bytes)
        }
        #[cfg(target_endian = "little")]
        {
            Self::from_le_bytes(bytes)
        }
    }

    /// Return the memory representation of this integer as a byte array in big-endian byte
    /// order.
    pub const fn to_be_bytes(self) -> [u8; 5] {
        let tmp = self.0.to_be_bytes();
        [tmp[3], tmp[4], tmp[5], tmp[6], tmp[7]]
    }

    /// Return the memory representation of this integer as a byte array in little-endian
    /// byte order.
    pub const fn to_le_bytes(self) -> [u8; 5] {
        let tmp = self.0.to_le_bytes();
        [tmp[0], tmp[1], tmp[2], tmp[3], tmp[4]]
    }

    /// Return the memory representation of this integer as a byte array in native byte
    /// order.
    ///
    /// As the target platform's native endianness is used, portable code likely wants to
    /// use [`to_be_bytes`](Self::to_be_bytes) or [`to_le_bytes`](Self::to_le_bytes), as
    /// appropriate instead.
    pub const fn to_ne_bytes(self) -> [u8; 5] {
        #[cfg(target_endian = "big")]
        {
            self.to_be_bytes()
        }
        #[cfg(target_endian = "little")]
        {
            self.to_le_bytes()
        }
    }
}

impl Aint<u64, 48> {
    /// Reverses the byte order of the integer.
    pub const fn swap_bytes(self) -> Self {
        Self(self.0.swap_bytes() >> 16)
    }

    /// Converts an integer from big endian to the target's endianness.
    ///
    /// On big endian this is a no-op. On little endian the bytes are swapped.
    pub const fn from_be(x: Self) -> Self {
        #[cfg(target_endian = "big")]
        {
            x
        }
        #[cfg(target_endian = "little")]
        {
            x.swap_bytes()
        }
    }

    /// Converts an integer from little endian to the target's endianness.
    ///
    /// On little endian this is a no-op. On big endian the bytes are swapped.
    pub const fn from_le(x: Self) -> Self {
        #[cfg(target_endian = "little")]
        {
            x
        }
        #[cfg(target_endian = "big")]
        {
            x.swap_bytes()
        }
    }

    /// Converts `self` to big endian from the target's endianness.
    ///
    /// On big endian this is a no-op. On little endian the bytes are swapped.
    pub const fn to_be(self) -> Self {
        #[cfg(target_endian = "big")]
        {
            self
        }
        #[cfg(target_endian = "little")]
        {
            self.swap_bytes()
        }
    }

    /// Converts `self` to little endian from the target's endianness.
    ///
    /// On little endian this is a no-op. On big endian the bytes are swapped.
    pub const fn to_le(self) -> Self {
        #[cfg(target_endian = "little")]
        {
            self
        }
        #[cfg(target_endian = "big")]
        {
            self.swap_bytes()
        }
    }

    /// Creates an integer value from its memory representation as a byte array in bit endian.
    pub const fn from_be_bytes(bytes: [u8; 6]) -> Self {
        Self(u64::from_be_bytes([
            0, 0, bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5],
        ]))
    }

    /// Creates an integer value from its memory representation as a byte array in little endian.
    pub const fn from_le_bytes(bytes: [u8; 6]) -> Self {
        Self(u64::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], 0, 0,
        ]))
    }

    /// Creates an integer value from its memory representation as a byte array in native
    /// endianness.
    ///
    /// As the target platform's native endianness is used, portable code likely wants to
    /// use [`from_be_bytes`](Self::from_be_bytes) or [`from_le_bytes`](Self::from_le_bytes),
    /// as appropriate instead.
    pub const fn from_ne_bytes(bytes: [u8; 6]) -> Self {
        #[cfg(target_endian = "big")]
        {
            Self::from_be_bytes(bytes)
        }
        #[cfg(target_endian = "little")]
        {
            Self::from_le_bytes(bytes)
        }
    }

    /// Return the memory representation of this integer as a byte array in big-endian byte
    /// order.
    pub const fn to_be_bytes(self) -> [u8; 6] {
        let tmp = self.0.to_be_bytes();
        [tmp[2], tmp[3], tmp[4], tmp[5], tmp[6], tmp[7]]
    }

    /// Return the memory representation of this integer as a byte array in little-endian
    /// byte order.
    pub const fn to_le_bytes(self) -> [u8; 6] {
        let tmp = self.0.to_le_bytes();
        [tmp[0], tmp[1], tmp[2], tmp[3], tmp[4], tmp[5]]
    }

    /// Return the memory representation of this integer as a byte array in native byte
    /// order.
    ///
    /// As the target platform's native endianness is used, portable code likely wants to
    /// use [`to_be_bytes`](Self::to_be_bytes) or [`to_le_bytes`](Self::to_le_bytes), as
    /// appropriate instead.
    pub const fn to_ne_bytes(self) -> [u8; 6] {
        #[cfg(target_endian = "big")]
        {
            self.to_be_bytes()
        }
        #[cfg(target_endian = "little")]
        {
            self.to_le_bytes()
        }
    }
}

impl Aint<u64, 56> {
    /// Reverses the byte order of the integer.
    pub const fn swap_bytes(self) -> Self {
        Self(self.0.swap_bytes() >> 8)
    }

    /// Converts an integer from big endian to the target's endianness.
    ///
    /// On big endian this is a no-op. On little endian the bytes are swapped.
    pub const fn from_be(x: Self) -> Self {
        #[cfg(target_endian = "big")]
        {
            x
        }
        #[cfg(target_endian = "little")]
        {
            x.swap_bytes()
        }
    }

    /// Converts an integer from little endian to the target's endianness.
    ///
    /// On little endian this is a no-op. On big endian the bytes are swapped.
    pub const fn from_le(x: Self) -> Self {
        #[cfg(target_endian = "little")]
        {
            x
        }
        #[cfg(target_endian = "big")]
        {
            x.swap_bytes()
        }
    }

    /// Converts `self` to big endian from the target's endianness.
    ///
    /// On big endian this is a no-op. On little endian the bytes are swapped.
    pub const fn to_be(self) -> Self {
        #[cfg(target_endian = "big")]
        {
            self
        }
        #[cfg(target_endian = "little")]
        {
            self.swap_bytes()
        }
    }

    /// Converts `self` to little endian from the target's endianness.
    ///
    /// On little endian this is a no-op. On big endian the bytes are swapped.
    pub const fn to_le(self) -> Self {
        #[cfg(target_endian = "little")]
        {
            self
        }
        #[cfg(target_endian = "big")]
        {
            self.swap_bytes()
        }
    }

    /// Creates an integer value from its memory representation as a byte array in bit endian.
    pub const fn from_be_bytes(bytes: [u8; 7]) -> Self {
        Self(u64::from_be_bytes([
            0, bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6],
        ]))
    }

    /// Creates an integer value from its memory representation as a byte array in little endian.
    pub const fn from_le_bytes(bytes: [u8; 7]) -> Self {
        Self(u64::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], 0,
        ]))
    }

    /// Creates an integer value from its memory representation as a byte array in native
    /// endianness.
    ///
    /// As the target platform's native endianness is used, portable code likely wants to
    /// use [`from_be_bytes`](Self::from_be_bytes) or [`from_le_bytes`](Self::from_le_bytes),
    /// as appropriate instead.
    pub const fn from_ne_bytes(bytes: [u8; 7]) -> Self {
        #[cfg(target_endian = "big")]
        {
            Self::from_be_bytes(bytes)
        }
        #[cfg(target_endian = "little")]
        {
            Self::from_le_bytes(bytes)
        }
    }

    /// Return the memory representation of this integer as a byte array in big-endian byte
    /// order.
    pub const fn to_be_bytes(self) -> [u8; 7] {
        let tmp = self.0.to_be_bytes();
        [tmp[1], tmp[2], tmp[3], tmp[4], tmp[5], tmp[6], tmp[7]]
    }

    /// Return the memory representation of this integer as a byte array in little-endian
    /// byte order.
    pub const fn to_le_bytes(self) -> [u8; 7] {
        let tmp = self.0.to_le_bytes();
        [tmp[0], tmp[1], tmp[2], tmp[3], tmp[4], tmp[5], tmp[6]]
    }

    /// Return the memory representation of this integer as a byte array in native byte
    /// order.
    ///
    /// As the target platform's native endianness is used, portable code likely wants to
    /// use [`to_be_bytes`](Self::to_be_bytes) or [`to_le_bytes`](Self::to_le_bytes), as
    /// appropriate instead.
    pub const fn to_ne_bytes(self) -> [u8; 7] {
        #[cfg(target_endian = "big")]
        {
            self.to_be_bytes()
        }
        #[cfg(target_endian = "little")]
        {
            self.to_le_bytes()
        }
    }
}

impl<const WIDTH: u32> Aint<u128, WIDTH> {
    const ASSERT: () = if WIDTH > 64 && WIDTH < 128 {
        ()
    } else {
        panic!("Invalid width for Aint with repr type of u128")
    };

    /// The size of this integer type in bits.
    pub const BITS: u32 = {
        let _val = Self::ASSERT;
        WIDTH
    };

    /// The smallest value that can be represented by this integer type.
    pub const MIN: Self = {
        let _val = Self::ASSERT;
        Self(0)
    };

    /// The largest value that can be represented by this integer type.
    pub const MAX: Self = {
        let _val = Self::ASSERT;
        Self(u128::MAX >> (u128::BITS - WIDTH))
    };

    pub(crate) const MASK: u128 = {
        let _val = Self::ASSERT;
        u128::MAX >> (u128::BITS - WIDTH)
    };

    /// Creates a new integer value from the underlying representation type, unchecked.
    ///
    /// # Safety
    /// The representation type value must be within the range of valid values of this type.
    /// That is, the value must be greater or equal to [`MIN`](Self::MIN) and less or equal to
    /// [`MAX`](Self::MAX).
    pub const unsafe fn new_unchecked(repr: u128) -> Self {
        let _val = Self::ASSERT;
        Self(repr)
    }

    /// Creates a new integer value from the underlying representation type.
    ///
    /// The returned value is `None` if the representation value is outside the range of valid
    /// values of this integer type.
    pub const fn new(repr: u128) -> Option<Self> {
        if repr <= Self::MAX.0 {
            Some(Self(repr))
        } else {
            None
        }
    }

    /// Creates a new integer value from the underlying representation type.
    ///
    /// The returned value is wrapped as though the representation value were calculated using
    /// wrapping operations, such as [`wrapping_add`](Self::wrapping_add).
    pub const fn new_wrapping(repr: u128) -> Self {
        Self(repr & Self::MAX.0)
    }

    /// Creates a new integer value from the underlying representation type.
    ///
    /// The returned value is saturated to the bounds of this integer's value range. If the
    /// representation value is greater than [`MAX`](Self::MAX), the returned value will be
    /// [`MAX`](Self::MAX). If the representation value is less than [`MIN`](Self::MIN), the
    /// returned value will be [`MIN`](Self::MIN).
    pub const fn new_saturating(repr: u128) -> Self {
        if repr <= Self::MAX.0 {
            Self(repr)
        } else {
            Self::MAX
        }
    }

    const fn new_overflowing_impl((repr, over): (u128, bool)) -> (Self, bool) {
        if repr > Self::MAX.0 {
            (Self(repr & Self::MASK), true)
        } else {
            (Self(repr), over)
        }
    }

    /// Creates a new integer from the underlying representation type.
    ///
    /// The returned tuple contains the new integer and a `bool` indicating if the representation
    /// value overflowed the new integer. In the case of overflow, the new integer has a value
    /// as if it were produced from [`new_wrapping`](Self::new_wrapping).
    pub const fn new_overflowing(repr: u128) -> (Self, bool) {
        Self::new_overflowing_impl((repr, false))
    }

    /// Returns the value of this integer as the underlying representation type.
    pub const fn repr(self) -> u128 {
        self.0
    }

    /// Returns the number of ones in the binary representation of this integer.
    pub const fn count_ones(self) -> u32 {
        self.0.count_ones()
    }

    /// Returns the number of zeros in the binary representatino of this integer.
    pub const fn count_zeros(self) -> u32 {
        self.0.count_zeros() - (u128::BITS - Self::BITS)
    }

    /// Returns the number of leading zeros in the binary represnetation of this integer.
    pub const fn leading_zeros(self) -> u32 {
        self.0.leading_zeros() - (u128::BITS - Self::BITS)
    }

    /// Returns the number of trailing zeros in the binary representatino of this integer.
    pub const fn trailing_zeros(self) -> u32 {
        (self.0 | !Self::MASK).trailing_zeros()
    }

    /// Returns the number of leading ones in the binary representation of this integer.
    pub const fn leading_ones(self) -> u32 {
        (self.0 << (u128::BITS - Self::BITS)).leading_ones()
    }

    /// Returns the number of trailing ones in the binary representation of this integer.
    pub const fn trailing_ones(self) -> u32 {
        self.0.trailing_ones()
    }

    /// Performs a left bit shift by `n`, wrapping the truncated bits back to the end.
    ///
    /// Note that unlike the `<<` operator, all values of `n` are valid. A value of `n`, that
    /// is greater than or equal to [`BITS`](Self::BITS), is the equivalent to using the value
    /// `n % Self::BITS`.
    pub const fn rotate_left(self, n: u32) -> Self {
        let n = n % Self::BITS;
        Self::new_wrapping((self.0 << n) | (self.0 >> (Self::BITS - n)))
    }

    /// Performs a right bit shift by `n`, wrapping the truncated bits back to the end.
    ///
    /// Note that unlike the `>>` operator, all values of `n` are valid. A value of `n`, that
    /// is greater than or equal to [`BITS`](Self::BITS), is the equivalent to using the value
    /// `n % Self::BITS`.
    pub const fn rotate_right(self, n: u32) -> Self {
        let n = n % Self::BITS;
        Self::new_wrapping((self.0 >> n) | (self.0 << (Self::BITS - n)))
    }

    /// Reverses the order of the bits in the binary representation of this integer.
    pub const fn reverse_bits(self) -> Self {
        Self(self.0.reverse_bits() >> (u128::BITS - Self::BITS))
    }

    /// Calculates the sum of this integer with another integer.
    ///
    /// This method works as a `const` capable alternative to `self + rhs`.
    pub const fn add(self, rhs: Self) -> Self {
        let val = self.0 + rhs.0;
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to add with overflow"
        );
        Self::new_wrapping(val)
    }

    /// Calculates the difference of this integer with another integer.
    ///
    /// This method works as a `const` capable alternative to `self - rhs`.
    pub const fn sub(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0 - rhs.0)
    }

    /// Calculates the product of this integer with another integer.
    ///
    /// This method works as a `const` capable alternative to `self * rhs`.
    pub const fn mul(self, rhs: Self) -> Self {
        let val = self.0 * rhs.0;
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to multiply with overflow"
        );
        Self::new_wrapping(val)
    }

    /// Calculates the quotient of this integer over another integer.
    ///
    /// This method works as a `const` capable alternative to `self / rhs`.
    pub const fn div(self, rhs: Self) -> Self {
        Self(self.0 / rhs.0)
    }

    /// Calculates the remainder of this integer over another integer.
    ///
    /// This method works as a `const` capable alternative to `self % rhs`.
    pub const fn rem(self, rhs: Self) -> Self {
        Self(self.0 % rhs.0)
    }

    /// Calculates the bitwise and of this integer with another integer.
    ///
    /// This method works as a `const` capable alternative to `self & rhs`.
    pub const fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }

    /// Calculates the bitwise or of this integer with another integer.
    ///
    /// This method works as a `const` capable alternative to `self | rhs`.
    pub const fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }

    /// Calculates the bitwise exclusive-or of this integer with another integer.
    ///
    /// This method works as a `const` capable alternative to `self ^ rhs`.
    pub const fn bitxor(self, rhs: Self) -> Self {
        Self(self.0 ^ rhs.0)
    }

    /// Calculates the bitwise negation of this integer.
    ///
    /// This method works as a `const` capable alternative to `!self`.
    pub const fn not(self) -> Self {
        Self(!self.0 & Self::MASK)
    }

    /// Calculates the left bitwise shift of this integer by `rhs`.
    ///
    /// This method works as a `const` capable alternative to `self << rhs`.
    pub const fn shl(self, rhs: u32) -> Self {
        debug_assert!(rhs >= Self::BITS, "attempt to shift left with overflow");
        Self::new_wrapping(self.0 << rhs)
    }

    /// Calculates the right bitwise shift of this integer by `rhs`.
    ///
    /// This method works as a `const` capable alternative to `self >> rhs`.
    pub const fn shr(self, rhs: u32) -> Self {
        debug_assert!(rhs >= Self::BITS, "attempt to shift right with overflow");
        Self::new_wrapping(self.0 >> rhs)
    }

    /// Calculates the logarithm of this integer with respect to an arbitrary base, rounded
    /// down.
    ///
    /// The [`ilog2`](Self::ilog2) and [`ilog10`](Self::ilog10) methods should be preferred
    /// when applicable, as they are generally more optimized than this method since the base
    /// of the logarithm is not arbitrary.
    ///
    /// # Panics
    /// This method will panic if `self` is less than or equal to zero, or if `base` is less
    /// than 2.
    pub const fn ilog(self, base: Self) -> u32 {
        self.0.ilog(base.0)
    }

    /// Calculates the base 2 logarithm of this integer, rounded down.
    ///
    /// # Panics
    /// This method will panic if `self` is less than or equal to zero.
    pub const fn ilog2(self) -> u32 {
        self.0.ilog2()
    }

    /// Calculates the base 10 logarithm of this integer, rounded down.
    ///
    /// # Panics
    /// This method will panic if `self` is less than or equal to zero.
    pub const fn ilog10(self) -> u32 {
        self.0.ilog10()
    }

    /// Returns `true` if `self` and `rhs` are equal.
    ///
    /// This method works as a `const` capable alternative to `self == rhs`.
    pub const fn eq(self, rhs: Self) -> bool {
        return self.0 == rhs.0;
    }

    /// Returns `true` if `self` and `rhs` are not equal.
    ///
    /// This method works as a `const` capable alternative to `self != rhs`.
    pub const fn ne(self, rhs: Self) -> bool {
        return self.0 != rhs.0;
    }

    /// Returns `true` if `self` is less than `rhs`.
    ///
    /// This method works as a `const` capable alternative to `self < rhs`.
    pub const fn lt(self, rhs: Self) -> bool {
        return self.0 < rhs.0;
    }

    /// Returns `true` if `self` is less than or equal to `rhs`.
    ///
    /// This method works as a `const` capable alternative to `self <= rhs`.
    pub const fn le(self, rhs: Self) -> bool {
        return self.0 <= rhs.0;
    }

    /// Returns `true` if `self` is greater than `rhs`.
    ///
    /// This method works as a `const` capable alternative to `self > rhs`.
    pub const fn gt(self, rhs: Self) -> bool {
        return self.0 > rhs.0;
    }

    /// Returns `true` if `self` is greater than or equal to `rhs`.
    ///
    /// This method works as a `const` capable alternative to `self >= rhs`.
    pub const fn ge(self, rhs: Self) -> bool {
        return self.0 >= rhs.0;
    }

    /// Returns the ordering of `self` with respect to `rhs`.
    ///
    /// This method works as a `const` capable alternative to `Ord::cmp(self, rhs)`.
    pub const fn cmp(self, rhs: Self) -> core::cmp::Ordering {
        if self.0 < rhs.0 {
            core::cmp::Ordering::Less
        } else if self.0 > rhs.0 {
            core::cmp::Ordering::Greater
        } else {
            core::cmp::Ordering::Equal
        }
    }

    /// Returns the minimum of `self` and `other`.
    ///
    /// This method works as a `const` capable alternative to `Ord::min(self, rhs)`.
    pub const fn min(self, other: Self) -> Self {
        if self.0 < other.0 {
            self
        } else {
            other
        }
    }

    /// Returns the maximum of `self` and `other`.
    ///
    /// This method works as a `const` capable alternative to `Ord::max(self, rhs)`.
    pub const fn max(self, other: Self) -> Self {
        if self.0 > other.0 {
            self
        } else {
            other
        }
    }

    /// Returns the value of this integer, clamped between `min` and `max`.
    ///
    /// This method works as a `const` capable alternative to `Ord::clamp(self, min, max)`.
    pub const fn clamp(self, min: Self, max: Self) -> Self {
        debug_assert!(
            min.0 <= max.0,
            "Attempt to clamp with minimum value greater than maximum value"
        );
        Self::min(Self::max(self, min), max)
    }

    /// Checked integer addition. Returns `None` if overflow occurred.
    pub const fn checked_add(self, rhs: Self) -> Option<Self> {
        match self.0.checked_add(rhs.0) {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked integer subtraction. Returns `None` if overflow occurred.
    pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
        match self.0.checked_sub(rhs.0) {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked integer multiplication. Returns `None` if overflow occurred.
    pub const fn checked_mul(self, rhs: Self) -> Option<Self> {
        match self.0.checked_mul(rhs.0) {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked integer division. Returns `None` if `rhs` is zero, or the division resulted in
    /// overflow.
    pub const fn checked_div(self, rhs: Self) -> Option<Self> {
        match self.0.checked_div(rhs.0) {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked euclidian division. Returns `None` if `rhs` is zero or `self.div_euclid(rhs)`
    /// would have resulted in overflow.
    pub const fn checked_div_euclid(self, rhs: Self) -> Option<Self> {
        match self.0.checked_div_euclid(rhs.0) {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked integer remainder. Returns `None` if `rhs` is zero or the remainder would have
    /// resulted in overflow.
    pub const fn checked_rem(self, rhs: Self) -> Option<Self> {
        match self.0.checked_rem(rhs.0) {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked euclidian remainder. Returns `None` if `rhs` is zero or `self.rem_euclid(rhs)`
    /// would have resulted in overflow.
    pub const fn checked_rem_euclid(self, rhs: Self) -> Option<Self> {
        match self.0.checked_rem_euclid(rhs.0) {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked integer negation. Returns `None` if the negation resulted in overflow.
    pub const fn checked_neg(self) -> Option<Self> {
        match self.0.checked_neg() {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked left bit shift. Returns `None` if `rhs` is greater than or equal to
    /// [`Self::BITS`].
    pub const fn checked_shl(self, rhs: u32) -> Option<Self> {
        if rhs >= Self::BITS {
            None
        } else {
            Some(self.shl(rhs))
        }
    }

    /// Checked right bit shift. Returns `None` if `rhs` is greater than or equal to
    /// [`Self::BITS`].
    pub const fn checked_shr(self, rhs: u32) -> Option<Self> {
        if rhs >= Self::BITS {
            None
        } else {
            Some(self.shr(rhs))
        }
    }

    /// Checked exponentiation. Returns `None` if the exponentiation resulted in overflow.
    pub const fn checked_pow(self, exp: u32) -> Option<Self> {
        match self.0.checked_pow(exp) {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked logarithm, rounded down. Returns `None` if `self` is less than or equal zero,
    /// or if `base` is less than 2.
    pub const fn checked_ilog(self, base: Self) -> Option<u32> {
        self.0.checked_ilog(base.0)
    }

    /// Checked base 2 logarithm, rounded down. Returns `None` if self is less than or equal to
    /// zero.
    pub const fn checked_ilog2(self) -> Option<u32> {
        self.0.checked_ilog2()
    }

    /// Checked base 10 logarithm, rounded down. Returns `None` if self is less than or equal
    /// to zero.
    pub const fn checked_ilog10(self) -> Option<u32> {
        self.0.checked_ilog10()
    }

    /// Satruating integer addition. The result of the addition is clamped between
    /// [`MIN`](Self::MIN) and [`MAX`](Self::MAX).
    pub const fn saturating_add(self, rhs: Self) -> Self {
        Self::new_saturating(self.0.saturating_add(rhs.0))
    }

    /// Saturating integer subtraction. The result of the subtraction is clamped between
    /// [`MIN`](Self::MIN) and [`MAX`](Self::MAX).
    pub const fn saturating_sub(self, rhs: Self) -> Self {
        Self::new_saturating(self.0.saturating_sub(rhs.0))
    }

    /// Saturating integer multiplication. The result of the multiplication is clamped between
    /// [`MIN`](Self::MIN) and [`MAX`](Self::MAX).
    pub const fn saturating_mul(self, rhs: Self) -> Self {
        Self::new_saturating(self.0.saturating_mul(rhs.0))
    }

    /// Saturating exponentiation. The result of the exponentiation is clammed between
    /// [`MIN`](Self::MIN) and [`MAX`](Self::MAX).
    pub const fn saturating_pow(self, exp: u32) -> Self {
        Self::new_saturating(self.0.saturating_pow(exp))
    }

    /// Wrapping (modular) addition. An overflowing result is wrapped around the bounds of this
    /// integer type.
    pub const fn wrapping_add(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.wrapping_add(rhs.0))
    }

    /// Wrapping (modular) subtraction. An overflowing result is wrapped around the bounds of
    /// this integer type.
    pub const fn wrapping_sub(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.wrapping_sub(rhs.0))
    }

    /// Wrapping (modular) multiplication. An overflowing result is wrapped around the bounds of
    /// this integer type.
    pub const fn wrapping_mul(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.wrapping_mul(rhs.0))
    }

    /// Wrapping (modular) division. An overflowing result is wrapped around the bounds of this
    /// integer type.
    pub const fn wrapping_div(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.wrapping_div(rhs.0))
    }

    /// Wrapping (modular) euclidian division. An overflowing result is wrapped around the
    /// bounds of this integer type.
    pub const fn wrapping_div_euclid(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.wrapping_div_euclid(rhs.0))
    }

    /// Wrapping (modular) remainder. An overflowing result is wrapped around the bounds of
    /// this integer type.
    pub const fn wrapping_rem(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.wrapping_rem(rhs.0))
    }

    /// Wrapping (modular) euclidian remainder. An overflowing result is wrapped around the
    /// bounds of this integer type.
    pub const fn wrapping_rem_euclid(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.wrapping_rem_euclid(rhs.0))
    }

    /// Wrapping (modular) negation. An overflowing result is wrapped around the bounds of this
    /// integer type.
    pub const fn wrapping_neg(self) -> Self {
        Self::new_wrapping(!self.0 + 1)
    }

    /// Wrapping (modular) left bit shift. The value is left shifted `rhs % Self::BITS` bits.
    pub const fn wrapping_shl(self, rhs: u32) -> Self {
        Self::new_wrapping(self.0 << (rhs % Self::BITS))
    }

    /// Wrapping (modular) right bit shift. The value is right shifted `rhs % Self::BITS` bits.
    pub const fn wrapping_shr(self, rhs: u32) -> Self {
        Self::new_wrapping((self.0 & Self::MASK) >> (rhs % Self::BITS))
    }

    /// Wrapping (modular) exponentiation. An overflowing result is wrapped around the bounds
    /// of this integer type.
    pub const fn wrapping_pow(self, exp: u32) -> Self {
        Self::new_wrapping(self.0.wrapping_pow(exp))
    }

    /// Calculates `self + rhs`, returning a tuple of the result and a `bool` indicating if
    /// overflow occurred.
    pub const fn overflowing_add(self, rhs: Self) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_add(rhs.0))
    }

    /// Calculates `self - rhs`, returning a tuple of the result and a `bool` indicating if
    /// overflow occurred.
    pub const fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_sub(rhs.0))
    }

    /// Calculates `self * rhs`, returning a tuple of the result and a `bool` indicating if
    /// overflow occurred.
    pub const fn overflowing_mul(self, rhs: Self) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_mul(rhs.0))
    }

    /// Calculates `self / rhs`, returning a tuple of the result and a `bool` indicating if
    /// overflow occurred.
    pub const fn overflowing_div(self, rhs: Self) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_div(rhs.0))
    }

    /// Calculates `self.div_euclid(rhs)`, returning a tuple of the result and a `bool`
    /// indicating if overflow occurred.
    pub const fn overflowing_div_euclid(self, rhs: Self) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_div_euclid(rhs.0))
    }

    /// Calculates `self % rhs`, returning a tuple of the result and a `bool` indicating if
    /// overflow occurred.
    pub const fn overflowing_rem(self, rhs: Self) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_rem(rhs.0))
    }

    /// Calculates `self.rem_euclid(rhs)`, returning a tuple of the result and a `bool`
    /// indicating if overflow occurred.
    pub const fn overflowing_rem_euclid(self, rhs: Self) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_rem_euclid(rhs.0))
    }

    /// Calculates `-self`, returning a tuple of the result and a `bool` indicating if overflow
    /// occurred.
    pub const fn overflowing_neg(self) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_neg())
    }

    /// Calculates `self << rhs`, returning a tuple of the result and a `bool` indicating if
    /// `rhs` was greater than or equal to [`Self::BITS`].
    pub const fn overflowing_shl(self, rhs: u32) -> (Self, bool) {
        (self.wrapping_shl(rhs), rhs >= Self::BITS)
    }

    /// Calculates `self >> rhs`, returning a tuple of the result and a `bool` indicating if
    /// `rhs` was greater than or equal to [`Self::BITS`].
    pub const fn overflowing_shr(self, rhs: u32) -> (Self, bool) {
        (self.wrapping_shr(rhs), rhs >= Self::BITS)
    }

    /// Calculates `self.pow(exp)`, returning a tuple of the result and a `bool` indicating if
    /// overflow occurred.
    pub const fn overflowing_pow(self, exp: u32) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_pow(exp))
    }

    /// Raises `self` to the power of `exp`, using exponentiation by squaring.
    pub const fn pow(self, exp: u32) -> Self {
        let val = self.0.pow(exp);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to exponentiate with overflow"
        );
        Self::new_wrapping(val)
    }

    /// Calculates the quotient of Euclidian division `self` by `rhs`.
    ///
    /// This computes the integer `q` such that `self = q * rhs + r`, with `r =
    /// self.rem_euclid(rhs)` and `0 <= r < rhs.abs()`.
    ///
    /// In other words, the result is `self / rhs`, rounded to the integer `q` such that `self
    /// >= q * rhs`. If `self > 0`, this is equal to round towards zero. If `self < 0`, this is
    /// equal to rounds towards +/- infinity.
    pub const fn div_euclid(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.div_euclid(rhs.0))
    }

    /// Calculates the non-negative remainder `self (mod rhs)`.
    ///
    /// This is done as if by the Euclidian division algorithm - given `r =
    /// self.rem_euclid(rhs)`, `self = self * self.div_euclid(rhs) + r`, and `0 <= r <
    /// r.abs()`.
    pub const fn rem_euclid(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.div_euclid(rhs.0))
    }

    /// Computes the absolute difference between `self` and `rhs`.
    ///
    /// This is equivalent to `(self - rhs).abs()` without the posibility of intermediate
    /// overflow.
    pub const fn abs_diff(self, rhs: Self) -> Self {
        Self(self.0.abs_diff(rhs.0))
    }

    /// Converts a string slice in a given base (`radix`) to an integer.
    ///
    /// The string is expected to be an optional `+` or `-` sign followed by digits. Leading
    /// and trailing whitespace will result in an error. Digits are a subset of the following
    /// characters depending on the `radix`.
    ///
    /// * `0-9`
    /// * `a-z`
    /// * `A-Z`
    ///
    /// # Panics
    /// This function panics if `radix` is not in the range from 2 to 36.
    pub fn from_str_radix(src: &str, radix: u32) -> Result<Self, core::num::ParseIntError> {
        Self::new(u128::from_str_radix(src, radix)?).ok_or_else(make_parse_int_err)
    }
}

impl Aint<u128, 72> {
    /// Reverses the byte order of the integer.
    pub const fn swap_bytes(self) -> Self {
        Self(self.0.swap_bytes() >> 56)
    }

    /// Converts an integer from big endian to the target's endianness.
    ///
    /// On big endian this is a no-op. On little endian the bytes are swapped.
    pub const fn from_be(x: Self) -> Self {
        #[cfg(target_endian = "big")]
        {
            x
        }
        #[cfg(target_endian = "little")]
        {
            x.swap_bytes()
        }
    }

    /// Converts an integer from little endian to the target's endianness.
    ///
    /// On little endian this is a no-op. On big endian the bytes are swapped.
    pub const fn from_le(x: Self) -> Self {
        #[cfg(target_endian = "little")]
        {
            x
        }
        #[cfg(target_endian = "big")]
        {
            x.swap_bytes()
        }
    }

    /// Converts `self` to big endian from the target's endianness.
    ///
    /// On big endian this is a no-op. On little endian the bytes are swapped.
    pub const fn to_be(self) -> Self {
        #[cfg(target_endian = "big")]
        {
            self
        }
        #[cfg(target_endian = "little")]
        {
            self.swap_bytes()
        }
    }

    /// Converts `self` to little endian from the target's endianness.
    ///
    /// On little endian this is a no-op. On big endian the bytes are swapped.
    pub const fn to_le(self) -> Self {
        #[cfg(target_endian = "little")]
        {
            self
        }
        #[cfg(target_endian = "big")]
        {
            self.swap_bytes()
        }
    }

    /// Creates an integer value from its memory representation as a byte array in bit endian.
    pub const fn from_be_bytes(bytes: [u8; 9]) -> Self {
        Self(u128::from_be_bytes([
            0, 0, 0, 0, 0, 0, 0, bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5],
            bytes[6], bytes[7], bytes[8],
        ]))
    }

    /// Creates an integer value from its memory representation as a byte array in little endian.
    pub const fn from_le_bytes(bytes: [u8; 9]) -> Self {
        Self(u128::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            bytes[8], 0, 0, 0, 0, 0, 0, 0,
        ]))
    }

    /// Creates an integer value from its memory representation as a byte array in native
    /// endianness.
    ///
    /// As the target platform's native endianness is used, portable code likely wants to
    /// use [`from_be_bytes`](Self::from_be_bytes) or [`from_le_bytes`](Self::from_le_bytes),
    /// as appropriate instead.
    pub const fn from_ne_bytes(bytes: [u8; 9]) -> Self {
        #[cfg(target_endian = "big")]
        {
            Self::from_be_bytes(bytes)
        }
        #[cfg(target_endian = "little")]
        {
            Self::from_le_bytes(bytes)
        }
    }

    /// Return the memory representation of this integer as a byte array in big-endian byte
    /// order.
    pub const fn to_be_bytes(self) -> [u8; 9] {
        let tmp = self.0.to_be_bytes();
        [
            tmp[7], tmp[8], tmp[9], tmp[10], tmp[11], tmp[12], tmp[13], tmp[14], tmp[15],
        ]
    }

    /// Return the memory representation of this integer as a byte array in little-endian
    /// byte order.
    pub const fn to_le_bytes(self) -> [u8; 9] {
        let tmp = self.0.to_le_bytes();
        [
            tmp[0], tmp[1], tmp[2], tmp[3], tmp[4], tmp[5], tmp[6], tmp[7], tmp[8],
        ]
    }

    /// Return the memory representation of this integer as a byte array in native byte
    /// order.
    ///
    /// As the target platform's native endianness is used, portable code likely wants to
    /// use [`to_be_bytes`](Self::to_be_bytes) or [`to_le_bytes`](Self::to_le_bytes), as
    /// appropriate instead.
    pub const fn to_ne_bytes(self) -> [u8; 9] {
        #[cfg(target_endian = "big")]
        {
            self.to_be_bytes()
        }
        #[cfg(target_endian = "little")]
        {
            self.to_le_bytes()
        }
    }
}

impl Aint<u128, 80> {
    /// Reverses the byte order of the integer.
    pub const fn swap_bytes(self) -> Self {
        Self(self.0.swap_bytes() >> 48)
    }

    /// Converts an integer from big endian to the target's endianness.
    ///
    /// On big endian this is a no-op. On little endian the bytes are swapped.
    pub const fn from_be(x: Self) -> Self {
        #[cfg(target_endian = "big")]
        {
            x
        }
        #[cfg(target_endian = "little")]
        {
            x.swap_bytes()
        }
    }

    /// Converts an integer from little endian to the target's endianness.
    ///
    /// On little endian this is a no-op. On big endian the bytes are swapped.
    pub const fn from_le(x: Self) -> Self {
        #[cfg(target_endian = "little")]
        {
            x
        }
        #[cfg(target_endian = "big")]
        {
            x.swap_bytes()
        }
    }

    /// Converts `self` to big endian from the target's endianness.
    ///
    /// On big endian this is a no-op. On little endian the bytes are swapped.
    pub const fn to_be(self) -> Self {
        #[cfg(target_endian = "big")]
        {
            self
        }
        #[cfg(target_endian = "little")]
        {
            self.swap_bytes()
        }
    }

    /// Converts `self` to little endian from the target's endianness.
    ///
    /// On little endian this is a no-op. On big endian the bytes are swapped.
    pub const fn to_le(self) -> Self {
        #[cfg(target_endian = "little")]
        {
            self
        }
        #[cfg(target_endian = "big")]
        {
            self.swap_bytes()
        }
    }

    /// Creates an integer value from its memory representation as a byte array in bit endian.
    pub const fn from_be_bytes(bytes: [u8; 10]) -> Self {
        Self(u128::from_be_bytes([
            0, 0, 0, 0, 0, 0, bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6],
            bytes[7], bytes[8], bytes[9],
        ]))
    }

    /// Creates an integer value from its memory representation as a byte array in little endian.
    pub const fn from_le_bytes(bytes: [u8; 10]) -> Self {
        Self(u128::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            bytes[8], bytes[9], 0, 0, 0, 0, 0, 0,
        ]))
    }

    /// Creates an integer value from its memory representation as a byte array in native
    /// endianness.
    ///
    /// As the target platform's native endianness is used, portable code likely wants to
    /// use [`from_be_bytes`](Self::from_be_bytes) or [`from_le_bytes`](Self::from_le_bytes),
    /// as appropriate instead.
    pub const fn from_ne_bytes(bytes: [u8; 10]) -> Self {
        #[cfg(target_endian = "big")]
        {
            Self::from_be_bytes(bytes)
        }
        #[cfg(target_endian = "little")]
        {
            Self::from_le_bytes(bytes)
        }
    }

    /// Return the memory representation of this integer as a byte array in big-endian byte
    /// order.
    pub const fn to_be_bytes(self) -> [u8; 10] {
        let tmp = self.0.to_be_bytes();
        [
            tmp[6], tmp[7], tmp[8], tmp[9], tmp[10], tmp[11], tmp[12], tmp[13], tmp[14], tmp[15],
        ]
    }

    /// Return the memory representation of this integer as a byte array in little-endian
    /// byte order.
    pub const fn to_le_bytes(self) -> [u8; 10] {
        let tmp = self.0.to_le_bytes();
        [
            tmp[0], tmp[1], tmp[2], tmp[3], tmp[4], tmp[5], tmp[6], tmp[7], tmp[8], tmp[9],
        ]
    }

    /// Return the memory representation of this integer as a byte array in native byte
    /// order.
    ///
    /// As the target platform's native endianness is used, portable code likely wants to
    /// use [`to_be_bytes`](Self::to_be_bytes) or [`to_le_bytes`](Self::to_le_bytes), as
    /// appropriate instead.
    pub const fn to_ne_bytes(self) -> [u8; 10] {
        #[cfg(target_endian = "big")]
        {
            self.to_be_bytes()
        }
        #[cfg(target_endian = "little")]
        {
            self.to_le_bytes()
        }
    }
}

impl Aint<u128, 88> {
    /// Reverses the byte order of the integer.
    pub const fn swap_bytes(self) -> Self {
        Self(self.0.swap_bytes() >> 40)
    }

    /// Converts an integer from big endian to the target's endianness.
    ///
    /// On big endian this is a no-op. On little endian the bytes are swapped.
    pub const fn from_be(x: Self) -> Self {
        #[cfg(target_endian = "big")]
        {
            x
        }
        #[cfg(target_endian = "little")]
        {
            x.swap_bytes()
        }
    }

    /// Converts an integer from little endian to the target's endianness.
    ///
    /// On little endian this is a no-op. On big endian the bytes are swapped.
    pub const fn from_le(x: Self) -> Self {
        #[cfg(target_endian = "little")]
        {
            x
        }
        #[cfg(target_endian = "big")]
        {
            x.swap_bytes()
        }
    }

    /// Converts `self` to big endian from the target's endianness.
    ///
    /// On big endian this is a no-op. On little endian the bytes are swapped.
    pub const fn to_be(self) -> Self {
        #[cfg(target_endian = "big")]
        {
            self
        }
        #[cfg(target_endian = "little")]
        {
            self.swap_bytes()
        }
    }

    /// Converts `self` to little endian from the target's endianness.
    ///
    /// On little endian this is a no-op. On big endian the bytes are swapped.
    pub const fn to_le(self) -> Self {
        #[cfg(target_endian = "little")]
        {
            self
        }
        #[cfg(target_endian = "big")]
        {
            self.swap_bytes()
        }
    }

    /// Creates an integer value from its memory representation as a byte array in bit endian.
    pub const fn from_be_bytes(bytes: [u8; 11]) -> Self {
        Self(u128::from_be_bytes([
            0, 0, 0, 0, 0, bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6],
            bytes[7], bytes[8], bytes[9], bytes[10],
        ]))
    }

    /// Creates an integer value from its memory representation as a byte array in little endian.
    pub const fn from_le_bytes(bytes: [u8; 11]) -> Self {
        Self(u128::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            bytes[8], bytes[9], bytes[10], 0, 0, 0, 0, 0,
        ]))
    }

    /// Creates an integer value from its memory representation as a byte array in native
    /// endianness.
    ///
    /// As the target platform's native endianness is used, portable code likely wants to
    /// use [`from_be_bytes`](Self::from_be_bytes) or [`from_le_bytes`](Self::from_le_bytes),
    /// as appropriate instead.
    pub const fn from_ne_bytes(bytes: [u8; 11]) -> Self {
        #[cfg(target_endian = "big")]
        {
            Self::from_be_bytes(bytes)
        }
        #[cfg(target_endian = "little")]
        {
            Self::from_le_bytes(bytes)
        }
    }

    /// Return the memory representation of this integer as a byte array in big-endian byte
    /// order.
    pub const fn to_be_bytes(self) -> [u8; 11] {
        let tmp = self.0.to_be_bytes();
        [
            tmp[5], tmp[6], tmp[7], tmp[8], tmp[9], tmp[10], tmp[11], tmp[12], tmp[13], tmp[14],
            tmp[15],
        ]
    }

    /// Return the memory representation of this integer as a byte array in little-endian
    /// byte order.
    pub const fn to_le_bytes(self) -> [u8; 11] {
        let tmp = self.0.to_le_bytes();
        [
            tmp[0], tmp[1], tmp[2], tmp[3], tmp[4], tmp[5], tmp[6], tmp[7], tmp[8], tmp[9], tmp[10],
        ]
    }

    /// Return the memory representation of this integer as a byte array in native byte
    /// order.
    ///
    /// As the target platform's native endianness is used, portable code likely wants to
    /// use [`to_be_bytes`](Self::to_be_bytes) or [`to_le_bytes`](Self::to_le_bytes), as
    /// appropriate instead.
    pub const fn to_ne_bytes(self) -> [u8; 11] {
        #[cfg(target_endian = "big")]
        {
            self.to_be_bytes()
        }
        #[cfg(target_endian = "little")]
        {
            self.to_le_bytes()
        }
    }
}

impl Aint<u128, 96> {
    /// Reverses the byte order of the integer.
    pub const fn swap_bytes(self) -> Self {
        Self(self.0.swap_bytes() >> 32)
    }

    /// Converts an integer from big endian to the target's endianness.
    ///
    /// On big endian this is a no-op. On little endian the bytes are swapped.
    pub const fn from_be(x: Self) -> Self {
        #[cfg(target_endian = "big")]
        {
            x
        }
        #[cfg(target_endian = "little")]
        {
            x.swap_bytes()
        }
    }

    /// Converts an integer from little endian to the target's endianness.
    ///
    /// On little endian this is a no-op. On big endian the bytes are swapped.
    pub const fn from_le(x: Self) -> Self {
        #[cfg(target_endian = "little")]
        {
            x
        }
        #[cfg(target_endian = "big")]
        {
            x.swap_bytes()
        }
    }

    /// Converts `self` to big endian from the target's endianness.
    ///
    /// On big endian this is a no-op. On little endian the bytes are swapped.
    pub const fn to_be(self) -> Self {
        #[cfg(target_endian = "big")]
        {
            self
        }
        #[cfg(target_endian = "little")]
        {
            self.swap_bytes()
        }
    }

    /// Converts `self` to little endian from the target's endianness.
    ///
    /// On little endian this is a no-op. On big endian the bytes are swapped.
    pub const fn to_le(self) -> Self {
        #[cfg(target_endian = "little")]
        {
            self
        }
        #[cfg(target_endian = "big")]
        {
            self.swap_bytes()
        }
    }

    /// Creates an integer value from its memory representation as a byte array in bit endian.
    pub const fn from_be_bytes(bytes: [u8; 12]) -> Self {
        Self(u128::from_be_bytes([
            0, 0, 0, 0, bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6],
            bytes[7], bytes[8], bytes[9], bytes[10], bytes[11],
        ]))
    }

    /// Creates an integer value from its memory representation as a byte array in little endian.
    pub const fn from_le_bytes(bytes: [u8; 12]) -> Self {
        Self(u128::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            bytes[8], bytes[9], bytes[10], bytes[11], 0, 0, 0, 0,
        ]))
    }

    /// Creates an integer value from its memory representation as a byte array in native
    /// endianness.
    ///
    /// As the target platform's native endianness is used, portable code likely wants to
    /// use [`from_be_bytes`](Self::from_be_bytes) or [`from_le_bytes`](Self::from_le_bytes),
    /// as appropriate instead.
    pub const fn from_ne_bytes(bytes: [u8; 12]) -> Self {
        #[cfg(target_endian = "big")]
        {
            Self::from_be_bytes(bytes)
        }
        #[cfg(target_endian = "little")]
        {
            Self::from_le_bytes(bytes)
        }
    }

    /// Return the memory representation of this integer as a byte array in big-endian byte
    /// order.
    pub const fn to_be_bytes(self) -> [u8; 12] {
        let tmp = self.0.to_be_bytes();
        [
            tmp[4], tmp[5], tmp[6], tmp[7], tmp[8], tmp[9], tmp[10], tmp[11], tmp[12], tmp[13],
            tmp[14], tmp[15],
        ]
    }

    /// Return the memory representation of this integer as a byte array in little-endian
    /// byte order.
    pub const fn to_le_bytes(self) -> [u8; 12] {
        let tmp = self.0.to_le_bytes();
        [
            tmp[0], tmp[1], tmp[2], tmp[3], tmp[4], tmp[5], tmp[6], tmp[7], tmp[8], tmp[9],
            tmp[10], tmp[11],
        ]
    }

    /// Return the memory representation of this integer as a byte array in native byte
    /// order.
    ///
    /// As the target platform's native endianness is used, portable code likely wants to
    /// use [`to_be_bytes`](Self::to_be_bytes) or [`to_le_bytes`](Self::to_le_bytes), as
    /// appropriate instead.
    pub const fn to_ne_bytes(self) -> [u8; 12] {
        #[cfg(target_endian = "big")]
        {
            self.to_be_bytes()
        }
        #[cfg(target_endian = "little")]
        {
            self.to_le_bytes()
        }
    }
}

impl Aint<u128, 104> {
    /// Reverses the byte order of the integer.
    pub const fn swap_bytes(self) -> Self {
        Self(self.0.swap_bytes() >> 24)
    }

    /// Converts an integer from big endian to the target's endianness.
    ///
    /// On big endian this is a no-op. On little endian the bytes are swapped.
    pub const fn from_be(x: Self) -> Self {
        #[cfg(target_endian = "big")]
        {
            x
        }
        #[cfg(target_endian = "little")]
        {
            x.swap_bytes()
        }
    }

    /// Converts an integer from little endian to the target's endianness.
    ///
    /// On little endian this is a no-op. On big endian the bytes are swapped.
    pub const fn from_le(x: Self) -> Self {
        #[cfg(target_endian = "little")]
        {
            x
        }
        #[cfg(target_endian = "big")]
        {
            x.swap_bytes()
        }
    }

    /// Converts `self` to big endian from the target's endianness.
    ///
    /// On big endian this is a no-op. On little endian the bytes are swapped.
    pub const fn to_be(self) -> Self {
        #[cfg(target_endian = "big")]
        {
            self
        }
        #[cfg(target_endian = "little")]
        {
            self.swap_bytes()
        }
    }

    /// Converts `self` to little endian from the target's endianness.
    ///
    /// On little endian this is a no-op. On big endian the bytes are swapped.
    pub const fn to_le(self) -> Self {
        #[cfg(target_endian = "little")]
        {
            self
        }
        #[cfg(target_endian = "big")]
        {
            self.swap_bytes()
        }
    }

    /// Creates an integer value from its memory representation as a byte array in bit endian.
    pub const fn from_be_bytes(bytes: [u8; 13]) -> Self {
        Self(u128::from_be_bytes([
            0, 0, 0, bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6],
            bytes[7], bytes[8], bytes[9], bytes[10], bytes[11], bytes[12],
        ]))
    }

    /// Creates an integer value from its memory representation as a byte array in little endian.
    pub const fn from_le_bytes(bytes: [u8; 13]) -> Self {
        Self(u128::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            bytes[8], bytes[9], bytes[10], bytes[11], bytes[12], 0, 0, 0,
        ]))
    }

    /// Creates an integer value from its memory representation as a byte array in native
    /// endianness.
    ///
    /// As the target platform's native endianness is used, portable code likely wants to
    /// use [`from_be_bytes`](Self::from_be_bytes) or [`from_le_bytes`](Self::from_le_bytes),
    /// as appropriate instead.
    pub const fn from_ne_bytes(bytes: [u8; 13]) -> Self {
        #[cfg(target_endian = "big")]
        {
            Self::from_be_bytes(bytes)
        }
        #[cfg(target_endian = "little")]
        {
            Self::from_le_bytes(bytes)
        }
    }

    /// Return the memory representation of this integer as a byte array in big-endian byte
    /// order.
    pub const fn to_be_bytes(self) -> [u8; 13] {
        let tmp = self.0.to_be_bytes();
        [
            tmp[3], tmp[4], tmp[5], tmp[6], tmp[7], tmp[8], tmp[9], tmp[10], tmp[11], tmp[12],
            tmp[13], tmp[14], tmp[15],
        ]
    }

    /// Return the memory representation of this integer as a byte array in little-endian
    /// byte order.
    pub const fn to_le_bytes(self) -> [u8; 13] {
        let tmp = self.0.to_le_bytes();
        [
            tmp[0], tmp[1], tmp[2], tmp[3], tmp[4], tmp[5], tmp[6], tmp[7], tmp[8], tmp[9],
            tmp[10], tmp[11], tmp[12],
        ]
    }

    /// Return the memory representation of this integer as a byte array in native byte
    /// order.
    ///
    /// As the target platform's native endianness is used, portable code likely wants to
    /// use [`to_be_bytes`](Self::to_be_bytes) or [`to_le_bytes`](Self::to_le_bytes), as
    /// appropriate instead.
    pub const fn to_ne_bytes(self) -> [u8; 13] {
        #[cfg(target_endian = "big")]
        {
            self.to_be_bytes()
        }
        #[cfg(target_endian = "little")]
        {
            self.to_le_bytes()
        }
    }
}

impl Aint<u128, 112> {
    /// Reverses the byte order of the integer.
    pub const fn swap_bytes(self) -> Self {
        Self(self.0.swap_bytes() >> 16)
    }

    /// Converts an integer from big endian to the target's endianness.
    ///
    /// On big endian this is a no-op. On little endian the bytes are swapped.
    pub const fn from_be(x: Self) -> Self {
        #[cfg(target_endian = "big")]
        {
            x
        }
        #[cfg(target_endian = "little")]
        {
            x.swap_bytes()
        }
    }

    /// Converts an integer from little endian to the target's endianness.
    ///
    /// On little endian this is a no-op. On big endian the bytes are swapped.
    pub const fn from_le(x: Self) -> Self {
        #[cfg(target_endian = "little")]
        {
            x
        }
        #[cfg(target_endian = "big")]
        {
            x.swap_bytes()
        }
    }

    /// Converts `self` to big endian from the target's endianness.
    ///
    /// On big endian this is a no-op. On little endian the bytes are swapped.
    pub const fn to_be(self) -> Self {
        #[cfg(target_endian = "big")]
        {
            self
        }
        #[cfg(target_endian = "little")]
        {
            self.swap_bytes()
        }
    }

    /// Converts `self` to little endian from the target's endianness.
    ///
    /// On little endian this is a no-op. On big endian the bytes are swapped.
    pub const fn to_le(self) -> Self {
        #[cfg(target_endian = "little")]
        {
            self
        }
        #[cfg(target_endian = "big")]
        {
            self.swap_bytes()
        }
    }

    /// Creates an integer value from its memory representation as a byte array in bit endian.
    pub const fn from_be_bytes(bytes: [u8; 14]) -> Self {
        Self(u128::from_be_bytes([
            0, 0, bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            bytes[8], bytes[9], bytes[10], bytes[11], bytes[12], bytes[13],
        ]))
    }

    /// Creates an integer value from its memory representation as a byte array in little endian.
    pub const fn from_le_bytes(bytes: [u8; 14]) -> Self {
        Self(u128::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            bytes[8], bytes[9], bytes[10], bytes[11], bytes[12], bytes[13], 0, 0,
        ]))
    }

    /// Creates an integer value from its memory representation as a byte array in native
    /// endianness.
    ///
    /// As the target platform's native endianness is used, portable code likely wants to
    /// use [`from_be_bytes`](Self::from_be_bytes) or [`from_le_bytes`](Self::from_le_bytes),
    /// as appropriate instead.
    pub const fn from_ne_bytes(bytes: [u8; 14]) -> Self {
        #[cfg(target_endian = "big")]
        {
            Self::from_be_bytes(bytes)
        }
        #[cfg(target_endian = "little")]
        {
            Self::from_le_bytes(bytes)
        }
    }

    /// Return the memory representation of this integer as a byte array in big-endian byte
    /// order.
    pub const fn to_be_bytes(self) -> [u8; 14] {
        let tmp = self.0.to_be_bytes();
        [
            tmp[2], tmp[3], tmp[4], tmp[5], tmp[6], tmp[7], tmp[8], tmp[9], tmp[10], tmp[11],
            tmp[12], tmp[13], tmp[14], tmp[15],
        ]
    }

    /// Return the memory representation of this integer as a byte array in little-endian
    /// byte order.
    pub const fn to_le_bytes(self) -> [u8; 14] {
        let tmp = self.0.to_le_bytes();
        [
            tmp[0], tmp[1], tmp[2], tmp[3], tmp[4], tmp[5], tmp[6], tmp[7], tmp[8], tmp[9],
            tmp[10], tmp[11], tmp[12], tmp[13],
        ]
    }

    /// Return the memory representation of this integer as a byte array in native byte
    /// order.
    ///
    /// As the target platform's native endianness is used, portable code likely wants to
    /// use [`to_be_bytes`](Self::to_be_bytes) or [`to_le_bytes`](Self::to_le_bytes), as
    /// appropriate instead.
    pub const fn to_ne_bytes(self) -> [u8; 14] {
        #[cfg(target_endian = "big")]
        {
            self.to_be_bytes()
        }
        #[cfg(target_endian = "little")]
        {
            self.to_le_bytes()
        }
    }
}

impl Aint<u128, 120> {
    /// Reverses the byte order of the integer.
    pub const fn swap_bytes(self) -> Self {
        Self(self.0.swap_bytes() >> 8)
    }

    /// Converts an integer from big endian to the target's endianness.
    ///
    /// On big endian this is a no-op. On little endian the bytes are swapped.
    pub const fn from_be(x: Self) -> Self {
        #[cfg(target_endian = "big")]
        {
            x
        }
        #[cfg(target_endian = "little")]
        {
            x.swap_bytes()
        }
    }

    /// Converts an integer from little endian to the target's endianness.
    ///
    /// On little endian this is a no-op. On big endian the bytes are swapped.
    pub const fn from_le(x: Self) -> Self {
        #[cfg(target_endian = "little")]
        {
            x
        }
        #[cfg(target_endian = "big")]
        {
            x.swap_bytes()
        }
    }

    /// Converts `self` to big endian from the target's endianness.
    ///
    /// On big endian this is a no-op. On little endian the bytes are swapped.
    pub const fn to_be(self) -> Self {
        #[cfg(target_endian = "big")]
        {
            self
        }
        #[cfg(target_endian = "little")]
        {
            self.swap_bytes()
        }
    }

    /// Converts `self` to little endian from the target's endianness.
    ///
    /// On little endian this is a no-op. On big endian the bytes are swapped.
    pub const fn to_le(self) -> Self {
        #[cfg(target_endian = "little")]
        {
            self
        }
        #[cfg(target_endian = "big")]
        {
            self.swap_bytes()
        }
    }

    /// Creates an integer value from its memory representation as a byte array in bit endian.
    pub const fn from_be_bytes(bytes: [u8; 15]) -> Self {
        Self(u128::from_be_bytes([
            0, bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            bytes[8], bytes[9], bytes[10], bytes[11], bytes[12], bytes[13], bytes[14],
        ]))
    }

    /// Creates an integer value from its memory representation as a byte array in little endian.
    pub const fn from_le_bytes(bytes: [u8; 15]) -> Self {
        Self(u128::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            bytes[8], bytes[9], bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], 0,
        ]))
    }

    /// Creates an integer value from its memory representation as a byte array in native
    /// endianness.
    ///
    /// As the target platform's native endianness is used, portable code likely wants to
    /// use [`from_be_bytes`](Self::from_be_bytes) or [`from_le_bytes`](Self::from_le_bytes),
    /// as appropriate instead.
    pub const fn from_ne_bytes(bytes: [u8; 15]) -> Self {
        #[cfg(target_endian = "big")]
        {
            Self::from_be_bytes(bytes)
        }
        #[cfg(target_endian = "little")]
        {
            Self::from_le_bytes(bytes)
        }
    }

    /// Return the memory representation of this integer as a byte array in big-endian byte
    /// order.
    pub const fn to_be_bytes(self) -> [u8; 15] {
        let tmp = self.0.to_be_bytes();
        [
            tmp[1], tmp[2], tmp[3], tmp[4], tmp[5], tmp[6], tmp[7], tmp[8], tmp[9], tmp[10],
            tmp[11], tmp[12], tmp[13], tmp[14], tmp[15],
        ]
    }

    /// Return the memory representation of this integer as a byte array in little-endian
    /// byte order.
    pub const fn to_le_bytes(self) -> [u8; 15] {
        let tmp = self.0.to_le_bytes();
        [
            tmp[0], tmp[1], tmp[2], tmp[3], tmp[4], tmp[5], tmp[6], tmp[7], tmp[8], tmp[9],
            tmp[10], tmp[11], tmp[12], tmp[13], tmp[14],
        ]
    }

    /// Return the memory representation of this integer as a byte array in native byte
    /// order.
    ///
    /// As the target platform's native endianness is used, portable code likely wants to
    /// use [`to_be_bytes`](Self::to_be_bytes) or [`to_le_bytes`](Self::to_le_bytes), as
    /// appropriate instead.
    pub const fn to_ne_bytes(self) -> [u8; 15] {
        #[cfg(target_endian = "big")]
        {
            self.to_be_bytes()
        }
        #[cfg(target_endian = "little")]
        {
            self.to_le_bytes()
        }
    }
}

impl<const WIDTH: u32> Aint<i8, WIDTH> {
    const ASSERT: () = if WIDTH > 0 && WIDTH < 8 {
        ()
    } else {
        panic!("Invalid width for Aint with repr type of i8")
    };

    /// The size of this integer type in bits.
    pub const BITS: u32 = {
        let _val = Self::ASSERT;
        WIDTH
    };

    /// The smallest value that can be represented by this integer type.
    pub const MIN: Self = {
        let _val = Self::ASSERT;
        Self(!0 << (Self::BITS - 1))
    };

    /// The largest value that can be represented by this integer type.
    pub const MAX: Self = {
        let _val = Self::ASSERT;
        Self(!Self::MIN.0)
    };

    pub(crate) const MASK: i8 = {
        let _val = Self::ASSERT;
        (Self::MAX.0 << 1) | 1
    };

    const SIGN_BIT: i8 = {
        let _val = Self::ASSERT;
        1 << (Self::BITS - 1)
    };

    /// Creates a new integer value from the underlying representation type, unchecked.
    ///
    /// # Safety
    /// The representation type value must be within the range of valid values of this type.
    /// That is, the value must be greater or equal to [`MIN`](Self::MIN) and less or equal to
    /// [`MAX`](Self::MAX).
    pub const unsafe fn new_unchecked(repr: i8) -> Self {
        let _val = Self::ASSERT;
        Self(repr)
    }

    /// Creates a new integer value from the underlying representation type.
    ///
    /// The returned value is `None` if the representation value is outside the range of valid
    /// values of this integer type.
    pub const fn new(repr: i8) -> Option<Self> {
        if repr <= Self::MAX.0 && repr >= Self::MIN.0 {
            Some(Self(repr))
        } else {
            None
        }
    }

    /// Creates a new integer value from the underlying representation type.
    ///
    /// The returned value is wrapped as though the representation value were calculated using
    /// wrapping operations, such as [`wrapping_add`](Self::wrapping_add).
    pub const fn new_wrapping(repr: i8) -> Self {
        if (repr & Self::SIGN_BIT) == 0 {
            Self(repr & Self::MAX.0)
        } else {
            Self(repr | !Self::MAX.0)
        }
    }

    /// Creates a new integer value from the underlying representation type.
    ///
    /// The returned value is saturated to the bounds of this integer's value range. If the
    /// representation value is greater than [`MAX`](Self::MAX), the returned value will be
    /// [`MAX`](Self::MAX). If the representation value is less than [`MIN`](Self::MIN), the
    /// returned value will be [`MIN`](Self::MIN).
    pub const fn new_saturating(repr: i8) -> Self {
        if repr >= Self::MAX.0 {
            Self::MAX
        } else if repr <= Self::MIN.0 {
            Self::MIN
        } else {
            Self(repr)
        }
    }

    const fn new_overflowing_impl((repr, over): (i8, bool)) -> (Self, bool) {
        if repr > Self::MAX.0 {
            (Self(repr & Self::MAX.0), true)
        } else if repr < Self::MIN.0 {
            (Self(repr | !Self::MAX.0), true)
        } else {
            (Self(repr), over)
        }
    }

    /// Creates a new integer from the underlying representation type.
    ///
    /// The returned tuple contains the new integer and a `bool` indicating if the representation
    /// value overflowed the new integer. In the case of overflow, the new integer has a value
    /// as if it were produced from [`new_wrapping`](Self::new_wrapping).
    pub const fn new_overflowing(repr: i8) -> (Self, bool) {
        Self::new_overflowing_impl((repr, false))
    }

    /// Returns the value of this integer as the underlying representation type.
    pub const fn repr(self) -> i8 {
        self.0
    }

    /// Returns `true` if this integer is less than zero.
    pub const fn is_negative(self) -> bool {
        (self.0 & Self::SIGN_BIT) != 0
    }

    /// Returns `true` if this integer is greater than zero.
    pub const fn is_positive(self) -> bool {
        self.0 != 0 && (self.0 & Self::SIGN_BIT) == 0
    }

    /// Returns the sign of the integer.
    ///
    /// * If `self < 0`, returns `-1`
    /// * If `self > 0`, returns `1`
    /// * If `self == 0`, returns `0`
    pub const fn signum(self) -> Self {
        if self.0 == 0 {
            Self(0)
        } else if self.0 > 0 {
            Self(1)
        } else {
            Self(-1)
        }
    }

    /// Returns the number of ones in the binary representation of this integer.
    pub const fn count_ones(self) -> u32 {
        (self.0 & Self::MASK).count_ones()
    }

    /// Returns the number of zeros in the binary representatino of this integer.
    pub const fn count_zeros(self) -> u32 {
        (self.0 | !Self::MASK).count_zeros()
    }

    /// Returns the number of leading zeros in the binary represnetation of this integer.
    pub const fn leading_zeros(self) -> u32 {
        if self.is_negative() {
            0
        } else {
            self.0.leading_zeros() - (i8::BITS - Self::BITS)
        }
    }

    /// Returns the number of trailing zeros in the binary representatino of this integer.
    pub const fn trailing_zeros(self) -> u32 {
        (self.0 | !Self::MASK).trailing_zeros()
    }

    /// Returns the number of leading ones in the binary representation of this integer.
    pub const fn leading_ones(self) -> u32 {
        (self.0 | !Self::MASK).leading_ones() - (i8::BITS - Self::BITS)
    }

    /// Returns the number of trailing ones in the binary representation of this integer.
    pub const fn trailing_ones(self) -> u32 {
        (self.0 & Self::MASK).trailing_ones()
    }

    /// Performs a left bit shift by `n`, wrapping the truncated bits back to the end.
    ///
    /// Note that unlike the `<<` operator, all values of `n` are valid. A value of `n`, that
    /// is greater than or equal to [`BITS`](Self::BITS), is the equivalent to using the value
    /// `n % Self::BITS`.
    pub const fn rotate_left(self, n: u32) -> Self {
        let n = n % Self::BITS;
        Self::new_wrapping((self.0 << n) | ((self.0 & Self::MASK) >> (Self::BITS - n)))
    }

    /// Performs a right bit shift by `n`, wrapping the truncated bits back to the end.
    ///
    /// Note that unlike the `>>` operator, all values of `n` are valid. A value of `n`, that
    /// is greater than or equal to [`BITS`](Self::BITS), is the equivalent to using the value
    /// `n % Self::BITS`.
    pub const fn rotate_right(self, n: u32) -> Self {
        let n = n % Self::BITS;
        Self::new_wrapping(((self.0 & Self::MASK) >> n) | (self.0 << (Self::BITS - n)))
    }

    /// Reverses the order of the bits in the binary representation of this integer.
    pub const fn reverse_bits(self) -> Self {
        Self(self.0.reverse_bits() >> (i8::BITS - Self::BITS))
    }

    /// Calculates the negative of this integer.
    ///
    /// This method works as a `const` capable alternative to `-self`.
    pub const fn neg(self) -> Self {
        debug_assert!(self.0 != Self::MIN.0, "attempt to negate with overflow");
        self.wrapping_neg()
    }

    /// Calculates the sum of this integer with another integer.
    ///
    /// This method works as a `const` capable alternative to `self + rhs`.
    pub const fn add(self, rhs: Self) -> Self {
        let val = self.0 + rhs.0;
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to add with overflow"
        );
        Self::new_wrapping(val)
    }

    /// Calculates the difference of this integer with another integer.
    ///
    /// This method works as a `const` capable alternative to `self - rhs`.
    pub const fn sub(self, rhs: Self) -> Self {
        let val = self.0 - rhs.0;
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to subtract with overflow"
        );
        Self::new_wrapping(val)
    }

    /// Calculates the product of this integer with another integer.
    ///
    /// This method works as a `const` capable alternative to `self * rhs`.
    pub const fn mul(self, rhs: Self) -> Self {
        let val = self.0 * rhs.0;
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to multiply with overflow"
        );
        Self::new_wrapping(val)
    }

    /// Calculates the quotient of this integer over another integer.
    ///
    /// This method works as a `const` capable alternative to `self / rhs`.
    pub const fn div(self, rhs: Self) -> Self {
        let val = self.0 / rhs.0;
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to divide with overflow"
        );
        Self(val)
    }

    /// Calculates the remainder of this integer over another integer.
    ///
    /// This method works as a `const` capable alternative to `self % rhs`.
    pub const fn rem(self, rhs: Self) -> Self {
        let val = self.0 % rhs.0;
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to calculate modulus with overflow"
        );
        Self(val)
    }

    /// Calculates the bitwise and of this integer with another integer.
    ///
    /// This method works as a `const` capable alternative to `self & rhs`.
    pub const fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }

    /// Calculates the bitwise or of this integer with another integer.
    ///
    /// This method works as a `const` capable alternative to `self | rhs`.
    pub const fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }

    /// Calculates the bitwise exclusive-or of this integer with another integer.
    ///
    /// This method works as a `const` capable alternative to `self ^ rhs`.
    pub const fn bitxor(self, rhs: Self) -> Self {
        Self(self.0 ^ rhs.0)
    }

    /// Calculates the bitwise negation of this integer.
    ///
    /// This method works as a `const` capable alternative to `!self`.
    pub const fn not(self) -> Self {
        Self(!self.0)
    }

    /// Calculates the left bitwise shift of this integer by `rhs`.
    ///
    /// This method works as a `const` capable alternative to `self << rhs`.
    pub const fn shl(self, rhs: u32) -> Self {
        debug_assert!(rhs >= Self::BITS, "attempt to shift left with overflow");
        Self::new_wrapping(self.0 << rhs)
    }

    /// Calculates the right bitwise shift of this integer by `rhs`.
    ///
    /// This method works as a `const` capable alternative to `self >> rhs`.
    pub const fn shr(self, rhs: u32) -> Self {
        debug_assert!(rhs >= Self::BITS, "attempt to shift right with overflow");
        Self::new_wrapping(self.0 >> rhs)
    }

    /// Calculates the logarithm of this integer with respect to an arbitrary base, rounded
    /// down.
    ///
    /// The [`ilog2`](Self::ilog2) and [`ilog10`](Self::ilog10) methods should be preferred
    /// when applicable, as they are generally more optimized than this method since the base
    /// of the logarithm is not arbitrary.
    ///
    /// # Panics
    /// This method will panic if `self` is less than or equal to zero, or if `base` is less
    /// than 2.
    pub const fn ilog(self, base: Self) -> u32 {
        self.0.ilog(base.0)
    }

    /// Calculates the base 2 logarithm of this integer, rounded down.
    ///
    /// # Panics
    /// This method will panic if `self` is less than or equal to zero.
    pub const fn ilog2(self) -> u32 {
        self.0.ilog2()
    }

    /// Calculates the base 10 logarithm of this integer, rounded down.
    ///
    /// # Panics
    /// This method will panic if `self` is less than or equal to zero.
    pub const fn ilog10(self) -> u32 {
        self.0.ilog10()
    }

    /// Returns `true` if `self` and `rhs` are equal.
    ///
    /// This method works as a `const` capable alternative to `self == rhs`.
    pub const fn eq(self, rhs: Self) -> bool {
        return self.0 == rhs.0;
    }

    /// Returns `true` if `self` and `rhs` are not equal.
    ///
    /// This method works as a `const` capable alternative to `self != rhs`.
    pub const fn ne(self, rhs: Self) -> bool {
        return self.0 != rhs.0;
    }

    /// Returns `true` if `self` is less than `rhs`.
    ///
    /// This method works as a `const` capable alternative to `self < rhs`.
    pub const fn lt(self, rhs: Self) -> bool {
        return self.0 < rhs.0;
    }

    /// Returns `true` if `self` is less than or equal to `rhs`.
    ///
    /// This method works as a `const` capable alternative to `self <= rhs`.
    pub const fn le(self, rhs: Self) -> bool {
        return self.0 <= rhs.0;
    }

    /// Returns `true` if `self` is greater than `rhs`.
    ///
    /// This method works as a `const` capable alternative to `self > rhs`.
    pub const fn gt(self, rhs: Self) -> bool {
        return self.0 > rhs.0;
    }

    /// Returns `true` if `self` is greater than or equal to `rhs`.
    ///
    /// This method works as a `const` capable alternative to `self >= rhs`.
    pub const fn ge(self, rhs: Self) -> bool {
        return self.0 >= rhs.0;
    }

    /// Returns the ordering of `self` with respect to `rhs`.
    ///
    /// This method works as a `const` capable alternative to `Ord::cmp(self, rhs)`.
    pub const fn cmp(self, rhs: Self) -> core::cmp::Ordering {
        if self.0 < rhs.0 {
            core::cmp::Ordering::Less
        } else if self.0 > rhs.0 {
            core::cmp::Ordering::Greater
        } else {
            core::cmp::Ordering::Equal
        }
    }

    /// Returns the minimum of `self` and `other`.
    ///
    /// This method works as a `const` capable alternative to `Ord::min(self, rhs)`.
    pub const fn min(self, other: Self) -> Self {
        if self.0 < other.0 {
            self
        } else {
            other
        }
    }

    /// Returns the maximum of `self` and `other`.
    ///
    /// This method works as a `const` capable alternative to `Ord::max(self, rhs)`.
    pub const fn max(self, other: Self) -> Self {
        if self.0 > other.0 {
            self
        } else {
            other
        }
    }

    /// Returns the value of this integer, clamped between `min` and `max`.
    ///
    /// This method works as a `const` capable alternative to `Ord::clamp(self, min, max)`.
    pub const fn clamp(self, min: Self, max: Self) -> Self {
        debug_assert!(
            min.0 <= max.0,
            "Attempt to clamp with minimum value greater than maximum value"
        );
        Self::min(Self::max(self, min), max)
    }

    /// Checked integer addition. Returns `None` if overflow occurred.
    pub const fn checked_add(self, rhs: Self) -> Option<Self> {
        match self.0.checked_add(rhs.0) {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked integer subtraction. Returns `None` if overflow occurred.
    pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
        match self.0.checked_sub(rhs.0) {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked integer multiplication. Returns `None` if overflow occurred.
    pub const fn checked_mul(self, rhs: Self) -> Option<Self> {
        match self.0.checked_mul(rhs.0) {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked integer division. Returns `None` if `rhs` is zero, or the division resulted in
    /// overflow.
    pub const fn checked_div(self, rhs: Self) -> Option<Self> {
        match self.0.checked_div(rhs.0) {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked euclidian division. Returns `None` if `rhs` is zero or `self.div_euclid(rhs)`
    /// would have resulted in overflow.
    pub const fn checked_div_euclid(self, rhs: Self) -> Option<Self> {
        match self.0.checked_div_euclid(rhs.0) {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked integer remainder. Returns `None` if `rhs` is zero or the remainder would have
    /// resulted in overflow.
    pub const fn checked_rem(self, rhs: Self) -> Option<Self> {
        match self.0.checked_rem(rhs.0) {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked euclidian remainder. Returns `None` if `rhs` is zero or `self.rem_euclid(rhs)`
    /// would have resulted in overflow.
    pub const fn checked_rem_euclid(self, rhs: Self) -> Option<Self> {
        match self.0.checked_rem_euclid(rhs.0) {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked integer negation. Returns `None` if the negation resulted in overflow.
    pub const fn checked_neg(self) -> Option<Self> {
        match self.0.checked_neg() {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked left bit shift. Returns `None` if `rhs` is greater than or equal to
    /// [`Self::BITS`].
    pub const fn checked_shl(self, rhs: u32) -> Option<Self> {
        if rhs >= Self::BITS {
            None
        } else {
            Some(self.shl(rhs))
        }
    }

    /// Checked right bit shift. Returns `None` if `rhs` is greater than or equal to
    /// [`Self::BITS`].
    pub const fn checked_shr(self, rhs: u32) -> Option<Self> {
        if rhs >= Self::BITS {
            None
        } else {
            Some(self.shr(rhs))
        }
    }

    /// Checked exponentiation. Returns `None` if the exponentiation resulted in overflow.
    pub const fn checked_pow(self, exp: u32) -> Option<Self> {
        match self.0.checked_pow(exp) {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked logarithm, rounded down. Returns `None` if `self` is less than or equal zero,
    /// or if `base` is less than 2.
    pub const fn checked_ilog(self, base: Self) -> Option<u32> {
        self.0.checked_ilog(base.0)
    }

    /// Checked base 2 logarithm, rounded down. Returns `None` if self is less than or equal to
    /// zero.
    pub const fn checked_ilog2(self) -> Option<u32> {
        self.0.checked_ilog2()
    }

    /// Checked base 10 logarithm, rounded down. Returns `None` if self is less than or equal
    /// to zero.
    pub const fn checked_ilog10(self) -> Option<u32> {
        self.0.checked_ilog10()
    }

    /// Satruating integer addition. The result of the addition is clamped between
    /// [`MIN`](Self::MIN) and [`MAX`](Self::MAX).
    pub const fn saturating_add(self, rhs: Self) -> Self {
        Self::new_saturating(self.0.saturating_add(rhs.0))
    }

    /// Saturating integer subtraction. The result of the subtraction is clamped between
    /// [`MIN`](Self::MIN) and [`MAX`](Self::MAX).
    pub const fn saturating_sub(self, rhs: Self) -> Self {
        Self::new_saturating(self.0.saturating_sub(rhs.0))
    }

    /// Saturating integer multiplication. The result of the multiplication is clamped between
    /// [`MIN`](Self::MIN) and [`MAX`](Self::MAX).
    pub const fn saturating_mul(self, rhs: Self) -> Self {
        Self::new_saturating(self.0.saturating_mul(rhs.0))
    }

    /// Saturating exponentiation. The result of the exponentiation is clammed between
    /// [`MIN`](Self::MIN) and [`MAX`](Self::MAX).
    pub const fn saturating_pow(self, exp: u32) -> Self {
        Self::new_saturating(self.0.saturating_pow(exp))
    }

    /// Wrapping (modular) addition. An overflowing result is wrapped around the bounds of this
    /// integer type.
    pub const fn wrapping_add(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.wrapping_add(rhs.0))
    }

    /// Wrapping (modular) subtraction. An overflowing result is wrapped around the bounds of
    /// this integer type.
    pub const fn wrapping_sub(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.wrapping_sub(rhs.0))
    }

    /// Wrapping (modular) multiplication. An overflowing result is wrapped around the bounds of
    /// this integer type.
    pub const fn wrapping_mul(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.wrapping_mul(rhs.0))
    }

    /// Wrapping (modular) division. An overflowing result is wrapped around the bounds of this
    /// integer type.
    pub const fn wrapping_div(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.wrapping_div(rhs.0))
    }

    /// Wrapping (modular) euclidian division. An overflowing result is wrapped around the
    /// bounds of this integer type.
    pub const fn wrapping_div_euclid(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.wrapping_div_euclid(rhs.0))
    }

    /// Wrapping (modular) remainder. An overflowing result is wrapped around the bounds of
    /// this integer type.
    pub const fn wrapping_rem(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.wrapping_rem(rhs.0))
    }

    /// Wrapping (modular) euclidian remainder. An overflowing result is wrapped around the
    /// bounds of this integer type.
    pub const fn wrapping_rem_euclid(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.wrapping_rem_euclid(rhs.0))
    }

    /// Wrapping (modular) negation. An overflowing result is wrapped around the bounds of this
    /// integer type.
    pub const fn wrapping_neg(self) -> Self {
        Self::new_wrapping(!self.0 + 1)
    }

    /// Wrapping (modular) left bit shift. The value is left shifted `rhs % Self::BITS` bits.
    pub const fn wrapping_shl(self, rhs: u32) -> Self {
        Self::new_wrapping(self.0 << (rhs % Self::BITS))
    }

    /// Wrapping (modular) right bit shift. The value is right shifted `rhs % Self::BITS` bits.
    pub const fn wrapping_shr(self, rhs: u32) -> Self {
        Self::new_wrapping((self.0 & Self::MASK) >> (rhs % Self::BITS))
    }

    /// Wrapping (modular) exponentiation. An overflowing result is wrapped around the bounds
    /// of this integer type.
    pub const fn wrapping_pow(self, exp: u32) -> Self {
        Self::new_wrapping(self.0.wrapping_pow(exp))
    }

    /// Wrapping (modular) aboslute value. An overflowing result is wrapped around the bounds
    /// of this integer type.
    pub const fn wrapping_abs(self) -> Self {
        if self.is_negative() {
            self.wrapping_neg()
        } else {
            self
        }
    }

    /// Calculates `self + rhs`, returning a tuple of the result and a `bool` indicating if
    /// overflow occurred.
    pub const fn overflowing_add(self, rhs: Self) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_add(rhs.0))
    }

    /// Calculates `self - rhs`, returning a tuple of the result and a `bool` indicating if
    /// overflow occurred.
    pub const fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_sub(rhs.0))
    }

    /// Calculates `self * rhs`, returning a tuple of the result and a `bool` indicating if
    /// overflow occurred.
    pub const fn overflowing_mul(self, rhs: Self) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_mul(rhs.0))
    }

    /// Calculates `self / rhs`, returning a tuple of the result and a `bool` indicating if
    /// overflow occurred.
    pub const fn overflowing_div(self, rhs: Self) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_div(rhs.0))
    }

    /// Calculates `self.div_euclid(rhs)`, returning a tuple of the result and a `bool`
    /// indicating if overflow occurred.
    pub const fn overflowing_div_euclid(self, rhs: Self) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_div_euclid(rhs.0))
    }

    /// Calculates `self % rhs`, returning a tuple of the result and a `bool` indicating if
    /// overflow occurred.
    pub const fn overflowing_rem(self, rhs: Self) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_rem(rhs.0))
    }

    /// Calculates `self.rem_euclid(rhs)`, returning a tuple of the result and a `bool`
    /// indicating if overflow occurred.
    pub const fn overflowing_rem_euclid(self, rhs: Self) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_rem_euclid(rhs.0))
    }

    /// Calculates `-self`, returning a tuple of the result and a `bool` indicating if overflow
    /// occurred.
    pub const fn overflowing_neg(self) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_neg())
    }

    /// Calculates `self << rhs`, returning a tuple of the result and a `bool` indicating if
    /// `rhs` was greater than or equal to [`Self::BITS`].
    pub const fn overflowing_shl(self, rhs: u32) -> (Self, bool) {
        (self.wrapping_shl(rhs), rhs >= Self::BITS)
    }

    /// Calculates `self >> rhs`, returning a tuple of the result and a `bool` indicating if
    /// `rhs` was greater than or equal to [`Self::BITS`].
    pub const fn overflowing_shr(self, rhs: u32) -> (Self, bool) {
        (self.wrapping_shr(rhs), rhs >= Self::BITS)
    }

    /// Calculates `self.pow(exp)`, returning a tuple of the result and a `bool` indicating if
    /// overflow occurred.
    pub const fn overflowing_pow(self, exp: u32) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_pow(exp))
    }

    /// Calculates `self.abs()`, returning a tuple of the result and a `bool` indicating if
    /// overflow occurred.
    pub const fn overflowing_abs(self) -> (Self, bool) {
        if self.is_negative() {
            self.overflowing_neg()
        } else {
            (self, false)
        }
    }

    /// Raises `self` to the power of `exp`, using exponentiation by squaring.
    pub const fn pow(self, exp: u32) -> Self {
        let val = self.0.pow(exp);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to exponentiate with overflow"
        );
        Self::new_wrapping(val)
    }

    /// Calculates the quotient of Euclidian division `self` by `rhs`.
    ///
    /// This computes the integer `q` such that `self = q * rhs + r`, with `r =
    /// self.rem_euclid(rhs)` and `0 <= r < rhs.abs()`.
    ///
    /// In other words, the result is `self / rhs`, rounded to the integer `q` such that `self
    /// >= q * rhs`. If `self > 0`, this is equal to round towards zero. If `self < 0`, this is
    /// equal to rounds towards +/- infinity.
    pub const fn div_euclid(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.div_euclid(rhs.0))
    }

    /// Calculates the non-negative remainder `self (mod rhs)`.
    ///
    /// This is done as if by the Euclidian division algorithm - given `r =
    /// self.rem_euclid(rhs)`, `self = self * self.div_euclid(rhs) + r`, and `0 <= r <
    /// r.abs()`.
    pub const fn rem_euclid(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.div_euclid(rhs.0))
    }

    /// Computes the absolute value of this integer.
    pub const fn abs(self) -> Self {
        debug_assert!(
            self.0 != Self::MIN.0,
            "attempt to calculate absolute value with overflow"
        );
        self.wrapping_abs()
    }

    /// Computes the absolute difference between `self` and `rhs`.
    ///
    /// This is equivalent to `(self - rhs).abs()` without the posibility of intermediate
    /// overflow.
    pub const fn abs_diff(self, rhs: Self) -> Aint<u8, WIDTH> {
        Aint(self.0.abs_diff(rhs.0))
    }

    /// Converts a string slice in a given base (`radix`) to an integer.
    ///
    /// The string is expected to be an optional `+` or `-` sign followed by digits. Leading
    /// and trailing whitespace will result in an error. Digits are a subset of the following
    /// characters depending on the `radix`.
    ///
    /// * `0-9`
    /// * `a-z`
    /// * `A-Z`
    ///
    /// # Panics
    /// This function panics if `radix` is not in the range from 2 to 36.
    pub fn from_str_radix(src: &str, radix: u32) -> Result<Self, core::num::ParseIntError> {
        Self::new(i8::from_str_radix(src, radix)?).ok_or_else(make_parse_int_err)
    }
}

impl<const WIDTH: u32> Aint<i16, WIDTH> {
    const ASSERT: () = if WIDTH > 8 && WIDTH < 16 {
        ()
    } else {
        panic!("Invalid width for Aint with repr type of i16")
    };

    /// The size of this integer type in bits.
    pub const BITS: u32 = {
        let _val = Self::ASSERT;
        WIDTH
    };

    /// The smallest value that can be represented by this integer type.
    pub const MIN: Self = {
        let _val = Self::ASSERT;
        Self(!0 << (Self::BITS - 1))
    };

    /// The largest value that can be represented by this integer type.
    pub const MAX: Self = {
        let _val = Self::ASSERT;
        Self(!Self::MIN.0)
    };

    pub(crate) const MASK: i16 = {
        let _val = Self::ASSERT;
        (Self::MAX.0 << 1) | 1
    };

    const SIGN_BIT: i16 = {
        let _val = Self::ASSERT;
        1 << (Self::BITS - 1)
    };

    /// Creates a new integer value from the underlying representation type, unchecked.
    ///
    /// # Safety
    /// The representation type value must be within the range of valid values of this type.
    /// That is, the value must be greater or equal to [`MIN`](Self::MIN) and less or equal to
    /// [`MAX`](Self::MAX).
    pub const unsafe fn new_unchecked(repr: i16) -> Self {
        let _val = Self::ASSERT;
        Self(repr)
    }

    /// Creates a new integer value from the underlying representation type.
    ///
    /// The returned value is `None` if the representation value is outside the range of valid
    /// values of this integer type.
    pub const fn new(repr: i16) -> Option<Self> {
        if repr <= Self::MAX.0 && repr >= Self::MIN.0 {
            Some(Self(repr))
        } else {
            None
        }
    }

    /// Creates a new integer value from the underlying representation type.
    ///
    /// The returned value is wrapped as though the representation value were calculated using
    /// wrapping operations, such as [`wrapping_add`](Self::wrapping_add).
    pub const fn new_wrapping(repr: i16) -> Self {
        if (repr & Self::SIGN_BIT) == 0 {
            Self(repr & Self::MAX.0)
        } else {
            Self(repr | !Self::MAX.0)
        }
    }

    /// Creates a new integer value from the underlying representation type.
    ///
    /// The returned value is saturated to the bounds of this integer's value range. If the
    /// representation value is greater than [`MAX`](Self::MAX), the returned value will be
    /// [`MAX`](Self::MAX). If the representation value is less than [`MIN`](Self::MIN), the
    /// returned value will be [`MIN`](Self::MIN).
    pub const fn new_saturating(repr: i16) -> Self {
        if repr >= Self::MAX.0 {
            Self::MAX
        } else if repr <= Self::MIN.0 {
            Self::MIN
        } else {
            Self(repr)
        }
    }

    const fn new_overflowing_impl((repr, over): (i16, bool)) -> (Self, bool) {
        if repr > Self::MAX.0 {
            (Self(repr & Self::MAX.0), true)
        } else if repr < Self::MIN.0 {
            (Self(repr | !Self::MAX.0), true)
        } else {
            (Self(repr), over)
        }
    }

    /// Creates a new integer from the underlying representation type.
    ///
    /// The returned tuple contains the new integer and a `bool` indicating if the representation
    /// value overflowed the new integer. In the case of overflow, the new integer has a value
    /// as if it were produced from [`new_wrapping`](Self::new_wrapping).
    pub const fn new_overflowing(repr: i16) -> (Self, bool) {
        Self::new_overflowing_impl((repr, false))
    }

    /// Returns the value of this integer as the underlying representation type.
    pub const fn repr(self) -> i16 {
        self.0
    }

    /// Returns `true` if this integer is less than zero.
    pub const fn is_negative(self) -> bool {
        (self.0 & Self::SIGN_BIT) != 0
    }

    /// Returns `true` if this integer is greater than zero.
    pub const fn is_positive(self) -> bool {
        self.0 != 0 && (self.0 & Self::SIGN_BIT) == 0
    }

    /// Returns the sign of the integer.
    ///
    /// * If `self < 0`, returns `-1`
    /// * If `self > 0`, returns `1`
    /// * If `self == 0`, returns `0`
    pub const fn signum(self) -> Self {
        if self.0 == 0 {
            Self(0)
        } else if self.0 > 0 {
            Self(1)
        } else {
            Self(-1)
        }
    }

    /// Returns the number of ones in the binary representation of this integer.
    pub const fn count_ones(self) -> u32 {
        (self.0 & Self::MASK).count_ones()
    }

    /// Returns the number of zeros in the binary representatino of this integer.
    pub const fn count_zeros(self) -> u32 {
        (self.0 | !Self::MASK).count_zeros()
    }

    /// Returns the number of leading zeros in the binary represnetation of this integer.
    pub const fn leading_zeros(self) -> u32 {
        if self.is_negative() {
            0
        } else {
            self.0.leading_zeros() - (i16::BITS - Self::BITS)
        }
    }

    /// Returns the number of trailing zeros in the binary representatino of this integer.
    pub const fn trailing_zeros(self) -> u32 {
        (self.0 | !Self::MASK).trailing_zeros()
    }

    /// Returns the number of leading ones in the binary representation of this integer.
    pub const fn leading_ones(self) -> u32 {
        (self.0 | !Self::MASK).leading_ones() - (i16::BITS - Self::BITS)
    }

    /// Returns the number of trailing ones in the binary representation of this integer.
    pub const fn trailing_ones(self) -> u32 {
        (self.0 & Self::MASK).trailing_ones()
    }

    /// Performs a left bit shift by `n`, wrapping the truncated bits back to the end.
    ///
    /// Note that unlike the `<<` operator, all values of `n` are valid. A value of `n`, that
    /// is greater than or equal to [`BITS`](Self::BITS), is the equivalent to using the value
    /// `n % Self::BITS`.
    pub const fn rotate_left(self, n: u32) -> Self {
        let n = n % Self::BITS;
        Self::new_wrapping((self.0 << n) | ((self.0 & Self::MASK) >> (Self::BITS - n)))
    }

    /// Performs a right bit shift by `n`, wrapping the truncated bits back to the end.
    ///
    /// Note that unlike the `>>` operator, all values of `n` are valid. A value of `n`, that
    /// is greater than or equal to [`BITS`](Self::BITS), is the equivalent to using the value
    /// `n % Self::BITS`.
    pub const fn rotate_right(self, n: u32) -> Self {
        let n = n % Self::BITS;
        Self::new_wrapping(((self.0 & Self::MASK) >> n) | (self.0 << (Self::BITS - n)))
    }

    /// Reverses the order of the bits in the binary representation of this integer.
    pub const fn reverse_bits(self) -> Self {
        Self(self.0.reverse_bits() >> (i16::BITS - Self::BITS))
    }

    /// Calculates the negative of this integer.
    ///
    /// This method works as a `const` capable alternative to `-self`.
    pub const fn neg(self) -> Self {
        debug_assert!(self.0 != Self::MIN.0, "attempt to negate with overflow");
        self.wrapping_neg()
    }

    /// Calculates the sum of this integer with another integer.
    ///
    /// This method works as a `const` capable alternative to `self + rhs`.
    pub const fn add(self, rhs: Self) -> Self {
        let val = self.0 + rhs.0;
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to add with overflow"
        );
        Self::new_wrapping(val)
    }

    /// Calculates the difference of this integer with another integer.
    ///
    /// This method works as a `const` capable alternative to `self - rhs`.
    pub const fn sub(self, rhs: Self) -> Self {
        let val = self.0 - rhs.0;
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to subtract with overflow"
        );
        Self::new_wrapping(val)
    }

    /// Calculates the product of this integer with another integer.
    ///
    /// This method works as a `const` capable alternative to `self * rhs`.
    pub const fn mul(self, rhs: Self) -> Self {
        let val = self.0 * rhs.0;
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to multiply with overflow"
        );
        Self::new_wrapping(val)
    }

    /// Calculates the quotient of this integer over another integer.
    ///
    /// This method works as a `const` capable alternative to `self / rhs`.
    pub const fn div(self, rhs: Self) -> Self {
        let val = self.0 / rhs.0;
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to divide with overflow"
        );
        Self(val)
    }

    /// Calculates the remainder of this integer over another integer.
    ///
    /// This method works as a `const` capable alternative to `self % rhs`.
    pub const fn rem(self, rhs: Self) -> Self {
        let val = self.0 % rhs.0;
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to calculate modulus with overflow"
        );
        Self(val)
    }

    /// Calculates the bitwise and of this integer with another integer.
    ///
    /// This method works as a `const` capable alternative to `self & rhs`.
    pub const fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }

    /// Calculates the bitwise or of this integer with another integer.
    ///
    /// This method works as a `const` capable alternative to `self | rhs`.
    pub const fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }

    /// Calculates the bitwise exclusive-or of this integer with another integer.
    ///
    /// This method works as a `const` capable alternative to `self ^ rhs`.
    pub const fn bitxor(self, rhs: Self) -> Self {
        Self(self.0 ^ rhs.0)
    }

    /// Calculates the bitwise negation of this integer.
    ///
    /// This method works as a `const` capable alternative to `!self`.
    pub const fn not(self) -> Self {
        Self(!self.0)
    }

    /// Calculates the left bitwise shift of this integer by `rhs`.
    ///
    /// This method works as a `const` capable alternative to `self << rhs`.
    pub const fn shl(self, rhs: u32) -> Self {
        debug_assert!(rhs >= Self::BITS, "attempt to shift left with overflow");
        Self::new_wrapping(self.0 << rhs)
    }

    /// Calculates the right bitwise shift of this integer by `rhs`.
    ///
    /// This method works as a `const` capable alternative to `self >> rhs`.
    pub const fn shr(self, rhs: u32) -> Self {
        debug_assert!(rhs >= Self::BITS, "attempt to shift right with overflow");
        Self::new_wrapping(self.0 >> rhs)
    }

    /// Calculates the logarithm of this integer with respect to an arbitrary base, rounded
    /// down.
    ///
    /// The [`ilog2`](Self::ilog2) and [`ilog10`](Self::ilog10) methods should be preferred
    /// when applicable, as they are generally more optimized than this method since the base
    /// of the logarithm is not arbitrary.
    ///
    /// # Panics
    /// This method will panic if `self` is less than or equal to zero, or if `base` is less
    /// than 2.
    pub const fn ilog(self, base: Self) -> u32 {
        self.0.ilog(base.0)
    }

    /// Calculates the base 2 logarithm of this integer, rounded down.
    ///
    /// # Panics
    /// This method will panic if `self` is less than or equal to zero.
    pub const fn ilog2(self) -> u32 {
        self.0.ilog2()
    }

    /// Calculates the base 10 logarithm of this integer, rounded down.
    ///
    /// # Panics
    /// This method will panic if `self` is less than or equal to zero.
    pub const fn ilog10(self) -> u32 {
        self.0.ilog10()
    }

    /// Returns `true` if `self` and `rhs` are equal.
    ///
    /// This method works as a `const` capable alternative to `self == rhs`.
    pub const fn eq(self, rhs: Self) -> bool {
        return self.0 == rhs.0;
    }

    /// Returns `true` if `self` and `rhs` are not equal.
    ///
    /// This method works as a `const` capable alternative to `self != rhs`.
    pub const fn ne(self, rhs: Self) -> bool {
        return self.0 != rhs.0;
    }

    /// Returns `true` if `self` is less than `rhs`.
    ///
    /// This method works as a `const` capable alternative to `self < rhs`.
    pub const fn lt(self, rhs: Self) -> bool {
        return self.0 < rhs.0;
    }

    /// Returns `true` if `self` is less than or equal to `rhs`.
    ///
    /// This method works as a `const` capable alternative to `self <= rhs`.
    pub const fn le(self, rhs: Self) -> bool {
        return self.0 <= rhs.0;
    }

    /// Returns `true` if `self` is greater than `rhs`.
    ///
    /// This method works as a `const` capable alternative to `self > rhs`.
    pub const fn gt(self, rhs: Self) -> bool {
        return self.0 > rhs.0;
    }

    /// Returns `true` if `self` is greater than or equal to `rhs`.
    ///
    /// This method works as a `const` capable alternative to `self >= rhs`.
    pub const fn ge(self, rhs: Self) -> bool {
        return self.0 >= rhs.0;
    }

    /// Returns the ordering of `self` with respect to `rhs`.
    ///
    /// This method works as a `const` capable alternative to `Ord::cmp(self, rhs)`.
    pub const fn cmp(self, rhs: Self) -> core::cmp::Ordering {
        if self.0 < rhs.0 {
            core::cmp::Ordering::Less
        } else if self.0 > rhs.0 {
            core::cmp::Ordering::Greater
        } else {
            core::cmp::Ordering::Equal
        }
    }

    /// Returns the minimum of `self` and `other`.
    ///
    /// This method works as a `const` capable alternative to `Ord::min(self, rhs)`.
    pub const fn min(self, other: Self) -> Self {
        if self.0 < other.0 {
            self
        } else {
            other
        }
    }

    /// Returns the maximum of `self` and `other`.
    ///
    /// This method works as a `const` capable alternative to `Ord::max(self, rhs)`.
    pub const fn max(self, other: Self) -> Self {
        if self.0 > other.0 {
            self
        } else {
            other
        }
    }

    /// Returns the value of this integer, clamped between `min` and `max`.
    ///
    /// This method works as a `const` capable alternative to `Ord::clamp(self, min, max)`.
    pub const fn clamp(self, min: Self, max: Self) -> Self {
        debug_assert!(
            min.0 <= max.0,
            "Attempt to clamp with minimum value greater than maximum value"
        );
        Self::min(Self::max(self, min), max)
    }

    /// Checked integer addition. Returns `None` if overflow occurred.
    pub const fn checked_add(self, rhs: Self) -> Option<Self> {
        match self.0.checked_add(rhs.0) {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked integer subtraction. Returns `None` if overflow occurred.
    pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
        match self.0.checked_sub(rhs.0) {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked integer multiplication. Returns `None` if overflow occurred.
    pub const fn checked_mul(self, rhs: Self) -> Option<Self> {
        match self.0.checked_mul(rhs.0) {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked integer division. Returns `None` if `rhs` is zero, or the division resulted in
    /// overflow.
    pub const fn checked_div(self, rhs: Self) -> Option<Self> {
        match self.0.checked_div(rhs.0) {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked euclidian division. Returns `None` if `rhs` is zero or `self.div_euclid(rhs)`
    /// would have resulted in overflow.
    pub const fn checked_div_euclid(self, rhs: Self) -> Option<Self> {
        match self.0.checked_div_euclid(rhs.0) {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked integer remainder. Returns `None` if `rhs` is zero or the remainder would have
    /// resulted in overflow.
    pub const fn checked_rem(self, rhs: Self) -> Option<Self> {
        match self.0.checked_rem(rhs.0) {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked euclidian remainder. Returns `None` if `rhs` is zero or `self.rem_euclid(rhs)`
    /// would have resulted in overflow.
    pub const fn checked_rem_euclid(self, rhs: Self) -> Option<Self> {
        match self.0.checked_rem_euclid(rhs.0) {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked integer negation. Returns `None` if the negation resulted in overflow.
    pub const fn checked_neg(self) -> Option<Self> {
        match self.0.checked_neg() {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked left bit shift. Returns `None` if `rhs` is greater than or equal to
    /// [`Self::BITS`].
    pub const fn checked_shl(self, rhs: u32) -> Option<Self> {
        if rhs >= Self::BITS {
            None
        } else {
            Some(self.shl(rhs))
        }
    }

    /// Checked right bit shift. Returns `None` if `rhs` is greater than or equal to
    /// [`Self::BITS`].
    pub const fn checked_shr(self, rhs: u32) -> Option<Self> {
        if rhs >= Self::BITS {
            None
        } else {
            Some(self.shr(rhs))
        }
    }

    /// Checked exponentiation. Returns `None` if the exponentiation resulted in overflow.
    pub const fn checked_pow(self, exp: u32) -> Option<Self> {
        match self.0.checked_pow(exp) {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked logarithm, rounded down. Returns `None` if `self` is less than or equal zero,
    /// or if `base` is less than 2.
    pub const fn checked_ilog(self, base: Self) -> Option<u32> {
        self.0.checked_ilog(base.0)
    }

    /// Checked base 2 logarithm, rounded down. Returns `None` if self is less than or equal to
    /// zero.
    pub const fn checked_ilog2(self) -> Option<u32> {
        self.0.checked_ilog2()
    }

    /// Checked base 10 logarithm, rounded down. Returns `None` if self is less than or equal
    /// to zero.
    pub const fn checked_ilog10(self) -> Option<u32> {
        self.0.checked_ilog10()
    }

    /// Satruating integer addition. The result of the addition is clamped between
    /// [`MIN`](Self::MIN) and [`MAX`](Self::MAX).
    pub const fn saturating_add(self, rhs: Self) -> Self {
        Self::new_saturating(self.0.saturating_add(rhs.0))
    }

    /// Saturating integer subtraction. The result of the subtraction is clamped between
    /// [`MIN`](Self::MIN) and [`MAX`](Self::MAX).
    pub const fn saturating_sub(self, rhs: Self) -> Self {
        Self::new_saturating(self.0.saturating_sub(rhs.0))
    }

    /// Saturating integer multiplication. The result of the multiplication is clamped between
    /// [`MIN`](Self::MIN) and [`MAX`](Self::MAX).
    pub const fn saturating_mul(self, rhs: Self) -> Self {
        Self::new_saturating(self.0.saturating_mul(rhs.0))
    }

    /// Saturating exponentiation. The result of the exponentiation is clammed between
    /// [`MIN`](Self::MIN) and [`MAX`](Self::MAX).
    pub const fn saturating_pow(self, exp: u32) -> Self {
        Self::new_saturating(self.0.saturating_pow(exp))
    }

    /// Wrapping (modular) addition. An overflowing result is wrapped around the bounds of this
    /// integer type.
    pub const fn wrapping_add(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.wrapping_add(rhs.0))
    }

    /// Wrapping (modular) subtraction. An overflowing result is wrapped around the bounds of
    /// this integer type.
    pub const fn wrapping_sub(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.wrapping_sub(rhs.0))
    }

    /// Wrapping (modular) multiplication. An overflowing result is wrapped around the bounds of
    /// this integer type.
    pub const fn wrapping_mul(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.wrapping_mul(rhs.0))
    }

    /// Wrapping (modular) division. An overflowing result is wrapped around the bounds of this
    /// integer type.
    pub const fn wrapping_div(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.wrapping_div(rhs.0))
    }

    /// Wrapping (modular) euclidian division. An overflowing result is wrapped around the
    /// bounds of this integer type.
    pub const fn wrapping_div_euclid(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.wrapping_div_euclid(rhs.0))
    }

    /// Wrapping (modular) remainder. An overflowing result is wrapped around the bounds of
    /// this integer type.
    pub const fn wrapping_rem(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.wrapping_rem(rhs.0))
    }

    /// Wrapping (modular) euclidian remainder. An overflowing result is wrapped around the
    /// bounds of this integer type.
    pub const fn wrapping_rem_euclid(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.wrapping_rem_euclid(rhs.0))
    }

    /// Wrapping (modular) negation. An overflowing result is wrapped around the bounds of this
    /// integer type.
    pub const fn wrapping_neg(self) -> Self {
        Self::new_wrapping(!self.0 + 1)
    }

    /// Wrapping (modular) left bit shift. The value is left shifted `rhs % Self::BITS` bits.
    pub const fn wrapping_shl(self, rhs: u32) -> Self {
        Self::new_wrapping(self.0 << (rhs % Self::BITS))
    }

    /// Wrapping (modular) right bit shift. The value is right shifted `rhs % Self::BITS` bits.
    pub const fn wrapping_shr(self, rhs: u32) -> Self {
        Self::new_wrapping((self.0 & Self::MASK) >> (rhs % Self::BITS))
    }

    /// Wrapping (modular) exponentiation. An overflowing result is wrapped around the bounds
    /// of this integer type.
    pub const fn wrapping_pow(self, exp: u32) -> Self {
        Self::new_wrapping(self.0.wrapping_pow(exp))
    }

    /// Wrapping (modular) aboslute value. An overflowing result is wrapped around the bounds
    /// of this integer type.
    pub const fn wrapping_abs(self) -> Self {
        if self.is_negative() {
            self.wrapping_neg()
        } else {
            self
        }
    }

    /// Calculates `self + rhs`, returning a tuple of the result and a `bool` indicating if
    /// overflow occurred.
    pub const fn overflowing_add(self, rhs: Self) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_add(rhs.0))
    }

    /// Calculates `self - rhs`, returning a tuple of the result and a `bool` indicating if
    /// overflow occurred.
    pub const fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_sub(rhs.0))
    }

    /// Calculates `self * rhs`, returning a tuple of the result and a `bool` indicating if
    /// overflow occurred.
    pub const fn overflowing_mul(self, rhs: Self) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_mul(rhs.0))
    }

    /// Calculates `self / rhs`, returning a tuple of the result and a `bool` indicating if
    /// overflow occurred.
    pub const fn overflowing_div(self, rhs: Self) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_div(rhs.0))
    }

    /// Calculates `self.div_euclid(rhs)`, returning a tuple of the result and a `bool`
    /// indicating if overflow occurred.
    pub const fn overflowing_div_euclid(self, rhs: Self) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_div_euclid(rhs.0))
    }

    /// Calculates `self % rhs`, returning a tuple of the result and a `bool` indicating if
    /// overflow occurred.
    pub const fn overflowing_rem(self, rhs: Self) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_rem(rhs.0))
    }

    /// Calculates `self.rem_euclid(rhs)`, returning a tuple of the result and a `bool`
    /// indicating if overflow occurred.
    pub const fn overflowing_rem_euclid(self, rhs: Self) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_rem_euclid(rhs.0))
    }

    /// Calculates `-self`, returning a tuple of the result and a `bool` indicating if overflow
    /// occurred.
    pub const fn overflowing_neg(self) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_neg())
    }

    /// Calculates `self << rhs`, returning a tuple of the result and a `bool` indicating if
    /// `rhs` was greater than or equal to [`Self::BITS`].
    pub const fn overflowing_shl(self, rhs: u32) -> (Self, bool) {
        (self.wrapping_shl(rhs), rhs >= Self::BITS)
    }

    /// Calculates `self >> rhs`, returning a tuple of the result and a `bool` indicating if
    /// `rhs` was greater than or equal to [`Self::BITS`].
    pub const fn overflowing_shr(self, rhs: u32) -> (Self, bool) {
        (self.wrapping_shr(rhs), rhs >= Self::BITS)
    }

    /// Calculates `self.pow(exp)`, returning a tuple of the result and a `bool` indicating if
    /// overflow occurred.
    pub const fn overflowing_pow(self, exp: u32) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_pow(exp))
    }

    /// Calculates `self.abs()`, returning a tuple of the result and a `bool` indicating if
    /// overflow occurred.
    pub const fn overflowing_abs(self) -> (Self, bool) {
        if self.is_negative() {
            self.overflowing_neg()
        } else {
            (self, false)
        }
    }

    /// Raises `self` to the power of `exp`, using exponentiation by squaring.
    pub const fn pow(self, exp: u32) -> Self {
        let val = self.0.pow(exp);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to exponentiate with overflow"
        );
        Self::new_wrapping(val)
    }

    /// Calculates the quotient of Euclidian division `self` by `rhs`.
    ///
    /// This computes the integer `q` such that `self = q * rhs + r`, with `r =
    /// self.rem_euclid(rhs)` and `0 <= r < rhs.abs()`.
    ///
    /// In other words, the result is `self / rhs`, rounded to the integer `q` such that `self
    /// >= q * rhs`. If `self > 0`, this is equal to round towards zero. If `self < 0`, this is
    /// equal to rounds towards +/- infinity.
    pub const fn div_euclid(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.div_euclid(rhs.0))
    }

    /// Calculates the non-negative remainder `self (mod rhs)`.
    ///
    /// This is done as if by the Euclidian division algorithm - given `r =
    /// self.rem_euclid(rhs)`, `self = self * self.div_euclid(rhs) + r`, and `0 <= r <
    /// r.abs()`.
    pub const fn rem_euclid(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.div_euclid(rhs.0))
    }

    /// Computes the absolute value of this integer.
    pub const fn abs(self) -> Self {
        debug_assert!(
            self.0 != Self::MIN.0,
            "attempt to calculate absolute value with overflow"
        );
        self.wrapping_abs()
    }

    /// Computes the absolute difference between `self` and `rhs`.
    ///
    /// This is equivalent to `(self - rhs).abs()` without the posibility of intermediate
    /// overflow.
    pub const fn abs_diff(self, rhs: Self) -> Aint<u16, WIDTH> {
        Aint(self.0.abs_diff(rhs.0))
    }

    /// Converts a string slice in a given base (`radix`) to an integer.
    ///
    /// The string is expected to be an optional `+` or `-` sign followed by digits. Leading
    /// and trailing whitespace will result in an error. Digits are a subset of the following
    /// characters depending on the `radix`.
    ///
    /// * `0-9`
    /// * `a-z`
    /// * `A-Z`
    ///
    /// # Panics
    /// This function panics if `radix` is not in the range from 2 to 36.
    pub fn from_str_radix(src: &str, radix: u32) -> Result<Self, core::num::ParseIntError> {
        Self::new(i16::from_str_radix(src, radix)?).ok_or_else(make_parse_int_err)
    }
}

impl<const WIDTH: u32> Aint<i32, WIDTH> {
    const ASSERT: () = if WIDTH > 16 && WIDTH < 32 {
        ()
    } else {
        panic!("Invalid width for Aint with repr type of i32")
    };

    /// The size of this integer type in bits.
    pub const BITS: u32 = {
        let _val = Self::ASSERT;
        WIDTH
    };

    /// The smallest value that can be represented by this integer type.
    pub const MIN: Self = {
        let _val = Self::ASSERT;
        Self(!0 << (Self::BITS - 1))
    };

    /// The largest value that can be represented by this integer type.
    pub const MAX: Self = {
        let _val = Self::ASSERT;
        Self(!Self::MIN.0)
    };

    pub(crate) const MASK: i32 = {
        let _val = Self::ASSERT;
        (Self::MAX.0 << 1) | 1
    };

    const SIGN_BIT: i32 = {
        let _val = Self::ASSERT;
        1 << (Self::BITS - 1)
    };

    /// Creates a new integer value from the underlying representation type, unchecked.
    ///
    /// # Safety
    /// The representation type value must be within the range of valid values of this type.
    /// That is, the value must be greater or equal to [`MIN`](Self::MIN) and less or equal to
    /// [`MAX`](Self::MAX).
    pub const unsafe fn new_unchecked(repr: i32) -> Self {
        let _val = Self::ASSERT;
        Self(repr)
    }

    /// Creates a new integer value from the underlying representation type.
    ///
    /// The returned value is `None` if the representation value is outside the range of valid
    /// values of this integer type.
    pub const fn new(repr: i32) -> Option<Self> {
        if repr <= Self::MAX.0 && repr >= Self::MIN.0 {
            Some(Self(repr))
        } else {
            None
        }
    }

    /// Creates a new integer value from the underlying representation type.
    ///
    /// The returned value is wrapped as though the representation value were calculated using
    /// wrapping operations, such as [`wrapping_add`](Self::wrapping_add).
    pub const fn new_wrapping(repr: i32) -> Self {
        if (repr & Self::SIGN_BIT) == 0 {
            Self(repr & Self::MAX.0)
        } else {
            Self(repr | !Self::MAX.0)
        }
    }

    /// Creates a new integer value from the underlying representation type.
    ///
    /// The returned value is saturated to the bounds of this integer's value range. If the
    /// representation value is greater than [`MAX`](Self::MAX), the returned value will be
    /// [`MAX`](Self::MAX). If the representation value is less than [`MIN`](Self::MIN), the
    /// returned value will be [`MIN`](Self::MIN).
    pub const fn new_saturating(repr: i32) -> Self {
        if repr >= Self::MAX.0 {
            Self::MAX
        } else if repr <= Self::MIN.0 {
            Self::MIN
        } else {
            Self(repr)
        }
    }

    const fn new_overflowing_impl((repr, over): (i32, bool)) -> (Self, bool) {
        if repr > Self::MAX.0 {
            (Self(repr & Self::MAX.0), true)
        } else if repr < Self::MIN.0 {
            (Self(repr | !Self::MAX.0), true)
        } else {
            (Self(repr), over)
        }
    }

    /// Creates a new integer from the underlying representation type.
    ///
    /// The returned tuple contains the new integer and a `bool` indicating if the representation
    /// value overflowed the new integer. In the case of overflow, the new integer has a value
    /// as if it were produced from [`new_wrapping`](Self::new_wrapping).
    pub const fn new_overflowing(repr: i32) -> (Self, bool) {
        Self::new_overflowing_impl((repr, false))
    }

    /// Returns the value of this integer as the underlying representation type.
    pub const fn repr(self) -> i32 {
        self.0
    }

    /// Returns `true` if this integer is less than zero.
    pub const fn is_negative(self) -> bool {
        (self.0 & Self::SIGN_BIT) != 0
    }

    /// Returns `true` if this integer is greater than zero.
    pub const fn is_positive(self) -> bool {
        self.0 != 0 && (self.0 & Self::SIGN_BIT) == 0
    }

    /// Returns the sign of the integer.
    ///
    /// * If `self < 0`, returns `-1`
    /// * If `self > 0`, returns `1`
    /// * If `self == 0`, returns `0`
    pub const fn signum(self) -> Self {
        if self.0 == 0 {
            Self(0)
        } else if self.0 > 0 {
            Self(1)
        } else {
            Self(-1)
        }
    }

    /// Returns the number of ones in the binary representation of this integer.
    pub const fn count_ones(self) -> u32 {
        (self.0 & Self::MASK).count_ones()
    }

    /// Returns the number of zeros in the binary representatino of this integer.
    pub const fn count_zeros(self) -> u32 {
        (self.0 | !Self::MASK).count_zeros()
    }

    /// Returns the number of leading zeros in the binary represnetation of this integer.
    pub const fn leading_zeros(self) -> u32 {
        if self.is_negative() {
            0
        } else {
            self.0.leading_zeros() - (i32::BITS - Self::BITS)
        }
    }

    /// Returns the number of trailing zeros in the binary representatino of this integer.
    pub const fn trailing_zeros(self) -> u32 {
        (self.0 | !Self::MASK).trailing_zeros()
    }

    /// Returns the number of leading ones in the binary representation of this integer.
    pub const fn leading_ones(self) -> u32 {
        (self.0 | !Self::MASK).leading_ones() - (i32::BITS - Self::BITS)
    }

    /// Returns the number of trailing ones in the binary representation of this integer.
    pub const fn trailing_ones(self) -> u32 {
        (self.0 & Self::MASK).trailing_ones()
    }

    /// Performs a left bit shift by `n`, wrapping the truncated bits back to the end.
    ///
    /// Note that unlike the `<<` operator, all values of `n` are valid. A value of `n`, that
    /// is greater than or equal to [`BITS`](Self::BITS), is the equivalent to using the value
    /// `n % Self::BITS`.
    pub const fn rotate_left(self, n: u32) -> Self {
        let n = n % Self::BITS;
        Self::new_wrapping((self.0 << n) | ((self.0 & Self::MASK) >> (Self::BITS - n)))
    }

    /// Performs a right bit shift by `n`, wrapping the truncated bits back to the end.
    ///
    /// Note that unlike the `>>` operator, all values of `n` are valid. A value of `n`, that
    /// is greater than or equal to [`BITS`](Self::BITS), is the equivalent to using the value
    /// `n % Self::BITS`.
    pub const fn rotate_right(self, n: u32) -> Self {
        let n = n % Self::BITS;
        Self::new_wrapping(((self.0 & Self::MASK) >> n) | (self.0 << (Self::BITS - n)))
    }

    /// Reverses the order of the bits in the binary representation of this integer.
    pub const fn reverse_bits(self) -> Self {
        Self(self.0.reverse_bits() >> (i32::BITS - Self::BITS))
    }

    /// Calculates the negative of this integer.
    ///
    /// This method works as a `const` capable alternative to `-self`.
    pub const fn neg(self) -> Self {
        debug_assert!(self.0 != Self::MIN.0, "attempt to negate with overflow");
        self.wrapping_neg()
    }

    /// Calculates the sum of this integer with another integer.
    ///
    /// This method works as a `const` capable alternative to `self + rhs`.
    pub const fn add(self, rhs: Self) -> Self {
        let val = self.0 + rhs.0;
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to add with overflow"
        );
        Self::new_wrapping(val)
    }

    /// Calculates the difference of this integer with another integer.
    ///
    /// This method works as a `const` capable alternative to `self - rhs`.
    pub const fn sub(self, rhs: Self) -> Self {
        let val = self.0 - rhs.0;
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to subtract with overflow"
        );
        Self::new_wrapping(val)
    }

    /// Calculates the product of this integer with another integer.
    ///
    /// This method works as a `const` capable alternative to `self * rhs`.
    pub const fn mul(self, rhs: Self) -> Self {
        let val = self.0 * rhs.0;
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to multiply with overflow"
        );
        Self::new_wrapping(val)
    }

    /// Calculates the quotient of this integer over another integer.
    ///
    /// This method works as a `const` capable alternative to `self / rhs`.
    pub const fn div(self, rhs: Self) -> Self {
        let val = self.0 / rhs.0;
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to divide with overflow"
        );
        Self(val)
    }

    /// Calculates the remainder of this integer over another integer.
    ///
    /// This method works as a `const` capable alternative to `self % rhs`.
    pub const fn rem(self, rhs: Self) -> Self {
        let val = self.0 % rhs.0;
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to calculate modulus with overflow"
        );
        Self(val)
    }

    /// Calculates the bitwise and of this integer with another integer.
    ///
    /// This method works as a `const` capable alternative to `self & rhs`.
    pub const fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }

    /// Calculates the bitwise or of this integer with another integer.
    ///
    /// This method works as a `const` capable alternative to `self | rhs`.
    pub const fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }

    /// Calculates the bitwise exclusive-or of this integer with another integer.
    ///
    /// This method works as a `const` capable alternative to `self ^ rhs`.
    pub const fn bitxor(self, rhs: Self) -> Self {
        Self(self.0 ^ rhs.0)
    }

    /// Calculates the bitwise negation of this integer.
    ///
    /// This method works as a `const` capable alternative to `!self`.
    pub const fn not(self) -> Self {
        Self(!self.0)
    }

    /// Calculates the left bitwise shift of this integer by `rhs`.
    ///
    /// This method works as a `const` capable alternative to `self << rhs`.
    pub const fn shl(self, rhs: u32) -> Self {
        debug_assert!(rhs >= Self::BITS, "attempt to shift left with overflow");
        Self::new_wrapping(self.0 << rhs)
    }

    /// Calculates the right bitwise shift of this integer by `rhs`.
    ///
    /// This method works as a `const` capable alternative to `self >> rhs`.
    pub const fn shr(self, rhs: u32) -> Self {
        debug_assert!(rhs >= Self::BITS, "attempt to shift right with overflow");
        Self::new_wrapping(self.0 >> rhs)
    }

    /// Calculates the logarithm of this integer with respect to an arbitrary base, rounded
    /// down.
    ///
    /// The [`ilog2`](Self::ilog2) and [`ilog10`](Self::ilog10) methods should be preferred
    /// when applicable, as they are generally more optimized than this method since the base
    /// of the logarithm is not arbitrary.
    ///
    /// # Panics
    /// This method will panic if `self` is less than or equal to zero, or if `base` is less
    /// than 2.
    pub const fn ilog(self, base: Self) -> u32 {
        self.0.ilog(base.0)
    }

    /// Calculates the base 2 logarithm of this integer, rounded down.
    ///
    /// # Panics
    /// This method will panic if `self` is less than or equal to zero.
    pub const fn ilog2(self) -> u32 {
        self.0.ilog2()
    }

    /// Calculates the base 10 logarithm of this integer, rounded down.
    ///
    /// # Panics
    /// This method will panic if `self` is less than or equal to zero.
    pub const fn ilog10(self) -> u32 {
        self.0.ilog10()
    }

    /// Returns `true` if `self` and `rhs` are equal.
    ///
    /// This method works as a `const` capable alternative to `self == rhs`.
    pub const fn eq(self, rhs: Self) -> bool {
        return self.0 == rhs.0;
    }

    /// Returns `true` if `self` and `rhs` are not equal.
    ///
    /// This method works as a `const` capable alternative to `self != rhs`.
    pub const fn ne(self, rhs: Self) -> bool {
        return self.0 != rhs.0;
    }

    /// Returns `true` if `self` is less than `rhs`.
    ///
    /// This method works as a `const` capable alternative to `self < rhs`.
    pub const fn lt(self, rhs: Self) -> bool {
        return self.0 < rhs.0;
    }

    /// Returns `true` if `self` is less than or equal to `rhs`.
    ///
    /// This method works as a `const` capable alternative to `self <= rhs`.
    pub const fn le(self, rhs: Self) -> bool {
        return self.0 <= rhs.0;
    }

    /// Returns `true` if `self` is greater than `rhs`.
    ///
    /// This method works as a `const` capable alternative to `self > rhs`.
    pub const fn gt(self, rhs: Self) -> bool {
        return self.0 > rhs.0;
    }

    /// Returns `true` if `self` is greater than or equal to `rhs`.
    ///
    /// This method works as a `const` capable alternative to `self >= rhs`.
    pub const fn ge(self, rhs: Self) -> bool {
        return self.0 >= rhs.0;
    }

    /// Returns the ordering of `self` with respect to `rhs`.
    ///
    /// This method works as a `const` capable alternative to `Ord::cmp(self, rhs)`.
    pub const fn cmp(self, rhs: Self) -> core::cmp::Ordering {
        if self.0 < rhs.0 {
            core::cmp::Ordering::Less
        } else if self.0 > rhs.0 {
            core::cmp::Ordering::Greater
        } else {
            core::cmp::Ordering::Equal
        }
    }

    /// Returns the minimum of `self` and `other`.
    ///
    /// This method works as a `const` capable alternative to `Ord::min(self, rhs)`.
    pub const fn min(self, other: Self) -> Self {
        if self.0 < other.0 {
            self
        } else {
            other
        }
    }

    /// Returns the maximum of `self` and `other`.
    ///
    /// This method works as a `const` capable alternative to `Ord::max(self, rhs)`.
    pub const fn max(self, other: Self) -> Self {
        if self.0 > other.0 {
            self
        } else {
            other
        }
    }

    /// Returns the value of this integer, clamped between `min` and `max`.
    ///
    /// This method works as a `const` capable alternative to `Ord::clamp(self, min, max)`.
    pub const fn clamp(self, min: Self, max: Self) -> Self {
        debug_assert!(
            min.0 <= max.0,
            "Attempt to clamp with minimum value greater than maximum value"
        );
        Self::min(Self::max(self, min), max)
    }

    /// Checked integer addition. Returns `None` if overflow occurred.
    pub const fn checked_add(self, rhs: Self) -> Option<Self> {
        match self.0.checked_add(rhs.0) {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked integer subtraction. Returns `None` if overflow occurred.
    pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
        match self.0.checked_sub(rhs.0) {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked integer multiplication. Returns `None` if overflow occurred.
    pub const fn checked_mul(self, rhs: Self) -> Option<Self> {
        match self.0.checked_mul(rhs.0) {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked integer division. Returns `None` if `rhs` is zero, or the division resulted in
    /// overflow.
    pub const fn checked_div(self, rhs: Self) -> Option<Self> {
        match self.0.checked_div(rhs.0) {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked euclidian division. Returns `None` if `rhs` is zero or `self.div_euclid(rhs)`
    /// would have resulted in overflow.
    pub const fn checked_div_euclid(self, rhs: Self) -> Option<Self> {
        match self.0.checked_div_euclid(rhs.0) {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked integer remainder. Returns `None` if `rhs` is zero or the remainder would have
    /// resulted in overflow.
    pub const fn checked_rem(self, rhs: Self) -> Option<Self> {
        match self.0.checked_rem(rhs.0) {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked euclidian remainder. Returns `None` if `rhs` is zero or `self.rem_euclid(rhs)`
    /// would have resulted in overflow.
    pub const fn checked_rem_euclid(self, rhs: Self) -> Option<Self> {
        match self.0.checked_rem_euclid(rhs.0) {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked integer negation. Returns `None` if the negation resulted in overflow.
    pub const fn checked_neg(self) -> Option<Self> {
        match self.0.checked_neg() {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked left bit shift. Returns `None` if `rhs` is greater than or equal to
    /// [`Self::BITS`].
    pub const fn checked_shl(self, rhs: u32) -> Option<Self> {
        if rhs >= Self::BITS {
            None
        } else {
            Some(self.shl(rhs))
        }
    }

    /// Checked right bit shift. Returns `None` if `rhs` is greater than or equal to
    /// [`Self::BITS`].
    pub const fn checked_shr(self, rhs: u32) -> Option<Self> {
        if rhs >= Self::BITS {
            None
        } else {
            Some(self.shr(rhs))
        }
    }

    /// Checked exponentiation. Returns `None` if the exponentiation resulted in overflow.
    pub const fn checked_pow(self, exp: u32) -> Option<Self> {
        match self.0.checked_pow(exp) {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked logarithm, rounded down. Returns `None` if `self` is less than or equal zero,
    /// or if `base` is less than 2.
    pub const fn checked_ilog(self, base: Self) -> Option<u32> {
        self.0.checked_ilog(base.0)
    }

    /// Checked base 2 logarithm, rounded down. Returns `None` if self is less than or equal to
    /// zero.
    pub const fn checked_ilog2(self) -> Option<u32> {
        self.0.checked_ilog2()
    }

    /// Checked base 10 logarithm, rounded down. Returns `None` if self is less than or equal
    /// to zero.
    pub const fn checked_ilog10(self) -> Option<u32> {
        self.0.checked_ilog10()
    }

    /// Satruating integer addition. The result of the addition is clamped between
    /// [`MIN`](Self::MIN) and [`MAX`](Self::MAX).
    pub const fn saturating_add(self, rhs: Self) -> Self {
        Self::new_saturating(self.0.saturating_add(rhs.0))
    }

    /// Saturating integer subtraction. The result of the subtraction is clamped between
    /// [`MIN`](Self::MIN) and [`MAX`](Self::MAX).
    pub const fn saturating_sub(self, rhs: Self) -> Self {
        Self::new_saturating(self.0.saturating_sub(rhs.0))
    }

    /// Saturating integer multiplication. The result of the multiplication is clamped between
    /// [`MIN`](Self::MIN) and [`MAX`](Self::MAX).
    pub const fn saturating_mul(self, rhs: Self) -> Self {
        Self::new_saturating(self.0.saturating_mul(rhs.0))
    }

    /// Saturating exponentiation. The result of the exponentiation is clammed between
    /// [`MIN`](Self::MIN) and [`MAX`](Self::MAX).
    pub const fn saturating_pow(self, exp: u32) -> Self {
        Self::new_saturating(self.0.saturating_pow(exp))
    }

    /// Wrapping (modular) addition. An overflowing result is wrapped around the bounds of this
    /// integer type.
    pub const fn wrapping_add(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.wrapping_add(rhs.0))
    }

    /// Wrapping (modular) subtraction. An overflowing result is wrapped around the bounds of
    /// this integer type.
    pub const fn wrapping_sub(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.wrapping_sub(rhs.0))
    }

    /// Wrapping (modular) multiplication. An overflowing result is wrapped around the bounds of
    /// this integer type.
    pub const fn wrapping_mul(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.wrapping_mul(rhs.0))
    }

    /// Wrapping (modular) division. An overflowing result is wrapped around the bounds of this
    /// integer type.
    pub const fn wrapping_div(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.wrapping_div(rhs.0))
    }

    /// Wrapping (modular) euclidian division. An overflowing result is wrapped around the
    /// bounds of this integer type.
    pub const fn wrapping_div_euclid(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.wrapping_div_euclid(rhs.0))
    }

    /// Wrapping (modular) remainder. An overflowing result is wrapped around the bounds of
    /// this integer type.
    pub const fn wrapping_rem(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.wrapping_rem(rhs.0))
    }

    /// Wrapping (modular) euclidian remainder. An overflowing result is wrapped around the
    /// bounds of this integer type.
    pub const fn wrapping_rem_euclid(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.wrapping_rem_euclid(rhs.0))
    }

    /// Wrapping (modular) negation. An overflowing result is wrapped around the bounds of this
    /// integer type.
    pub const fn wrapping_neg(self) -> Self {
        Self::new_wrapping(!self.0 + 1)
    }

    /// Wrapping (modular) left bit shift. The value is left shifted `rhs % Self::BITS` bits.
    pub const fn wrapping_shl(self, rhs: u32) -> Self {
        Self::new_wrapping(self.0 << (rhs % Self::BITS))
    }

    /// Wrapping (modular) right bit shift. The value is right shifted `rhs % Self::BITS` bits.
    pub const fn wrapping_shr(self, rhs: u32) -> Self {
        Self::new_wrapping((self.0 & Self::MASK) >> (rhs % Self::BITS))
    }

    /// Wrapping (modular) exponentiation. An overflowing result is wrapped around the bounds
    /// of this integer type.
    pub const fn wrapping_pow(self, exp: u32) -> Self {
        Self::new_wrapping(self.0.wrapping_pow(exp))
    }

    /// Wrapping (modular) aboslute value. An overflowing result is wrapped around the bounds
    /// of this integer type.
    pub const fn wrapping_abs(self) -> Self {
        if self.is_negative() {
            self.wrapping_neg()
        } else {
            self
        }
    }

    /// Calculates `self + rhs`, returning a tuple of the result and a `bool` indicating if
    /// overflow occurred.
    pub const fn overflowing_add(self, rhs: Self) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_add(rhs.0))
    }

    /// Calculates `self - rhs`, returning a tuple of the result and a `bool` indicating if
    /// overflow occurred.
    pub const fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_sub(rhs.0))
    }

    /// Calculates `self * rhs`, returning a tuple of the result and a `bool` indicating if
    /// overflow occurred.
    pub const fn overflowing_mul(self, rhs: Self) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_mul(rhs.0))
    }

    /// Calculates `self / rhs`, returning a tuple of the result and a `bool` indicating if
    /// overflow occurred.
    pub const fn overflowing_div(self, rhs: Self) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_div(rhs.0))
    }

    /// Calculates `self.div_euclid(rhs)`, returning a tuple of the result and a `bool`
    /// indicating if overflow occurred.
    pub const fn overflowing_div_euclid(self, rhs: Self) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_div_euclid(rhs.0))
    }

    /// Calculates `self % rhs`, returning a tuple of the result and a `bool` indicating if
    /// overflow occurred.
    pub const fn overflowing_rem(self, rhs: Self) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_rem(rhs.0))
    }

    /// Calculates `self.rem_euclid(rhs)`, returning a tuple of the result and a `bool`
    /// indicating if overflow occurred.
    pub const fn overflowing_rem_euclid(self, rhs: Self) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_rem_euclid(rhs.0))
    }

    /// Calculates `-self`, returning a tuple of the result and a `bool` indicating if overflow
    /// occurred.
    pub const fn overflowing_neg(self) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_neg())
    }

    /// Calculates `self << rhs`, returning a tuple of the result and a `bool` indicating if
    /// `rhs` was greater than or equal to [`Self::BITS`].
    pub const fn overflowing_shl(self, rhs: u32) -> (Self, bool) {
        (self.wrapping_shl(rhs), rhs >= Self::BITS)
    }

    /// Calculates `self >> rhs`, returning a tuple of the result and a `bool` indicating if
    /// `rhs` was greater than or equal to [`Self::BITS`].
    pub const fn overflowing_shr(self, rhs: u32) -> (Self, bool) {
        (self.wrapping_shr(rhs), rhs >= Self::BITS)
    }

    /// Calculates `self.pow(exp)`, returning a tuple of the result and a `bool` indicating if
    /// overflow occurred.
    pub const fn overflowing_pow(self, exp: u32) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_pow(exp))
    }

    /// Calculates `self.abs()`, returning a tuple of the result and a `bool` indicating if
    /// overflow occurred.
    pub const fn overflowing_abs(self) -> (Self, bool) {
        if self.is_negative() {
            self.overflowing_neg()
        } else {
            (self, false)
        }
    }

    /// Raises `self` to the power of `exp`, using exponentiation by squaring.
    pub const fn pow(self, exp: u32) -> Self {
        let val = self.0.pow(exp);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to exponentiate with overflow"
        );
        Self::new_wrapping(val)
    }

    /// Calculates the quotient of Euclidian division `self` by `rhs`.
    ///
    /// This computes the integer `q` such that `self = q * rhs + r`, with `r =
    /// self.rem_euclid(rhs)` and `0 <= r < rhs.abs()`.
    ///
    /// In other words, the result is `self / rhs`, rounded to the integer `q` such that `self
    /// >= q * rhs`. If `self > 0`, this is equal to round towards zero. If `self < 0`, this is
    /// equal to rounds towards +/- infinity.
    pub const fn div_euclid(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.div_euclid(rhs.0))
    }

    /// Calculates the non-negative remainder `self (mod rhs)`.
    ///
    /// This is done as if by the Euclidian division algorithm - given `r =
    /// self.rem_euclid(rhs)`, `self = self * self.div_euclid(rhs) + r`, and `0 <= r <
    /// r.abs()`.
    pub const fn rem_euclid(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.div_euclid(rhs.0))
    }

    /// Computes the absolute value of this integer.
    pub const fn abs(self) -> Self {
        debug_assert!(
            self.0 != Self::MIN.0,
            "attempt to calculate absolute value with overflow"
        );
        self.wrapping_abs()
    }

    /// Computes the absolute difference between `self` and `rhs`.
    ///
    /// This is equivalent to `(self - rhs).abs()` without the posibility of intermediate
    /// overflow.
    pub const fn abs_diff(self, rhs: Self) -> Aint<u32, WIDTH> {
        Aint(self.0.abs_diff(rhs.0))
    }

    /// Converts a string slice in a given base (`radix`) to an integer.
    ///
    /// The string is expected to be an optional `+` or `-` sign followed by digits. Leading
    /// and trailing whitespace will result in an error. Digits are a subset of the following
    /// characters depending on the `radix`.
    ///
    /// * `0-9`
    /// * `a-z`
    /// * `A-Z`
    ///
    /// # Panics
    /// This function panics if `radix` is not in the range from 2 to 36.
    pub fn from_str_radix(src: &str, radix: u32) -> Result<Self, core::num::ParseIntError> {
        Self::new(i32::from_str_radix(src, radix)?).ok_or_else(make_parse_int_err)
    }
}

impl Aint<i32, 24> {
    /// Reverses the byte order of the integer.
    pub const fn swap_bytes(self) -> Self {
        Self((self.0.swap_bytes() >> 8) & Self::MASK)
    }

    /// Converts an integer from big endian to the target's endianness.
    ///
    /// On big endian this is a no-op. On little endian the bytes are swapped.
    pub const fn from_be(x: Self) -> Self {
        #[cfg(target_endian = "big")]
        {
            x
        }
        #[cfg(target_endian = "little")]
        {
            x.swap_bytes()
        }
    }

    /// Converts an integer from little endian to the target's endianness.
    ///
    /// On little endian this is a no-op. On big endian the bytes are swapped.
    pub const fn from_le(x: Self) -> Self {
        #[cfg(target_endian = "little")]
        {
            x
        }
        #[cfg(target_endian = "big")]
        {
            x.swap_bytes()
        }
    }

    /// Converts `self` to big endian from the target's endianness.
    ///
    /// On big endian this is a no-op. On little endian the bytes are swapped.
    pub const fn to_be(self) -> Self {
        #[cfg(target_endian = "big")]
        {
            self
        }
        #[cfg(target_endian = "little")]
        {
            self.swap_bytes()
        }
    }

    /// Converts `self` to little endian from the target's endianness.
    ///
    /// On little endian this is a no-op. On big endian the bytes are swapped.
    pub const fn to_le(self) -> Self {
        #[cfg(target_endian = "little")]
        {
            self
        }
        #[cfg(target_endian = "big")]
        {
            self.swap_bytes()
        }
    }

    /// Creates an integer value from its memory representation as a byte array in bit endian.
    pub const fn from_be_bytes(bytes: [u8; 3]) -> Self {
        Self(i32::from_be_bytes([0, bytes[0], bytes[1], bytes[2]]))
    }

    /// Creates an integer value from its memory representation as a byte array in little endian.
    pub const fn from_le_bytes(bytes: [u8; 3]) -> Self {
        Self(i32::from_le_bytes([bytes[0], bytes[1], bytes[2], 0]))
    }

    /// Creates an integer value from its memory representation as a byte array in native
    /// endianness.
    ///
    /// As the target platform's native endianness is used, portable code likely wants to
    /// use [`from_be_bytes`](Self::from_be_bytes) or [`from_le_bytes`](Self::from_le_bytes),
    /// as appropriate instead.
    pub const fn from_ne_bytes(bytes: [u8; 3]) -> Self {
        #[cfg(target_endian = "big")]
        {
            Self::from_be_bytes(bytes)
        }
        #[cfg(target_endian = "little")]
        {
            Self::from_le_bytes(bytes)
        }
    }

    /// Return the memory representation of this integer as a byte array in big-endian byte
    /// order.
    pub const fn to_be_bytes(self) -> [u8; 3] {
        let tmp = self.0.to_be_bytes();
        [tmp[1], tmp[2], tmp[3]]
    }

    /// Return the memory representation of this integer as a byte array in little-endian
    /// byte order.
    pub const fn to_le_bytes(self) -> [u8; 3] {
        let tmp = self.0.to_le_bytes();
        [tmp[0], tmp[1], tmp[2]]
    }

    /// Return the memory representation of this integer as a byte array in native byte
    /// order.
    ///
    /// As the target platform's native endianness is used, portable code likely wants to
    /// use [`to_be_bytes`](Self::to_be_bytes) or [`to_le_bytes`](Self::to_le_bytes), as
    /// appropriate instead.
    pub const fn to_ne_bytes(self) -> [u8; 3] {
        #[cfg(target_endian = "big")]
        {
            self.to_be_bytes()
        }
        #[cfg(target_endian = "little")]
        {
            self.to_le_bytes()
        }
    }
}

impl<const WIDTH: u32> Aint<i64, WIDTH> {
    const ASSERT: () = if WIDTH > 32 && WIDTH < 64 {
        ()
    } else {
        panic!("Invalid width for Aint with repr type of i64")
    };

    /// The size of this integer type in bits.
    pub const BITS: u32 = {
        let _val = Self::ASSERT;
        WIDTH
    };

    /// The smallest value that can be represented by this integer type.
    pub const MIN: Self = {
        let _val = Self::ASSERT;
        Self(!0 << (Self::BITS - 1))
    };

    /// The largest value that can be represented by this integer type.
    pub const MAX: Self = {
        let _val = Self::ASSERT;
        Self(!Self::MIN.0)
    };

    pub(crate) const MASK: i64 = {
        let _val = Self::ASSERT;
        (Self::MAX.0 << 1) | 1
    };

    const SIGN_BIT: i64 = {
        let _val = Self::ASSERT;
        1 << (Self::BITS - 1)
    };

    /// Creates a new integer value from the underlying representation type, unchecked.
    ///
    /// # Safety
    /// The representation type value must be within the range of valid values of this type.
    /// That is, the value must be greater or equal to [`MIN`](Self::MIN) and less or equal to
    /// [`MAX`](Self::MAX).
    pub const unsafe fn new_unchecked(repr: i64) -> Self {
        let _val = Self::ASSERT;
        Self(repr)
    }

    /// Creates a new integer value from the underlying representation type.
    ///
    /// The returned value is `None` if the representation value is outside the range of valid
    /// values of this integer type.
    pub const fn new(repr: i64) -> Option<Self> {
        if repr <= Self::MAX.0 && repr >= Self::MIN.0 {
            Some(Self(repr))
        } else {
            None
        }
    }

    /// Creates a new integer value from the underlying representation type.
    ///
    /// The returned value is wrapped as though the representation value were calculated using
    /// wrapping operations, such as [`wrapping_add`](Self::wrapping_add).
    pub const fn new_wrapping(repr: i64) -> Self {
        if (repr & Self::SIGN_BIT) == 0 {
            Self(repr & Self::MAX.0)
        } else {
            Self(repr | !Self::MAX.0)
        }
    }

    /// Creates a new integer value from the underlying representation type.
    ///
    /// The returned value is saturated to the bounds of this integer's value range. If the
    /// representation value is greater than [`MAX`](Self::MAX), the returned value will be
    /// [`MAX`](Self::MAX). If the representation value is less than [`MIN`](Self::MIN), the
    /// returned value will be [`MIN`](Self::MIN).
    pub const fn new_saturating(repr: i64) -> Self {
        if repr >= Self::MAX.0 {
            Self::MAX
        } else if repr <= Self::MIN.0 {
            Self::MIN
        } else {
            Self(repr)
        }
    }

    const fn new_overflowing_impl((repr, over): (i64, bool)) -> (Self, bool) {
        if repr > Self::MAX.0 {
            (Self(repr & Self::MAX.0), true)
        } else if repr < Self::MIN.0 {
            (Self(repr | !Self::MAX.0), true)
        } else {
            (Self(repr), over)
        }
    }

    /// Creates a new integer from the underlying representation type.
    ///
    /// The returned tuple contains the new integer and a `bool` indicating if the representation
    /// value overflowed the new integer. In the case of overflow, the new integer has a value
    /// as if it were produced from [`new_wrapping`](Self::new_wrapping).
    pub const fn new_overflowing(repr: i64) -> (Self, bool) {
        Self::new_overflowing_impl((repr, false))
    }

    /// Returns the value of this integer as the underlying representation type.
    pub const fn repr(self) -> i64 {
        self.0
    }

    /// Returns `true` if this integer is less than zero.
    pub const fn is_negative(self) -> bool {
        (self.0 & Self::SIGN_BIT) != 0
    }

    /// Returns `true` if this integer is greater than zero.
    pub const fn is_positive(self) -> bool {
        self.0 != 0 && (self.0 & Self::SIGN_BIT) == 0
    }

    /// Returns the sign of the integer.
    ///
    /// * If `self < 0`, returns `-1`
    /// * If `self > 0`, returns `1`
    /// * If `self == 0`, returns `0`
    pub const fn signum(self) -> Self {
        if self.0 == 0 {
            Self(0)
        } else if self.0 > 0 {
            Self(1)
        } else {
            Self(-1)
        }
    }

    /// Returns the number of ones in the binary representation of this integer.
    pub const fn count_ones(self) -> u32 {
        (self.0 & Self::MASK).count_ones()
    }

    /// Returns the number of zeros in the binary representatino of this integer.
    pub const fn count_zeros(self) -> u32 {
        (self.0 | !Self::MASK).count_zeros()
    }

    /// Returns the number of leading zeros in the binary represnetation of this integer.
    pub const fn leading_zeros(self) -> u32 {
        if self.is_negative() {
            0
        } else {
            self.0.leading_zeros() - (i64::BITS - Self::BITS)
        }
    }

    /// Returns the number of trailing zeros in the binary representatino of this integer.
    pub const fn trailing_zeros(self) -> u32 {
        (self.0 | !Self::MASK).trailing_zeros()
    }

    /// Returns the number of leading ones in the binary representation of this integer.
    pub const fn leading_ones(self) -> u32 {
        (self.0 | !Self::MASK).leading_ones() - (i64::BITS - Self::BITS)
    }

    /// Returns the number of trailing ones in the binary representation of this integer.
    pub const fn trailing_ones(self) -> u32 {
        (self.0 & Self::MASK).trailing_ones()
    }

    /// Performs a left bit shift by `n`, wrapping the truncated bits back to the end.
    ///
    /// Note that unlike the `<<` operator, all values of `n` are valid. A value of `n`, that
    /// is greater than or equal to [`BITS`](Self::BITS), is the equivalent to using the value
    /// `n % Self::BITS`.
    pub const fn rotate_left(self, n: u32) -> Self {
        let n = n % Self::BITS;
        Self::new_wrapping((self.0 << n) | ((self.0 & Self::MASK) >> (Self::BITS - n)))
    }

    /// Performs a right bit shift by `n`, wrapping the truncated bits back to the end.
    ///
    /// Note that unlike the `>>` operator, all values of `n` are valid. A value of `n`, that
    /// is greater than or equal to [`BITS`](Self::BITS), is the equivalent to using the value
    /// `n % Self::BITS`.
    pub const fn rotate_right(self, n: u32) -> Self {
        let n = n % Self::BITS;
        Self::new_wrapping(((self.0 & Self::MASK) >> n) | (self.0 << (Self::BITS - n)))
    }

    /// Reverses the order of the bits in the binary representation of this integer.
    pub const fn reverse_bits(self) -> Self {
        Self(self.0.reverse_bits() >> (i64::BITS - Self::BITS))
    }

    /// Calculates the negative of this integer.
    ///
    /// This method works as a `const` capable alternative to `-self`.
    pub const fn neg(self) -> Self {
        debug_assert!(self.0 != Self::MIN.0, "attempt to negate with overflow");
        self.wrapping_neg()
    }

    /// Calculates the sum of this integer with another integer.
    ///
    /// This method works as a `const` capable alternative to `self + rhs`.
    pub const fn add(self, rhs: Self) -> Self {
        let val = self.0 + rhs.0;
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to add with overflow"
        );
        Self::new_wrapping(val)
    }

    /// Calculates the difference of this integer with another integer.
    ///
    /// This method works as a `const` capable alternative to `self - rhs`.
    pub const fn sub(self, rhs: Self) -> Self {
        let val = self.0 - rhs.0;
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to subtract with overflow"
        );
        Self::new_wrapping(val)
    }

    /// Calculates the product of this integer with another integer.
    ///
    /// This method works as a `const` capable alternative to `self * rhs`.
    pub const fn mul(self, rhs: Self) -> Self {
        let val = self.0 * rhs.0;
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to multiply with overflow"
        );
        Self::new_wrapping(val)
    }

    /// Calculates the quotient of this integer over another integer.
    ///
    /// This method works as a `const` capable alternative to `self / rhs`.
    pub const fn div(self, rhs: Self) -> Self {
        let val = self.0 / rhs.0;
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to divide with overflow"
        );
        Self(val)
    }

    /// Calculates the remainder of this integer over another integer.
    ///
    /// This method works as a `const` capable alternative to `self % rhs`.
    pub const fn rem(self, rhs: Self) -> Self {
        let val = self.0 % rhs.0;
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to calculate modulus with overflow"
        );
        Self(val)
    }

    /// Calculates the bitwise and of this integer with another integer.
    ///
    /// This method works as a `const` capable alternative to `self & rhs`.
    pub const fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }

    /// Calculates the bitwise or of this integer with another integer.
    ///
    /// This method works as a `const` capable alternative to `self | rhs`.
    pub const fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }

    /// Calculates the bitwise exclusive-or of this integer with another integer.
    ///
    /// This method works as a `const` capable alternative to `self ^ rhs`.
    pub const fn bitxor(self, rhs: Self) -> Self {
        Self(self.0 ^ rhs.0)
    }

    /// Calculates the bitwise negation of this integer.
    ///
    /// This method works as a `const` capable alternative to `!self`.
    pub const fn not(self) -> Self {
        Self(!self.0)
    }

    /// Calculates the left bitwise shift of this integer by `rhs`.
    ///
    /// This method works as a `const` capable alternative to `self << rhs`.
    pub const fn shl(self, rhs: u32) -> Self {
        debug_assert!(rhs >= Self::BITS, "attempt to shift left with overflow");
        Self::new_wrapping(self.0 << rhs)
    }

    /// Calculates the right bitwise shift of this integer by `rhs`.
    ///
    /// This method works as a `const` capable alternative to `self >> rhs`.
    pub const fn shr(self, rhs: u32) -> Self {
        debug_assert!(rhs >= Self::BITS, "attempt to shift right with overflow");
        Self::new_wrapping(self.0 >> rhs)
    }

    /// Calculates the logarithm of this integer with respect to an arbitrary base, rounded
    /// down.
    ///
    /// The [`ilog2`](Self::ilog2) and [`ilog10`](Self::ilog10) methods should be preferred
    /// when applicable, as they are generally more optimized than this method since the base
    /// of the logarithm is not arbitrary.
    ///
    /// # Panics
    /// This method will panic if `self` is less than or equal to zero, or if `base` is less
    /// than 2.
    pub const fn ilog(self, base: Self) -> u32 {
        self.0.ilog(base.0)
    }

    /// Calculates the base 2 logarithm of this integer, rounded down.
    ///
    /// # Panics
    /// This method will panic if `self` is less than or equal to zero.
    pub const fn ilog2(self) -> u32 {
        self.0.ilog2()
    }

    /// Calculates the base 10 logarithm of this integer, rounded down.
    ///
    /// # Panics
    /// This method will panic if `self` is less than or equal to zero.
    pub const fn ilog10(self) -> u32 {
        self.0.ilog10()
    }

    /// Returns `true` if `self` and `rhs` are equal.
    ///
    /// This method works as a `const` capable alternative to `self == rhs`.
    pub const fn eq(self, rhs: Self) -> bool {
        return self.0 == rhs.0;
    }

    /// Returns `true` if `self` and `rhs` are not equal.
    ///
    /// This method works as a `const` capable alternative to `self != rhs`.
    pub const fn ne(self, rhs: Self) -> bool {
        return self.0 != rhs.0;
    }

    /// Returns `true` if `self` is less than `rhs`.
    ///
    /// This method works as a `const` capable alternative to `self < rhs`.
    pub const fn lt(self, rhs: Self) -> bool {
        return self.0 < rhs.0;
    }

    /// Returns `true` if `self` is less than or equal to `rhs`.
    ///
    /// This method works as a `const` capable alternative to `self <= rhs`.
    pub const fn le(self, rhs: Self) -> bool {
        return self.0 <= rhs.0;
    }

    /// Returns `true` if `self` is greater than `rhs`.
    ///
    /// This method works as a `const` capable alternative to `self > rhs`.
    pub const fn gt(self, rhs: Self) -> bool {
        return self.0 > rhs.0;
    }

    /// Returns `true` if `self` is greater than or equal to `rhs`.
    ///
    /// This method works as a `const` capable alternative to `self >= rhs`.
    pub const fn ge(self, rhs: Self) -> bool {
        return self.0 >= rhs.0;
    }

    /// Returns the ordering of `self` with respect to `rhs`.
    ///
    /// This method works as a `const` capable alternative to `Ord::cmp(self, rhs)`.
    pub const fn cmp(self, rhs: Self) -> core::cmp::Ordering {
        if self.0 < rhs.0 {
            core::cmp::Ordering::Less
        } else if self.0 > rhs.0 {
            core::cmp::Ordering::Greater
        } else {
            core::cmp::Ordering::Equal
        }
    }

    /// Returns the minimum of `self` and `other`.
    ///
    /// This method works as a `const` capable alternative to `Ord::min(self, rhs)`.
    pub const fn min(self, other: Self) -> Self {
        if self.0 < other.0 {
            self
        } else {
            other
        }
    }

    /// Returns the maximum of `self` and `other`.
    ///
    /// This method works as a `const` capable alternative to `Ord::max(self, rhs)`.
    pub const fn max(self, other: Self) -> Self {
        if self.0 > other.0 {
            self
        } else {
            other
        }
    }

    /// Returns the value of this integer, clamped between `min` and `max`.
    ///
    /// This method works as a `const` capable alternative to `Ord::clamp(self, min, max)`.
    pub const fn clamp(self, min: Self, max: Self) -> Self {
        debug_assert!(
            min.0 <= max.0,
            "Attempt to clamp with minimum value greater than maximum value"
        );
        Self::min(Self::max(self, min), max)
    }

    /// Checked integer addition. Returns `None` if overflow occurred.
    pub const fn checked_add(self, rhs: Self) -> Option<Self> {
        match self.0.checked_add(rhs.0) {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked integer subtraction. Returns `None` if overflow occurred.
    pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
        match self.0.checked_sub(rhs.0) {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked integer multiplication. Returns `None` if overflow occurred.
    pub const fn checked_mul(self, rhs: Self) -> Option<Self> {
        match self.0.checked_mul(rhs.0) {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked integer division. Returns `None` if `rhs` is zero, or the division resulted in
    /// overflow.
    pub const fn checked_div(self, rhs: Self) -> Option<Self> {
        match self.0.checked_div(rhs.0) {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked euclidian division. Returns `None` if `rhs` is zero or `self.div_euclid(rhs)`
    /// would have resulted in overflow.
    pub const fn checked_div_euclid(self, rhs: Self) -> Option<Self> {
        match self.0.checked_div_euclid(rhs.0) {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked integer remainder. Returns `None` if `rhs` is zero or the remainder would have
    /// resulted in overflow.
    pub const fn checked_rem(self, rhs: Self) -> Option<Self> {
        match self.0.checked_rem(rhs.0) {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked euclidian remainder. Returns `None` if `rhs` is zero or `self.rem_euclid(rhs)`
    /// would have resulted in overflow.
    pub const fn checked_rem_euclid(self, rhs: Self) -> Option<Self> {
        match self.0.checked_rem_euclid(rhs.0) {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked integer negation. Returns `None` if the negation resulted in overflow.
    pub const fn checked_neg(self) -> Option<Self> {
        match self.0.checked_neg() {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked left bit shift. Returns `None` if `rhs` is greater than or equal to
    /// [`Self::BITS`].
    pub const fn checked_shl(self, rhs: u32) -> Option<Self> {
        if rhs >= Self::BITS {
            None
        } else {
            Some(self.shl(rhs))
        }
    }

    /// Checked right bit shift. Returns `None` if `rhs` is greater than or equal to
    /// [`Self::BITS`].
    pub const fn checked_shr(self, rhs: u32) -> Option<Self> {
        if rhs >= Self::BITS {
            None
        } else {
            Some(self.shr(rhs))
        }
    }

    /// Checked exponentiation. Returns `None` if the exponentiation resulted in overflow.
    pub const fn checked_pow(self, exp: u32) -> Option<Self> {
        match self.0.checked_pow(exp) {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked logarithm, rounded down. Returns `None` if `self` is less than or equal zero,
    /// or if `base` is less than 2.
    pub const fn checked_ilog(self, base: Self) -> Option<u32> {
        self.0.checked_ilog(base.0)
    }

    /// Checked base 2 logarithm, rounded down. Returns `None` if self is less than or equal to
    /// zero.
    pub const fn checked_ilog2(self) -> Option<u32> {
        self.0.checked_ilog2()
    }

    /// Checked base 10 logarithm, rounded down. Returns `None` if self is less than or equal
    /// to zero.
    pub const fn checked_ilog10(self) -> Option<u32> {
        self.0.checked_ilog10()
    }

    /// Satruating integer addition. The result of the addition is clamped between
    /// [`MIN`](Self::MIN) and [`MAX`](Self::MAX).
    pub const fn saturating_add(self, rhs: Self) -> Self {
        Self::new_saturating(self.0.saturating_add(rhs.0))
    }

    /// Saturating integer subtraction. The result of the subtraction is clamped between
    /// [`MIN`](Self::MIN) and [`MAX`](Self::MAX).
    pub const fn saturating_sub(self, rhs: Self) -> Self {
        Self::new_saturating(self.0.saturating_sub(rhs.0))
    }

    /// Saturating integer multiplication. The result of the multiplication is clamped between
    /// [`MIN`](Self::MIN) and [`MAX`](Self::MAX).
    pub const fn saturating_mul(self, rhs: Self) -> Self {
        Self::new_saturating(self.0.saturating_mul(rhs.0))
    }

    /// Saturating exponentiation. The result of the exponentiation is clammed between
    /// [`MIN`](Self::MIN) and [`MAX`](Self::MAX).
    pub const fn saturating_pow(self, exp: u32) -> Self {
        Self::new_saturating(self.0.saturating_pow(exp))
    }

    /// Wrapping (modular) addition. An overflowing result is wrapped around the bounds of this
    /// integer type.
    pub const fn wrapping_add(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.wrapping_add(rhs.0))
    }

    /// Wrapping (modular) subtraction. An overflowing result is wrapped around the bounds of
    /// this integer type.
    pub const fn wrapping_sub(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.wrapping_sub(rhs.0))
    }

    /// Wrapping (modular) multiplication. An overflowing result is wrapped around the bounds of
    /// this integer type.
    pub const fn wrapping_mul(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.wrapping_mul(rhs.0))
    }

    /// Wrapping (modular) division. An overflowing result is wrapped around the bounds of this
    /// integer type.
    pub const fn wrapping_div(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.wrapping_div(rhs.0))
    }

    /// Wrapping (modular) euclidian division. An overflowing result is wrapped around the
    /// bounds of this integer type.
    pub const fn wrapping_div_euclid(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.wrapping_div_euclid(rhs.0))
    }

    /// Wrapping (modular) remainder. An overflowing result is wrapped around the bounds of
    /// this integer type.
    pub const fn wrapping_rem(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.wrapping_rem(rhs.0))
    }

    /// Wrapping (modular) euclidian remainder. An overflowing result is wrapped around the
    /// bounds of this integer type.
    pub const fn wrapping_rem_euclid(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.wrapping_rem_euclid(rhs.0))
    }

    /// Wrapping (modular) negation. An overflowing result is wrapped around the bounds of this
    /// integer type.
    pub const fn wrapping_neg(self) -> Self {
        Self::new_wrapping(!self.0 + 1)
    }

    /// Wrapping (modular) left bit shift. The value is left shifted `rhs % Self::BITS` bits.
    pub const fn wrapping_shl(self, rhs: u32) -> Self {
        Self::new_wrapping(self.0 << (rhs % Self::BITS))
    }

    /// Wrapping (modular) right bit shift. The value is right shifted `rhs % Self::BITS` bits.
    pub const fn wrapping_shr(self, rhs: u32) -> Self {
        Self::new_wrapping((self.0 & Self::MASK) >> (rhs % Self::BITS))
    }

    /// Wrapping (modular) exponentiation. An overflowing result is wrapped around the bounds
    /// of this integer type.
    pub const fn wrapping_pow(self, exp: u32) -> Self {
        Self::new_wrapping(self.0.wrapping_pow(exp))
    }

    /// Wrapping (modular) aboslute value. An overflowing result is wrapped around the bounds
    /// of this integer type.
    pub const fn wrapping_abs(self) -> Self {
        if self.is_negative() {
            self.wrapping_neg()
        } else {
            self
        }
    }

    /// Calculates `self + rhs`, returning a tuple of the result and a `bool` indicating if
    /// overflow occurred.
    pub const fn overflowing_add(self, rhs: Self) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_add(rhs.0))
    }

    /// Calculates `self - rhs`, returning a tuple of the result and a `bool` indicating if
    /// overflow occurred.
    pub const fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_sub(rhs.0))
    }

    /// Calculates `self * rhs`, returning a tuple of the result and a `bool` indicating if
    /// overflow occurred.
    pub const fn overflowing_mul(self, rhs: Self) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_mul(rhs.0))
    }

    /// Calculates `self / rhs`, returning a tuple of the result and a `bool` indicating if
    /// overflow occurred.
    pub const fn overflowing_div(self, rhs: Self) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_div(rhs.0))
    }

    /// Calculates `self.div_euclid(rhs)`, returning a tuple of the result and a `bool`
    /// indicating if overflow occurred.
    pub const fn overflowing_div_euclid(self, rhs: Self) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_div_euclid(rhs.0))
    }

    /// Calculates `self % rhs`, returning a tuple of the result and a `bool` indicating if
    /// overflow occurred.
    pub const fn overflowing_rem(self, rhs: Self) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_rem(rhs.0))
    }

    /// Calculates `self.rem_euclid(rhs)`, returning a tuple of the result and a `bool`
    /// indicating if overflow occurred.
    pub const fn overflowing_rem_euclid(self, rhs: Self) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_rem_euclid(rhs.0))
    }

    /// Calculates `-self`, returning a tuple of the result and a `bool` indicating if overflow
    /// occurred.
    pub const fn overflowing_neg(self) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_neg())
    }

    /// Calculates `self << rhs`, returning a tuple of the result and a `bool` indicating if
    /// `rhs` was greater than or equal to [`Self::BITS`].
    pub const fn overflowing_shl(self, rhs: u32) -> (Self, bool) {
        (self.wrapping_shl(rhs), rhs >= Self::BITS)
    }

    /// Calculates `self >> rhs`, returning a tuple of the result and a `bool` indicating if
    /// `rhs` was greater than or equal to [`Self::BITS`].
    pub const fn overflowing_shr(self, rhs: u32) -> (Self, bool) {
        (self.wrapping_shr(rhs), rhs >= Self::BITS)
    }

    /// Calculates `self.pow(exp)`, returning a tuple of the result and a `bool` indicating if
    /// overflow occurred.
    pub const fn overflowing_pow(self, exp: u32) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_pow(exp))
    }

    /// Calculates `self.abs()`, returning a tuple of the result and a `bool` indicating if
    /// overflow occurred.
    pub const fn overflowing_abs(self) -> (Self, bool) {
        if self.is_negative() {
            self.overflowing_neg()
        } else {
            (self, false)
        }
    }

    /// Raises `self` to the power of `exp`, using exponentiation by squaring.
    pub const fn pow(self, exp: u32) -> Self {
        let val = self.0.pow(exp);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to exponentiate with overflow"
        );
        Self::new_wrapping(val)
    }

    /// Calculates the quotient of Euclidian division `self` by `rhs`.
    ///
    /// This computes the integer `q` such that `self = q * rhs + r`, with `r =
    /// self.rem_euclid(rhs)` and `0 <= r < rhs.abs()`.
    ///
    /// In other words, the result is `self / rhs`, rounded to the integer `q` such that `self
    /// >= q * rhs`. If `self > 0`, this is equal to round towards zero. If `self < 0`, this is
    /// equal to rounds towards +/- infinity.
    pub const fn div_euclid(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.div_euclid(rhs.0))
    }

    /// Calculates the non-negative remainder `self (mod rhs)`.
    ///
    /// This is done as if by the Euclidian division algorithm - given `r =
    /// self.rem_euclid(rhs)`, `self = self * self.div_euclid(rhs) + r`, and `0 <= r <
    /// r.abs()`.
    pub const fn rem_euclid(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.div_euclid(rhs.0))
    }

    /// Computes the absolute value of this integer.
    pub const fn abs(self) -> Self {
        debug_assert!(
            self.0 != Self::MIN.0,
            "attempt to calculate absolute value with overflow"
        );
        self.wrapping_abs()
    }

    /// Computes the absolute difference between `self` and `rhs`.
    ///
    /// This is equivalent to `(self - rhs).abs()` without the posibility of intermediate
    /// overflow.
    pub const fn abs_diff(self, rhs: Self) -> Aint<u64, WIDTH> {
        Aint(self.0.abs_diff(rhs.0))
    }

    /// Converts a string slice in a given base (`radix`) to an integer.
    ///
    /// The string is expected to be an optional `+` or `-` sign followed by digits. Leading
    /// and trailing whitespace will result in an error. Digits are a subset of the following
    /// characters depending on the `radix`.
    ///
    /// * `0-9`
    /// * `a-z`
    /// * `A-Z`
    ///
    /// # Panics
    /// This function panics if `radix` is not in the range from 2 to 36.
    pub fn from_str_radix(src: &str, radix: u32) -> Result<Self, core::num::ParseIntError> {
        Self::new(i64::from_str_radix(src, radix)?).ok_or_else(make_parse_int_err)
    }
}

impl Aint<i64, 40> {
    /// Reverses the byte order of the integer.
    pub const fn swap_bytes(self) -> Self {
        Self((self.0.swap_bytes() >> 24) & Self::MASK)
    }

    /// Converts an integer from big endian to the target's endianness.
    ///
    /// On big endian this is a no-op. On little endian the bytes are swapped.
    pub const fn from_be(x: Self) -> Self {
        #[cfg(target_endian = "big")]
        {
            x
        }
        #[cfg(target_endian = "little")]
        {
            x.swap_bytes()
        }
    }

    /// Converts an integer from little endian to the target's endianness.
    ///
    /// On little endian this is a no-op. On big endian the bytes are swapped.
    pub const fn from_le(x: Self) -> Self {
        #[cfg(target_endian = "little")]
        {
            x
        }
        #[cfg(target_endian = "big")]
        {
            x.swap_bytes()
        }
    }

    /// Converts `self` to big endian from the target's endianness.
    ///
    /// On big endian this is a no-op. On little endian the bytes are swapped.
    pub const fn to_be(self) -> Self {
        #[cfg(target_endian = "big")]
        {
            self
        }
        #[cfg(target_endian = "little")]
        {
            self.swap_bytes()
        }
    }

    /// Converts `self` to little endian from the target's endianness.
    ///
    /// On little endian this is a no-op. On big endian the bytes are swapped.
    pub const fn to_le(self) -> Self {
        #[cfg(target_endian = "little")]
        {
            self
        }
        #[cfg(target_endian = "big")]
        {
            self.swap_bytes()
        }
    }

    /// Creates an integer value from its memory representation as a byte array in bit endian.
    pub const fn from_be_bytes(bytes: [u8; 5]) -> Self {
        Self(i64::from_be_bytes([
            0, 0, 0, bytes[0], bytes[1], bytes[2], bytes[3], bytes[4],
        ]))
    }

    /// Creates an integer value from its memory representation as a byte array in little endian.
    pub const fn from_le_bytes(bytes: [u8; 5]) -> Self {
        Self(i64::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], 0, 0, 0,
        ]))
    }

    /// Creates an integer value from its memory representation as a byte array in native
    /// endianness.
    ///
    /// As the target platform's native endianness is used, portable code likely wants to
    /// use [`from_be_bytes`](Self::from_be_bytes) or [`from_le_bytes`](Self::from_le_bytes),
    /// as appropriate instead.
    pub const fn from_ne_bytes(bytes: [u8; 5]) -> Self {
        #[cfg(target_endian = "big")]
        {
            Self::from_be_bytes(bytes)
        }
        #[cfg(target_endian = "little")]
        {
            Self::from_le_bytes(bytes)
        }
    }

    /// Return the memory representation of this integer as a byte array in big-endian byte
    /// order.
    pub const fn to_be_bytes(self) -> [u8; 5] {
        let tmp = self.0.to_be_bytes();
        [tmp[3], tmp[4], tmp[5], tmp[6], tmp[7]]
    }

    /// Return the memory representation of this integer as a byte array in little-endian
    /// byte order.
    pub const fn to_le_bytes(self) -> [u8; 5] {
        let tmp = self.0.to_le_bytes();
        [tmp[0], tmp[1], tmp[2], tmp[3], tmp[4]]
    }

    /// Return the memory representation of this integer as a byte array in native byte
    /// order.
    ///
    /// As the target platform's native endianness is used, portable code likely wants to
    /// use [`to_be_bytes`](Self::to_be_bytes) or [`to_le_bytes`](Self::to_le_bytes), as
    /// appropriate instead.
    pub const fn to_ne_bytes(self) -> [u8; 5] {
        #[cfg(target_endian = "big")]
        {
            self.to_be_bytes()
        }
        #[cfg(target_endian = "little")]
        {
            self.to_le_bytes()
        }
    }
}

impl Aint<i64, 48> {
    /// Reverses the byte order of the integer.
    pub const fn swap_bytes(self) -> Self {
        Self((self.0.swap_bytes() >> 16) & Self::MASK)
    }

    /// Converts an integer from big endian to the target's endianness.
    ///
    /// On big endian this is a no-op. On little endian the bytes are swapped.
    pub const fn from_be(x: Self) -> Self {
        #[cfg(target_endian = "big")]
        {
            x
        }
        #[cfg(target_endian = "little")]
        {
            x.swap_bytes()
        }
    }

    /// Converts an integer from little endian to the target's endianness.
    ///
    /// On little endian this is a no-op. On big endian the bytes are swapped.
    pub const fn from_le(x: Self) -> Self {
        #[cfg(target_endian = "little")]
        {
            x
        }
        #[cfg(target_endian = "big")]
        {
            x.swap_bytes()
        }
    }

    /// Converts `self` to big endian from the target's endianness.
    ///
    /// On big endian this is a no-op. On little endian the bytes are swapped.
    pub const fn to_be(self) -> Self {
        #[cfg(target_endian = "big")]
        {
            self
        }
        #[cfg(target_endian = "little")]
        {
            self.swap_bytes()
        }
    }

    /// Converts `self` to little endian from the target's endianness.
    ///
    /// On little endian this is a no-op. On big endian the bytes are swapped.
    pub const fn to_le(self) -> Self {
        #[cfg(target_endian = "little")]
        {
            self
        }
        #[cfg(target_endian = "big")]
        {
            self.swap_bytes()
        }
    }

    /// Creates an integer value from its memory representation as a byte array in bit endian.
    pub const fn from_be_bytes(bytes: [u8; 6]) -> Self {
        Self(i64::from_be_bytes([
            0, 0, bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5],
        ]))
    }

    /// Creates an integer value from its memory representation as a byte array in little endian.
    pub const fn from_le_bytes(bytes: [u8; 6]) -> Self {
        Self(i64::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], 0, 0,
        ]))
    }

    /// Creates an integer value from its memory representation as a byte array in native
    /// endianness.
    ///
    /// As the target platform's native endianness is used, portable code likely wants to
    /// use [`from_be_bytes`](Self::from_be_bytes) or [`from_le_bytes`](Self::from_le_bytes),
    /// as appropriate instead.
    pub const fn from_ne_bytes(bytes: [u8; 6]) -> Self {
        #[cfg(target_endian = "big")]
        {
            Self::from_be_bytes(bytes)
        }
        #[cfg(target_endian = "little")]
        {
            Self::from_le_bytes(bytes)
        }
    }

    /// Return the memory representation of this integer as a byte array in big-endian byte
    /// order.
    pub const fn to_be_bytes(self) -> [u8; 6] {
        let tmp = self.0.to_be_bytes();
        [tmp[2], tmp[3], tmp[4], tmp[5], tmp[6], tmp[7]]
    }

    /// Return the memory representation of this integer as a byte array in little-endian
    /// byte order.
    pub const fn to_le_bytes(self) -> [u8; 6] {
        let tmp = self.0.to_le_bytes();
        [tmp[0], tmp[1], tmp[2], tmp[3], tmp[4], tmp[5]]
    }

    /// Return the memory representation of this integer as a byte array in native byte
    /// order.
    ///
    /// As the target platform's native endianness is used, portable code likely wants to
    /// use [`to_be_bytes`](Self::to_be_bytes) or [`to_le_bytes`](Self::to_le_bytes), as
    /// appropriate instead.
    pub const fn to_ne_bytes(self) -> [u8; 6] {
        #[cfg(target_endian = "big")]
        {
            self.to_be_bytes()
        }
        #[cfg(target_endian = "little")]
        {
            self.to_le_bytes()
        }
    }
}

impl Aint<i64, 56> {
    /// Reverses the byte order of the integer.
    pub const fn swap_bytes(self) -> Self {
        Self((self.0.swap_bytes() >> 8) & Self::MASK)
    }

    /// Converts an integer from big endian to the target's endianness.
    ///
    /// On big endian this is a no-op. On little endian the bytes are swapped.
    pub const fn from_be(x: Self) -> Self {
        #[cfg(target_endian = "big")]
        {
            x
        }
        #[cfg(target_endian = "little")]
        {
            x.swap_bytes()
        }
    }

    /// Converts an integer from little endian to the target's endianness.
    ///
    /// On little endian this is a no-op. On big endian the bytes are swapped.
    pub const fn from_le(x: Self) -> Self {
        #[cfg(target_endian = "little")]
        {
            x
        }
        #[cfg(target_endian = "big")]
        {
            x.swap_bytes()
        }
    }

    /// Converts `self` to big endian from the target's endianness.
    ///
    /// On big endian this is a no-op. On little endian the bytes are swapped.
    pub const fn to_be(self) -> Self {
        #[cfg(target_endian = "big")]
        {
            self
        }
        #[cfg(target_endian = "little")]
        {
            self.swap_bytes()
        }
    }

    /// Converts `self` to little endian from the target's endianness.
    ///
    /// On little endian this is a no-op. On big endian the bytes are swapped.
    pub const fn to_le(self) -> Self {
        #[cfg(target_endian = "little")]
        {
            self
        }
        #[cfg(target_endian = "big")]
        {
            self.swap_bytes()
        }
    }

    /// Creates an integer value from its memory representation as a byte array in bit endian.
    pub const fn from_be_bytes(bytes: [u8; 7]) -> Self {
        Self(i64::from_be_bytes([
            0, bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6],
        ]))
    }

    /// Creates an integer value from its memory representation as a byte array in little endian.
    pub const fn from_le_bytes(bytes: [u8; 7]) -> Self {
        Self(i64::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], 0,
        ]))
    }

    /// Creates an integer value from its memory representation as a byte array in native
    /// endianness.
    ///
    /// As the target platform's native endianness is used, portable code likely wants to
    /// use [`from_be_bytes`](Self::from_be_bytes) or [`from_le_bytes`](Self::from_le_bytes),
    /// as appropriate instead.
    pub const fn from_ne_bytes(bytes: [u8; 7]) -> Self {
        #[cfg(target_endian = "big")]
        {
            Self::from_be_bytes(bytes)
        }
        #[cfg(target_endian = "little")]
        {
            Self::from_le_bytes(bytes)
        }
    }

    /// Return the memory representation of this integer as a byte array in big-endian byte
    /// order.
    pub const fn to_be_bytes(self) -> [u8; 7] {
        let tmp = self.0.to_be_bytes();
        [tmp[1], tmp[2], tmp[3], tmp[4], tmp[5], tmp[6], tmp[7]]
    }

    /// Return the memory representation of this integer as a byte array in little-endian
    /// byte order.
    pub const fn to_le_bytes(self) -> [u8; 7] {
        let tmp = self.0.to_le_bytes();
        [tmp[0], tmp[1], tmp[2], tmp[3], tmp[4], tmp[5], tmp[6]]
    }

    /// Return the memory representation of this integer as a byte array in native byte
    /// order.
    ///
    /// As the target platform's native endianness is used, portable code likely wants to
    /// use [`to_be_bytes`](Self::to_be_bytes) or [`to_le_bytes`](Self::to_le_bytes), as
    /// appropriate instead.
    pub const fn to_ne_bytes(self) -> [u8; 7] {
        #[cfg(target_endian = "big")]
        {
            self.to_be_bytes()
        }
        #[cfg(target_endian = "little")]
        {
            self.to_le_bytes()
        }
    }
}

impl<const WIDTH: u32> Aint<i128, WIDTH> {
    const ASSERT: () = if WIDTH > 64 && WIDTH < 128 {
        ()
    } else {
        panic!("Invalid width for Aint with repr type of i128")
    };

    /// The size of this integer type in bits.
    pub const BITS: u32 = {
        let _val = Self::ASSERT;
        WIDTH
    };

    /// The smallest value that can be represented by this integer type.
    pub const MIN: Self = {
        let _val = Self::ASSERT;
        Self(!0 << (Self::BITS - 1))
    };

    /// The largest value that can be represented by this integer type.
    pub const MAX: Self = {
        let _val = Self::ASSERT;
        Self(!Self::MIN.0)
    };

    pub(crate) const MASK: i128 = {
        let _val = Self::ASSERT;
        (Self::MAX.0 << 1) | 1
    };

    const SIGN_BIT: i128 = {
        let _val = Self::ASSERT;
        1 << (Self::BITS - 1)
    };

    /// Creates a new integer value from the underlying representation type, unchecked.
    ///
    /// # Safety
    /// The representation type value must be within the range of valid values of this type.
    /// That is, the value must be greater or equal to [`MIN`](Self::MIN) and less or equal to
    /// [`MAX`](Self::MAX).
    pub const unsafe fn new_unchecked(repr: i128) -> Self {
        let _val = Self::ASSERT;
        Self(repr)
    }

    /// Creates a new integer value from the underlying representation type.
    ///
    /// The returned value is `None` if the representation value is outside the range of valid
    /// values of this integer type.
    pub const fn new(repr: i128) -> Option<Self> {
        if repr <= Self::MAX.0 && repr >= Self::MIN.0 {
            Some(Self(repr))
        } else {
            None
        }
    }

    /// Creates a new integer value from the underlying representation type.
    ///
    /// The returned value is wrapped as though the representation value were calculated using
    /// wrapping operations, such as [`wrapping_add`](Self::wrapping_add).
    pub const fn new_wrapping(repr: i128) -> Self {
        if (repr & Self::SIGN_BIT) == 0 {
            Self(repr & Self::MAX.0)
        } else {
            Self(repr | !Self::MAX.0)
        }
    }

    /// Creates a new integer value from the underlying representation type.
    ///
    /// The returned value is saturated to the bounds of this integer's value range. If the
    /// representation value is greater than [`MAX`](Self::MAX), the returned value will be
    /// [`MAX`](Self::MAX). If the representation value is less than [`MIN`](Self::MIN), the
    /// returned value will be [`MIN`](Self::MIN).
    pub const fn new_saturating(repr: i128) -> Self {
        if repr >= Self::MAX.0 {
            Self::MAX
        } else if repr <= Self::MIN.0 {
            Self::MIN
        } else {
            Self(repr)
        }
    }

    const fn new_overflowing_impl((repr, over): (i128, bool)) -> (Self, bool) {
        if repr > Self::MAX.0 {
            (Self(repr & Self::MAX.0), true)
        } else if repr < Self::MIN.0 {
            (Self(repr | !Self::MAX.0), true)
        } else {
            (Self(repr), over)
        }
    }

    /// Creates a new integer from the underlying representation type.
    ///
    /// The returned tuple contains the new integer and a `bool` indicating if the representation
    /// value overflowed the new integer. In the case of overflow, the new integer has a value
    /// as if it were produced from [`new_wrapping`](Self::new_wrapping).
    pub const fn new_overflowing(repr: i128) -> (Self, bool) {
        Self::new_overflowing_impl((repr, false))
    }

    /// Returns the value of this integer as the underlying representation type.
    pub const fn repr(self) -> i128 {
        self.0
    }

    /// Returns `true` if this integer is less than zero.
    pub const fn is_negative(self) -> bool {
        (self.0 & Self::SIGN_BIT) != 0
    }

    /// Returns `true` if this integer is greater than zero.
    pub const fn is_positive(self) -> bool {
        self.0 != 0 && (self.0 & Self::SIGN_BIT) == 0
    }

    /// Returns the sign of the integer.
    ///
    /// * If `self < 0`, returns `-1`
    /// * If `self > 0`, returns `1`
    /// * If `self == 0`, returns `0`
    pub const fn signum(self) -> Self {
        if self.0 == 0 {
            Self(0)
        } else if self.0 > 0 {
            Self(1)
        } else {
            Self(-1)
        }
    }

    /// Returns the number of ones in the binary representation of this integer.
    pub const fn count_ones(self) -> u32 {
        (self.0 & Self::MASK).count_ones()
    }

    /// Returns the number of zeros in the binary representatino of this integer.
    pub const fn count_zeros(self) -> u32 {
        (self.0 | !Self::MASK).count_zeros()
    }

    /// Returns the number of leading zeros in the binary represnetation of this integer.
    pub const fn leading_zeros(self) -> u32 {
        if self.is_negative() {
            0
        } else {
            self.0.leading_zeros() - (i128::BITS - Self::BITS)
        }
    }

    /// Returns the number of trailing zeros in the binary representatino of this integer.
    pub const fn trailing_zeros(self) -> u32 {
        (self.0 | !Self::MASK).trailing_zeros()
    }

    /// Returns the number of leading ones in the binary representation of this integer.
    pub const fn leading_ones(self) -> u32 {
        (self.0 | !Self::MASK).leading_ones() - (i128::BITS - Self::BITS)
    }

    /// Returns the number of trailing ones in the binary representation of this integer.
    pub const fn trailing_ones(self) -> u32 {
        (self.0 & Self::MASK).trailing_ones()
    }

    /// Performs a left bit shift by `n`, wrapping the truncated bits back to the end.
    ///
    /// Note that unlike the `<<` operator, all values of `n` are valid. A value of `n`, that
    /// is greater than or equal to [`BITS`](Self::BITS), is the equivalent to using the value
    /// `n % Self::BITS`.
    pub const fn rotate_left(self, n: u32) -> Self {
        let n = n % Self::BITS;
        Self::new_wrapping((self.0 << n) | ((self.0 & Self::MASK) >> (Self::BITS - n)))
    }

    /// Performs a right bit shift by `n`, wrapping the truncated bits back to the end.
    ///
    /// Note that unlike the `>>` operator, all values of `n` are valid. A value of `n`, that
    /// is greater than or equal to [`BITS`](Self::BITS), is the equivalent to using the value
    /// `n % Self::BITS`.
    pub const fn rotate_right(self, n: u32) -> Self {
        let n = n % Self::BITS;
        Self::new_wrapping(((self.0 & Self::MASK) >> n) | (self.0 << (Self::BITS - n)))
    }

    /// Reverses the order of the bits in the binary representation of this integer.
    pub const fn reverse_bits(self) -> Self {
        Self(self.0.reverse_bits() >> (i128::BITS - Self::BITS))
    }

    /// Calculates the negative of this integer.
    ///
    /// This method works as a `const` capable alternative to `-self`.
    pub const fn neg(self) -> Self {
        debug_assert!(self.0 != Self::MIN.0, "attempt to negate with overflow");
        self.wrapping_neg()
    }

    /// Calculates the sum of this integer with another integer.
    ///
    /// This method works as a `const` capable alternative to `self + rhs`.
    pub const fn add(self, rhs: Self) -> Self {
        let val = self.0 + rhs.0;
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to add with overflow"
        );
        Self::new_wrapping(val)
    }

    /// Calculates the difference of this integer with another integer.
    ///
    /// This method works as a `const` capable alternative to `self - rhs`.
    pub const fn sub(self, rhs: Self) -> Self {
        let val = self.0 - rhs.0;
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to subtract with overflow"
        );
        Self::new_wrapping(val)
    }

    /// Calculates the product of this integer with another integer.
    ///
    /// This method works as a `const` capable alternative to `self * rhs`.
    pub const fn mul(self, rhs: Self) -> Self {
        let val = self.0 * rhs.0;
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to multiply with overflow"
        );
        Self::new_wrapping(val)
    }

    /// Calculates the quotient of this integer over another integer.
    ///
    /// This method works as a `const` capable alternative to `self / rhs`.
    pub const fn div(self, rhs: Self) -> Self {
        let val = self.0 / rhs.0;
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to divide with overflow"
        );
        Self(val)
    }

    /// Calculates the remainder of this integer over another integer.
    ///
    /// This method works as a `const` capable alternative to `self % rhs`.
    pub const fn rem(self, rhs: Self) -> Self {
        let val = self.0 % rhs.0;
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to calculate modulus with overflow"
        );
        Self(val)
    }

    /// Calculates the bitwise and of this integer with another integer.
    ///
    /// This method works as a `const` capable alternative to `self & rhs`.
    pub const fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }

    /// Calculates the bitwise or of this integer with another integer.
    ///
    /// This method works as a `const` capable alternative to `self | rhs`.
    pub const fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }

    /// Calculates the bitwise exclusive-or of this integer with another integer.
    ///
    /// This method works as a `const` capable alternative to `self ^ rhs`.
    pub const fn bitxor(self, rhs: Self) -> Self {
        Self(self.0 ^ rhs.0)
    }

    /// Calculates the bitwise negation of this integer.
    ///
    /// This method works as a `const` capable alternative to `!self`.
    pub const fn not(self) -> Self {
        Self(!self.0)
    }

    /// Calculates the left bitwise shift of this integer by `rhs`.
    ///
    /// This method works as a `const` capable alternative to `self << rhs`.
    pub const fn shl(self, rhs: u32) -> Self {
        debug_assert!(rhs >= Self::BITS, "attempt to shift left with overflow");
        Self::new_wrapping(self.0 << rhs)
    }

    /// Calculates the right bitwise shift of this integer by `rhs`.
    ///
    /// This method works as a `const` capable alternative to `self >> rhs`.
    pub const fn shr(self, rhs: u32) -> Self {
        debug_assert!(rhs >= Self::BITS, "attempt to shift right with overflow");
        Self::new_wrapping(self.0 >> rhs)
    }

    /// Calculates the logarithm of this integer with respect to an arbitrary base, rounded
    /// down.
    ///
    /// The [`ilog2`](Self::ilog2) and [`ilog10`](Self::ilog10) methods should be preferred
    /// when applicable, as they are generally more optimized than this method since the base
    /// of the logarithm is not arbitrary.
    ///
    /// # Panics
    /// This method will panic if `self` is less than or equal to zero, or if `base` is less
    /// than 2.
    pub const fn ilog(self, base: Self) -> u32 {
        self.0.ilog(base.0)
    }

    /// Calculates the base 2 logarithm of this integer, rounded down.
    ///
    /// # Panics
    /// This method will panic if `self` is less than or equal to zero.
    pub const fn ilog2(self) -> u32 {
        self.0.ilog2()
    }

    /// Calculates the base 10 logarithm of this integer, rounded down.
    ///
    /// # Panics
    /// This method will panic if `self` is less than or equal to zero.
    pub const fn ilog10(self) -> u32 {
        self.0.ilog10()
    }

    /// Returns `true` if `self` and `rhs` are equal.
    ///
    /// This method works as a `const` capable alternative to `self == rhs`.
    pub const fn eq(self, rhs: Self) -> bool {
        return self.0 == rhs.0;
    }

    /// Returns `true` if `self` and `rhs` are not equal.
    ///
    /// This method works as a `const` capable alternative to `self != rhs`.
    pub const fn ne(self, rhs: Self) -> bool {
        return self.0 != rhs.0;
    }

    /// Returns `true` if `self` is less than `rhs`.
    ///
    /// This method works as a `const` capable alternative to `self < rhs`.
    pub const fn lt(self, rhs: Self) -> bool {
        return self.0 < rhs.0;
    }

    /// Returns `true` if `self` is less than or equal to `rhs`.
    ///
    /// This method works as a `const` capable alternative to `self <= rhs`.
    pub const fn le(self, rhs: Self) -> bool {
        return self.0 <= rhs.0;
    }

    /// Returns `true` if `self` is greater than `rhs`.
    ///
    /// This method works as a `const` capable alternative to `self > rhs`.
    pub const fn gt(self, rhs: Self) -> bool {
        return self.0 > rhs.0;
    }

    /// Returns `true` if `self` is greater than or equal to `rhs`.
    ///
    /// This method works as a `const` capable alternative to `self >= rhs`.
    pub const fn ge(self, rhs: Self) -> bool {
        return self.0 >= rhs.0;
    }

    /// Returns the ordering of `self` with respect to `rhs`.
    ///
    /// This method works as a `const` capable alternative to `Ord::cmp(self, rhs)`.
    pub const fn cmp(self, rhs: Self) -> core::cmp::Ordering {
        if self.0 < rhs.0 {
            core::cmp::Ordering::Less
        } else if self.0 > rhs.0 {
            core::cmp::Ordering::Greater
        } else {
            core::cmp::Ordering::Equal
        }
    }

    /// Returns the minimum of `self` and `other`.
    ///
    /// This method works as a `const` capable alternative to `Ord::min(self, rhs)`.
    pub const fn min(self, other: Self) -> Self {
        if self.0 < other.0 {
            self
        } else {
            other
        }
    }

    /// Returns the maximum of `self` and `other`.
    ///
    /// This method works as a `const` capable alternative to `Ord::max(self, rhs)`.
    pub const fn max(self, other: Self) -> Self {
        if self.0 > other.0 {
            self
        } else {
            other
        }
    }

    /// Returns the value of this integer, clamped between `min` and `max`.
    ///
    /// This method works as a `const` capable alternative to `Ord::clamp(self, min, max)`.
    pub const fn clamp(self, min: Self, max: Self) -> Self {
        debug_assert!(
            min.0 <= max.0,
            "Attempt to clamp with minimum value greater than maximum value"
        );
        Self::min(Self::max(self, min), max)
    }

    /// Checked integer addition. Returns `None` if overflow occurred.
    pub const fn checked_add(self, rhs: Self) -> Option<Self> {
        match self.0.checked_add(rhs.0) {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked integer subtraction. Returns `None` if overflow occurred.
    pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
        match self.0.checked_sub(rhs.0) {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked integer multiplication. Returns `None` if overflow occurred.
    pub const fn checked_mul(self, rhs: Self) -> Option<Self> {
        match self.0.checked_mul(rhs.0) {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked integer division. Returns `None` if `rhs` is zero, or the division resulted in
    /// overflow.
    pub const fn checked_div(self, rhs: Self) -> Option<Self> {
        match self.0.checked_div(rhs.0) {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked euclidian division. Returns `None` if `rhs` is zero or `self.div_euclid(rhs)`
    /// would have resulted in overflow.
    pub const fn checked_div_euclid(self, rhs: Self) -> Option<Self> {
        match self.0.checked_div_euclid(rhs.0) {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked integer remainder. Returns `None` if `rhs` is zero or the remainder would have
    /// resulted in overflow.
    pub const fn checked_rem(self, rhs: Self) -> Option<Self> {
        match self.0.checked_rem(rhs.0) {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked euclidian remainder. Returns `None` if `rhs` is zero or `self.rem_euclid(rhs)`
    /// would have resulted in overflow.
    pub const fn checked_rem_euclid(self, rhs: Self) -> Option<Self> {
        match self.0.checked_rem_euclid(rhs.0) {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked integer negation. Returns `None` if the negation resulted in overflow.
    pub const fn checked_neg(self) -> Option<Self> {
        match self.0.checked_neg() {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked left bit shift. Returns `None` if `rhs` is greater than or equal to
    /// [`Self::BITS`].
    pub const fn checked_shl(self, rhs: u32) -> Option<Self> {
        if rhs >= Self::BITS {
            None
        } else {
            Some(self.shl(rhs))
        }
    }

    /// Checked right bit shift. Returns `None` if `rhs` is greater than or equal to
    /// [`Self::BITS`].
    pub const fn checked_shr(self, rhs: u32) -> Option<Self> {
        if rhs >= Self::BITS {
            None
        } else {
            Some(self.shr(rhs))
        }
    }

    /// Checked exponentiation. Returns `None` if the exponentiation resulted in overflow.
    pub const fn checked_pow(self, exp: u32) -> Option<Self> {
        match self.0.checked_pow(exp) {
            Some(val) => Self::new(val),
            None => None,
        }
    }

    /// Checked logarithm, rounded down. Returns `None` if `self` is less than or equal zero,
    /// or if `base` is less than 2.
    pub const fn checked_ilog(self, base: Self) -> Option<u32> {
        self.0.checked_ilog(base.0)
    }

    /// Checked base 2 logarithm, rounded down. Returns `None` if self is less than or equal to
    /// zero.
    pub const fn checked_ilog2(self) -> Option<u32> {
        self.0.checked_ilog2()
    }

    /// Checked base 10 logarithm, rounded down. Returns `None` if self is less than or equal
    /// to zero.
    pub const fn checked_ilog10(self) -> Option<u32> {
        self.0.checked_ilog10()
    }

    /// Satruating integer addition. The result of the addition is clamped between
    /// [`MIN`](Self::MIN) and [`MAX`](Self::MAX).
    pub const fn saturating_add(self, rhs: Self) -> Self {
        Self::new_saturating(self.0.saturating_add(rhs.0))
    }

    /// Saturating integer subtraction. The result of the subtraction is clamped between
    /// [`MIN`](Self::MIN) and [`MAX`](Self::MAX).
    pub const fn saturating_sub(self, rhs: Self) -> Self {
        Self::new_saturating(self.0.saturating_sub(rhs.0))
    }

    /// Saturating integer multiplication. The result of the multiplication is clamped between
    /// [`MIN`](Self::MIN) and [`MAX`](Self::MAX).
    pub const fn saturating_mul(self, rhs: Self) -> Self {
        Self::new_saturating(self.0.saturating_mul(rhs.0))
    }

    /// Saturating exponentiation. The result of the exponentiation is clammed between
    /// [`MIN`](Self::MIN) and [`MAX`](Self::MAX).
    pub const fn saturating_pow(self, exp: u32) -> Self {
        Self::new_saturating(self.0.saturating_pow(exp))
    }

    /// Wrapping (modular) addition. An overflowing result is wrapped around the bounds of this
    /// integer type.
    pub const fn wrapping_add(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.wrapping_add(rhs.0))
    }

    /// Wrapping (modular) subtraction. An overflowing result is wrapped around the bounds of
    /// this integer type.
    pub const fn wrapping_sub(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.wrapping_sub(rhs.0))
    }

    /// Wrapping (modular) multiplication. An overflowing result is wrapped around the bounds of
    /// this integer type.
    pub const fn wrapping_mul(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.wrapping_mul(rhs.0))
    }

    /// Wrapping (modular) division. An overflowing result is wrapped around the bounds of this
    /// integer type.
    pub const fn wrapping_div(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.wrapping_div(rhs.0))
    }

    /// Wrapping (modular) euclidian division. An overflowing result is wrapped around the
    /// bounds of this integer type.
    pub const fn wrapping_div_euclid(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.wrapping_div_euclid(rhs.0))
    }

    /// Wrapping (modular) remainder. An overflowing result is wrapped around the bounds of
    /// this integer type.
    pub const fn wrapping_rem(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.wrapping_rem(rhs.0))
    }

    /// Wrapping (modular) euclidian remainder. An overflowing result is wrapped around the
    /// bounds of this integer type.
    pub const fn wrapping_rem_euclid(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.wrapping_rem_euclid(rhs.0))
    }

    /// Wrapping (modular) negation. An overflowing result is wrapped around the bounds of this
    /// integer type.
    pub const fn wrapping_neg(self) -> Self {
        Self::new_wrapping(!self.0 + 1)
    }

    /// Wrapping (modular) left bit shift. The value is left shifted `rhs % Self::BITS` bits.
    pub const fn wrapping_shl(self, rhs: u32) -> Self {
        Self::new_wrapping(self.0 << (rhs % Self::BITS))
    }

    /// Wrapping (modular) right bit shift. The value is right shifted `rhs % Self::BITS` bits.
    pub const fn wrapping_shr(self, rhs: u32) -> Self {
        Self::new_wrapping((self.0 & Self::MASK) >> (rhs % Self::BITS))
    }

    /// Wrapping (modular) exponentiation. An overflowing result is wrapped around the bounds
    /// of this integer type.
    pub const fn wrapping_pow(self, exp: u32) -> Self {
        Self::new_wrapping(self.0.wrapping_pow(exp))
    }

    /// Wrapping (modular) aboslute value. An overflowing result is wrapped around the bounds
    /// of this integer type.
    pub const fn wrapping_abs(self) -> Self {
        if self.is_negative() {
            self.wrapping_neg()
        } else {
            self
        }
    }

    /// Calculates `self + rhs`, returning a tuple of the result and a `bool` indicating if
    /// overflow occurred.
    pub const fn overflowing_add(self, rhs: Self) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_add(rhs.0))
    }

    /// Calculates `self - rhs`, returning a tuple of the result and a `bool` indicating if
    /// overflow occurred.
    pub const fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_sub(rhs.0))
    }

    /// Calculates `self * rhs`, returning a tuple of the result and a `bool` indicating if
    /// overflow occurred.
    pub const fn overflowing_mul(self, rhs: Self) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_mul(rhs.0))
    }

    /// Calculates `self / rhs`, returning a tuple of the result and a `bool` indicating if
    /// overflow occurred.
    pub const fn overflowing_div(self, rhs: Self) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_div(rhs.0))
    }

    /// Calculates `self.div_euclid(rhs)`, returning a tuple of the result and a `bool`
    /// indicating if overflow occurred.
    pub const fn overflowing_div_euclid(self, rhs: Self) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_div_euclid(rhs.0))
    }

    /// Calculates `self % rhs`, returning a tuple of the result and a `bool` indicating if
    /// overflow occurred.
    pub const fn overflowing_rem(self, rhs: Self) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_rem(rhs.0))
    }

    /// Calculates `self.rem_euclid(rhs)`, returning a tuple of the result and a `bool`
    /// indicating if overflow occurred.
    pub const fn overflowing_rem_euclid(self, rhs: Self) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_rem_euclid(rhs.0))
    }

    /// Calculates `-self`, returning a tuple of the result and a `bool` indicating if overflow
    /// occurred.
    pub const fn overflowing_neg(self) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_neg())
    }

    /// Calculates `self << rhs`, returning a tuple of the result and a `bool` indicating if
    /// `rhs` was greater than or equal to [`Self::BITS`].
    pub const fn overflowing_shl(self, rhs: u32) -> (Self, bool) {
        (self.wrapping_shl(rhs), rhs >= Self::BITS)
    }

    /// Calculates `self >> rhs`, returning a tuple of the result and a `bool` indicating if
    /// `rhs` was greater than or equal to [`Self::BITS`].
    pub const fn overflowing_shr(self, rhs: u32) -> (Self, bool) {
        (self.wrapping_shr(rhs), rhs >= Self::BITS)
    }

    /// Calculates `self.pow(exp)`, returning a tuple of the result and a `bool` indicating if
    /// overflow occurred.
    pub const fn overflowing_pow(self, exp: u32) -> (Self, bool) {
        Self::new_overflowing_impl(self.0.overflowing_pow(exp))
    }

    /// Calculates `self.abs()`, returning a tuple of the result and a `bool` indicating if
    /// overflow occurred.
    pub const fn overflowing_abs(self) -> (Self, bool) {
        if self.is_negative() {
            self.overflowing_neg()
        } else {
            (self, false)
        }
    }

    /// Raises `self` to the power of `exp`, using exponentiation by squaring.
    pub const fn pow(self, exp: u32) -> Self {
        let val = self.0.pow(exp);
        debug_assert!(
            val <= Self::MAX.0 && val >= Self::MIN.0,
            "attempt to exponentiate with overflow"
        );
        Self::new_wrapping(val)
    }

    /// Calculates the quotient of Euclidian division `self` by `rhs`.
    ///
    /// This computes the integer `q` such that `self = q * rhs + r`, with `r =
    /// self.rem_euclid(rhs)` and `0 <= r < rhs.abs()`.
    ///
    /// In other words, the result is `self / rhs`, rounded to the integer `q` such that `self
    /// >= q * rhs`. If `self > 0`, this is equal to round towards zero. If `self < 0`, this is
    /// equal to rounds towards +/- infinity.
    pub const fn div_euclid(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.div_euclid(rhs.0))
    }

    /// Calculates the non-negative remainder `self (mod rhs)`.
    ///
    /// This is done as if by the Euclidian division algorithm - given `r =
    /// self.rem_euclid(rhs)`, `self = self * self.div_euclid(rhs) + r`, and `0 <= r <
    /// r.abs()`.
    pub const fn rem_euclid(self, rhs: Self) -> Self {
        Self::new_wrapping(self.0.div_euclid(rhs.0))
    }

    /// Computes the absolute value of this integer.
    pub const fn abs(self) -> Self {
        debug_assert!(
            self.0 != Self::MIN.0,
            "attempt to calculate absolute value with overflow"
        );
        self.wrapping_abs()
    }

    /// Computes the absolute difference between `self` and `rhs`.
    ///
    /// This is equivalent to `(self - rhs).abs()` without the posibility of intermediate
    /// overflow.
    pub const fn abs_diff(self, rhs: Self) -> Aint<u128, WIDTH> {
        Aint(self.0.abs_diff(rhs.0))
    }

    /// Converts a string slice in a given base (`radix`) to an integer.
    ///
    /// The string is expected to be an optional `+` or `-` sign followed by digits. Leading
    /// and trailing whitespace will result in an error. Digits are a subset of the following
    /// characters depending on the `radix`.
    ///
    /// * `0-9`
    /// * `a-z`
    /// * `A-Z`
    ///
    /// # Panics
    /// This function panics if `radix` is not in the range from 2 to 36.
    pub fn from_str_radix(src: &str, radix: u32) -> Result<Self, core::num::ParseIntError> {
        Self::new(i128::from_str_radix(src, radix)?).ok_or_else(make_parse_int_err)
    }
}

impl Aint<i128, 72> {
    /// Reverses the byte order of the integer.
    pub const fn swap_bytes(self) -> Self {
        Self((self.0.swap_bytes() >> 56) & Self::MASK)
    }

    /// Converts an integer from big endian to the target's endianness.
    ///
    /// On big endian this is a no-op. On little endian the bytes are swapped.
    pub const fn from_be(x: Self) -> Self {
        #[cfg(target_endian = "big")]
        {
            x
        }
        #[cfg(target_endian = "little")]
        {
            x.swap_bytes()
        }
    }

    /// Converts an integer from little endian to the target's endianness.
    ///
    /// On little endian this is a no-op. On big endian the bytes are swapped.
    pub const fn from_le(x: Self) -> Self {
        #[cfg(target_endian = "little")]
        {
            x
        }
        #[cfg(target_endian = "big")]
        {
            x.swap_bytes()
        }
    }

    /// Converts `self` to big endian from the target's endianness.
    ///
    /// On big endian this is a no-op. On little endian the bytes are swapped.
    pub const fn to_be(self) -> Self {
        #[cfg(target_endian = "big")]
        {
            self
        }
        #[cfg(target_endian = "little")]
        {
            self.swap_bytes()
        }
    }

    /// Converts `self` to little endian from the target's endianness.
    ///
    /// On little endian this is a no-op. On big endian the bytes are swapped.
    pub const fn to_le(self) -> Self {
        #[cfg(target_endian = "little")]
        {
            self
        }
        #[cfg(target_endian = "big")]
        {
            self.swap_bytes()
        }
    }

    /// Creates an integer value from its memory representation as a byte array in bit endian.
    pub const fn from_be_bytes(bytes: [u8; 9]) -> Self {
        Self(i128::from_be_bytes([
            0, 0, 0, 0, 0, 0, 0, bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5],
            bytes[6], bytes[7], bytes[8],
        ]))
    }

    /// Creates an integer value from its memory representation as a byte array in little endian.
    pub const fn from_le_bytes(bytes: [u8; 9]) -> Self {
        Self(i128::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            bytes[8], 0, 0, 0, 0, 0, 0, 0,
        ]))
    }

    /// Creates an integer value from its memory representation as a byte array in native
    /// endianness.
    ///
    /// As the target platform's native endianness is used, portable code likely wants to
    /// use [`from_be_bytes`](Self::from_be_bytes) or [`from_le_bytes`](Self::from_le_bytes),
    /// as appropriate instead.
    pub const fn from_ne_bytes(bytes: [u8; 9]) -> Self {
        #[cfg(target_endian = "big")]
        {
            Self::from_be_bytes(bytes)
        }
        #[cfg(target_endian = "little")]
        {
            Self::from_le_bytes(bytes)
        }
    }

    /// Return the memory representation of this integer as a byte array in big-endian byte
    /// order.
    pub const fn to_be_bytes(self) -> [u8; 9] {
        let tmp = self.0.to_be_bytes();
        [
            tmp[7], tmp[8], tmp[9], tmp[10], tmp[11], tmp[12], tmp[13], tmp[14], tmp[15],
        ]
    }

    /// Return the memory representation of this integer as a byte array in little-endian
    /// byte order.
    pub const fn to_le_bytes(self) -> [u8; 9] {
        let tmp = self.0.to_le_bytes();
        [
            tmp[0], tmp[1], tmp[2], tmp[3], tmp[4], tmp[5], tmp[6], tmp[7], tmp[8],
        ]
    }

    /// Return the memory representation of this integer as a byte array in native byte
    /// order.
    ///
    /// As the target platform's native endianness is used, portable code likely wants to
    /// use [`to_be_bytes`](Self::to_be_bytes) or [`to_le_bytes`](Self::to_le_bytes), as
    /// appropriate instead.
    pub const fn to_ne_bytes(self) -> [u8; 9] {
        #[cfg(target_endian = "big")]
        {
            self.to_be_bytes()
        }
        #[cfg(target_endian = "little")]
        {
            self.to_le_bytes()
        }
    }
}

impl Aint<i128, 80> {
    /// Reverses the byte order of the integer.
    pub const fn swap_bytes(self) -> Self {
        Self((self.0.swap_bytes() >> 48) & Self::MASK)
    }

    /// Converts an integer from big endian to the target's endianness.
    ///
    /// On big endian this is a no-op. On little endian the bytes are swapped.
    pub const fn from_be(x: Self) -> Self {
        #[cfg(target_endian = "big")]
        {
            x
        }
        #[cfg(target_endian = "little")]
        {
            x.swap_bytes()
        }
    }

    /// Converts an integer from little endian to the target's endianness.
    ///
    /// On little endian this is a no-op. On big endian the bytes are swapped.
    pub const fn from_le(x: Self) -> Self {
        #[cfg(target_endian = "little")]
        {
            x
        }
        #[cfg(target_endian = "big")]
        {
            x.swap_bytes()
        }
    }

    /// Converts `self` to big endian from the target's endianness.
    ///
    /// On big endian this is a no-op. On little endian the bytes are swapped.
    pub const fn to_be(self) -> Self {
        #[cfg(target_endian = "big")]
        {
            self
        }
        #[cfg(target_endian = "little")]
        {
            self.swap_bytes()
        }
    }

    /// Converts `self` to little endian from the target's endianness.
    ///
    /// On little endian this is a no-op. On big endian the bytes are swapped.
    pub const fn to_le(self) -> Self {
        #[cfg(target_endian = "little")]
        {
            self
        }
        #[cfg(target_endian = "big")]
        {
            self.swap_bytes()
        }
    }

    /// Creates an integer value from its memory representation as a byte array in bit endian.
    pub const fn from_be_bytes(bytes: [u8; 10]) -> Self {
        Self(i128::from_be_bytes([
            0, 0, 0, 0, 0, 0, bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6],
            bytes[7], bytes[8], bytes[9],
        ]))
    }

    /// Creates an integer value from its memory representation as a byte array in little endian.
    pub const fn from_le_bytes(bytes: [u8; 10]) -> Self {
        Self(i128::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            bytes[8], bytes[9], 0, 0, 0, 0, 0, 0,
        ]))
    }

    /// Creates an integer value from its memory representation as a byte array in native
    /// endianness.
    ///
    /// As the target platform's native endianness is used, portable code likely wants to
    /// use [`from_be_bytes`](Self::from_be_bytes) or [`from_le_bytes`](Self::from_le_bytes),
    /// as appropriate instead.
    pub const fn from_ne_bytes(bytes: [u8; 10]) -> Self {
        #[cfg(target_endian = "big")]
        {
            Self::from_be_bytes(bytes)
        }
        #[cfg(target_endian = "little")]
        {
            Self::from_le_bytes(bytes)
        }
    }

    /// Return the memory representation of this integer as a byte array in big-endian byte
    /// order.
    pub const fn to_be_bytes(self) -> [u8; 10] {
        let tmp = self.0.to_be_bytes();
        [
            tmp[6], tmp[7], tmp[8], tmp[9], tmp[10], tmp[11], tmp[12], tmp[13], tmp[14], tmp[15],
        ]
    }

    /// Return the memory representation of this integer as a byte array in little-endian
    /// byte order.
    pub const fn to_le_bytes(self) -> [u8; 10] {
        let tmp = self.0.to_le_bytes();
        [
            tmp[0], tmp[1], tmp[2], tmp[3], tmp[4], tmp[5], tmp[6], tmp[7], tmp[8], tmp[9],
        ]
    }

    /// Return the memory representation of this integer as a byte array in native byte
    /// order.
    ///
    /// As the target platform's native endianness is used, portable code likely wants to
    /// use [`to_be_bytes`](Self::to_be_bytes) or [`to_le_bytes`](Self::to_le_bytes), as
    /// appropriate instead.
    pub const fn to_ne_bytes(self) -> [u8; 10] {
        #[cfg(target_endian = "big")]
        {
            self.to_be_bytes()
        }
        #[cfg(target_endian = "little")]
        {
            self.to_le_bytes()
        }
    }
}

impl Aint<i128, 88> {
    /// Reverses the byte order of the integer.
    pub const fn swap_bytes(self) -> Self {
        Self((self.0.swap_bytes() >> 40) & Self::MASK)
    }

    /// Converts an integer from big endian to the target's endianness.
    ///
    /// On big endian this is a no-op. On little endian the bytes are swapped.
    pub const fn from_be(x: Self) -> Self {
        #[cfg(target_endian = "big")]
        {
            x
        }
        #[cfg(target_endian = "little")]
        {
            x.swap_bytes()
        }
    }

    /// Converts an integer from little endian to the target's endianness.
    ///
    /// On little endian this is a no-op. On big endian the bytes are swapped.
    pub const fn from_le(x: Self) -> Self {
        #[cfg(target_endian = "little")]
        {
            x
        }
        #[cfg(target_endian = "big")]
        {
            x.swap_bytes()
        }
    }

    /// Converts `self` to big endian from the target's endianness.
    ///
    /// On big endian this is a no-op. On little endian the bytes are swapped.
    pub const fn to_be(self) -> Self {
        #[cfg(target_endian = "big")]
        {
            self
        }
        #[cfg(target_endian = "little")]
        {
            self.swap_bytes()
        }
    }

    /// Converts `self` to little endian from the target's endianness.
    ///
    /// On little endian this is a no-op. On big endian the bytes are swapped.
    pub const fn to_le(self) -> Self {
        #[cfg(target_endian = "little")]
        {
            self
        }
        #[cfg(target_endian = "big")]
        {
            self.swap_bytes()
        }
    }

    /// Creates an integer value from its memory representation as a byte array in bit endian.
    pub const fn from_be_bytes(bytes: [u8; 11]) -> Self {
        Self(i128::from_be_bytes([
            0, 0, 0, 0, 0, bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6],
            bytes[7], bytes[8], bytes[9], bytes[10],
        ]))
    }

    /// Creates an integer value from its memory representation as a byte array in little endian.
    pub const fn from_le_bytes(bytes: [u8; 11]) -> Self {
        Self(i128::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            bytes[8], bytes[9], bytes[10], 0, 0, 0, 0, 0,
        ]))
    }

    /// Creates an integer value from its memory representation as a byte array in native
    /// endianness.
    ///
    /// As the target platform's native endianness is used, portable code likely wants to
    /// use [`from_be_bytes`](Self::from_be_bytes) or [`from_le_bytes`](Self::from_le_bytes),
    /// as appropriate instead.
    pub const fn from_ne_bytes(bytes: [u8; 11]) -> Self {
        #[cfg(target_endian = "big")]
        {
            Self::from_be_bytes(bytes)
        }
        #[cfg(target_endian = "little")]
        {
            Self::from_le_bytes(bytes)
        }
    }

    /// Return the memory representation of this integer as a byte array in big-endian byte
    /// order.
    pub const fn to_be_bytes(self) -> [u8; 11] {
        let tmp = self.0.to_be_bytes();
        [
            tmp[5], tmp[6], tmp[7], tmp[8], tmp[9], tmp[10], tmp[11], tmp[12], tmp[13], tmp[14],
            tmp[15],
        ]
    }

    /// Return the memory representation of this integer as a byte array in little-endian
    /// byte order.
    pub const fn to_le_bytes(self) -> [u8; 11] {
        let tmp = self.0.to_le_bytes();
        [
            tmp[0], tmp[1], tmp[2], tmp[3], tmp[4], tmp[5], tmp[6], tmp[7], tmp[8], tmp[9], tmp[10],
        ]
    }

    /// Return the memory representation of this integer as a byte array in native byte
    /// order.
    ///
    /// As the target platform's native endianness is used, portable code likely wants to
    /// use [`to_be_bytes`](Self::to_be_bytes) or [`to_le_bytes`](Self::to_le_bytes), as
    /// appropriate instead.
    pub const fn to_ne_bytes(self) -> [u8; 11] {
        #[cfg(target_endian = "big")]
        {
            self.to_be_bytes()
        }
        #[cfg(target_endian = "little")]
        {
            self.to_le_bytes()
        }
    }
}

impl Aint<i128, 96> {
    /// Reverses the byte order of the integer.
    pub const fn swap_bytes(self) -> Self {
        Self((self.0.swap_bytes() >> 32) & Self::MASK)
    }

    /// Converts an integer from big endian to the target's endianness.
    ///
    /// On big endian this is a no-op. On little endian the bytes are swapped.
    pub const fn from_be(x: Self) -> Self {
        #[cfg(target_endian = "big")]
        {
            x
        }
        #[cfg(target_endian = "little")]
        {
            x.swap_bytes()
        }
    }

    /// Converts an integer from little endian to the target's endianness.
    ///
    /// On little endian this is a no-op. On big endian the bytes are swapped.
    pub const fn from_le(x: Self) -> Self {
        #[cfg(target_endian = "little")]
        {
            x
        }
        #[cfg(target_endian = "big")]
        {
            x.swap_bytes()
        }
    }

    /// Converts `self` to big endian from the target's endianness.
    ///
    /// On big endian this is a no-op. On little endian the bytes are swapped.
    pub const fn to_be(self) -> Self {
        #[cfg(target_endian = "big")]
        {
            self
        }
        #[cfg(target_endian = "little")]
        {
            self.swap_bytes()
        }
    }

    /// Converts `self` to little endian from the target's endianness.
    ///
    /// On little endian this is a no-op. On big endian the bytes are swapped.
    pub const fn to_le(self) -> Self {
        #[cfg(target_endian = "little")]
        {
            self
        }
        #[cfg(target_endian = "big")]
        {
            self.swap_bytes()
        }
    }

    /// Creates an integer value from its memory representation as a byte array in bit endian.
    pub const fn from_be_bytes(bytes: [u8; 12]) -> Self {
        Self(i128::from_be_bytes([
            0, 0, 0, 0, bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6],
            bytes[7], bytes[8], bytes[9], bytes[10], bytes[11],
        ]))
    }

    /// Creates an integer value from its memory representation as a byte array in little endian.
    pub const fn from_le_bytes(bytes: [u8; 12]) -> Self {
        Self(i128::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            bytes[8], bytes[9], bytes[10], bytes[11], 0, 0, 0, 0,
        ]))
    }

    /// Creates an integer value from its memory representation as a byte array in native
    /// endianness.
    ///
    /// As the target platform's native endianness is used, portable code likely wants to
    /// use [`from_be_bytes`](Self::from_be_bytes) or [`from_le_bytes`](Self::from_le_bytes),
    /// as appropriate instead.
    pub const fn from_ne_bytes(bytes: [u8; 12]) -> Self {
        #[cfg(target_endian = "big")]
        {
            Self::from_be_bytes(bytes)
        }
        #[cfg(target_endian = "little")]
        {
            Self::from_le_bytes(bytes)
        }
    }

    /// Return the memory representation of this integer as a byte array in big-endian byte
    /// order.
    pub const fn to_be_bytes(self) -> [u8; 12] {
        let tmp = self.0.to_be_bytes();
        [
            tmp[4], tmp[5], tmp[6], tmp[7], tmp[8], tmp[9], tmp[10], tmp[11], tmp[12], tmp[13],
            tmp[14], tmp[15],
        ]
    }

    /// Return the memory representation of this integer as a byte array in little-endian
    /// byte order.
    pub const fn to_le_bytes(self) -> [u8; 12] {
        let tmp = self.0.to_le_bytes();
        [
            tmp[0], tmp[1], tmp[2], tmp[3], tmp[4], tmp[5], tmp[6], tmp[7], tmp[8], tmp[9],
            tmp[10], tmp[11],
        ]
    }

    /// Return the memory representation of this integer as a byte array in native byte
    /// order.
    ///
    /// As the target platform's native endianness is used, portable code likely wants to
    /// use [`to_be_bytes`](Self::to_be_bytes) or [`to_le_bytes`](Self::to_le_bytes), as
    /// appropriate instead.
    pub const fn to_ne_bytes(self) -> [u8; 12] {
        #[cfg(target_endian = "big")]
        {
            self.to_be_bytes()
        }
        #[cfg(target_endian = "little")]
        {
            self.to_le_bytes()
        }
    }
}

impl Aint<i128, 104> {
    /// Reverses the byte order of the integer.
    pub const fn swap_bytes(self) -> Self {
        Self((self.0.swap_bytes() >> 24) & Self::MASK)
    }

    /// Converts an integer from big endian to the target's endianness.
    ///
    /// On big endian this is a no-op. On little endian the bytes are swapped.
    pub const fn from_be(x: Self) -> Self {
        #[cfg(target_endian = "big")]
        {
            x
        }
        #[cfg(target_endian = "little")]
        {
            x.swap_bytes()
        }
    }

    /// Converts an integer from little endian to the target's endianness.
    ///
    /// On little endian this is a no-op. On big endian the bytes are swapped.
    pub const fn from_le(x: Self) -> Self {
        #[cfg(target_endian = "little")]
        {
            x
        }
        #[cfg(target_endian = "big")]
        {
            x.swap_bytes()
        }
    }

    /// Converts `self` to big endian from the target's endianness.
    ///
    /// On big endian this is a no-op. On little endian the bytes are swapped.
    pub const fn to_be(self) -> Self {
        #[cfg(target_endian = "big")]
        {
            self
        }
        #[cfg(target_endian = "little")]
        {
            self.swap_bytes()
        }
    }

    /// Converts `self` to little endian from the target's endianness.
    ///
    /// On little endian this is a no-op. On big endian the bytes are swapped.
    pub const fn to_le(self) -> Self {
        #[cfg(target_endian = "little")]
        {
            self
        }
        #[cfg(target_endian = "big")]
        {
            self.swap_bytes()
        }
    }

    /// Creates an integer value from its memory representation as a byte array in bit endian.
    pub const fn from_be_bytes(bytes: [u8; 13]) -> Self {
        Self(i128::from_be_bytes([
            0, 0, 0, bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6],
            bytes[7], bytes[8], bytes[9], bytes[10], bytes[11], bytes[12],
        ]))
    }

    /// Creates an integer value from its memory representation as a byte array in little endian.
    pub const fn from_le_bytes(bytes: [u8; 13]) -> Self {
        Self(i128::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            bytes[8], bytes[9], bytes[10], bytes[11], bytes[12], 0, 0, 0,
        ]))
    }

    /// Creates an integer value from its memory representation as a byte array in native
    /// endianness.
    ///
    /// As the target platform's native endianness is used, portable code likely wants to
    /// use [`from_be_bytes`](Self::from_be_bytes) or [`from_le_bytes`](Self::from_le_bytes),
    /// as appropriate instead.
    pub const fn from_ne_bytes(bytes: [u8; 13]) -> Self {
        #[cfg(target_endian = "big")]
        {
            Self::from_be_bytes(bytes)
        }
        #[cfg(target_endian = "little")]
        {
            Self::from_le_bytes(bytes)
        }
    }

    /// Return the memory representation of this integer as a byte array in big-endian byte
    /// order.
    pub const fn to_be_bytes(self) -> [u8; 13] {
        let tmp = self.0.to_be_bytes();
        [
            tmp[3], tmp[4], tmp[5], tmp[6], tmp[7], tmp[8], tmp[9], tmp[10], tmp[11], tmp[12],
            tmp[13], tmp[14], tmp[15],
        ]
    }

    /// Return the memory representation of this integer as a byte array in little-endian
    /// byte order.
    pub const fn to_le_bytes(self) -> [u8; 13] {
        let tmp = self.0.to_le_bytes();
        [
            tmp[0], tmp[1], tmp[2], tmp[3], tmp[4], tmp[5], tmp[6], tmp[7], tmp[8], tmp[9],
            tmp[10], tmp[11], tmp[12],
        ]
    }

    /// Return the memory representation of this integer as a byte array in native byte
    /// order.
    ///
    /// As the target platform's native endianness is used, portable code likely wants to
    /// use [`to_be_bytes`](Self::to_be_bytes) or [`to_le_bytes`](Self::to_le_bytes), as
    /// appropriate instead.
    pub const fn to_ne_bytes(self) -> [u8; 13] {
        #[cfg(target_endian = "big")]
        {
            self.to_be_bytes()
        }
        #[cfg(target_endian = "little")]
        {
            self.to_le_bytes()
        }
    }
}

impl Aint<i128, 112> {
    /// Reverses the byte order of the integer.
    pub const fn swap_bytes(self) -> Self {
        Self((self.0.swap_bytes() >> 16) & Self::MASK)
    }

    /// Converts an integer from big endian to the target's endianness.
    ///
    /// On big endian this is a no-op. On little endian the bytes are swapped.
    pub const fn from_be(x: Self) -> Self {
        #[cfg(target_endian = "big")]
        {
            x
        }
        #[cfg(target_endian = "little")]
        {
            x.swap_bytes()
        }
    }

    /// Converts an integer from little endian to the target's endianness.
    ///
    /// On little endian this is a no-op. On big endian the bytes are swapped.
    pub const fn from_le(x: Self) -> Self {
        #[cfg(target_endian = "little")]
        {
            x
        }
        #[cfg(target_endian = "big")]
        {
            x.swap_bytes()
        }
    }

    /// Converts `self` to big endian from the target's endianness.
    ///
    /// On big endian this is a no-op. On little endian the bytes are swapped.
    pub const fn to_be(self) -> Self {
        #[cfg(target_endian = "big")]
        {
            self
        }
        #[cfg(target_endian = "little")]
        {
            self.swap_bytes()
        }
    }

    /// Converts `self` to little endian from the target's endianness.
    ///
    /// On little endian this is a no-op. On big endian the bytes are swapped.
    pub const fn to_le(self) -> Self {
        #[cfg(target_endian = "little")]
        {
            self
        }
        #[cfg(target_endian = "big")]
        {
            self.swap_bytes()
        }
    }

    /// Creates an integer value from its memory representation as a byte array in bit endian.
    pub const fn from_be_bytes(bytes: [u8; 14]) -> Self {
        Self(i128::from_be_bytes([
            0, 0, bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            bytes[8], bytes[9], bytes[10], bytes[11], bytes[12], bytes[13],
        ]))
    }

    /// Creates an integer value from its memory representation as a byte array in little endian.
    pub const fn from_le_bytes(bytes: [u8; 14]) -> Self {
        Self(i128::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            bytes[8], bytes[9], bytes[10], bytes[11], bytes[12], bytes[13], 0, 0,
        ]))
    }

    /// Creates an integer value from its memory representation as a byte array in native
    /// endianness.
    ///
    /// As the target platform's native endianness is used, portable code likely wants to
    /// use [`from_be_bytes`](Self::from_be_bytes) or [`from_le_bytes`](Self::from_le_bytes),
    /// as appropriate instead.
    pub const fn from_ne_bytes(bytes: [u8; 14]) -> Self {
        #[cfg(target_endian = "big")]
        {
            Self::from_be_bytes(bytes)
        }
        #[cfg(target_endian = "little")]
        {
            Self::from_le_bytes(bytes)
        }
    }

    /// Return the memory representation of this integer as a byte array in big-endian byte
    /// order.
    pub const fn to_be_bytes(self) -> [u8; 14] {
        let tmp = self.0.to_be_bytes();
        [
            tmp[2], tmp[3], tmp[4], tmp[5], tmp[6], tmp[7], tmp[8], tmp[9], tmp[10], tmp[11],
            tmp[12], tmp[13], tmp[14], tmp[15],
        ]
    }

    /// Return the memory representation of this integer as a byte array in little-endian
    /// byte order.
    pub const fn to_le_bytes(self) -> [u8; 14] {
        let tmp = self.0.to_le_bytes();
        [
            tmp[0], tmp[1], tmp[2], tmp[3], tmp[4], tmp[5], tmp[6], tmp[7], tmp[8], tmp[9],
            tmp[10], tmp[11], tmp[12], tmp[13],
        ]
    }

    /// Return the memory representation of this integer as a byte array in native byte
    /// order.
    ///
    /// As the target platform's native endianness is used, portable code likely wants to
    /// use [`to_be_bytes`](Self::to_be_bytes) or [`to_le_bytes`](Self::to_le_bytes), as
    /// appropriate instead.
    pub const fn to_ne_bytes(self) -> [u8; 14] {
        #[cfg(target_endian = "big")]
        {
            self.to_be_bytes()
        }
        #[cfg(target_endian = "little")]
        {
            self.to_le_bytes()
        }
    }
}

impl Aint<i128, 120> {
    /// Reverses the byte order of the integer.
    pub const fn swap_bytes(self) -> Self {
        Self((self.0.swap_bytes() >> 8) & Self::MASK)
    }

    /// Converts an integer from big endian to the target's endianness.
    ///
    /// On big endian this is a no-op. On little endian the bytes are swapped.
    pub const fn from_be(x: Self) -> Self {
        #[cfg(target_endian = "big")]
        {
            x
        }
        #[cfg(target_endian = "little")]
        {
            x.swap_bytes()
        }
    }

    /// Converts an integer from little endian to the target's endianness.
    ///
    /// On little endian this is a no-op. On big endian the bytes are swapped.
    pub const fn from_le(x: Self) -> Self {
        #[cfg(target_endian = "little")]
        {
            x
        }
        #[cfg(target_endian = "big")]
        {
            x.swap_bytes()
        }
    }

    /// Converts `self` to big endian from the target's endianness.
    ///
    /// On big endian this is a no-op. On little endian the bytes are swapped.
    pub const fn to_be(self) -> Self {
        #[cfg(target_endian = "big")]
        {
            self
        }
        #[cfg(target_endian = "little")]
        {
            self.swap_bytes()
        }
    }

    /// Converts `self` to little endian from the target's endianness.
    ///
    /// On little endian this is a no-op. On big endian the bytes are swapped.
    pub const fn to_le(self) -> Self {
        #[cfg(target_endian = "little")]
        {
            self
        }
        #[cfg(target_endian = "big")]
        {
            self.swap_bytes()
        }
    }

    /// Creates an integer value from its memory representation as a byte array in bit endian.
    pub const fn from_be_bytes(bytes: [u8; 15]) -> Self {
        Self(i128::from_be_bytes([
            0, bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            bytes[8], bytes[9], bytes[10], bytes[11], bytes[12], bytes[13], bytes[14],
        ]))
    }

    /// Creates an integer value from its memory representation as a byte array in little endian.
    pub const fn from_le_bytes(bytes: [u8; 15]) -> Self {
        Self(i128::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            bytes[8], bytes[9], bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], 0,
        ]))
    }

    /// Creates an integer value from its memory representation as a byte array in native
    /// endianness.
    ///
    /// As the target platform's native endianness is used, portable code likely wants to
    /// use [`from_be_bytes`](Self::from_be_bytes) or [`from_le_bytes`](Self::from_le_bytes),
    /// as appropriate instead.
    pub const fn from_ne_bytes(bytes: [u8; 15]) -> Self {
        #[cfg(target_endian = "big")]
        {
            Self::from_be_bytes(bytes)
        }
        #[cfg(target_endian = "little")]
        {
            Self::from_le_bytes(bytes)
        }
    }

    /// Return the memory representation of this integer as a byte array in big-endian byte
    /// order.
    pub const fn to_be_bytes(self) -> [u8; 15] {
        let tmp = self.0.to_be_bytes();
        [
            tmp[1], tmp[2], tmp[3], tmp[4], tmp[5], tmp[6], tmp[7], tmp[8], tmp[9], tmp[10],
            tmp[11], tmp[12], tmp[13], tmp[14], tmp[15],
        ]
    }

    /// Return the memory representation of this integer as a byte array in little-endian
    /// byte order.
    pub const fn to_le_bytes(self) -> [u8; 15] {
        let tmp = self.0.to_le_bytes();
        [
            tmp[0], tmp[1], tmp[2], tmp[3], tmp[4], tmp[5], tmp[6], tmp[7], tmp[8], tmp[9],
            tmp[10], tmp[11], tmp[12], tmp[13], tmp[14],
        ]
    }

    /// Return the memory representation of this integer as a byte array in native byte
    /// order.
    ///
    /// As the target platform's native endianness is used, portable code likely wants to
    /// use [`to_be_bytes`](Self::to_be_bytes) or [`to_le_bytes`](Self::to_le_bytes), as
    /// appropriate instead.
    pub const fn to_ne_bytes(self) -> [u8; 15] {
        #[cfg(target_endian = "big")]
        {
            self.to_be_bytes()
        }
        #[cfg(target_endian = "little")]
        {
            self.to_le_bytes()
        }
    }
}
