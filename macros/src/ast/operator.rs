/*
    Appellation: operator <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use proc_macro2::Span;
use syn::meta::ParseNestedMeta;
use syn::{Item, LitBool, Result};

// pub fn from_proc_macro_attribute(args: TokenStream, item: TokenStream) -> OperatorAst {
//     let mut attrs = OperatorAttr::new();
//     let op_parser = syn::meta::parser(|meta| attrs.parse(meta));
//     let _ = syn::parse_macro_input!(args with op_parser);
//     let item = syn::parse_macro_input!(item as syn::Item);
//     return OperatorAst::new(attrs, item);
// }

pub struct OperatorAst {
    pub attrs: OperatorAttr,
    pub item: Item,
    pub(crate) span: Span,
}

impl OperatorAst {
    pub fn new(attrs: OperatorAttr, item: Item) -> Self {
        Self {
            attrs,
            item,
            span: Span::call_site(),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct OperatorAttr {
    pub lexical: bool,
}

impl OperatorAttr {
    pub fn new() -> Self {
        Self { lexical: false }
    }

    pub fn parser(&mut self, meta: ParseNestedMeta) -> Result<()> {
        if meta.path.is_ident("lexical") {
            let value: LitBool = meta.value()?.parse()?;
            self.lexical = value.value();
        } else {
            return Err(meta.error("Unknown attribute"));
        }
        Ok(())
    }

    pub fn is_lexical(&self) -> bool {
        self.lexical
    }

    pub fn set_lexical(&mut self, value: bool) {
        self.lexical = value;
    }

    pub fn lex(mut self) -> Self {
        self.lexical = true;
        self
    }
}
