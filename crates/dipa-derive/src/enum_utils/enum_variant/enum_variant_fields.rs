use crate::multi_field_utils::StructOrTupleField;
use std::ops::Deref;
use syn::Ident;
use syn::__private::TokenStream2;

/// Tuple -> MyEnum::Variant(...)
/// Struct -> MyEnum::Variant { ... }
#[derive(Clone)]
pub enum EnumVariantFields {
    Tuple(Vec<StructOrTupleField>),
    Struct(Vec<StructOrTupleField>),
    Unit,
}

impl EnumVariantFields {
    pub fn fields(&self) -> &[StructOrTupleField] {
        match self {
            EnumVariantFields::Tuple(v) => &v,
            EnumVariantFields::Struct(v) => &v,
            EnumVariantFields::Unit => &[],
        }
    }

    /// Tuple:
    ///   (prefix_field0, prefix_field1)
    /// Struct:
    ///   { field0: prefix_field0, field1: prefix_field1 }
    pub fn to_pattern_match_tokens(&self, prefix: &'static str) -> TokenStream2 {
        match self {
            EnumVariantFields::Tuple(fields) => {
                let mut fields_expanded = vec![];
                for field in fields.iter() {
                    let field_name = &field.name.to_string();
                    let field_name = field_name.trim();
                    let field_name_prefixed =
                        Ident::new(&format!("{}{}", prefix, field_name), field.span);

                    fields_expanded.push(quote! {#field_name_prefixed});
                }

                quote! {( #(#fields_expanded),* )}
            }
            EnumVariantFields::Struct(fields) => {
                let mut fields_expanded = vec![];
                for field in fields.iter() {
                    let field_name = &field.name;

                    let field_name_str = &field.name.to_string();
                    let field_name_str = field_name_str.trim();
                    let field_name_prefixed =
                        Ident::new(&format!("{}{}", prefix, field_name_str), field.span);

                    fields_expanded.push(quote! {#field_name: #field_name_prefixed});
                }

                quote! {{ #(#fields_expanded),* }}
            }
            EnumVariantFields::Unit => {
                quote! {}
            }
        }
    }

    /// If there are fields:
    ///  (prefix_field0, prefix_field1)
    ///
    /// Otherwise don't return any tokens.
    pub fn to_field_value_tokens_parenthesized(&self, prefix: &'static str) -> TokenStream2 {
        let mut fields_expanded = vec![];

        for field in self.iter() {
            let field_name = &field.name.to_string();
            let field_name = field_name.trim();
            let field_name_prefixed = Ident::new(&format!("{}{}", prefix, field_name), field.span);

            fields_expanded.push(quote! {#field_name_prefixed});
        }

        if fields_expanded.len() > 0 {
            quote! { ( #(#fields_expanded),* ) }
        } else {
            quote! {}
        }
    }

    /// Returns `true` if the enum_variant_fields is [`Unit`].
    pub fn is_unit(&self) -> bool {
        matches!(self, Self::Unit)
    }

    /// Get the field at the given index.
    pub fn field_at_idx(&self, idx: usize) -> Option<&StructOrTupleField> {
        self.fields().get(idx)
    }
}

impl Deref for EnumVariantFields {
    type Target = [StructOrTupleField];

    fn deref(&self) -> &Self::Target {
        self.fields()
    }
}
