/*
    Appellation: params <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub trait Params {
    type Pattern;

    fn into_pattern(self) -> Self::Pattern;
}

macro_rules! args_impl {
    () => {
        impl Params for () {
            type Pattern = ();

            fn into_pattern(self) -> Self::Pattern {
                ()
            }
        }

    };
    ($n:ident) => {
        impl<$n> Params for ($n,) {
            type Pattern = ($n,);

            fn into_pattern(self) -> Self::Pattern {
                self
            }
        }

        impl<$n> Params for [$n; 1] where $n: Copy {
            type Pattern = ($n,);

            fn into_pattern(self) -> Self::Pattern {
                (self[0],)
            }
        }
    };
    ($($n:tt),*) => {
        args_impl!(@loop $($n),*);
    };
    (@loop $(($($n:ident),*)),*) => {
        $(
            args_impl!(@loop $($n),*);
        )*
    };
    (@loop $($n:ident),*) => {
        impl<$($n),*> Params for ($($n),*) {
            type Pattern = ($($n),*);

            fn into_pattern(self) -> Self::Pattern {
                self
            }
        }
    };
}

/*
 **************** implementations ****************
*/
args_impl!();
args_impl!(A);
args_impl!((A, B), (A, B, C), (A, B, C, D), (A, B, C, D, E));
