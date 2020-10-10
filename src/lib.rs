//! # **min-max**: `max!` and `min!` macros for Rust
//!
//! [![crates.io](https://img.shields.io/crates/d/min-max.svg)](https://crates.io/crates/min-max)
//! [![crates.io](https://img.shields.io/crates/v/min-max.svg)](https://crates.io/crates/min-max)
//! [![crates.io](https://img.shields.io/crates/l/min-max.svg)](https://crates.io/crates/min-max)
//!
//! ## Why?
//!
//! Sometimes you want to find the maximum of a bunch of **scalars**. Usually you would write something like `max(x1, max(x2, max(x3, x4)))`. The `max!` macro provided by this crate simplifies that to `max!(x1, x2, x3, x4)`. (Note, for an **iterable** data structure, you would use `xx.iter().max()`).
//!
//! ## Usage
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! min-max = "0.1"
//! ```
//!
//! Then, for example:
//!
//! ```rust
//! use min_max::*;
//!
//! fn main() {
//!     let max = max!(1, 5, 7, 2, 4, 9, 3);
//!     assert_eq!(max, 9);
//!     let min = min!(1, 5, 7, 2, 4, 9, 3);
//!     assert_eq!(min, 1);
//! }
//! ```
//!
//! ### Does it work on floats?
//!
//! Yep. But you need to use `max_partial!`/`min_partial!`
//!
//! ```rust
//! use min_max::*;
//!
//! fn main() {
//!     let partial_max = max_partial!(1.8f64, 5.8, 7.8, 2.8, 4.8, 9.8, 3.8);
//!     assert!((9.8 - partial_max).abs() < 1e-5);
//!     let partial_min = min_partial!(1.8f64, 5.8, 7.8, 2.8, 4.8, 9.8, 3.8);
//!     assert!((1.8 - partial_min).abs() < 1e-5);
//! }
//! ```
//!
//! ### What about `NaN`?
//!
//! Do not use when your data contains `NaN`. When `NaN` is at the end, `NaN` is returned. Otherwise, the min/max excluding `NaN` is returned.
//!
//! ```rust
//! use min_max::*;
//!
//! fn main() {
//!     let partial_max = max_partial!(1.8, 5.8, f64::NAN, 2.8, 4.8, 9.8, 3.8);
//!     assert!((9.8 - partial_max).abs() < 1e-5);
//!     let partial_max = max_partial!(1.8, 5.8, 2.8, 4.8, 9.8, 3.8, f64::NAN);
//!     assert!(partial_max.is_nan());
//!     let partial_min = min_partial!(1.8, 5.8, f64::NAN, 2.8, 4.8, 9.8, 3.8);
//!     assert!((1.8 - partial_min).abs() < 1e-5);
//!     let partial_min = max_partial!(1.8, 5.8, 2.8, 4.8, 9.8, 3.8, f64::NAN);
//!     assert!(partial_min.is_nan());
//! }
//! ```
//!
//! ### Can I use custom types?
//!
//! Sure, why not?
//!
//! ```rust
//! use min_max::*;
//!
//! #[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone, Copy)]
//! struct Point {
//!     x: u16,
//!     y: u16,
//! }
//!
//! fn main() {
//!     let a = Point { x: 5, y: 8 };
//!     let b = Point { x: 10, y: 92 };
//!     let c = Point { x: 0, y: 3 };
//!     let max = max!(a, b, c);
//!     assert_eq!(max, b);
//! }
//! ```
//!
//! ## What's going on under the hood?
//!
//! Well, `max!(x1, x2, x3)` expands to:
//!
//! ```rust
//! std::cmp::max(x1, std::cmp::max(x2, std::cmp::max(x3)))
//! ```
//!
//! and so on. `min!` works similarly, but with `std::cmp::min`.
//!
//! `min_partial!` and `max_partial` uses the `min` and `max` functions from the [`partial-min-max` crate](https://crates.io/crates/partial-min-max).

/// Returns the maximum element of the arguments.
#[macro_export]
macro_rules! max {
    ($x:expr) => ( $x );
    ($x:expr, $($xs:expr),+) => {
        std::cmp::max($x, max!( $($xs),+ ))
    };
}

/// Returns the minimum element of the arguments.
#[macro_export]
macro_rules! min {
    ($x:expr) => ( $x );
    ($x:expr, $($xs:expr),+) => {
        std::cmp::min($x, min!( $($xs),+ ))
    };
}

/// Returns the maximum element of the arguments. Uses [`partial_min_max::max`](https://docs.rs/partial-min-max/0.4.0/partial_min_max/fn.max.html) for comparison.
#[macro_export]
macro_rules! max_partial {
    ($x:expr) => ( $x );
    ($x:expr, $($xs:expr),+) => {
        partial_min_max::max($x, max_partial!( $($xs),+ ))
    };
}

/// Returns the maximum element of the arguments. Uses [`partial_min_max::min`](https://docs.rs/partial-min-max/0.4.0/partial_min_max/fn.min.html) for comparison.
#[macro_export]
macro_rules! min_partial {
    ($x:expr) => ( $x );
    ($x:expr, $($xs:expr),+) => {
        partial_min_max::min($x, min_partial!( $($xs),+ ))
    };
}
