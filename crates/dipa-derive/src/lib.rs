use crate::multi_field_struct::{generate_multi_field_struct_impl, StructField};
use crate::single_field_struct::generate_single_field_struct_impl;
use crate::zst_impl::create_zst_impl;
use proc_macro::TokenStream;
use quote::quote;
use syn::__private::TokenStream2;
use syn::spanned::Spanned;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[macro_use]
extern crate quote;

mod multi_field_struct;
mod single_field_struct;
mod zst_impl;

/// #[derive(DiffPatch)]
// Tested in dipa-derive-test crate
#[proc_macro_derive(DiffPatch)]
pub fn diff_patch(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let enum_or_struct_name = input.ident;

    let zero_sized_diff = create_zst_impl(&enum_or_struct_name);

    // Generate:
    // impl<'p> DiffPatch<'p> for MyType { ... }
    let dipa_impl = match input.data {
        Data::Struct(struct_data) => match struct_data.fields {
            Fields::Named(fields) => {
                if fields.named.len() == 0 {
                    zero_sized_diff
                } else if fields.named.len() == 1 {
                    let field = &fields.named[0];
                    let field_name = field.ident.as_ref().unwrap();

                    generate_single_field_struct_impl(
                        &enum_or_struct_name,
                        quote_spanned! {field.span() => #field_name},
                        &field.ty,
                    )
                } else {
                    generate_multi_field_struct_impl(
                        &enum_or_struct_name,
                        fields
                            .named
                            .iter()
                            .map(|f| {
                                let field_name = f.ident.as_ref().unwrap();

                                StructField {
                                    name: quote! {#field_name},
                                    ty: &f.ty,
                                    span: f.span(),
                                }
                            })
                            .collect(),
                    )
                }
            }
            Fields::Unnamed(struct_data) => {
                if struct_data.unnamed.len() == 0 {
                    zero_sized_diff
                } else if struct_data.unnamed.len() == 1 {
                    generate_single_field_struct_impl(
                        &enum_or_struct_name,
                        quote_spanned! {struct_data.unnamed[0].span() => 0},
                        &struct_data.unnamed[0].ty,
                    )
                } else {
                    let tuple_field_names = [quote! {0}, quote! {1}, quote! {2}, quote! {3}];

                    generate_multi_field_struct_impl(
                        &enum_or_struct_name,
                        struct_data
                            .unnamed
                            .iter()
                            .enumerate()
                            .map(|(idx, f)| {
                                let field_name = &tuple_field_names[idx];

                                StructField {
                                    name: quote_spanned! {f.span() => #field_name},
                                    ty: &f.ty,
                                    span: f.span(),
                                }
                            })
                            .collect(),
                    )
                }
            }
            Fields::Unit => zero_sized_diff,
        },
        Data::Enum(enum_data) => {
            if enum_data.variants.len() == 1 {
                zero_sized_diff
            } else {
                todo_quote()
            }
        }
        Data::Union(_) => todo_quote(),
    };

    let expanded = quote! {
    #dipa_impl
    };

    TokenStream::from(expanded)
}

fn todo_quote() -> TokenStream2 {
    quote! {
    // TODO
    }
}

fn impl_dipa(
    enum_or_struct_name: &syn::Ident,
    diff_type: TokenStream2,
    owned_diff_type: TokenStream2,
    create_patch_towards_inner: TokenStream2,
    apply_patch_inner: TokenStream2,
) -> TokenStream2 {
    quote! {
     impl<'p> dipa::Diffable<'p> for #enum_or_struct_name {
        type Diff = #diff_type;

        type OwnedDiff = #owned_diff_type;

        fn create_patch_towards (&self, end_state: &'p Self)
          -> dipa::CreatePatchTowardsReturn<Self::Diff> {
            #create_patch_towards_inner
        }

        fn apply_patch (&mut self, patch: Self::OwnedDiff) {
            #apply_patch_inner
        }
     }
    }
}
