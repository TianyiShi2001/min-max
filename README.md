# **min-max**: `max!` and `min!` macros for Rust

```toml
[dependencies]
min-max = "0.1"
```

## Usage

```rust
use min_max::*;

fn main() {
    let max = max!(1, 5, 7, 2, 4, 9, 3);
    assert_eq!(max, 9);
    let min = min!(1, 5, 7, 2, 4, 9, 3);
    assert_eq!(min, 1);
}
```

### Does it work on floats?

Yep. But you need to use `max_partial!`/`min_partial!`

```rust
use min_max::*;

fn main() {
    let partial_max = max_partial!(1.8f64, 5.8, 7.8, 2.8, 4.8, 9.8, 3.8);
    assert!((9.8 - partial_max).abs() < 1e-5);
    let partial_min = min_partial!(1.8f64, 5.8, 7.8, 2.8, 4.8, 9.8, 3.8);
    assert!((1.8 - partial_min).abs() < 1e-5);
}
```

### What about `NaN`?

Do not use when your data contains `NaN`. When `NaN` is at the end, `NaN` is returned. Otherwise, the min/max excluding `NaN` is returned.

```rust
use min_max::*;

fn main() {
    let partial_max = max_partial!(1.8, 5.8, f64::NAN, 2.8, 4.8, 9.8, 3.8);
    assert!((9.8 - partial_max).abs() < 1e-5);
    let partial_max = max_partial!(1.8, 5.8, 2.8, 4.8, 9.8, 3.8, f64::NAN);
    assert!(partial_max.is_nan());
    let partial_min = min_partial!(1.8, 5.8, f64::NAN, 2.8, 4.8, 9.8, 3.8);
    assert!((1.8 - partial_min).abs() < 1e-5);
    let partial_min = max_partial!(1.8, 5.8, 2.8, 4.8, 9.8, 3.8, f64::NAN);
    assert!(partial_min.is_nan());
}
```