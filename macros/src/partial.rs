/*
    Appellation: partial <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::ast::Partial;

use proc_macro2::TokenStream;
use quote::quote;
use syn::fold::Fold;
use syn::{Expr, ExprBinary, ExprCall, ExprMethodCall, ExprParen, ExprUnary, Ident};

pub fn generate_partial(partial: &Partial) -> TokenStream {
    let Partial { expr, var } = partial;
    let grad = handle_expr(expr, var);
    grad
}

pub fn handle_expr(expr: &Expr, variable: &Ident) -> TokenStream {
    match expr {
        Expr::Binary(inner) => handle_binary(inner, variable),
        // Differentiate constants
        Expr::Const(_) => quote! { 0.0 },
        Expr::Group(inner) => handle_expr(&inner.expr, variable),
        // Differentiate literals
        Expr::Lit(_) => quote! { 0.0 },
        // Differentiate parenthesized expressions
        Expr::Paren(inner) => handle_expr(&inner.expr, variable),
        // Differentiate variable expressions
        Expr::Path(inner) => {
            if inner.path.segments.len() != 1 {
                panic!("Unsupported path!");
            }
            let path = &inner.path;
            if path.segments[0].ident == *variable {
                quote! { 1.0 }
            } else {
                quote! { 0.0 }
            }
        }
        Expr::Reference(inner) => handle_expr(&inner.expr, variable),
        // Differentiate unary expressions
        Expr::Unary(inner) => handle_unary(inner, variable),
        // Differentiate other expressions
        _ => panic!("Unsupported expression!"),
    }
}

fn handle_binary(expr: &ExprBinary, variable: &Ident) -> TokenStream {
    use syn::BinOp;
    let ExprBinary {
        left, right, op, ..
    } = expr;

    // Differentiate left and right subexpressions
    let dl = handle_expr(&left, variable);
    let dr = handle_expr(&right, variable);

    // Apply the chain rule based on the operator
    match op {
        // Differentiate addition
        BinOp::Add(_) => {
            quote! {
                #dl + #dr
            }
        }
        BinOp::AddAssign(_) => {
            quote! {
                #dl + #dr
            }
        }
        // Differentiate division using the quotient rule
        BinOp::Div(_) => {
            quote! {
                (#dl * #right - #left * #dr) / (#right * #right)
            }
        }
        BinOp::DivAssign(_) => {
            quote! {
                (#dl * #right - #left * #dr) / (#right * #right)
            }
        }
        // Differentiate multiplication
        BinOp::Mul(star) => {
            if let Expr::Paren(inner) = *right.clone() {
                let ExprParen { expr, .. } = inner;
                if let Expr::Binary(inner) = *expr {
                    let ExprBinary {
                        left: le,
                        right: re,
                        ..
                    } = inner;
                    let pleft = ExprBinary {
                        left: left.clone(),
                        right: le.clone(),
                        op: BinOp::Mul(star.clone()),
                        attrs: vec![],
                    };
                    let pright = ExprBinary {
                        left: left.clone(),
                        right: re.clone(),
                        op: BinOp::Mul(star.clone()),
                        attrs: vec![],
                    };

                    let dl = handle_expr(&pleft.into(), variable);
                    let dr = handle_expr(&pright.into(), variable);
                    return quote! {
                        #dl + #dr
                    };
                }
            }
            if let Expr::Paren(inner) = *left.clone() {
                let ExprParen { expr, .. } = inner;
                if let Expr::Binary(inner) = *expr {
                    let ExprBinary {
                        left: le,
                        right: re,
                        ..
                    } = inner;
                    let pleft = ExprBinary {
                        left: le.clone(),
                        right: right.clone(),
                        op: BinOp::Mul(star.clone()),
                        attrs: vec![],
                    };
                    let pright = ExprBinary {
                        left: re.clone(),
                        right: right.clone(),
                        op: BinOp::Mul(star.clone()),
                        attrs: vec![],
                    };

                    let dl = handle_expr(&pleft.into(), variable);
                    let dr = handle_expr(&pright.into(), variable);
                    return quote! {
                        #dl + #dr
                    };
                }
            }
            quote! {
                #dl * #right + #dr * #left
            }
        }
        BinOp::MulAssign(_) => {
            quote! {
                #dl * #right + #dr * #left
            }
        }
        // Differentiate subtraction
        BinOp::Sub(_) => {
            quote! {
                #dl - #dr
            }
        }
        BinOp::SubAssign(_) => {
            quote! {
                #dl - #dr
            }
        }
        _ => panic!("Unsupported operation!"),
    }
}

fn handle_unary(expr: &ExprUnary, variable: &Ident) -> TokenStream {
    use syn::UnOp;
    let dv = handle_expr(&expr.expr, variable);
    match expr.op {
        UnOp::Neg(_) => {
            quote! { -#dv }
        }
        _ => panic!("Unsupported unary operator!"),
    }
}

fn foil(left: &Expr, right: &Expr) -> TokenStream {
    quote! {}
}

fn handle_method_call(expr: &ExprMethodCall, variable: &Ident) -> TokenStream {
    let ExprMethodCall { args, receiver, .. } = expr;
    let dr = handle_expr(&receiver, variable);
    let da = args
        .into_iter()
        .map(|arg| handle_expr(&arg, variable))
        .collect::<Vec<_>>();
    let df = da
        .iter()
        .fold(quote! { 0.0 }, |da, grad| quote! { #grad + #dr * #da });
    quote! {
        #df
    }
}

pub struct ExtractExpr;

impl Fold for ExtractExpr {
    fn fold_expr(&mut self, expr: Expr) -> Expr {
        match expr {
            Expr::Binary(inner) => {
                let ExprBinary {
                    left, right, op, ..
                } = inner;
                let left = self.fold_expr(*left);
                let right = self.fold_expr(*right);
                Expr::Binary(ExprBinary {
                    left: Box::new(left),
                    right: Box::new(right),
                    op,
                    attrs: vec![],
                    ..inner
                })
            }
            Expr::MethodCall(inner) => {
                let ExprMethodCall { receiver, args, .. } = inner;
                let receiver = self.fold_expr(*receiver);
                let args = args.into_iter().map(|arg| self.fold_expr(arg)).collect();
                Expr::MethodCall(ExprMethodCall {
                    receiver: Box::new(receiver),
                    args,
                    attrs: vec![],
                    ..inner
                })
            }
            Expr::Unary(inner) => {
                let ExprUnary { expr, op, .. } = inner;
                let expr = self.fold_expr(*expr);
                Expr::Unary(ExprUnary {
                    expr: Box::new(expr),
                    op,
                    attrs: vec![],
                    ..inner
                })
            }
            _ => expr,
        }
    }
}
