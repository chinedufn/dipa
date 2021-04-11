use crate::dipa_attribute::DipaAttrs;
use crate::multi_field_utils::ChangedFieldIndices;
use crate::parsed_enum::{delta_type_name, EnumVariantFields, ParsedEnum};
use syn::__private::TokenStream2;
use syn::spanned::Spanned;
use syn::Ident;

impl ParsedEnum {
    /// Generate create_delta_towards tokens for an enum that has a single variant with multiple
    /// fields that is using the `field_batching_strategy = "one_batch"`.
    pub(super) fn generate_single_variant_multi_field_one_batch_create_delta_tokens(
        &self,
        dipa_attrs: &DipaAttrs,
    ) -> TokenStream2 {
        let enum_name = &self.name;

        let variant = &self.variants[0];
        let variant_name = &variant.name;

        let fields = &variant.fields;

        let start_fields = fields.to_pattern_match_tokens("start_");
        let end_fields = fields.to_pattern_match_tokens("end_");

        let field_diff_statements = self.field_diff_statements(fields);
        let did_change_tokens = self.did_change_tokens(fields);

        let delta_type_name = delta_type_name(enum_name);

        let mut match_blocks = vec![];

        for changed_indices in
            ChangedFieldIndices::all_changed_index_combinations(fields.len(), dipa_attrs)
        {
            let mut changed_as_bools = vec![quote! {false}; fields.len()];

            for idx in changed_indices.iter() {
                changed_as_bools[*idx as usize] = quote! {true};
            }

            let variant_name = changed_indices.variant_name_ident("", variant.name.span());

            let diffs = changed_indices.diffs(enum_name.span());

            let left = quote! {
                (#(#changed_as_bools),*)
            };
            let right = quote! {
                #delta_type_name::#variant_name(#(#diffs),*)
            };

            match_blocks.push(quote! {
               #left => #right,
            });
        }

        let all_false = vec![quote! {false}; fields.len()];

        quote! {
            match (&self, end_state) {
                (
                    #enum_name::#variant_name#start_fields,
                    #enum_name::#variant_name#end_fields
                ) => {
                    #(#field_diff_statements)*

                    let did_change = #(#did_change_tokens)||*;

                    let delta = match (#(#did_change_tokens),*) {
                        (#(#all_false),*) => #delta_type_name::NoChange,
                        #(#match_blocks)*
                    };

                    dipa::CreatedDelta {
                        delta,
                        did_change,
                    }
                }
            }
        }
    }

    /// let diff0 = start_fielda.create_delta_towards(&end_fielda);
    /// let diff1 = start_fieldb.create_delta_towards(&end_fieldb);
    fn field_diff_statements(&self, fields: &EnumVariantFields) -> Vec<TokenStream2> {
        fields
            .iter()
            .enumerate()
            .map(|(field_idx, field)| {
                let field_name = &field.name;

                let diff_idx_ident = Ident::new(&format!("diff{}", field_idx), field_name.span());

                let start_ident = format_ident!("start_{}", field.name.to_string());
                let end_ident = format_ident!("end_{}", field.name.to_string());

                quote! {
                    let #diff_idx_ident = #start_ident.create_delta_towards(&#end_ident);
                }
            })
            .collect()
    }

    fn did_change_tokens(&self, fields: &EnumVariantFields) -> Vec<TokenStream2> {
        fields
            .iter()
            .enumerate()
            .map(|(field_idx, field)| {
                let field_name = &field.name;

                let diff_idx_ident = Ident::new(&format!("diff{}", field_idx), field_name.span());

                quote! {
                    #diff_idx_ident.did_change
                }
            })
            .collect()
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

        let tokens = parsed_enum.generate_single_variant_multi_field_one_batch_create_delta_tokens(
            &DipaAttrs::default(),
        );

        let expected = quote! {
            match (&self, end_state) {
                (
                    MyEnum::MyVariant(start_0, start_1),
                    MyEnum::MyVariant(end_0, end_1)
                ) => {
                    let diff0 = start_0.create_delta_towards(&end_0);
                    let diff1 = start_1.create_delta_towards(&end_1);

                    let did_change = diff0.did_change || diff1.did_change;

                    let delta = match (diff0.did_change, diff1.did_change) {
                        (false, false) => MyEnumDelta::NoChange,
                        (true, false) => MyEnumDelta::Change_0(diff0.delta),
                        (false, true) => MyEnumDelta::Change_1(diff1.delta),
                        (true, true) => MyEnumDelta::Change_0_1(diff0.delta, diff1.delta),
                    };

                    dipa::CreatedDelta {
                        delta,
                        did_change,
                    }
                }
            }
        };

        assert_tokens_eq(&tokens, &expected);
    }
}
