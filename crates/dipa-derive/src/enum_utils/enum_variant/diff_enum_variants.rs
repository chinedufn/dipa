use crate::enum_utils::{diff_type_name, EnumVariant};
use crate::multi_field_utils::make_match_diff_tokens;
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
    ) -> TokenStream2 {
        let left_variant = &self.name;
        let right_variant = &other.name;

        let same_variant = left_variant.to_string() == right_variant.to_string();

        if same_variant {
            if self.fields.is_unit() {
                return self.block_same_variant_no_data(enum_name);
            } else {
                return self.block_same_variant_with_data(enum_name, other);
            }
        }

        if other.fields.len() == 0 {
            self.block_different_variant_no_data(enum_name, other)
        } else {
            self.block_different_variant_with_data(enum_name, other)
        }
    }

    /// ```
    /// quote! {
    ///   (
    ///       MyEnum::MyVariant,
    ///       MyEnum::MyVariant,
    ///   ) => {
    ///       let macro_hints = dipa::MacroOptimizationHints {
    ///           did_change: false
    ///       };
    ///
    ///       let diff = MyEnumDiff::MyVariantNoChange;
    ///
    ///       (diff, macro_hints)
    ///   }
    /// };
    /// ```
    fn block_same_variant_no_data(&self, enum_name: &Ident) -> TokenStream2 {
        let macro_hints_no_change = self.create_macro_hints_no_change();

        let variant = &self.name;
        let variant_no_change = self.variant_no_change();

        let diff_ty = diff_type_name(enum_name);

        quote! {
            (
                #enum_name::#variant,
                #enum_name::#variant,
            ) => {
                #macro_hints_no_change

                let diff = #diff_ty::#variant_no_change;

                (diff, macro_hints)
            }
        }
    }

    /// ```
    /// quote! {
    ///   (
    ///       MyEnum::AnotherVariant { some_field: start_some_field, another: start_another },
    ///       MyEnum::AnotherVariant { some_field: end_some_field, another: end_another },
    ///   ) => {
    ///       let diff0 = start_some_field.create_delta_towards(&end_some_field);
    ///       let diff1 = start_another.create_delta_towards(&end_another);
    ///
    ///       let diff = match (diff0.1.did_change, diff1.1.did_change) => {
    ///           (false, false) => MyEnumDiff::AnotherVariantNoChange,
    ///           (true, false) => MyEnumDiff::AnotherVariantChange_0(diff_0.0),
    ///           (false, true) => MyEnumDiff::AnotherVariantChange_1(diff_1.0),
    ///           (true, true) => MyEnumDiff::AnotherVariantChange_0_1(diff_0.0, diff_1.0)
    ///       };
    ///
    ///       let did_change = diff_0.1.did_change || diff_1.1.did_change;
    ///
    ///       let macro_hints = dipa::MacroOptimizationHints {
    ///           did_change
    ///       };
    ///
    ///       (diff, macro_hints)
    ///   }
    /// };
    /// ```
    fn block_same_variant_with_data(&self, enum_name: &Ident, other: &EnumVariant) -> TokenStream2 {
        let macro_hints_based_on_did_change = self.create_macro_hints_based_on_did_change();

        let variant = &self.name;
        let variant_no_change = self.variant_no_change();

        let diff_ty = diff_type_name(enum_name);

        let start_pattern_fields = self.fields.to_pattern_match_tokens("start_");
        let end_pattern_fields = other.fields.to_pattern_match_tokens("end_");

        let field_diff_statements = self.field_diff_statements(other);

        let match_diff_statements = make_match_diff_tokens(
            Type::Verbatim(quote! {#diff_ty}),
            other.name.to_string().trim(),
            enum_name.span(),
            &other.fields,
        );

        quote! {
            (
                #enum_name::#variant#start_pattern_fields,
                #enum_name::#variant#end_pattern_fields,
            ) => {
                #(#field_diff_statements)*

                #match_diff_statements

                #macro_hints_based_on_did_change

                (diff, macro_hints)
            }
        }
    }

    /// ```
    /// quote! {
    ///   (
    ///       MyEnum::MyVariant,
    ///       MyEnum::AnotherVariant,
    ///   ) => {
    ///       let macro_hints = dipa::MacroOptimizationHints {
    ///           did_change: false
    ///       };
    ///
    ///       let diff = MyEnumDiff::ChangedToAnotherVariant;
    ///
    ///       (diff, macro_hints)
    ///   }
    /// };
    /// ```
    fn block_different_variant_no_data(
        &self,
        enum_name: &Ident,
        other: &EnumVariant,
    ) -> TokenStream2 {
        let macro_hints_no_change = self.create_macro_hints_changed();

        let variant_1 = &self.name;
        let variant_1_fields = self.fields.to_pattern_match_tokens("_");

        let variant_2 = &other.name;

        let changed_to_variant = other.changed_to_variant();

        let diff_ty = diff_type_name(enum_name);

        quote! {
            (
                #enum_name::#variant_1#variant_1_fields,
                #enum_name::#variant_2,
            ) => {
                #macro_hints_no_change

                let diff = #diff_ty::#changed_to_variant;

                (diff, macro_hints)
            }
        }
    }

    /// ```
    /// quote! {
    ///   (
    ///       MyEnum::MyVariant,
    ///       MyEnum::AnotherVariant { some_field, another_field },
    ///   ) => {
    ///       let macro_hints = dipa::MacroOptimizationHints {
    ///           did_change: true
    ///       };
    ///
    ///       let diff = MyEnumDiff::ChangedToAnotherVariant(some_field, another_field);
    ///
    ///       (diff, macro_hints)
    ///   }
    /// };
    /// ```
    fn block_different_variant_with_data(
        &self,
        enum_name: &Ident,
        other: &EnumVariant,
    ) -> TokenStream2 {
        let macro_hints_no_change = self.create_macro_hints_changed();

        let variant_1 = &self.name;
        let variant_1_pattern_fields = self.fields.to_pattern_match_tokens("_");

        let variant_2 = &other.name;

        let changed_to_variant = other.changed_to_variant();

        let diff_ty = diff_type_name(enum_name);

        let variant_2_pattern_fields = other.fields.to_pattern_match_tokens("end_");
        let variant_2_field_values = other.fields.to_field_value_tokens_parenthesized("end_");

        quote! {
            (
                #enum_name::#variant_1#variant_1_pattern_fields,
                #enum_name::#variant_2#variant_2_pattern_fields,
            ) => {
                #macro_hints_no_change

                let diff = #diff_ty::#changed_to_variant#variant_2_field_values;

                (diff, macro_hints)
            }
        }
    }

    fn create_macro_hints_no_change(&self) -> TokenStream2 {
        quote! {
          let macro_hints = dipa::MacroOptimizationHints {
              did_change: false
          };
        }
    }

    fn create_macro_hints_changed(&self) -> TokenStream2 {
        quote! {
          let macro_hints = dipa::MacroOptimizationHints {
              did_change: true
          };
        }
    }

    fn create_macro_hints_based_on_did_change(&self) -> TokenStream2 {
        quote! {
          let macro_hints = dipa::MacroOptimizationHints {
              did_change
          };
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
    use crate::enum_utils::EnumVariantFields;
    use crate::multi_field_utils::StructOrTupleField;
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

        let tokens = enum_variant.diff_match_block_one_or_more_data(&enum_variant, &enum_name());

        let expected_tokens = quote! {
          (
              MyEnum::FirstVariant,
              MyEnum::FirstVariant,
          ) => {
              let macro_hints = dipa::MacroOptimizationHints {
                  did_change: false
              };

              let diff = MyEnumDiff::FirstVariantNoChange;

              (diff, macro_hints)
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

        let tokens = old_variant.diff_match_block_one_or_more_data(&new_variant, &enum_name());

        let expected_tokens = quote! {
            (
                MyEnum::FirstVariant,
                MyEnum::Variant2,
            ) => {
                let macro_hints = dipa::MacroOptimizationHints {
                    did_change: true
                };

                let diff = MyEnumDiff::ChangedToVariantVariant2;

                (diff, macro_hints)
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
        let new_variant = EnumVariant {
            name: variant_name_2(),
            fields: EnumVariantFields::Struct(vec![StructOrTupleField {
                name: quote! {field},
                ty: ty,
                span: Span::call_site(),
            }]),
        };

        let tokens = old_variant.diff_match_block_one_or_more_data(&new_variant, &enum_name());

        let expected_tokens = quote! {
            (
                MyEnum::FirstVariant,
                MyEnum::Variant2 { field: end_field },
            ) => {
                let macro_hints = dipa::MacroOptimizationHints {
                    did_change: true
                };

                let diff = MyEnumDiff::ChangedToVariantVariant2(end_field);

                (diff, macro_hints)
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
        let new_variant = EnumVariant {
            name: variant_name_2(),
            fields: EnumVariantFields::Tuple(vec![StructOrTupleField {
                name: quote! {field},
                ty: ty,
                span: Span::call_site(),
            }]),
        };

        let tokens = old_variant.diff_match_block_one_or_more_data(&new_variant, &enum_name());

        let expected_tokens = quote! {
            (
                MyEnum::FirstVariant,
                MyEnum::Variant2(end_field),
            ) => {
                let macro_hints = dipa::MacroOptimizationHints {
                    did_change: true
                };

                let diff = MyEnumDiff::ChangedToVariantVariant2(end_field);

                (diff, macro_hints)
            }
        };

        assert_tokens_eq(&tokens, &expected_tokens);
    }

    /// Verify that we properly generate a match block for the same enum variant with two tuple
    /// fields.
    #[test]
    fn same_variant_new_has_two_tuple_fields() {
        let ty = Type::Verbatim(quote! {u32});

        let old_variant = EnumVariant {
            name: variant_name_2(),
            fields: EnumVariantFields::Tuple(vec![
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
            ]),
        };

        let new_variant = EnumVariant {
            name: variant_name_2(),
            fields: EnumVariantFields::Tuple(vec![
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
            ]),
        };

        let tokens = old_variant.diff_match_block_one_or_more_data(&new_variant, &enum_name());

        let expected_tokens = quote! {
            (
                MyEnum::Variant2(start_0, start_1),
                MyEnum::Variant2(end_0, end_1),
            ) => {
                let diff0 = start_0.create_delta_towards(&end_0);
                let diff1 = start_1.create_delta_towards(&end_1);

                let diff = match (diff0.1.did_change, diff1.1.did_change) {
                    (false, false) => MyEnumDiff::Variant2NoChange,
                    (false, true) => MyEnumDiff::Variant2Change_1(diff1.0),
                    (true, false) => MyEnumDiff::Variant2Change_0(diff0.0),
                    (true, true) => MyEnumDiff::Variant2Change_0_1(diff0.0, diff1.0),
                };

                let did_change = diff0.1.did_change || diff1.1.did_change;

                let macro_hints = dipa::MacroOptimizationHints {
                    did_change
                };

                (diff, macro_hints)
            }
        };

        assert_tokens_eq(&tokens, &expected_tokens);
    }

    /// Verify that we properly generate a match block for the same enum variant with two struct
    /// fields.
    #[test]
    fn same_variant_new_has_two_struct_fields() {
        let ty = Type::Verbatim(quote! {u32});

        let old_variant = EnumVariant {
            name: variant_name_2(),
            fields: EnumVariantFields::Struct(vec![
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
            ]),
        };

        let new_variant = EnumVariant {
            name: variant_name_2(),
            fields: EnumVariantFields::Struct(vec![
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
            ]),
        };

        let tokens = old_variant.diff_match_block_one_or_more_data(&new_variant, &enum_name());

        let expected_tokens = quote! {
            (
                MyEnum::Variant2 { field_a: start_field_a, field_b: start_field_b },
                MyEnum::Variant2 { field_a: end_field_a, field_b: end_field_b },
            ) => {
                let diff0 = start_field_a.create_delta_towards(&end_field_a);
                let diff1 = start_field_b.create_delta_towards(&end_field_b);

                let diff = match (diff0.1.did_change, diff1.1.did_change) {
                    (false, false) => MyEnumDiff::Variant2NoChange,
                    (false, true) => MyEnumDiff::Variant2Change_1(diff1.0),
                    (true, false) => MyEnumDiff::Variant2Change_0(diff0.0),
                    (true, true) => MyEnumDiff::Variant2Change_0_1(diff0.0, diff1.0),
                };

                let did_change = diff0.1.did_change || diff1.1.did_change;

                let macro_hints = dipa::MacroOptimizationHints {
                    did_change
                };

                (diff, macro_hints)
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
