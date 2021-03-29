use crate::impl_dipa;
use crate::multi_field_utils::{
    field_associated_diff_types, field_associated_patch_types, make_diff_n_ident,
    make_match_diff_tokens, make_match_patch_tokens, StructOrTupleField,
};
use syn::__private::TokenStream2;
use syn::spanned::Spanned;
use syn::{Ident, Type};

/// Generate an implementation of Diffable for a struct with 2 or more fields.
pub(super) fn generate_multi_field_struct_impl(
    struct_name: &syn::Ident,
    fields: Vec<StructOrTupleField>,
) -> TokenStream2 {
    let field_diff_types = field_associated_diff_types(&fields);
    let field_patch_types: Vec<TokenStream2> = field_associated_patch_types(&fields);

    let diff_n = make_diff_n_ident(fields.len(), struct_name.span());

    let field_diffs_statements = field_diff_statements(&fields);
    let match_diff_tokens = make_match_diff_tokens(
        Type::Verbatim(quote! {#diff_n}),
        "",
        struct_name.span(),
        &fields,
    );

    let field_mut_refs = field_mutable_references(&fields);

    let diff_ty = Type::Verbatim(quote! {#diff_n});

    let match_patch_tokens =
        make_match_patch_tokens(struct_name.span(), &diff_ty, &fields, field_mut_refs);

    impl_dipa(
        struct_name,
        quote! {dipa::private::#diff_ty<#(#field_diff_types),*>},
        quote! {dipa::private::#diff_ty<#(#field_patch_types),*>},
        quote! {
           use dipa::private::#diff_ty;
           use dipa::MacroOptimizationHints;

           #(#field_diffs_statements)*;
           #match_diff_tokens

           let macro_hints = MacroOptimizationHints {
               did_change
           };

          (diff, macro_hints)
        },
        quote! {
           use dipa::private::#diff_ty;

           #match_patch_tokens
        },
    )
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

/// let diff0 = self.some_field_name.create_patch_towards(&end_state.some_field_name);
/// let diff1 = self.another_field_name.create_patch_towards(&end_state.another_field_name);
fn field_diff_statements(fields: &[StructOrTupleField]) -> Vec<TokenStream2> {
    fields
        .iter()
        .enumerate()
        .map(|(field_idx, field)| {
            let field_name = &field.name;

            let diff_idx_ident = Ident::new(&format!("diff{}", field_idx), field_name.span());

            quote! {
            let #diff_idx_ident = self.#field_name.create_patch_towards(&end_state.#field_name);
            }
        })
        .collect()
}
