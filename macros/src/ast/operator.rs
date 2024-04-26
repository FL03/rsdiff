/*
    Appellation: operator <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use proc_macro2::Span;
use syn::meta::ParseNestedMeta;
use syn::{Ident, Item};

pub struct OperatorAst {
    pub attrs: Option<OperatorAttr>,
    pub item: Item,
    pub(crate) span: Span,
}

impl OperatorAst {
    pub fn new(attrs: Option<OperatorAttr>, item: Item) -> Self {
        Self {
            attrs,
            item,
            span: Span::call_site(),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct OperatorAttr {
    pub lex: Option<Ident>,
    pub params: Vec<Ident>,
}

impl OperatorAttr {
    pub fn new() -> Self {
        Self { 
            lex: None,
            params: Vec::new(),
        }
    }

    pub fn parser(&mut self, meta: ParseNestedMeta) -> syn::Result<()> {
        if meta.path.is_ident("lex") {
            let value = meta.value()?.parse()?;
            self.lex = Some(value);
        } else {
            return Err(meta.error("Unknown attribute"));
        }
        Ok(())
    }

    pub fn is_lexical(&self) -> bool {
        self.lex.is_some()
    }

    pub fn set_lexical(&mut self, value: Option<Ident>) {
        self.lex = value;
    }
}
