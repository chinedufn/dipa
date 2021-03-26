use crate::impl_dipa;
use syn::__private::TokenStream2;
use syn::{Field, Ident};

pub(super) fn generate_single_field_struct_impl(
    struct_name: &Ident,
    field: &Field,
) -> TokenStream2 {
    let field_name = field.ident.as_ref().unwrap();
    let field_type = &field.ty;

    impl_dipa(
        &struct_name,
        quote! {
        <#field_type as dipa::Diffable<'p>>::Diff
        },
        quote! {
        <#field_type as dipa::Diffable<'p>>::OwnedDiff
        },
        quote! {
        self.#field_name.create_patch_towards(&end_state.#field_name)
        },
        quote! { self.#field_name.apply_patch(patch) },
    )
}
