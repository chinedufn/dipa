use crate::impl_dipa;
use crate::multi_field_utils::{
    field_associated_diff_types, field_associated_patch_types, make_diff_n_ident,
    make_match_diff_tokens, make_match_patch_tokens, StructOrTupleField,
};
use syn::__private::TokenStream2;

/// Generate an implementation of Diffable for a struct with 2 or more fields.
pub(super) fn generate_multi_field_struct_impl(
    struct_name: &syn::Ident,
    fields: Vec<StructOrTupleField>,
) -> TokenStream2 {
    let field_diff_types = field_associated_diff_types(&fields);
    let field_patch_types: Vec<TokenStream2> = field_associated_patch_types(&fields);

    let diff_n = make_diff_n_ident(fields.len(), struct_name.span());

    let match_diff_tokens =
        make_match_diff_tokens(&struct_name, &fields, quote! {self.}, quote! {end_state.});
    let match_patch_tokens = make_match_patch_tokens(&struct_name, &fields);

    impl_dipa(
        struct_name,
        quote! {dipa::private::#diff_n<#(#field_diff_types),*>},
        quote! {dipa::private::#diff_n<#(#field_patch_types),*>},
        quote! {
           use dipa::private::#diff_n;
           use dipa::MacroOptimizationHints;

           #match_diff_tokens

           let macro_hints = MacroOptimizationHints {
               did_change
           };

          (diff, macro_hints)
        },
        quote! {
           use dipa::private::#diff_n;

           #match_patch_tokens
        },
    )
}
