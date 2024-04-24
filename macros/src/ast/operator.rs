/*
    Appellation: operator <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use proc_macro2::Span;
use syn::meta::ParseNestedMeta;
use syn::{Ident, Item, Result};

// pub fn from_proc_macro_attribute(args: TokenStream, item: TokenStream) -> OperatorAst {
//     let mut attrs = OperatorAttr::new();
//     let op_parser = syn::meta::parser(|meta| attrs.parse(meta));
//     let _ = syn::parse_macro_input!(args with op_parser);
//     let item = syn::parse_macro_input!(item as syn::Item);
//     return OperatorAst::new(attrs, item);
// }

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
    pub lexical: Option<Ident>,
}

impl OperatorAttr {
    pub fn new() -> Self {
        Self { lexical: None }
    }

    pub fn parser(&mut self, meta: ParseNestedMeta) -> Result<()> {
        if meta.path.is_ident("lexical") {
    
            let value: Ident = meta.value()?.parse()?;
            self.lexical = Some(value);
        } else {
            return Err(meta.error("Unknown attribute"));
        }
        Ok(())
    }

    pub fn is_lexical(&self) -> bool {
        self.lexical.is_some()
    }

    pub fn set_lexical(&mut self, value: Option<Ident>) {
        self.lexical = value;
    }
}
