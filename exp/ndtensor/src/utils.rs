/*
    Appellation: utils <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use acme::prelude::Stride;

pub(crate) fn _fastest_varying_stride_order(strides: &Stride) -> Stride {
    let mut indices = strides.clone();
    for (i, elt) in indices.as_slice_mut().into_iter().enumerate() {
        *elt = i;
    }
    let strides = strides.as_slice();
    indices
        .as_slice_mut()
        .sort_by_key(|&i| (strides[i] as isize).abs());
    indices
}

macro_rules! izip {
    // @closure creates a tuple-flattening closure for .map() call. usage:
    // @closure partial_pattern => partial_tuple , rest , of , iterators
    // eg. izip!( @closure ((a, b), c) => (a, b, c) , dd , ee )
    ( @closure $p:pat => $tup:expr ) => {
        |$p| $tup
    };

    // The "b" identifier is a different identifier on each recursion level thanks to hygiene.
    ( @closure $p:pat => ( $($tup:tt)* ) , $_iter:expr $( , $tail:expr )* ) => {
        izip!(@closure ($p, b) => ( $($tup)*, b ) $( , $tail )*)
    };

    // unary
    ($first:expr $(,)*) => {
        IntoIterator::into_iter($first)
    };

    // binary
    ($first:expr, $second:expr $(,)*) => {
        izip!($first)
            .zip($second)
    };

    // n-ary where n > 2
    ( $first:expr $( , $rest:expr )* $(,)* ) => {
        izip!($first)
            $(
                .zip($rest)
            )*
            .map(
                izip!(@closure a => (a) $( , $rest )*)
            )
    };
}
