/*
    Appellation: autodiff <example>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
// #![cfg(feature = "macros")]
#![allow(unused_variables)]
extern crate rsdiff;

use rsdiff::autodiff;

macro_rules! format_exp {
    (symbolic: {exp: $ex:expr, vars: [$($var:ident),*] }) => {
        {
            format!("f({})\t= {}", stringify!($($var),*), stringify!($ex))
        }

    }
}

macro_rules! eval {
    ($var:ident: $ex:expr) => {
        {
            let tmp = autodiff!($var: $ex);
            let var = stringify!($var);
            println!("*** Eval ***\nf({})\t= {}\nf({})\t= {:?}\nf'({})\t= {:?}\n", &var, stringify!($ex), $var, $ex, $var, &tmp);
            tmp
        }

    }
}

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let x = 2_f64;
    let y = 3f64;
    let exp = format_exp!(symbolic: {exp: x * y, vars: [x, y]});
    println!("{}", exp);
    // multiply(x, x);

    trig_functions(x);

    Ok(())
}

// #[partial]
// pub fn multiply<A, B, C>(x: A, y: B) -> C
// where
//     A: std::ops::Mul<B, Output = C>,
// {
//     x * y
// }

fn trig_functions(x: f64) {
    let _tangent = eval!(x: x.tan());

    let sine = eval!(x: x.sin());
    assert_eq!(sine, x.cos());
    let _cos_sin = eval!(x: x.cos().sin());
}
