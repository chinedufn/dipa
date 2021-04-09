//! Functions and types to help with code generation for implementation Diffable / Patchable for
//! enums.

use syn::Ident;
use syn::__private::TokenStream2;

pub use self::enum_variant::*;
pub use self::generate_associated_types::*;

mod enum_variant;

mod generate_associated_types;
mod generate_patch_enum_tokens;

mod generate_dipa_impl;

/// An enum
pub struct ParsedEnum {
    pub name: Ident,
    pub variants: Vec<EnumVariant>,
}

/// Create a match statement for comparing two enums.
///
/// match (self, other) {
///     #inner_tokens
/// };
pub fn make_two_enums_match_statement(
    left_ident: &Ident,
    right_ident: &Ident,
    inner_tokens: TokenStream2,
) -> TokenStream2 {
    quote! {
     match (#left_ident, #right_ident) {
         #inner_tokens
     }
    }
}

/// FIXME: Move this into a method on the ParsedEnum type
pub fn delta_type_name(enum_name: &Ident) -> Ident {
    Ident::new(&format!("{}Delta", enum_name.to_string()), enum_name.span())
}

/// FIXME: Move this into a method on the ParsedEnum type
pub fn delta_owned_type_name(enum_name: &Ident) -> Ident {
    Ident::new(
        &format!("{}DeltaOwned", enum_name.to_string()),
        enum_name.span(),
    )
}

#[cfg(test)]
mod test_extras {
    use quote::__private::{Ident, Span};
    use syn::Type;

    use crate::multi_field_utils::{ParsedFields, StructOrTupleField};
    use crate::parsed_enum::{EnumVariant, EnumVariantFields, ParsedEnum};

    impl ParsedEnum {
        /// ```
        /// # #[allow(unused)]
        /// enum MyEnum {
        ///     One(u16),
        ///     Two
        /// }
        /// ```
        pub fn new_test_two_variants_one_field() -> Self {
            let fields = vec![StructOrTupleField {
                name: Default::default(),
                ty: Type::Verbatim(quote! {u16}),
                span: Span::call_site(),
            }];

            ParsedEnum {
                name: Ident::new("MyEnum", Span::call_site()),
                variants: vec![
                    EnumVariant {
                        name: Ident::new("One", Span::call_site()),
                        fields: EnumVariantFields::Tuple(ParsedFields {
                            fields,
                            span: Span::call_site(),
                        }),
                    },
                    EnumVariant {
                        name: Ident::new("Two", Span::call_site()),
                        fields: EnumVariantFields::Unit,
                    },
                ],
            }
        }

        /// ```
        /// # #[allow(unused)]
        /// enum MyEnum {
        ///     MyVariant(u16, u32),
        /// }
        /// ```
        pub fn new_test_one_variant_two_unnamed_fields() -> Self {
            ParsedEnum {
                name: Ident::new("MyEnum", Span::call_site()),
                variants: vec![EnumVariant {
                    name: Ident::new("MyVariant", Span::call_site()),
                    fields: EnumVariantFields::Tuple(two_fields()),
                }],
            }
        }

        /// ```
        /// # #[allow(unused)]
        /// enum MyEnum {
        ///     MyVariant { fielda: u16, fieldb: u32 },
        /// }
        /// ```
        pub fn _new_test_one_variant_two_named_fields() -> Self {
            ParsedEnum {
                name: Ident::new("MyEnum", Span::call_site()),
                variants: vec![EnumVariant {
                    name: Ident::new("MyVariant", Span::call_site()),
                    fields: EnumVariantFields::Struct(two_fields()),
                }],
            }
        }
    }

    fn two_fields() -> ParsedFields {
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

        ParsedFields {
            fields,
            span: Span::call_site(),
        }
    }
}
