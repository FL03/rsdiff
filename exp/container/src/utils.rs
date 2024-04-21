/*
    Appellation: utils <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use acme::prelude::{Shape, Stride};

pub(crate) fn default_strides(shape: &Shape) -> Stride {
    // Compute default array strides
    // Shape (a, b, c) => Give strides (b * c, c, 1)
    let mut strides = Stride::zeros(shape.rank());
    // For empty arrays, use all zero strides.
    if shape.iter().all(|&d| d != 0) {
        let mut it = strides.as_slice_mut().iter_mut().rev();
        // Set first element to 1
        if let Some(rs) = it.next() {
            *rs = 1;
        }
        let mut cum_prod = 1;
        for (rs, dim) in it.zip(shape.iter().rev()) {
            cum_prod *= *dim;
            *rs = cum_prod;
        }
    }
    strides
}

pub(crate) fn _fastest_varying_stride_order(strides: &Stride) -> Stride {
    let mut indices = strides.clone();
    for (i, elt) in indices.as_slice_mut().iter_mut().enumerate() {
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
