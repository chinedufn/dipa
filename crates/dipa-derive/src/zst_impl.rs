use crate::impl_dipa;
use syn::Ident;
use syn::__private::TokenStream2;

pub(super) fn create_zst_impl(enum_or_struct_name: &Ident) -> TokenStream2 {
    impl_dipa(
        enum_or_struct_name,
        quote! {()},
        quote! {()},
        quote! {(
         (),
         dipa::MacroOptimizationHints {
             did_change: false
         }
        )},
        quote! {},
    )
}
