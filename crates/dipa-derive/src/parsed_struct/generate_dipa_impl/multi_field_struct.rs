use crate::dipa_attribute::{DipaAttrs, FieldBatchingStrategy};
use crate::impl_dipa;
use crate::multi_field_utils::{
    field_associated_patch_types, make_match_diff_tokens, make_match_patch_tokens,
    StructOrTupleField,
};
use crate::parsed_struct::ParsedStruct;
use syn::__private::TokenStream2;
use syn::spanned::Spanned;
use syn::{Ident, Type};

impl ParsedStruct {
    /// Generate an implementation of Diffable for a struct with 2 or more fields.
    pub fn generate_multi_field_struct_impl(
        &self,
        field_batching_strategy: FieldBatchingStrategy,
        dipa_attrs: Option<&DipaAttrs>,
    ) -> TokenStream2 {
        let struct_name = &self.name;

        let prefix = self.name.to_string();
        let prefix = &prefix;
        let delta_name = self.fields.delta_name(prefix);
        let delta_owned_name = self.fields.delta_owned_name(prefix);

        let field_diffs_statements = field_diff_statements(&self.fields);
        let match_diff_tokens = make_match_diff_tokens(
            Type::Verbatim(quote! {#delta_name}),
            "",
            struct_name.span(),
            &self.fields,
        );

        let field_mut_refs = field_mutable_references(&self.fields);

        let diff_ty = Type::Verbatim(quote! {#delta_name});
        let patch_ty = Type::Verbatim(quote! {#delta_owned_name});

        let match_patch_tokens =
            make_match_patch_tokens(struct_name.span(), &patch_ty, &self.fields, field_mut_refs);

        let delta_tys =
            self.fields
                .generate_delta_type(prefix, field_batching_strategy, dipa_attrs);

        let dipa_impl = impl_dipa(
            struct_name,
            quote! {#delta_name<'p>},
            quote! {#delta_owned_name},
            quote! {
               use dipa::MacroOptimizationHints;

               #(#field_diffs_statements)*;
               #match_diff_tokens

               let macro_hints = MacroOptimizationHints {
                   did_change
               };

              (diff, macro_hints)
            },
            quote! {
               #match_patch_tokens
            },
        );

        if self.fields.len() > 1 {
            // panic!("{}", dipa_impl.to_string());
        }

        quote! {
            #delta_tys

            #dipa_impl
        }
    }
}

/// let field0_mut_ref = &mut self.some_field_name;
/// let field1_mut_ref = &mut self.another_field_name;
fn field_mutable_references(fields: &[StructOrTupleField]) -> Vec<TokenStream2> {
    fields
        .iter()
        .enumerate()
        .map(|(field_idx, field)| {
            let field_name = &field.name;

            let mut_ref_ident =
                Ident::new(&format!("field{}_mut_ref", field_idx), field_name.span());

            quote! {
            let #mut_ref_ident = &mut self.#field_name;
            }
        })
        .collect()
}

/// let diff0 = self.some_field_name.create_delta_towards(&end_state.some_field_name);
/// let diff1 = self.another_field_name.create_delta_towards(&end_state.another_field_name);
fn field_diff_statements(fields: &[StructOrTupleField]) -> Vec<TokenStream2> {
    fields
        .iter()
        .enumerate()
        .map(|(field_idx, field)| {
            let field_name = &field.name;

            let diff_idx_ident = Ident::new(&format!("diff{}", field_idx), field_name.span());

            quote! {
            let #diff_idx_ident = self.#field_name.create_delta_towards(&end_state.#field_name);
            }
        })
        .collect()
}
