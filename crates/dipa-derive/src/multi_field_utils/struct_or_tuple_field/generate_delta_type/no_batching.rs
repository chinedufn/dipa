use crate::dipa_attribute::DipaAttrs;
use crate::multi_field_utils::ParsedFields;
use syn::__private::TokenStream2;

impl ParsedFields {
    pub(super) fn generate_delta_type_no_batching(
        &self,
        prefix: &str,
        dipa_attrs: &DipaAttrs,
    ) -> TokenStream2 {
        let delta_name = self.delta_name(prefix);
        let delta_owned_name = self.delta_owned_name(prefix);

        let mut delta_fields = vec![];
        let mut delta_owned_fields = vec![];

        for field in self.fields.iter() {
            let field_name = &field.name;
            let ty = &field.ty;

            delta_fields.push(quote! {
                #field_name: <#ty as dipa::Diffable<'p, #ty>>::Delta
            });
            delta_owned_fields.push(quote! {
                #field_name: <#ty as dipa::Diffable<'static, #ty>>::DeltaOwned
            });
        }

        let (diff_derives, patch_derives) = (&dipa_attrs.diff_derives, &dipa_attrs.patch_derives);

        quote! {
            #[derive(serde::Serialize, #(#diff_derives),*)]
            #[allow(non_camel_case_types)]
            struct #delta_name<'p> {
                #(#delta_fields),*
            }

            #[derive(serde::Deserialize, #(#patch_derives),*)]
            #[allow(non_camel_case_types)]
            struct #delta_owned_name {
                #(#delta_owned_fields),*
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
    fn generates_no_batching_delta() {
        let mut attrs = DipaAttrs::default();
        attrs.field_batching_strategy = Some(FieldBatchingStrategy::NoBatching);

        let parsed_fields = ParsedFields {
            fields: vec![
                StructOrTupleField {
                    name: quote! {field_a},
                    ty: Type::Verbatim(quote! {u16}),
                    span: Span::call_site(),
                },
                StructOrTupleField {
                    name: quote! {field_b},
                    ty: Type::Verbatim(quote! {u32}),
                    span: Span::call_site(),
                },
            ],
            span: Span::call_site(),
        };
        let tokens = parsed_fields.generate_delta_types("MyStruct", &attrs);

        let expected = quote! {
            #[derive(serde::Serialize,)]
            #[allow(non_camel_case_types)]
            struct MyStructDelta<'p> {
                field_a: <u16 as dipa::Diffable<'p, u16>>::Delta,
                field_b: <u32 as dipa::Diffable<'p, u32>>::Delta
            }

            #[derive(serde::Deserialize,)]
            #[allow(non_camel_case_types)]
            struct MyStructDeltaOwned {
                field_a: <u16 as dipa::Diffable<'static, u16>>::DeltaOwned,
                field_b: <u32 as dipa::Diffable<'static, u32>>::DeltaOwned
            }
        };

        assert_tokens_eq(&tokens, &expected);
    }
}
