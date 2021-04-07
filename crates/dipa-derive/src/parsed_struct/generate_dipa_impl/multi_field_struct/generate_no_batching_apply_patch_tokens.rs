use crate::parsed_struct::generate_dipa_impl::multi_field_struct::field_mutable_references;
use crate::parsed_struct::ParsedStruct;
use syn::Ident;
use syn::__private::TokenStream2;

impl ParsedStruct {
    /// ```no_run
    /// let field0_mut_ref = &mut self.some_field_name;
    /// let field1_mut_ref = &mut self.another_field_name;
    ///
    /// field0_mut_ref.apply_patch(patch.some_field_name);
    /// field1_mut_ref.fieldb.apply_patch(patch.1);
    /// ```
    pub(super) fn generate_no_batching_apply_patch_tokens(&self) -> TokenStream2 {
        let field_mut_refs = field_mutable_references(&self.fields);
        let mut apply_patches = vec![];

        for (idx, field) in self.fields.iter().enumerate() {
            let field_name = &field.name;
            let field_mut_ref_ident = Ident::new(&format!("field{}_mut_ref", idx), field.span);

            apply_patches.push(quote! {
                #field_mut_ref_ident.apply_patch(patch.#field_name);
            });
        }

        quote! {
            #(#field_mut_refs)*
            #(#apply_patches)*
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

        let tokens = parsed_struct.generate_no_batching_apply_patch_tokens();

        let expected = quote! {
            let field0_mut_ref = &mut self.fielda;
            let field1_mut_ref = &mut self.fieldb;

            field0_mut_ref.apply_patch(patch.fielda);
            field1_mut_ref.apply_patch(patch.fieldb);
        };

        assert_tokens_eq(&tokens, &expected);
    }
}
