/*
    Appellation: macros <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

macro_rules! entry {
    ($ctx:ident[$key:expr]) => {
        entry!(@base $ctx[$key]).or_default()
    };
    ($ctx:ident[$key:expr] or $val:expr) => {
        entry!(@base $ctx[$key].or_insert($val))
    };
    (@base $ctx:ident[$key:expr].$call:ident($val:expr)) => {
        entry!($ctx[$key]).$call:ident($val)
    };
    (@base $ctx:ident[$key:expr]) => {
        $ctx.entry($key)
    };

}

macro_rules! get {
    ($self:ident[$($index:expr),* $(,)?]) => {
        (
            $(
                get!(@impl $self[$index]),
            )*
        )
    };
    (@impl $self:ident[$index:expr]) => {
        &$self.store[$index]
    };

}
