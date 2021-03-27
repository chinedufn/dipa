use crate::multi_field_struct::generate_multi_field_struct_impl;
use crate::multi_field_utils::{fields_named_to_vec_fields, StructOrTupleField};
use crate::single_field_struct::generate_single_field_struct_impl;
use crate::single_variant_enum::{
    generate_single_variant_enum_multi_struct_field_impl,
    generate_single_variant_enum_single_struct_field_impl,
    generate_single_variant_enum_single_tuple_field_impl,
};
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
mod single_variant_enum;
mod zst_impl;

mod multi_field_utils;

/// #[derive(DiffPatch)]
// Tested in dipa-derive-test crate
#[proc_macro_derive(DiffPatch)]
pub fn diff_patch(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let enum_or_struct_name = input.ident;

    let zero_sized_diff = create_zst_impl(&enum_or_struct_name);

    let tuple_field_names = [quote! {0}, quote! {1}, quote! {2}, quote! {3}];

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
                        fields_named_to_vec_fields(&fields),
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
                    generate_multi_field_struct_impl(
                        &enum_or_struct_name,
                        struct_data
                            .unnamed
                            .iter()
                            .enumerate()
                            .map(|(idx, f)| {
                                let field_name = &tuple_field_names[idx];

                                StructOrTupleField {
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
                let variant = &enum_data.variants[0];

                let fields = &variant.fields;

                if fields.len() == 0 {
                    zero_sized_diff
                } else {
                    match &fields {
                        Fields::Named(fields) => {
                            if fields.named.len() == 1 {
                                let field = &fields.named[0];
                                let field_name = field.ident.as_ref().unwrap();

                                generate_single_variant_enum_single_struct_field_impl(
                                    enum_or_struct_name,
                                    &variant.ident,
                                    quote_spanned! {field.span() => #field_name},
                                    &field.ty,
                                )
                            } else {
                                generate_single_variant_enum_multi_struct_field_impl(
                                    enum_or_struct_name,
                                    &variant.ident,
                                    fields_named_to_vec_fields(&fields),
                                )
                            }
                        }
                        Fields::Unnamed(fields) => {
                            if fields.unnamed.len() == 1 {
                                let field = &fields.unnamed[0];

                                generate_single_variant_enum_single_tuple_field_impl(
                                    enum_or_struct_name,
                                    &variant.ident,
                                    quote_spanned! {field.span() => 0},
                                    &field.ty,
                                )
                            } else {
                                unimplemented!(r#"Enum one variant with fields unnamed"#)
                            }
                        }
                        Fields::Unit => {
                            unimplemented!()
                        }
                    }
                }
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
    patch_type: TokenStream2,
    create_patch_towards_inner: TokenStream2,
    apply_patch_inner: TokenStream2,
) -> TokenStream2 {
    quote! {
     impl<'p> dipa::Diffable<'p, #enum_or_struct_name> for #enum_or_struct_name {
        type Diff = #diff_type;

        type Patch = #patch_type;

        fn create_patch_towards (&self, end_state: &'p #enum_or_struct_name)
          -> dipa::CreatePatchTowardsReturn<Self::Diff> {
            #create_patch_towards_inner
        }
     }

     impl<'p> dipa::Patchable<#patch_type> for #enum_or_struct_name {
        fn apply_patch (&mut self, patch: #patch_type) {
            #apply_patch_inner
        }
     }
    }
}
