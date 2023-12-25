#![no_std]

//! Aint is a crate implementing integers of non-standard bit widths between
//! 1 and 127. These integer types are represented by the next largest built-in
//! Rust integer, but are bounded to the range and behaviors of the advertised
//! bit width. That is, `T::MIN` and `T::MAX`, are what would be expected for
//! the integer `T` with `N` bits, and similarly, wrapping, saturating, and
//! overflow behaviors match what would be expected for a hypothetical built-in
//! integer `T` with `N` bits.
//!
//! # Example
//! ```
//! # use aint::*;
//! fn add(a: i13, b: i13) -> i13 {
//!     a + b
//! }
//!
//! let x = i13!(100);
//! let y = add(x, i13!(-42));
//! assert_eq!(y, i13!(58));
//! ```
//!
//! The core implementation type is [`Aint<R, const WIDTH: u32>`](Aint), which
//! accepts a representation type, `R`, and a bit width, `WIDTH`. [`Aint`] is
//! intended to primarily be an implementation detail, not to be used directly.
//! Instead, it is preferred to use the type aliases provided by this crate.
//!
//! If [`Aint`] is used directly, it will only permit generic parameters that
//! match one of the existing type aliases provided by this crate. For example,
//! the type [`i24`](type@i24) is an alias of `Aint<i32, 24>`. In theory,
//! `Aint<i64, 24>` would be equivalent in functionality, but will result in a
//! compile time error if instantiated. This prevents the existence multiple
//! [`Aint`]s that are functionally the same but are incompatible types to the
//! compiler.
//!
//! # Byte Wide Types
//! Integer types that have an exact byte width provide additional methods and
//! functionality that other integer types do not. These are types whose width
//! is a multiple of 8, such as [`i24`](type@i24) (3 bytes), [`u56`](type@u56)
//! (7 bytes), and [`u120`](type@u120) (15 bytes). Such types provide the byte
//! and endianness manipulation APIs that built-in Rust integers provide, such
//! as `T::from_be_bytes`, `T::swap_bytes`, and `T::to_le`. Other integer types
//! do not implement these methods since their meaning becomes ambiguous or
//! perhaps even nonsensical.
//!
//! # Conversions
//! All integer types provided by this crate implement traits to convert to and
//! from all built-in Rust integer types. A conversion will be implemented as
//! [`From`] when the conversion is infallible, and [`TryFrom`] when the
//! conversion can fail. For example, converting from [`i4`](type@i4) to
//! [`i8`](prim@i8) is implemented as `impl From<i4> for i8`, but the reverse
//! conversion is implemented as `impl TryFrom<i8> for i4`.
//!
//! Unfortunately, this crate does not provide the same conversions between
//! each of the pairs of types provided, such as [`i4`](type@i4) to
//! [`i13`](type@i13). Existing blanket implmentations on the standard conversion
//! traits prevent a generic implementation of these traits that would cover all
//! the types provided by this crate, and implementing conversions for each pair
//! of types individually is infeasible due to extreme compiliation times, since
//! each pair would result on the order of 128<sup>2</sup> trait implementations.
//!
//! This crate also provides two alternate conversion traits that perform
//! infallible, but potentially lossy, conversions:
//! [`WrappingFrom`]/[`WrappingInto`] and [`SaturatingFrom`]/[`SaturatingInto`].
//! These are implemented for all combinations of built-in integers and integers
//! provided by this crate. See each trait's documentation for more details.
//!
//! # Concatenation and Splitting
//! This crate also provides two traits for concatenating and spliting integers
//! bitwise: [`BitConcat`] and [`BitSplit`]. Any pair of two integer types can
//! be concatenated as long as their combined bit width is at most 128, and
//! any integer can be split bitwise into any two integers whose combined bit
//! width equals the bit width of the original integer.
//!
//! ```
//! # use aint::*;
//! let x: u16 = 0b0101010100101_101;
//! let (x1, x2): (i13, u3) = x.bit_split();
//! let y = u16::bit_concat(x1, x2);
//!
//! assert_eq!(x, y);
//! assert_eq!(x1, i13!(0b0101010100101));
//! assert_eq!(x2, u3!(0b101));
//! ```
//!
//! # Macros
//! Each integer type provided by this crate comes with macro for constructing
//! these types from literals. Built-in integers can have typed literals, such
//! as `42u8` or `-7i64`. The macros provided by this crate perform a similar
//! function. `u13!(100)` is essentially a [`u13`](type@u13) literal with the
//! value `100`. The values provided to these macros are also checked at compile
//! time to ensure that they are valid for the type. For example, `u4!(100)`,
//! would result in a compile time error, because [`u4`](type@u4) can only
//! represent values from `0` to `15`, inclusive.
//!
//! # Features
//! This crate has two non-default features, `serde` and `num`, which enable
//! compaitbility with the `serde` and `num` crates.
//!
//! Enabling the `serde` feature will provide implementations of
//! [`serde::Serialize`](::serde::Serialize) and
//! [`serde::Deserialize`](::serde::Deserialize) for each integer type in
//! this crate.
//!
//! Enabling the `num` feature will provide implementations of all appropriate
//! traits from the [`num_traits`] and [`num_integer`] crates under the `num`
//! family of crates.
//!
//! ```toml
//! [dependencies]
//! aint = { version = "0.1", features = [ "serde", "num" ] }
//! ```
//!
//! # `no_std`
//! This crate is also `#![no_std]`. Unlike some other crates, this crate does
//! not have a `std` (or `no_std`) feature - it simply never uses or needs
//! `libstd`. Additionally, this crate does not use `alloc` either. `libcore`
//! is the only required dependency, and the crates pulled in by the `serde`
//! and `num` features are also `no_std` compatible.

mod aint;
mod bit_concat;
mod bit_split;
mod convs;
pub(crate) mod int;
mod macros;
mod misc;
pub(crate) mod non_prim;
#[cfg(feature = "num")]
mod num;
mod ops;
pub(crate) mod prim;
mod saturating;
pub(crate) mod sealed;
#[cfg(feature = "serde")]
mod serde;
mod shifts;
#[cfg(test)]
mod test;
pub(crate) mod tybit;
mod typedefs;
pub(crate) mod width;
mod wrapping;

pub use aint::*;
pub use bit_concat::*;
pub use bit_split::*;
pub use saturating::*;
pub use typedefs::*;
pub use wrapping::*;
