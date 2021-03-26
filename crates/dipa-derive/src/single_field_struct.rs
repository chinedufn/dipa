use crate::impl_dipa;
use syn::__private::TokenStream2;
use syn::{Ident, Type};

pub(super) fn generate_single_field_struct_impl(
    struct_name: &Ident,
    field_name: TokenStream2,
    field_type: &Type,
) -> TokenStream2 {
    impl_dipa(
        &struct_name,
        quote! {
        <#field_type as dipa::Diffable<'p>>::Diff
        },
        quote! {
        <#field_type as dipa::Diffable<'p>>::Patch
        },
        quote! {
        self.#field_name.create_patch_towards(&end_state.#field_name)
        },
        quote! { self.#field_name.apply_patch(patch) },
    )
}
