use crate::parsed_struct::ParsedStruct;
use syn::__private::TokenStream2;

impl ParsedStruct {
    /// ```no_run
    /// // Not generated here. Just here to illustrate.
    /// let diff0 = self.some_field_name.create_delta_towards(&end_state.some_field_name);
    /// let diff1 = self.another_field_name.create_delta_towards(&end_state.another_field_name);
    /// // End not generated here.
    ///
    /// let diff = MyStructDelta {
    ///     fielda: diff0.0,
    ///     fieldb: diff1.0
    /// };
    /// ```
    pub(super) fn generate_no_batching_create_delta_tokens(&self) -> TokenStream2 {
        let delta_name = self.fields.delta_name(&self.name.to_string());

        let mut fields = vec![];
        for (idx, field) in self.fields.iter().enumerate() {
            let field_name = &field.name;
            let diff_idx = format_ident!("diff{}", idx);

            fields.push(quote! {
                #field_name: #diff_idx.0
            })
        }

        quote! {
            let diff = #delta_name {
                #(#fields),*
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::multi_field_utils::{ParsedFields, StructOrTupleField};
    use crate::test_utils::assert_tokens_eq;
    use syn::Type;
    use syn::__private::Span;

    /// Verify that we properly generate the diff for a type that uses th no_batching field batching
    /// strategy.
    #[test]
    fn generates_no_batching_diff() {
        let parsed_struct = ParsedStruct {
            name: format_ident!("MyStruct"),
            fields: ParsedFields {
                fields: vec![
                    StructOrTupleField {
                        name: quote! {fielda},
                        ty: Type::Verbatim(quote! {u8}),
                        span: Span::call_site(),
                    },
                    StructOrTupleField {
                        name: quote! {fieldb},
                        ty: Type::Verbatim(quote! {i8}),
                        span: Span::call_site(),
                    },
                ],
                span: Span::call_site(),
            },
        };

        let tokens = parsed_struct.generate_no_batching_create_delta_tokens();

        let expected = quote! {
            let diff = MyStructDelta {
                fielda: diff0.0,
                fieldb: diff1.0
            };
        };

        assert_tokens_eq(&tokens, &expected);
    }
}
