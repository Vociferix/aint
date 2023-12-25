#[allow(unused_imports)]
use crate::*;

/// Returns a [`u8`](type@crate::u8) checked at compile time.
///
/// # Example
/// ```
/// # use aint::*;
/// let x = u8!(42);
/// assert_eq!(x, 42);
/// ```
#[macro_export]
macro_rules! u8 {
    ($val:literal) => {{
        const __AINT_LITERAL_VALUE: u8 = $val;
        __AINT_LITERAL_VALUE
    }};
}

/// Returns a [`u16`](type@crate::u16) checked at compile time.
///
/// # Example
/// ```
/// # use aint::*;
/// let x = u16!(42);
/// assert_eq!(x, 42);
/// ```
#[macro_export]
macro_rules! u16 {
    ($val:literal) => {{
        const __AINT_LITERAL_VALUE: u16 = $val;
        __AINT_LITERAL_VALUE
    }};
}

/// Returns a [`u32`](type@crate::u32) checked at compile time.
///
/// # Example
/// ```
/// # use aint::*;
/// let x = u32!(42);
/// assert_eq!(x, 42);
/// ```
#[macro_export]
macro_rules! u32 {
    ($val:literal) => {{
        const __AINT_LITERAL_VALUE: u32 = $val;
        __AINT_LITERAL_VALUE
    }};
}

/// Returns a [`u64`](type@crate::u64) checked at compile time.
///
/// # Example
/// ```
/// # use aint::*;
/// let x = u64!(42);
/// assert_eq!(x, 42);
/// ```
#[macro_export]
macro_rules! u64 {
    ($val:literal) => {{
        const __AINT_LITERAL_VALUE: u64 = $val;
        __AINT_LITERAL_VALUE
    }};
}

/// Returns a [`u128`](type@crate::u128) checked at compile time.
///
/// # Example
/// ```
/// # use aint::*;
/// let x = u128!(42);
/// assert_eq!(x, 42);
/// ```
#[macro_export]
macro_rules! u128 {
    ($val:literal) => {{
        const __AINT_LITERAL_VALUE: u128 = $val;
        __AINT_LITERAL_VALUE
    }};
}

/// Returns a [`i8`](type@crate::i8) checked at compile time.
///
/// # Example
/// ```
/// # use aint::*;
/// let x = i8!(42);
/// assert_eq!(x, 42);
/// ```
#[macro_export]
macro_rules! i8 {
    ($val:literal) => {{
        const __AINT_LITERAL_VALUE: i8 = $val;
        __AINT_LITERAL_VALUE
    }};
}

/// Returns a [`i16`](type@crate::i16) checked at compile time.
///
/// # Example
/// ```
/// # use aint::*;
/// let x = i16!(42);
/// assert_eq!(x, 42);
/// ```
#[macro_export]
macro_rules! i16 {
    ($val:literal) => {{
        const __AINT_LITERAL_VALUE: i16 = $val;
        __AINT_LITERAL_VALUE
    }};
}

/// Returns a [`i32`](type@crate::i32) checked at compile time.
///
/// # Example
/// ```
/// # use aint::*;
/// let x = i32!(42);
/// assert_eq!(x, 42);
/// ```
#[macro_export]
macro_rules! i32 {
    ($val:literal) => {{
        const __AINT_LITERAL_VALUE: i32 = $val;
        __AINT_LITERAL_VALUE
    }};
}

/// Returns a [`i64`](type@crate::i64) checked at compile time.
///
/// # Example
/// ```
/// # use aint::*;
/// let x = i64!(42);
/// assert_eq!(x, 42);
/// ```
#[macro_export]
macro_rules! i64 {
    ($val:literal) => {{
        const __AINT_LITERAL_VALUE: i64 = $val;
        __AINT_LITERAL_VALUE
    }};
}

/// Returns a [`i128`](type@crate::i128) checked at compile time.
///
/// # Example
/// ```
/// # use aint::*;
/// let x = i128!(42);
/// assert_eq!(x, 42);
/// ```
#[macro_export]
macro_rules! i128 {
    ($val:literal) => {{
        const __AINT_LITERAL_VALUE: i128 = $val;
        __AINT_LITERAL_VALUE
    }};
}

macro_rules! lit_macro {
    ($ty:ident) => {
        #[doc=core::concat!("Returns a [`", core::stringify!($ty), "`](type@", core::stringify!($ty), ") checked at compile time.")]
        #[doc=""]
        #[doc="# Example"]
        #[doc="```"]
        #[doc="# use aint::*;"]
        #[doc=core::concat!("let x = ", core::stringify!($ty), "!(0);")]
        #[doc="assert_eq!(x.repr(), 0);"]
        #[doc="```"]
        #[macro_export]
        macro_rules! $ty {
            ($val:literal) => {{
                const __AINT_LITERAL_VALUE: $ty = match $crate::$ty::new($val) {
                    Some(__aint_literal_value) => __aint_literal_value,
                    None => ::core::panic!(::core::concat!("Invalid value for ", ::core::stringify!($ty))),
                };
                __AINT_LITERAL_VALUE
            }};
        }
    };
}

lit_macro!(bit);
lit_macro!(u1);
lit_macro!(u2);
lit_macro!(u3);
lit_macro!(u4);
lit_macro!(u5);
lit_macro!(u6);
lit_macro!(u7);
lit_macro!(u9);
lit_macro!(u10);
lit_macro!(u11);
lit_macro!(u12);
lit_macro!(u13);
lit_macro!(u14);
lit_macro!(u15);
lit_macro!(u17);
lit_macro!(u18);
lit_macro!(u19);
lit_macro!(u20);
lit_macro!(u21);
lit_macro!(u22);
lit_macro!(u23);
lit_macro!(u24);
lit_macro!(u25);
lit_macro!(u26);
lit_macro!(u27);
lit_macro!(u28);
lit_macro!(u29);
lit_macro!(u30);
lit_macro!(u31);
lit_macro!(u33);
lit_macro!(u34);
lit_macro!(u35);
lit_macro!(u36);
lit_macro!(u37);
lit_macro!(u38);
lit_macro!(u39);
lit_macro!(u40);
lit_macro!(u41);
lit_macro!(u42);
lit_macro!(u43);
lit_macro!(u44);
lit_macro!(u45);
lit_macro!(u46);
lit_macro!(u47);
lit_macro!(u48);
lit_macro!(u49);
lit_macro!(u50);
lit_macro!(u51);
lit_macro!(u52);
lit_macro!(u53);
lit_macro!(u54);
lit_macro!(u55);
lit_macro!(u56);
lit_macro!(u57);
lit_macro!(u58);
lit_macro!(u59);
lit_macro!(u60);
lit_macro!(u61);
lit_macro!(u62);
lit_macro!(u63);
lit_macro!(u65);
lit_macro!(u66);
lit_macro!(u67);
lit_macro!(u68);
lit_macro!(u69);
lit_macro!(u70);
lit_macro!(u71);
lit_macro!(u72);
lit_macro!(u73);
lit_macro!(u74);
lit_macro!(u75);
lit_macro!(u76);
lit_macro!(u77);
lit_macro!(u78);
lit_macro!(u79);
lit_macro!(u80);
lit_macro!(u81);
lit_macro!(u82);
lit_macro!(u83);
lit_macro!(u84);
lit_macro!(u85);
lit_macro!(u86);
lit_macro!(u87);
lit_macro!(u88);
lit_macro!(u89);
lit_macro!(u90);
lit_macro!(u91);
lit_macro!(u92);
lit_macro!(u93);
lit_macro!(u94);
lit_macro!(u95);
lit_macro!(u96);
lit_macro!(u97);
lit_macro!(u98);
lit_macro!(u99);
lit_macro!(u100);
lit_macro!(u101);
lit_macro!(u102);
lit_macro!(u103);
lit_macro!(u104);
lit_macro!(u105);
lit_macro!(u106);
lit_macro!(u107);
lit_macro!(u108);
lit_macro!(u109);
lit_macro!(u110);
lit_macro!(u111);
lit_macro!(u112);
lit_macro!(u113);
lit_macro!(u114);
lit_macro!(u115);
lit_macro!(u116);
lit_macro!(u117);
lit_macro!(u118);
lit_macro!(u119);
lit_macro!(u120);
lit_macro!(u121);
lit_macro!(u122);
lit_macro!(u123);
lit_macro!(u124);
lit_macro!(u125);
lit_macro!(u126);
lit_macro!(u127);

lit_macro!(i1);
lit_macro!(i2);
lit_macro!(i3);
lit_macro!(i4);
lit_macro!(i5);
lit_macro!(i6);
lit_macro!(i7);
lit_macro!(i9);
lit_macro!(i10);
lit_macro!(i11);
lit_macro!(i12);
lit_macro!(i13);
lit_macro!(i14);
lit_macro!(i15);
lit_macro!(i17);
lit_macro!(i18);
lit_macro!(i19);
lit_macro!(i20);
lit_macro!(i21);
lit_macro!(i22);
lit_macro!(i23);
lit_macro!(i24);
lit_macro!(i25);
lit_macro!(i26);
lit_macro!(i27);
lit_macro!(i28);
lit_macro!(i29);
lit_macro!(i30);
lit_macro!(i31);
lit_macro!(i33);
lit_macro!(i34);
lit_macro!(i35);
lit_macro!(i36);
lit_macro!(i37);
lit_macro!(i38);
lit_macro!(i39);
lit_macro!(i40);
lit_macro!(i41);
lit_macro!(i42);
lit_macro!(i43);
lit_macro!(i44);
lit_macro!(i45);
lit_macro!(i46);
lit_macro!(i47);
lit_macro!(i48);
lit_macro!(i49);
lit_macro!(i50);
lit_macro!(i51);
lit_macro!(i52);
lit_macro!(i53);
lit_macro!(i54);
lit_macro!(i55);
lit_macro!(i56);
lit_macro!(i57);
lit_macro!(i58);
lit_macro!(i59);
lit_macro!(i60);
lit_macro!(i61);
lit_macro!(i62);
lit_macro!(i63);
lit_macro!(i65);
lit_macro!(i66);
lit_macro!(i67);
lit_macro!(i68);
lit_macro!(i69);
lit_macro!(i70);
lit_macro!(i71);
lit_macro!(i72);
lit_macro!(i73);
lit_macro!(i74);
lit_macro!(i75);
lit_macro!(i76);
lit_macro!(i77);
lit_macro!(i78);
lit_macro!(i79);
lit_macro!(i80);
lit_macro!(i81);
lit_macro!(i82);
lit_macro!(i83);
lit_macro!(i84);
lit_macro!(i85);
lit_macro!(i86);
lit_macro!(i87);
lit_macro!(i88);
lit_macro!(i89);
lit_macro!(i90);
lit_macro!(i91);
lit_macro!(i92);
lit_macro!(i93);
lit_macro!(i94);
lit_macro!(i95);
lit_macro!(i96);
lit_macro!(i97);
lit_macro!(i98);
lit_macro!(i99);
lit_macro!(i100);
lit_macro!(i101);
lit_macro!(i102);
lit_macro!(i103);
lit_macro!(i104);
lit_macro!(i105);
lit_macro!(i106);
lit_macro!(i107);
lit_macro!(i108);
lit_macro!(i109);
lit_macro!(i110);
lit_macro!(i111);
lit_macro!(i112);
lit_macro!(i113);
lit_macro!(i114);
lit_macro!(i115);
lit_macro!(i116);
lit_macro!(i117);
lit_macro!(i118);
lit_macro!(i119);
lit_macro!(i120);
lit_macro!(i121);
lit_macro!(i122);
lit_macro!(i123);
lit_macro!(i124);
lit_macro!(i125);
lit_macro!(i126);
lit_macro!(i127);
