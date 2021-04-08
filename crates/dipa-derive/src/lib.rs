use crate::dipa_attribute::{maybe_parse_raw_dipa_attribute, DipaAttrs};
use crate::multi_field_utils::{
    fields_named_to_vec_fields, fields_unnamed_to_vec_fields, ParsedFields,
};
use crate::multi_variant_enum::generate_multi_variant_enum_impl;
use crate::parsed_enum::{EnumVariant, EnumVariantFields, ParsedEnum};
use crate::parsed_struct::ParsedStruct;
use crate::single_field_struct::generate_single_field_struct_impl;
use crate::single_variant_enum::{
    generate_single_variant_enum_single_struct_field_impl,
    generate_single_variant_enum_single_tuple_field_impl,
};
use crate::zst_impl::create_zst_impl;
use proc_macro::TokenStream;
use quote::quote;
use syn::__private::TokenStream2;
use syn::spanned::Spanned;
use syn::{parse_macro_input, Data, DeriveInput, Fields};
use syn::{Error as SynError, Result as SynResult};

#[macro_use]
extern crate quote;

#[macro_use]
extern crate syn;

mod multi_variant_enum;
mod single_field_struct;
mod single_variant_enum;
mod zst_impl;

mod dipa_attribute;

mod multi_field_utils;
mod parsed_enum;
mod parsed_struct;

#[cfg(test)]
mod test_utils;

/// #[derive(DiffPatch)]
// cargo test -p dipa-derive      # Unit tests of the macro's implementation
// cargo test -p dipa-derive-test # Testing real usage of the macro
#[proc_macro_derive(DiffPatch, attributes(dipa))]
pub fn derive_diff_patch(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let dipa_attrs = match maybe_parse_raw_dipa_attribute(input.attrs) {
        Some(attrib) => {
            let attrib_tokens = attrib.tokens.into();

            Some(parse_macro_input!(attrib_tokens as DipaAttrs))
        }
        None => None,
    }
    .unwrap_or(DipaAttrs::default());

    let enum_or_struct_name = input.ident;

    let zero_sized_diff = create_zst_impl(&enum_or_struct_name);

    // Generate:
    // impl<'p, Other> Diffable<'p, Other> for MyType { ... }
    // impl Patchable<Patch> for MyType { ... }
    let dipa_impl = match input.data {
        Data::Struct(struct_data) => {
            let fields = match &struct_data.fields {
                Fields::Named(named_fields) => ParsedFields {
                    fields: fields_named_to_vec_fields(named_fields),
                    span: named_fields.span(),
                },
                Fields::Unnamed(unnamed_fields) => ParsedFields {
                    fields: fields_unnamed_to_vec_fields(unnamed_fields),
                    span: unnamed_fields.span(),
                },
                Fields::Unit => ParsedFields {
                    fields: vec![],
                    span: enum_or_struct_name.span(),
                },
            };
            let parsed_struct = ParsedStruct {
                // FIXME: Remove clone once we move the logic below into generate_dipa_impl()
                name: enum_or_struct_name.clone(),
                fields,
            };

            if let Err(err) = parsed_struct.validate_struct_container_attributes(&dipa_attrs) {
                return err.into();
            }

            // TODO: Move this logic into ParsedStruct.generate_dipa_impl()
            let struct_dipa_impl = match struct_data.fields {
                // struct Foo { field_a: type1, field_b: type2, ... }
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
                        parsed_struct.generate_multi_field_struct_impl(&dipa_attrs)
                    }
                }
                // struct Foo(type1, type2);
                Fields::Unnamed(fields) => {
                    if fields.unnamed.len() == 0 {
                        zero_sized_diff
                    } else if fields.unnamed.len() == 1 {
                        generate_single_field_struct_impl(
                            &enum_or_struct_name,
                            quote_spanned! {fields.unnamed[0].span() => 0},
                            &fields.unnamed[0].ty,
                        )
                    } else {
                        parsed_struct.generate_multi_field_struct_impl(&dipa_attrs)
                    }
                }
                // struct Foo;
                Fields::Unit => zero_sized_diff,
            };

            struct_dipa_impl
        }
        Data::Enum(enum_data) => {
            let variants = enum_data
                .variants
                .iter()
                .map(|v| {
                    let fields = match &v.fields {
                        Fields::Named(named_fields) => EnumVariantFields::Struct(ParsedFields {
                            fields: fields_named_to_vec_fields(named_fields),
                            span: named_fields.span(),
                        }),
                        Fields::Unnamed(unnamed_fields) => EnumVariantFields::Tuple(ParsedFields {
                            fields: fields_unnamed_to_vec_fields(unnamed_fields),
                            span: unnamed_fields.span(),
                        }),
                        Fields::Unit => EnumVariantFields::Unit,
                    };

                    EnumVariant {
                        name: v.ident.clone(),
                        fields,
                    }
                })
                .collect();
            let parsed_enum = ParsedEnum {
                name: enum_or_struct_name.clone(),
                variants,
            };

            if enum_data.variants.len() == 0 {
                zero_sized_diff
            } else if enum_data.variants.len() == 1 {
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
                                parsed_enum
                                    .generate_single_variant_multi_field_dipa_impl(&dipa_attrs)
                            }
                        }
                        Fields::Unnamed(fields) => {
                            if fields.unnamed.len() == 1 {
                                let field = &fields.unnamed[0];

                                generate_single_variant_enum_single_tuple_field_impl(
                                    enum_or_struct_name,
                                    &variant.ident,
                                    &field.ty,
                                )
                            } else {
                                parsed_enum
                                    .generate_single_variant_multi_field_dipa_impl(&dipa_attrs)
                            }
                        }
                        Fields::Unit => {
                            unimplemented!()
                        }
                    }
                }
            } else {
                generate_multi_variant_enum_impl(
                    enum_or_struct_name,
                    enum_data.variants,
                    dipa_attrs,
                )
            }
        }
        Data::Union(_) => unimplemented!(),
    };

    let expanded = quote! {
    #dipa_impl
    };

    TokenStream::from(expanded)
}

fn impl_dipa(
    enum_or_struct_name: &syn::Ident,
    delta_type: TokenStream2,
    delta_owned_type: TokenStream2,
    create_delta_inner: TokenStream2,
    apply_patch_inner: TokenStream2,
) -> TokenStream2 {
    quote! {
     impl<'p> dipa::Diffable<'p, #enum_or_struct_name> for #enum_or_struct_name {
        type Delta = #delta_type;

        type DeltaOwned = #delta_owned_type;

        fn create_delta_towards (&self, end_state: &'p #enum_or_struct_name)
          -> dipa::CreatePatchTowardsReturn<Self::Delta> {
            #create_delta_inner
        }
     }

     impl<'p> dipa::Patchable<#delta_owned_type> for #enum_or_struct_name {
        fn apply_patch (&mut self, patch: #delta_owned_type) {
            #apply_patch_inner
        }
     }
    }
}
