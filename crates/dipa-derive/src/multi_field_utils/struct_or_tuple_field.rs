use quote::__private::Span;
use std::ops::{Deref, DerefMut};
use syn::__private::TokenStream2;
use syn::spanned::Spanned;
use syn::{Ident, Type};

mod generate_delta_type;

#[derive(Clone)]
pub struct ParsedFields {
    pub fields: Vec<StructOrTupleField>,
    pub span: Span,
}

#[derive(Clone)]
pub struct StructOrTupleField {
    /// if struct field: some_name
    /// if tuple field: 0, 1, 2 etc
    pub name: TokenStream2,
    pub ty: Type,
    pub span: Span,
}

impl StructOrTupleField {
    /// "a_prefix":
    ///   some_name -> a_prefix_some_name
    pub fn prefixed_name(&self, prefix: &'static str) -> Ident {
        Ident::new(
            &format!("{}{}", prefix, self.name.to_string()),
            self.name.span(),
        )
    }
}

impl Deref for ParsedFields {
    type Target = Vec<StructOrTupleField>;

    fn deref(&self) -> &Self::Target {
        &self.fields
    }
}

impl DerefMut for ParsedFields {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.fields
    }
}
