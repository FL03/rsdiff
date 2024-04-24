/*
    Appellation: utils <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::handle::handle_expr;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{BinOp, Expr, ExprBinary, ExprParen, Ident, Token};

macro_rules! bin_op {
    ($($variant:ident($op:tt)),*) => {
        (
            $(
                bin_op!(@base $variant($op)),
            )*
        )

    };
    (@base $variant:ident($op:tt)) => {
        BinOp::$variant($op)
    };
}

macro_rules! binary_expr {
    ($ctx:expr, $left:expr, $right:expr) => {
        ExprBinary {
            attrs: $ctx.attrs.to_vec(),
            left: Box::new($left),
            op: $ctx.op,
            right: Box::new($right),
        }
    };
    ($attrs:expr, $left:expr, $right:expr, $variant:ident($op:tt)) => {
        ExprBinary {
            attrs: $attrs,
            left: $left,
            op: BinOp::$variant($op),
            right: $right,
        }
    };
    (base: $attrs:expr, $left:expr, $op:expr, $right:expr) => {
        ExprBinary {
            attrs: $attrs,
            left: $left,
            op: $op,
            right: $right,
        }
    };
}

#[allow(dead_code)]
pub fn attrs_is_lexical(attrs: &Vec<syn::Attribute>) -> bool {
    attrs.iter().any(|attr| match &attr.meta {
        syn::Meta::Path(inner) => inner.is_ident("lexical"),
        _ => false,
    })
}

#[allow(dead_code)]
pub fn fn_args_ident(arg: &syn::FnArg) -> Ident {
    use syn::Pat;
    match arg {
        syn::FnArg::Typed(inner) => match inner.pat.as_ref() {
            Pat::Ident(ident) => ident.ident.clone(),
            _ => panic!("Expected an identifier"),
        },
        _ => panic!("Expected a typed argument"),
    }
}

pub fn foil_expr(a: &Expr, b: &ExprParen, var: &Ident) -> TokenStream {
    let ExprParen {
        attrs,
        expr,
        paren_token,
    } = b;
    let box_a = Box::new(a.clone());
    let star = Token![*](paren_token.span.join());
    if let Expr::Binary(inner) = *expr.clone() {
        // Multiply the first term of the first expression by the first term of the second expression
        let pleft = binary_expr!(attrs.to_vec(), box_a.clone(), inner.left, Mul(star));
        let pright = binary_expr!(attrs.to_vec(), box_a, inner.right, Mul(star));
        // Create a new expression with the two new terms; (a + b) * c = a * c + b * c
        let new_expr = binary_expr!(inner, pleft.into(), pright.into());

        handle_expr(&new_expr.into(), var)
    } else {
        let da = handle_expr(a, var);
        let db = handle_expr(expr, var);
        quote! {
            #da * #db
        }
    }
}

fn foil_signs(a: &ExprBinary, b: &ExprBinary, var: &Ident) -> (BinOp, BinOp) {
    let plus = Token![+](var.span());
    let minus = Token![-](var.span());

    match (a.op, b.op) {
        (BinOp::Add(_), BinOp::Add(_)) => bin_op!(Add(plus), Add(plus)),
        (BinOp::Add(_), BinOp::Sub(_)) => bin_op!(Add(plus), Sub(minus)),
        (BinOp::Sub(_), BinOp::Add(_)) => bin_op!(Sub(minus), Add(plus)),
        (BinOp::Sub(_), BinOp::Sub(_)) => bin_op!(Sub(minus), Sub(minus)),
        _ => panic!("FOIL_SIGN"),
    }
}

// (a + b) * (c + d) = a * c + a * d + b * c + b * d
// (a + b) * (c - d) = a * c - a * d + b * c - b * d
pub fn foil(a: &ExprParen, b: &ExprParen, var: &Ident) -> TokenStream {
    let ExprParen { expr: expr_a, .. } = a;
    let ExprParen { expr: expr_b, .. } = b;
    let star = Token![*](var.span());
    if let Expr::Binary(inner_a) = *expr_a.clone() {
        if let Expr::Binary(inner_b) = *expr_b.clone() {
            // Determine the signs of the terms
            let (sign_a, sign_b) = foil_signs(&inner_a, &inner_b, var);
            //
            let al = binary_expr!(
                inner_a.attrs.to_vec(),
                inner_a.left.clone(),
                inner_b.left.clone(),
                Mul(star)
            );
            let ar = binary_expr!(
                inner_a.attrs.to_vec(),
                inner_a.left,
                inner_b.right.clone(),
                Mul(star)
            );
            let bl = binary_expr!(
                inner_a.attrs.to_vec(),
                inner_a.right.clone(),
                inner_b.left,
                Mul(star)
            );
            let br = binary_expr!(
                inner_a.attrs.to_vec(),
                inner_a.right,
                inner_b.right,
                Mul(star)
            );

            let pleft = binary_expr!(base: inner_a.attrs.clone(), Box::new(al.into()), sign_a, Box::new(ar.into()));
            let pright = binary_expr!(base: inner_a.attrs.clone(), Box::new(bl.into()), sign_b, Box::new(br.into()));
            let dl = handle_expr(&pleft.into(), var);
            let dr = handle_expr(&pright.into(), var);
            return quote! {
                #dl + #dr
            };
        }
    }
    panic!("FOILER")
}
