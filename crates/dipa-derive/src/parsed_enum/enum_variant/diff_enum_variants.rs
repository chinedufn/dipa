use crate::dipa_attribute::DipaAttrs;
use crate::multi_field_utils::make_match_diff_tokens;
use crate::parsed_enum::{delta_type_name, EnumVariant};
use syn::__private::TokenStream2;
use syn::spanned::Spanned;
use syn::{Ident, Type};

impl EnumVariant {
    /// Create a match block diffing this enum variant with another.
    ///
    /// Only used for enums where at least one variant contains data
    /// (i.e. a tuple or sstruct variant).
    ///
    /// Enums that do not contain data use a simpler diff type and match block creation method.
    ///
    /// # Examples
    ///
    /// All of the examples in the different private methods are based on the following enum:
    ///
    /// ```
    /// # #[allow(unused)]
    /// enum MyEnum {
    ///     MyVariant,
    ///     AnotherVariant { some_field: Vec<f32>, another_field: Option<u64> }
    /// }
    /// ```
    pub fn diff_match_block_one_or_more_data(
        &self,
        other: &EnumVariant,
        enum_name: &Ident,
        dipa_attrs: &DipaAttrs,
    ) -> TokenStream2 {
        let left_variant = &self.name;
        let right_variant = &other.name;

        let same_variant = left_variant.to_string() == right_variant.to_string();

        if same_variant {
            return if self.fields.is_unit() {
                self.block_same_variant_no_data(enum_name)
            } else {
                self.block_same_variant_with_data(enum_name, other, dipa_attrs)
            };
        }

        if other.fields.len() == 0 {
            self.block_different_variant_no_data(enum_name, other)
        } else {
            self.block_different_variant_with_data(enum_name, other)
        }
    }

    /// ```
    /// # use quote::quote;
    /// quote! {
    ///   (
    ///       MyEnum::MyVariant,
    ///       MyEnum::MyVariant,
    ///   ) => {
    ///       let delta = MyEnumDelta::MyVariantNoChange;
    ///
    ///       dipa::CreatedDelta {
    ///           delta,
    ///           did_change: false,
    ///       }
    ///   }
    /// };
    /// ```
    fn block_same_variant_no_data(&self, enum_name: &Ident) -> TokenStream2 {
        let variant = &self.name;
        let variant_no_change = self.variant_no_change();

        let diff_ty = delta_type_name(enum_name);

        quote! {
            (
                #enum_name::#variant,
                #enum_name::#variant,
            ) => {
                dipa::CreatedDelta {
                    delta: #diff_ty::#variant_no_change,
                    did_change: false
                }
            }
        }
    }

    /// ```
    /// # use quote::quote;
    /// quote! {
    ///   (
    ///       MyEnum::AnotherVariant { some_field: start_some_field, another: start_another },
    ///       MyEnum::AnotherVariant { some_field: end_some_field, another: end_another },
    ///   ) => {
    ///       let diff0 = start_some_field.create_delta_towards(&end_some_field);
    ///       let diff1 = start_another.create_delta_towards(&end_another);
    ///
    ///       let delta = match (diff0.did_change, diff1.did_change) => {
    ///           (false, false) => MyEnumDelta::AnotherVariantNoChange,
    ///           (true, false) => MyEnumDelta::AnotherVariantChange_0(diff_0.delta),
    ///           (false, true) => MyEnumDelta::AnotherVariantChange_1(diff_1.delta),
    ///           (true, true) => MyEnumDelta::AnotherVariantChange_0_1(diff_0.delta, diff_1.delta)
    ///       };
    ///
    ///       let did_change = diff_0.did_change || diff_1.did_change;
    ///
    ///       dipa::CreatedDelta {
    ///           delta,
    ///           did_change,
    ///       }
    ///   }
    /// };
    /// ```
    fn block_same_variant_with_data(
        &self,
        enum_name: &Ident,
        other: &EnumVariant,
        dipa_attrs: &DipaAttrs,
    ) -> TokenStream2 {
        let variant = &self.name;

        let diff_ty = delta_type_name(enum_name);

        let start_pattern_fields = self.fields.to_pattern_match_tokens("start_");
        let end_pattern_fields = other.fields.to_pattern_match_tokens("end_");

        let field_diff_statements = self.field_diff_statements(other);

        let match_diff_statements = make_match_diff_tokens(
            Type::Verbatim(quote! {#diff_ty}),
            other.name.to_string().trim(),
            enum_name.span(),
            &other.fields,
            &dipa_attrs,
        );

        quote! {
            (
                #enum_name::#variant#start_pattern_fields,
                #enum_name::#variant#end_pattern_fields,
            ) => {
                #(#field_diff_statements)*

                #match_diff_statements

                dipa::CreatedDelta {
                    delta,
                    did_change,
                }
            }
        }
    }

    /// ```
    /// # use quote::quote;
    /// quote! {
    ///   (
    ///       MyEnum::MyVariant,
    ///       MyEnum::AnotherVariant,
    ///   ) => {
    ///       dipa::CreatedDelta {
    ///           delta: MyEnumDelta::ChangedToAnotherVariant,
    ///           did_change: false,
    ///       }
    ///   }
    /// };
    /// ```
    fn block_different_variant_no_data(
        &self,
        enum_name: &Ident,
        other: &EnumVariant,
    ) -> TokenStream2 {
        let variant_1 = &self.name;
        let variant_1_fields = self.fields.to_pattern_match_tokens("_");

        let variant_2 = &other.name;

        let changed_to_variant = other.changed_to_variant();

        let diff_ty = delta_type_name(enum_name);

        quote! {
            (
                #enum_name::#variant_1#variant_1_fields,
                #enum_name::#variant_2,
            ) => {
                dipa::CreatedDelta {
                    delta: #diff_ty::#changed_to_variant,
                    did_change: true,
                }
            }
        }
    }

    /// ```
    /// # use quote::quote;
    /// quote! {
    ///   (
    ///       MyEnum::MyVariant,
    ///       MyEnum::AnotherVariant { some_field, another_field },
    ///   ) => {
    ///       dipa::CreatedDelta {
    ///           delta: MyEnumDelta::ChangedToAnotherVariant(some_field, another_field),
    ///           did_change: true,
    ///       }
    ///   }
    /// };
    /// ```
    fn block_different_variant_with_data(
        &self,
        enum_name: &Ident,
        other: &EnumVariant,
    ) -> TokenStream2 {
        let variant_1 = &self.name;
        let variant_1_pattern_fields = self.fields.to_pattern_match_tokens("_");

        let variant_2 = &other.name;

        let changed_to_variant = other.changed_to_variant();

        let diff_ty = delta_type_name(enum_name);

        let variant_2_pattern_fields = other.fields.to_pattern_match_tokens("end_");
        let variant_2_field_values = other.fields.to_field_value_tokens_parenthesized("end_");

        quote! {
            (
                #enum_name::#variant_1#variant_1_pattern_fields,
                #enum_name::#variant_2#variant_2_pattern_fields,
            ) => {
                dipa::CreatedDelta {
                    delta: #diff_ty::#changed_to_variant#variant_2_field_values,
                    did_change: true,
                }
            }
        }
    }

    /// Generate code to diff every field in a struct or tuple variant.
    ///
    /// let diff_0 = start0.create_delta_towards(&end0);
    /// let diff_1 = start1.create_delta_towards(&end1);
    fn field_diff_statements(&self, other: &EnumVariant) -> Vec<TokenStream2> {
        self.fields
            .iter()
            .enumerate()
            .map(|(field_idx, field)| {
                let field_name = &field.name;

                let diff_idx_ident = Ident::new(&format!("diff{}", field_idx), field_name.span());

                let start_ident = &field.prefixed_name("start_");
                let end_ident = &other.fields[field_idx].prefixed_name("end_");

                quote! {
                let #diff_idx_ident = #start_ident.create_delta_towards(&#end_ident);
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::multi_field_utils::{ParsedFields, StructOrTupleField};
    use crate::parsed_enum::EnumVariantFields;
    use crate::test_utils::assert_tokens_eq;
    use syn::Type;
    use syn::__private::Span;

    /// Verify that we properly generate tokens for two of the same variant with no fields.
    #[test]
    fn same_variant_no_fields() {
        let enum_variant = EnumVariant {
            name: variant_name_1(),
            fields: EnumVariantFields::Unit,
        };

        let tokens = enum_variant.diff_match_block_one_or_more_data(
            &enum_variant,
            &enum_name(),
            &DipaAttrs::default(),
        );

        let expected_tokens = quote! {
          (
              MyEnum::FirstVariant,
              MyEnum::FirstVariant,
          ) => {
              dipa::CreatedDelta {
                  delta: MyEnumDelta::FirstVariantNoChange,
                  did_change: false
              }
          }
        };

        assert_tokens_eq(&tokens, &expected_tokens);
    }

    /// Verify that we properly generate tokens for a diff where the new variant has no fields.
    #[test]
    fn different_variant_new_has_no_fields() {
        let old_variant = EnumVariant {
            name: variant_name_1(),
            fields: EnumVariantFields::Unit,
        };

        let new_variant = EnumVariant {
            name: variant_name_2(),
            fields: EnumVariantFields::Unit,
        };

        let tokens = old_variant.diff_match_block_one_or_more_data(
            &new_variant,
            &enum_name(),
            &DipaAttrs::default(),
        );

        let expected_tokens = quote! {
            (
                MyEnum::FirstVariant,
                MyEnum::Variant2,
            ) => {
                dipa::CreatedDelta {
                    delta: MyEnumDelta::ChangedToVariantVariant2,
                    did_change: true,
                }
            }
        };

        assert_tokens_eq(&tokens, &expected_tokens);
    }

    /// Verify that we properly generate a match block for two different variants where the new
    /// variant is struct like with one field.
    #[test]
    fn different_variant_new_has_one_struct_field() {
        let old_variant = EnumVariant {
            name: variant_name_1(),
            fields: EnumVariantFields::Unit,
        };

        let ty = Type::Verbatim(quote! {u32});
        let fields = vec![StructOrTupleField {
            name: quote! {field},
            ty,
            span: Span::call_site(),
        }];
        let new_variant = EnumVariant {
            name: variant_name_2(),
            fields: EnumVariantFields::Struct(ParsedFields {
                fields,
                span: Span::call_site(),
            }),
        };

        let tokens = old_variant.diff_match_block_one_or_more_data(
            &new_variant,
            &enum_name(),
            &DipaAttrs::default(),
        );

        let expected_tokens = quote! {
            (
                MyEnum::FirstVariant,
                MyEnum::Variant2 { field: end_field },
            ) => {
                dipa::CreatedDelta {
                    delta: MyEnumDelta::ChangedToVariantVariant2(end_field),
                    did_change: true,
                }
            }
        };

        assert_tokens_eq(&tokens, &expected_tokens);
    }

    /// Verify that we properly generate a match block for two different variants where the new
    /// variant is tuple like with one entry.
    #[test]
    fn different_variant_new_has_one_tuple_field() {
        let old_variant = EnumVariant {
            name: variant_name_1(),
            fields: EnumVariantFields::Unit,
        };

        let ty = Type::Verbatim(quote! {u32});
        let fields = vec![StructOrTupleField {
            name: quote! {field},
            ty,
            span: Span::call_site(),
        }];
        let new_variant = EnumVariant {
            name: variant_name_2(),
            fields: EnumVariantFields::Tuple(ParsedFields {
                fields,
                span: Span::call_site(),
            }),
        };

        let tokens = old_variant.diff_match_block_one_or_more_data(
            &new_variant,
            &enum_name(),
            &DipaAttrs::default(),
        );

        let expected_tokens = quote! {
            (
                MyEnum::FirstVariant,
                MyEnum::Variant2(end_field),
            ) => {
                dipa::CreatedDelta {
                    delta: MyEnumDelta::ChangedToVariantVariant2(end_field),
                    did_change: true,
                }
            }
        };

        assert_tokens_eq(&tokens, &expected_tokens);
    }

    /// Verify that we properly generate a match block for the same enum variant with two tuple
    /// fields.
    #[test]
    fn same_variant_new_has_two_tuple_fields() {
        let ty = Type::Verbatim(quote! {u32});

        let fields = vec![
            StructOrTupleField {
                name: quote! {0},
                ty: ty.clone(),
                span: Span::call_site(),
            },
            StructOrTupleField {
                name: quote! {1},
                ty: ty.clone(),
                span: Span::call_site(),
            },
        ];
        let old_variant = EnumVariant {
            name: variant_name_2(),
            fields: EnumVariantFields::Tuple(ParsedFields {
                fields,
                span: Span::call_site(),
            }),
        };

        let fields = vec![
            StructOrTupleField {
                name: quote! {0},
                ty: ty.clone(),
                span: Span::call_site(),
            },
            StructOrTupleField {
                name: quote! {1},
                ty,
                span: Span::call_site(),
            },
        ];
        let new_variant = EnumVariant {
            name: variant_name_2(),
            fields: EnumVariantFields::Tuple(ParsedFields {
                fields,
                span: Span::call_site(),
            }),
        };

        let tokens = old_variant.diff_match_block_one_or_more_data(
            &new_variant,
            &enum_name(),
            &DipaAttrs::default(),
        );

        let expected_tokens = quote! {
            (
                MyEnum::Variant2(start_0, start_1),
                MyEnum::Variant2(end_0, end_1),
            ) => {
                let diff0 = start_0.create_delta_towards(&end_0);
                let diff1 = start_1.create_delta_towards(&end_1);

                let delta = match (diff0.did_change, diff1.did_change) {
                    (false, false) => MyEnumDelta::Variant2NoChange,
                    (true, false) => MyEnumDelta::Variant2Change_0(diff0.delta),
                    (true, true) => MyEnumDelta::Variant2Change_0_1(diff0.delta, diff1.delta),
                    (false, true) => MyEnumDelta::Variant2Change_1(diff1.delta),
                };

                let did_change = diff0.did_change || diff1.did_change;

                dipa::CreatedDelta {
                    delta,
                    did_change,
                }
            }
        };

        assert_tokens_eq(&tokens, &expected_tokens);
    }

    /// Verify that we properly generate a match block for the same enum variant with two struct
    /// fields.
    #[test]
    fn same_variant_new_has_two_struct_fields() {
        let ty = Type::Verbatim(quote! {u32});

        let fields = vec![
            StructOrTupleField {
                name: quote! {field_a},
                ty: ty.clone(),
                span: Span::call_site(),
            },
            StructOrTupleField {
                name: quote! {field_b},
                ty: ty.clone(),
                span: Span::call_site(),
            },
        ];
        let old_variant = EnumVariant {
            name: variant_name_2(),
            fields: EnumVariantFields::Struct(ParsedFields {
                fields,
                span: Span::call_site(),
            }),
        };

        let fields = vec![
            StructOrTupleField {
                name: quote! {field_a},
                ty: ty.clone(),
                span: Span::call_site(),
            },
            StructOrTupleField {
                name: quote! {field_b},
                ty,
                span: Span::call_site(),
            },
        ];
        let new_variant = EnumVariant {
            name: variant_name_2(),
            fields: EnumVariantFields::Struct(ParsedFields {
                fields,
                span: Span::call_site(),
            }),
        };

        let tokens = old_variant.diff_match_block_one_or_more_data(
            &new_variant,
            &enum_name(),
            &DipaAttrs::default(),
        );

        let expected_tokens = quote! {
            (
                MyEnum::Variant2 { field_a: start_field_a, field_b: start_field_b },
                MyEnum::Variant2 { field_a: end_field_a, field_b: end_field_b },
            ) => {
                let diff0 = start_field_a.create_delta_towards(&end_field_a);
                let diff1 = start_field_b.create_delta_towards(&end_field_b);

                let delta = match (diff0.did_change, diff1.did_change) {
                    (false, false) => MyEnumDelta::Variant2NoChange,
                    (true, false) => MyEnumDelta::Variant2Change_0(diff0.delta),
                    (true, true) => MyEnumDelta::Variant2Change_0_1(diff0.delta, diff1.delta),
                    (false, true) => MyEnumDelta::Variant2Change_1(diff1.delta),
                };

                let did_change = diff0.did_change || diff1.did_change;

                dipa::CreatedDelta {
                    delta,
                    did_change,
                }
            }
        };

        assert_tokens_eq(&tokens, &expected_tokens);
    }

    fn enum_name() -> Ident {
        Ident::new("MyEnum", Span::call_site())
    }

    fn variant_name_1() -> Ident {
        Ident::new("FirstVariant", Span::call_site())
    }

    fn variant_name_2() -> Ident {
        Ident::new("Variant2", Span::call_site())
    }
}
