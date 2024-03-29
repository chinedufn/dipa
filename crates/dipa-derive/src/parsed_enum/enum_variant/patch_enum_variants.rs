use crate::dipa_attribute::DipaAttrs;
use crate::multi_field_utils::ChangedFieldIndices;
use crate::parsed_enum::{delta_owned_type_name, EnumVariant};
use syn::Ident;
use syn::__private::TokenStream2;

impl EnumVariant {
    /// Generate the various patch blocks for an enum variant.
    ///
    /// All of the example doc comments below are based on the following enum.
    ///
    /// ```
    /// # #[allow(unused)]
    /// enum MyEnum {
    ///     VariantA,
    ///     VariantB { some_field: Vec<f32>, another_field: Option<u64> },
    ///     VariantC (i16)
    /// }
    /// ```
    ///
    /// See [crate::enum_utils::EnumVariant.create_patch_match_stmt].
    pub fn generate_patch_blocks(
        &self,
        enum_name: &Ident,
        dipa_attrs: &DipaAttrs,
    ) -> Vec<TokenStream2> {
        let mut patch_blocks = vec![];

        patch_blocks.push(self.generate_no_change_block(enum_name));

        if self.fields.len() == 0 {
            patch_blocks.push(self.generate_changed_to_variant_block_no_fields(enum_name));
        } else {
            patch_blocks.push(self.generate_changed_to_variant_block_with_fields(enum_name));
            patch_blocks.push(self.generate_field_changes(enum_name, dipa_attrs));
        }

        patch_blocks
    }

    /// ```
    /// # use quote::quote;
    /// quote! {
    ///     MyEnumDeltaOwned::VariantANoChange => { }
    /// };
    /// ```
    fn generate_no_change_block(&self, enum_name: &Ident) -> TokenStream2 {
        let patch_name = delta_owned_type_name(enum_name);

        let no_change = self.variant_no_change();

        quote! {
            #patch_name::#no_change => {}
        }
    }

    /// ```
    /// # use quote::quote;
    /// quote! {
    ///     MyEnumDeltaOwned::ChangedToVariantA => {
    ///         *self = MyEnum::VariantA;
    ///     }
    /// };
    /// ```
    fn generate_changed_to_variant_block_no_fields(&self, enum_name: &Ident) -> TokenStream2 {
        let patch_name = delta_owned_type_name(enum_name);

        let changed_to = self.changed_to_variant();

        let variant_name = &self.name;

        quote! {
            #patch_name::#changed_to => {
                *self = #enum_name::#variant_name;
            }
        }
    }

    /// # Struct Like
    ///
    /// ```
    /// # use quote::quote;
    /// quote! {
    ///     MyEnumDeltaOwned::ChangedToVariantVariantB {
    ///         some_field: patch_some_field, patch_another_field
    ///     } => {
    ///         *self = MyEnum::VariantB {
    ///             some_field: patch_some_field,
    ///             another_field: patch_another_field,
    ///         };
    ///     }   
    /// };
    /// ```   
    ///
    /// # Enum Like
    ///
    /// ```
    /// # use quote::quote;
    /// quote! {
    ///     MyEnumDeltaOwned::ChangedToVariantVariantC(patch0)  => {
    ///         *self = MyEnum::VariantC(patch0);
    ///     }
    /// };
    /// ```      
    fn generate_changed_to_variant_block_with_fields(&self, enum_name: &Ident) -> TokenStream2 {
        let patch_name = delta_owned_type_name(enum_name);

        let change_to_variant = self.changed_to_variant();

        let patches = self.fields.to_field_value_tokens_parenthesized("patch_");
        let set_fields = self.fields.to_pattern_match_tokens("patch_");

        let variant_name = &self.name;

        quote! {
            #patch_name::#change_to_variant#patches => {
                *self = #enum_name::#variant_name#set_fields;
            }
        }
    }

    /// ```
    /// # use quote::quote;
    /// quote! {
    ///     MyEnumDeltaOwned::VariantBChange_0(patch0) => {
    ///         match self {
    ///             MyEnum::VariantB {
    ///                 some_field: field_some_field, another_field: field_another_field
    ///             } => {
    ///                 field_some_field.apply_patch(patch0);
    ///             }
    ///             _ => { panic!("TODO: Return Result::Err") }
    ///         }
    ///     }
    ///     MyEnumDeltaOwned::VariantBChange_1(patch1) => {
    ///         match self {
    ///             MyEnum::VariantB {
    ///                 some_field: field_some_field, another_field: field_another_field
    ///             } => {
    ///                 field_another_field.apply_patch(patch1);
    ///             }
    ///             _ => { panic!("TODO: Return Result::Err") }
    ///         }
    ///     }
    ///     MyEnumDeltaOwned::VariantBChange_0_1(patch0, patch1) => {
    ///         match self {
    ///             MyEnum::VariantB {
    ///                 some_field: field_some_field, another_field: field_another_field
    ///             } => {
    ///                 field_some_field.apply_patch(patch0);
    ///                 field_another_field.apply_patch(patch1);
    ///             }
    ///             _ => { panic!("TODO: Return Result::Err") }
    ///         }
    ///     }
    /// };
    /// ```      
    fn generate_field_changes(&self, enum_name: &Ident, dipa_attrs: &DipaAttrs) -> TokenStream2 {
        let patch_name = delta_owned_type_name(enum_name);

        let mut patch_blocks = vec![];

        let variant_pattern_fields = self.fields.to_pattern_match_tokens("field_");
        let span = self.name.span();

        for changed_indices in
            ChangedFieldIndices::all_changed_index_combinations(self.fields.len(), dipa_attrs)
        {
            let variant_name = &self.name;
            let patch_variant_name =
                changed_indices.variant_name_ident(&self.name.to_string(), span);

            let patch_fields = changed_indices.patch_field_idents(span);

            let mut patch_statements = vec![];
            for (change_idx, field_idx) in changed_indices.iter().enumerate() {
                let field_idx = *field_idx as usize;

                if let Some(field) = self.fields.field_at_idx(field_idx) {
                    let field_to_patch =
                        Ident::new(&format!("field_{}", field.name.to_string()), span);
                    let patch = &patch_fields[change_idx];

                    patch_statements.push(quote! {
                        #field_to_patch.apply_patch(#patch);
                    })
                }
            }

            let patch_block = quote! {
                #patch_name::#patch_variant_name(#(#patch_fields),*) => {
                    match self {
                        #enum_name::#variant_name#variant_pattern_fields => {
                            #(#patch_statements)*
                        }
                        _ => { panic!("TODO: Return Result::Err") }
                    }
                }
            };
            patch_blocks.push(patch_block);
        }

        quote! {
            #(#patch_blocks)*
        }
    }
}

#[cfg(test)]
mod tests {
    //! All of the tests use the following enum variants
    //! ```
    //! # #[allow(unused)]
    //! enum MyEnum {
    //!     VariantA,
    //!     VariantB { some_field: Vec<f32>, another_field: Option<u64> },
    //!     VariantC (i16)
    //! }
    //! ```
    //!
    use super::*;
    use crate::multi_field_utils::{ParsedFields, StructOrTupleField};
    use crate::parsed_enum::EnumVariantFields;
    use crate::test_utils::assert_tokens_eq;
    use quote::__private::Span;
    use syn::Type;

    /// Verify that we generate the proper match block tokens for a variant that has not changed.
    #[test]
    fn variant_no_change() {
        let tokens = variant_a().generate_no_change_block(&enum_name());

        let expected = quote! {MyEnumDeltaOwned::VariantANoChange => {}};

        assert_tokens_eq(&tokens, &expected);
    }

    /// Verify that we generate a match block for changing to an enum variant that doesn't have any
    /// fields.
    #[test]
    fn changed_to_no_fields() {
        let tokens = variant_a().generate_changed_to_variant_block_no_fields(&enum_name());

        let expected = quote! {MyEnumDeltaOwned::ChangedToVariantVariantA => {
           *self = MyEnum::VariantA;
        }};

        assert_tokens_eq(&tokens, &expected);
    }

    /// Verify that we generate a match block for changing to a struct like enum variant.
    #[test]
    fn changed_to_struct_fields() {
        let tokens = variant_b().generate_changed_to_variant_block_with_fields(&enum_name());

        let expected = quote! {
            MyEnumDeltaOwned::ChangedToVariantVariantB(patch_some_field, patch_another_field) => {
                *self = MyEnum::VariantB {
                    some_field: patch_some_field,
                    another_field: patch_another_field
                };
            }
        };

        assert_tokens_eq(&tokens, &expected);
    }

    /// Verify that we generate a match block for applying patches to the same struct like variant.
    #[test]
    fn same_variant_struct_fields() {
        let tokens = variant_b().generate_field_changes(&enum_name(), &DipaAttrs::default());

        let expected = quote! {
            MyEnumDeltaOwned::VariantBChange_0(patch0) => {
                match self {
                    MyEnum::VariantB {
                        some_field: field_some_field, another_field: field_another_field
                    } => {
                        field_some_field.apply_patch(patch0);
                    }
                    _ => { panic!("TODO: Return Result::Err") }
                }
            }
            MyEnumDeltaOwned::VariantBChange_1(patch1) => {
                match self {
                    MyEnum::VariantB {
                        some_field: field_some_field, another_field: field_another_field
                    } => {
                        field_another_field.apply_patch(patch1);
                    }
                    _ => { panic!("TODO: Return Result::Err") }
                }
            }
            MyEnumDeltaOwned::VariantBChange_0_1(patch0, patch1) => {
                match self {
                    MyEnum::VariantB {
                        some_field: field_some_field, another_field: field_another_field
                    } => {
                        field_some_field.apply_patch(patch0);
                        field_another_field.apply_patch(patch1);
                    }
                    _ => { panic!("TODO: Return Result::Err") }
                }
            }
        };
        assert_tokens_eq(&tokens, &expected);
    }

    fn enum_name() -> Ident {
        Ident::new("MyEnum", Span::call_site())
    }

    fn variant_a() -> EnumVariant {
        EnumVariant {
            name: Ident::new("VariantA", Span::call_site()),
            fields: EnumVariantFields::Unit,
        }
    }

    fn variant_b() -> EnumVariant {
        let fields = vec![
            StructOrTupleField {
                name: quote! {some_field},
                ty: Type::Verbatim(quote! {Vec<f32>}),
                span: Span::call_site(),
            },
            StructOrTupleField {
                name: quote! {another_field},
                ty: Type::Verbatim(quote! {Option<u64>}),
                span: Span::call_site(),
            },
        ];
        EnumVariant {
            name: Ident::new("VariantB", Span::call_site()),
            fields: EnumVariantFields::Struct(ParsedFields {
                fields,
                span: Span::call_site(),
            }),
        }
    }
}
