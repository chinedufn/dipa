use crate::dipa_attribute::DipaAttrs;
use crate::multi_field_utils::{ChangedFieldIndices, ParsedFields};
use syn::__private::TokenStream2;

impl ParsedFields {
    pub(super) fn generate_delta_type_one_batch(
        &self,
        prefix: &str,
        dipa_attrs: &DipaAttrs,
    ) -> TokenStream2 {
        let delta_name = self.delta_name(prefix);
        let delta_owned_name = self.delta_owned_name(prefix);

        let changed_field_indices =
            ChangedFieldIndices::all_changed_index_combinations(self.fields.len(), dipa_attrs);

        let mut ref_variants = vec![];
        let mut owned_variants = vec![];

        for change_combinations in changed_field_indices {
            let variant_name = change_combinations.variant_name_ident("", self.span);

            let mut changed_delta_tys = vec![];
            let mut changed_owned_tys = vec![];

            for idx in change_combinations.iter() {
                let ty = &self.fields[*idx as usize].ty;
                changed_delta_tys.push(quote! {
                    <#ty as dipa::Diffable<'s, 'e, #ty>>::Delta
                });
                changed_owned_tys.push(quote! {
                    <#ty as dipa::Diffable<'static, 'static, #ty>>::DeltaOwned
                });
            }

            let changed_delta_tys = if changed_delta_tys.len() == 0 {
                quote! {}
            } else {
                quote! {(#(#changed_delta_tys),*)}
            };

            let changed_owned_tys = if changed_owned_tys.len() == 0 {
                quote! {}
            } else {
                quote! {(#(#changed_owned_tys),*)}
            };

            ref_variants.push(quote! {
                #variant_name#changed_delta_tys
            });
            owned_variants.push(quote! {
                #variant_name#changed_owned_tys
            });
        }

        let (diff_derives, patch_derives) = (&dipa_attrs.diff_derives, &dipa_attrs.patch_derives);

        quote! {
            #[derive(serde::Serialize, #(#diff_derives),*)]
            #[allow(non_camel_case_types, missing_docs)]
            pub enum #delta_name<'s, 'e> {
                NoChange,
                #(#ref_variants),*
            }

            #[derive(serde::Deserialize, #(#patch_derives),*)]
            #[allow(non_camel_case_types, missing_docs)]
            pub enum #delta_owned_name {
                NoChange,
                #(#owned_variants),*
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dipa_attribute::FieldBatchingStrategy;
    use crate::multi_field_utils::StructOrTupleField;
    use crate::test_utils::assert_tokens_eq;
    use syn::Type;
    use syn::__private::Span;

    /// Verify that if the batching strategy is one_batch that we generate a single enum as the
    /// delta type.
    #[test]
    fn generates_one_batch() {
        let mut attrs = DipaAttrs::default();
        attrs.field_batching_strategy = Some(FieldBatchingStrategy::OneBatch);

        let parsed_fields = ParsedFields {
            fields: vec![
                StructOrTupleField {
                    name: Default::default(),
                    ty: Type::Verbatim(quote! {u16}),
                    span: Span::call_site(),
                },
                StructOrTupleField {
                    name: Default::default(),
                    ty: Type::Verbatim(quote! {u32}),
                    span: Span::call_site(),
                },
            ],
            span: Span::call_site(),
        };
        let tokens = parsed_fields.generate_delta_types("MyStruct", &attrs);

        let expected = quote! {
            #[derive(serde::Serialize,)]
            #[allow(non_camel_case_types, missing_docs)]
            pub enum MyStructDelta<'s, 'e> {
                NoChange,
                Change_0(<u16 as dipa::Diffable<'s, 'e, u16>>::Delta),
                Change_1(<u32 as dipa::Diffable<'s, 'e, u32>>::Delta),
                Change_0_1(
                    <u16 as dipa::Diffable<'s, 'e, u16>>::Delta,
                    <u32 as dipa::Diffable<'s, 'e, u32>>::Delta
                )
            }

            #[derive(serde::Deserialize,)]
            #[allow(non_camel_case_types, missing_docs)]
            pub enum MyStructDeltaOwned {
                NoChange,
                Change_0(<u16 as dipa::Diffable<'static, 'static, u16>>::DeltaOwned),
                Change_1(<u32 as dipa::Diffable<'static, 'static, u32>>::DeltaOwned),
                Change_0_1(
                    <u16 as dipa::Diffable<'static, 'static, u16>>::DeltaOwned,
                    <u32 as dipa::Diffable<'static, 'static, u32>>::DeltaOwned
                )
            }
        };

        assert_tokens_eq(&tokens, &expected);
    }
}
