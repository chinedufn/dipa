use syn::Ident;
use syn::__private::TokenStream2;

pub use self::enum_variant_fields::*;

mod enum_variant_fields;

mod diff_enum_variants;
mod patch_enum_variants;

mod diff_type_variants;

pub struct EnumVariant {
    pub name: Ident,
    pub fields: EnumVariantFields,
}

impl EnumVariant {
    /// Tuple:
    ///   VariantName (prefix_field0, prefix_field1)
    /// Struct:
    ///   VariantName { field0: prefix_field0, field1: prefix_field1 }
    pub fn to_tokens(&self, field_prefix: &'static str) -> TokenStream2 {
        let name = &self.name;
        let fields = self.fields.to_pattern_match_tokens(field_prefix);

        quote! {#name #fields}
    }

    fn variant_no_change(&self) -> Ident {
        Ident::new(
            &format!("{}NoChange", self.name.to_string()),
            self.name.span(),
        )
    }

    fn changed_to_variant(&self) -> Ident {
        let variant = &self.name;

        Ident::new(
            &format!("ChangedToVariant{}", variant.to_string()),
            self.name.span(),
        )
    }
}

#[cfg(test)]
mod test_extras {
    use crate::multi_field_utils::{ParsedFields, StructOrTupleField};
    use crate::parsed_enum::{EnumVariant, EnumVariantFields};
    use quote::__private::{Ident, Span};
    use syn::Type;

    impl EnumVariant {
        /// ```
        /// # #[allow(unused)]
        /// enum MyEnum {
        ///     Two,      // <--- This one
        /// }
        /// ```
        pub fn no_field_variant() -> Self {
            EnumVariant {
                name: Ident::new("Two", Span::call_site()),
                fields: EnumVariantFields::Unit,
            }
        }

        /// ```
        /// # #[allow(unused)]
        /// enum MyEnum {
        ///     One(u16), // <--- This one
        /// }
        /// ```
        pub fn one_field_variant() -> Self {
            let fields = vec![StructOrTupleField {
                name: quote! {0},
                ty: Type::Verbatim(quote! {u16}),
                span: Span::call_site(),
            }];

            EnumVariant {
                name: Ident::new("One", Span::call_site()),
                fields: EnumVariantFields::Tuple(ParsedFields {
                    fields,
                    span: Span::call_site(),
                }),
            }
        }

        /// ```
        /// # #[allow(unused)]
        /// enum MyEnum {
        ///     Two(u16, u32), // <--- This one
        /// }
        /// ```
        pub fn two_fields_variant() -> Self {
            let fields = vec![
                StructOrTupleField {
                    name: quote! {0},
                    ty: Type::Verbatim(quote! {u16}),
                    span: Span::call_site(),
                },
                StructOrTupleField {
                    name: quote! {1},
                    ty: Type::Verbatim(quote! {u32}),
                    span: Span::call_site(),
                },
            ];

            EnumVariant {
                name: Ident::new("Two", Span::call_site()),
                fields: EnumVariantFields::Tuple(ParsedFields {
                    fields,
                    span: Span::call_site(),
                }),
            }
        }
    }
}
