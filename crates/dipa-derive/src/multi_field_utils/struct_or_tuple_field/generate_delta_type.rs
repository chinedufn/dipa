use crate::dipa_attribute::{DipaAttrs, FieldBatchingStrategy};
use crate::multi_field_utils::{ChangedFieldIndices, ParsedFields};
use syn::Ident;
use syn::__private::TokenStream2;

impl ParsedFields {
    /// Given named or unnamed fields return a Delta type that encompasses all of those fields.
    /// Depending on the field batching strategy this might be an enum or a struct.
    pub fn generate_delta_type(
        &self,
        prefix: &str,
        field_batching_strategy: FieldBatchingStrategy,
        dipa_attrs: Option<&DipaAttrs>,
    ) -> TokenStream2 {
        if self.len() < 2 {
            unreachable!(
                r#"Out logic is spread out a bit. Need to move the logic for 0 and 1 field
into this function."#
            )
        }

        match field_batching_strategy {
            FieldBatchingStrategy::OneBatch => {
                let delta_name = self.delta_name(prefix);
                let delta_owned_name = self.delta_owned_name(prefix);

                let changed_field_indices =
                    ChangedFieldIndices::all_changed_index_combinations(self.fields.len());

                let mut ref_variants = vec![];
                let mut owned_variants = vec![];

                for change_combinations in changed_field_indices {
                    let variant_name = change_combinations.variant_name_ident("", self.span);

                    let mut changed_delta_tys = vec![];
                    let mut changed_owned_tys = vec![];

                    for idx in change_combinations.iter() {
                        let ty = &self.fields[*idx as usize].ty;
                        changed_delta_tys.push(quote! {
                            <#ty as dipa::Diffable<'p, #ty>>::Delta
                        });
                        changed_owned_tys.push(quote! {
                            <#ty as dipa::Diffable<'static, #ty>>::DeltaOwned
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

                let (diff_derives, patch_derives) = if let Some(attrs) = dipa_attrs.as_ref() {
                    attrs.parse_diff_and_patch_derives()
                } else {
                    (vec![], vec![])
                };

                quote! {
                    #[derive(serde::Serialize, #(#diff_derives),*)]
                    #[allow(non_camel_case_types, missing_docs)]
                    enum #delta_name<'p> {
                        NoChange,
                        #(#ref_variants),*
                    }

                    #[derive(serde::Deserialize, #(#patch_derives),*)]
                    #[allow(non_camel_case_types, missing_docs)]
                    enum #delta_owned_name {
                        NoChange,
                        #(#owned_variants),*
                    }
                }
            }
            FieldBatchingStrategy::ManyBatches => {
                unimplemented!()
            }
            FieldBatchingStrategy::NoBatching => {
                unimplemented!()
            }
        }
    }

    pub fn delta_name(&self, prefix: &str) -> Ident {
        Ident::new(&format!("{}Delta", prefix), self.span)
    }

    pub fn delta_owned_name(&self, prefix: &str) -> Ident {
        Ident::new(&format!("{}DeltaOwned", prefix), self.span)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::multi_field_utils::StructOrTupleField;
    use crate::test_utils::assert_tokens_eq;
    use syn::Type;
    use syn::__private::Span;

    /// Verify that if the batching strategy is one_batch that we generate a single enum as the
    /// delta type.
    #[test]
    fn generates_one_batch() {
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
        let tokens =
            parsed_fields.generate_delta_type("MyStruct", FieldBatchingStrategy::OneBatch, None);

        let expected = quote! {
            #[derive(Serialize,)]
            #[allow(non_camel_case_types, missing_docs)]
            enum MyStructDelta<'p> {
                NoChange,
                Change_0(<u16 as dipa::Diffable<'p, u16>>::Delta),
                Change_1(<u32 as dipa::Diffable<'p, u32>>::Delta),
                Change_0_1(
                    <u16 as dipa::Diffable<'p, u16>>::Delta,
                    <u32 as dipa::Diffable<'p, u32>>::Delta
                )
            }

            #[derive(Deserialize,)]
            #[allow(non_camel_case_types, missing_docs)]
            enum MyStructDeltaOwned {
                NoChange,
                Change_0(<u16 as dipa::Diffable<'_, u16>>::DeltaOwned),
                Change_1(<u32 as dipa::Diffable<'_, u32>>::DeltaOwned),
                Change_0_1(
                    <u16 as dipa::Diffable<'_, u16>>::DeltaOwned,
                    <u32 as dipa::Diffable<'_, u32>>::DeltaOwned
                )
            }
        };

        assert_tokens_eq(&tokens, &expected);
    }
}
