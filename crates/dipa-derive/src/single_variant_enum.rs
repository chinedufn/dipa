use crate::impl_dipa;

use syn::Type;
use syn::__private::TokenStream2;

/// #[derive(Dipa)] for an enum with one struct variant that has one field.
///
/// ```
/// # #[allow(unused)]
/// enum MyEnum {
///     OneVariant { one_field: Vec<u8> }
/// }
/// ```
pub(super) fn generate_single_variant_enum_single_struct_field_impl(
    enum_name: syn::Ident,
    variant_name: &syn::Ident,
    field_name: TokenStream2,
    field_type: &Type,
) -> TokenStream2 {
    impl_dipa(
        &enum_name,
        quote! {
        <#field_type as dipa::Diffable<'p, #field_type>>::Delta
        },
        quote! {
        <#field_type as dipa::Diffable<'p, #field_type>>::DeltaOwned
        },
        // match (self, end_state) {
        //     (Self::OnlyVariant { only_field: start }, Self::OnlyVariant { only_field: end }) => {
        //         start.create_delta_towards(end)
        //     }
        // }
        quote! {
          match (self, end_state) {
              (Self::#variant_name { #field_name: start }, Self::#variant_name { #field_name: end }) => {
                  start.create_delta_towards(end)
              }
          }
        },
        // match self {
        //     Self::OnlyVariant { only_field } => {
        //         only_field.patch(patch)
        //     }
        // }
        quote! {
          match self {
             Self::#variant_name { #field_name } => {
                 #field_name.apply_patch(patch)
             }
          }
        },
    )
}

/// #[derive(Dipa)] for an enum with one tuple variant that has one field.
///
/// ```
/// # #[allow(unused)]
/// enum MyEnum {
///     OneVariant (Vec<u8>)
/// }
/// ```
pub(super) fn generate_single_variant_enum_single_tuple_field_impl(
    enum_name: syn::Ident,
    variant_name: &syn::Ident,
    field_type: &Type,
) -> TokenStream2 {
    impl_dipa(
        &enum_name,
        quote! {
        <#field_type as dipa::Diffable<'p, #field_type>>::Delta
        },
        quote! {
        <#field_type as dipa::Diffable<'p, #field_type>>::DeltaOwned
        },
        // match (self, end_state) {
        //     (Self::OnlyVariant(only_field), Self::OnlyVariant(only_field)) => {
        //         start.create_delta_towards(end)
        //     }
        // }
        quote! {
          match (self, end_state) {
              (Self::#variant_name(start), Self::#variant_name(end)) => {
                  start.create_delta_towards(end)
              }
          }
        },
        // match self {
        //     Self::OnlyVariant(only_field) => {
        //         only_field.patch(patch)
        //     }
        // }
        quote! {
          match self {
             Self::#variant_name(current) => {
                 current.apply_patch(patch)
             }
          }
        },
    )
}
