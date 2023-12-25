# Aint

Aint is a crate implementing integers of non-standard bit widths between
1 and 127. These integer types are represented by the next largest built-in
Rust integer, but are bounded to the range and behaviors of the advertised
bit width. That is, `T::MIN` and `T::MAX`, are what would be expected for
the integer `T` with `N` bits, and similarly, wrapping, saturating, and
overflow behaviors match what would be expected for a hypothetical built-in
integer `T` with `N` bits.

### Example
```
# use aint::*;
fn add(a: i13, b: i13) -> i13 {
a + b
}

let x = i13!(100);
let y = add(x, i13!(-42));
assert_eq!(y, i13!(58));
```

See [the documentation](https://docs.rs/aint/latest/aint/) for details.
