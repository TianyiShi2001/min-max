#[macro_export]
macro_rules! max {
    ($x:expr) => ( $x );
    ($x:expr, $($xs:expr),+) => {
        std::cmp::max($x, max!( $($xs),+ ))
    };
}

#[macro_export]
macro_rules! min {
    ($x:expr) => ( $x );
    ($x:expr, $($xs:expr),+) => {
        std::cmp::min($x, min!( $($xs),+ ))
    };
}

#[macro_export]
macro_rules! max_partial {
    ($x:expr) => ( $x );
    ($x:expr, $($xs:expr),+) => {
        partial_min_max::max($x, max_partial!( $($xs),+ ))
    };
}

#[macro_export]
macro_rules! min_partial {
    ($x:expr) => ( $x );
    ($x:expr, $($xs:expr),+) => {
        partial_min_max::min($x, min_partial!( $($xs),+ ))
    };
}
