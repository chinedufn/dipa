use crate::dipa_attribute::DipaAttrs;
use crate::multi_field_utils::ChangedFieldIndices;
use crate::parsed_enum::{delta_owned_type_name, ParsedEnum};
use syn::__private::TokenStream2;

impl ParsedEnum {
    /// Generate apply_patch tokens for an enum that has a single variant with multiple
    /// fields that is using the `field_batching_strategy = "one_batch"`.
    pub(super) fn generate_single_variant_multi_field_one_batch_apply_patch_tokens(
        &self,
        dipa_attrs: &DipaAttrs,
    ) -> TokenStream2 {
        let enum_name = &self.name;
        let delta_owned_name = delta_owned_type_name(enum_name);

        let variant = &self.variants[0];
        let variant_name = &variant.name;

        let fields = &variant.fields;

        let field_patterns = variant.fields.to_pattern_match_tokens("field_");

        let mut patch_blocks = vec![];

        for changed_indices in
            ChangedFieldIndices::all_changed_index_combinations(fields.len(), dipa_attrs)
        {
            let change_name = changed_indices.variant_name_ident("", variant_name.span());
            let patches = changed_indices.patch_field_idents(variant_name.span());

            let mut field_applies = vec![];

            for (idx, field_idx) in changed_indices.iter().enumerate() {
                let field_idx = *field_idx as usize;

                let field = &fields[field_idx];
                let field_name = &field.name;

                let field_name = format_ident!("field_{}", field_name.to_string());

                let patch = &patches[idx];

                field_applies.push(quote! {
                    #field_name.apply_patch(#patch);
                })
            }

            patch_blocks.push(quote! {
                #delta_owned_name::#change_name(#(#patches),*) => {
                    #(#field_applies)*
                }
            });
        }

        quote! {
            match self {
                #enum_name::#variant_name#field_patterns => {
                    match patch {
                        #delta_owned_name::NoChange => {}
                        #(#patch_blocks)*
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::assert_tokens_eq;

    /// Verify that we properly generate the tokens for determining the delta between two single
    /// variant multi field `field_batching_strategy = "one_batch"` enums.
    #[test]
    fn generates_tokens() {
        let parsed_enum = ParsedEnum::new_test_one_variant_two_unnamed_fields();

        let tokens =
            parsed_enum.generate_single_variant_multi_field_one_batch_apply_patch_tokens(
                &DipaAttrs::default(),
            );

        let expected = quote! {
            match self {
                MyEnum::MyVariant(field_0, field_1) => {
                    match patch {
                        MyEnumDeltaOwned::NoChange => {}
                        MyEnumDeltaOwned::Change_0(patch0) => {
                            field_0.apply_patch(patch0);
                        }
                        MyEnumDeltaOwned::Change_1(patch1) => {
                            field_1.apply_patch(patch1);
                        }
                        MyEnumDeltaOwned::Change_0_1(patch0, patch1) => {
                            field_0.apply_patch(patch0);
                            field_1.apply_patch(patch1);
                        }
                    }
                }
            }
        };

        assert_tokens_eq(&tokens, &expected);
    }
}
