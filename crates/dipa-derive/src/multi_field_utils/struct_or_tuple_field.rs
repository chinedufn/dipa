use quote::__private::Span;
use syn::__private::TokenStream2;
use syn::spanned::Spanned;
use syn::{Ident, Type};

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
