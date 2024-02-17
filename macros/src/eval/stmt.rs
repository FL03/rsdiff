
/*
    Appellation: item <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::expr::handle_expr;
use super::item::handle_item;
use proc_macro2::TokenStream;
use syn::{Ident, Local, Stmt};

pub fn handle_stmt(stmt: &Stmt, var: &Ident) -> TokenStream {
    match stmt {
        Stmt::Local(local) => {
            let Local { init, .. } = local;
            if let Some(tmp) = init {
                return handle_expr(&tmp.expr, var);
            }
            panic!("Local variable not initialized!")
        }
        Stmt::Item(item) => {
            handle_item(item, var)
        }
        Stmt::Expr(expr, _) => {
            handle_expr(expr, var)
        }
        _ => panic!("Unsupported statement!"),
    }
}