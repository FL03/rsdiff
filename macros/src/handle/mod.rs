/*
    Appellation: handle <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Autodifferentiation (AD)
//!
#[allow(unused_imports)]
pub use self::{block::handle_block, expr::handle_expr, item::handle_item, stmt::handle_stmt};

pub mod block;
pub mod expr;
pub mod item;
pub mod stmt;
