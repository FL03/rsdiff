/*
    Appellation: tensor <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

macro_rules! new {
    {
        data:$data:expr,
        kind:$kind:expr,
        op:$op:expr,
    } => {
        $crate::tensor::TensorBase {
            id: $crate::prelude::TensorId::new(),
            ctx: $crate::Context::new($kind),

            data: $data,
            op: $op,
        }
    };
    ($data:expr) => {
        $crate::tensor::new($data, None, false)
    };
    ($data:expr, $op:expr) => {
        $crate::tensor::new($data, $op, false)
    };
    ($data:expr, $op:expr, $kind:expr) => {
        $crate::tensor::new($data, Some($op), $kind)
    };
}
