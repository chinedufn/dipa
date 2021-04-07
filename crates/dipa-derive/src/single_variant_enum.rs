use crate::dipa_attribute::DipaAttrs;
use crate::enum_utils::{
    field_diff_statements, make_enum_variant_comparison_match_block,
    make_two_enums_match_statement, EnumVariant, EnumVariantFields,
};
use crate::impl_dipa;
use crate::multi_field_utils::{
    field_associated_diff_types, field_associated_patch_types, make_diff_n_ident,
    make_match_diff_tokens, make_match_patch_tokens, StructOrTupleField,
};
use quote::__private::TokenStream;
use syn::__private::TokenStream2;
use syn::{Ident, Type};

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

/// #[derive(Dipa)] for an enum with one variant that has two or more struct fields.
///
/// ```
/// # use std::collections::HashSet;
/// #[allow(unused)]
/// enum MyEnum {
///     OneVariant { field_one: Vec<u8>, field_two: HashSet<u8> }
/// }
/// ```
///
/// FIXME: Make DRY with generate_single_variant_enum_multi_tuple_impl
pub(super) fn generate_single_variant_enum_multi_struct_field_impl(
    enum_name: syn::Ident,
    variant_name: &syn::Ident,
    fields: Vec<StructOrTupleField>,
    dipa_attrs: &DipaAttrs,
) -> TokenStream {
    let field_diff_types = field_associated_diff_types(&fields);
    let field_patch_types: Vec<TokenStream2> = field_associated_patch_types(&fields);

    let diff_n = make_diff_n_ident(fields.len(), enum_name.span());

    let mut field_mut_ref_names = vec![];

    for (idx, field) in fields.iter().enumerate() {
        let field_name = &field.name;

        let span = field.span;

        let field_mut_ref_name = Ident::new(&format!("field{}_mut_ref", idx), span);
        field_mut_ref_names.push(quote! {#field_name: #field_mut_ref_name});
    }

    let mut start_idents = vec![];
    let mut end_idents = vec![];

    for field in fields.iter() {
        let field_name = &field.name;
        let span = field.span;

        let field_name_trimmed = field_name.to_string();
        let field_name_trimmed = field_name_trimmed.trim();

        let start_ident = Ident::new(&format!("start_{}", field_name_trimmed), span);
        let end_ident = Ident::new(&format!("end_{}", field_name_trimmed), span);

        start_idents.push(start_ident);
        end_idents.push(end_ident);
    }

    // dipa::private::{Diff2, Diff3, ... etc}
    let diff_ty = Type::Verbatim(quote! {#diff_n});

    let match_patch_tokens = make_match_patch_tokens(
        enum_name.span(),
        &diff_ty,
        &fields
            .iter()
            .enumerate()
            .map(|(idx, f)| {
                let end_ident = &end_idents[idx];
                StructOrTupleField {
                    name: quote! {#end_ident},
                    ty: f.ty.clone(),
                    span: f.span,
                }
            })
            .collect::<Vec<StructOrTupleField>>(),
        vec![],
        dipa_attrs,
    );

    let fields = EnumVariantFields::Struct(fields);

    let diff_fn_tokens = make_diff_fn_tokens(
        &enum_name,
        &variant_name,
        &fields,
        &start_idents,
        &end_idents,
        dipa_attrs,
    );

    impl_dipa(
        &enum_name,
        quote! {dipa::private::#diff_n<#(#field_diff_types),*>},
        quote! {dipa::private::#diff_n<#(#field_patch_types),*>},
        // match (self, end_state) {
        //     (
        //         Self::OnlyVariant { field_one: start0, field_two: start1 },
        //         Self::OnlyVariant { field_one: end0, field_two: end1 },
        //     ) => {
        //         // ...
        //     }
        // }
        diff_fn_tokens,
        // match self {
        //     Self::OnlyVariant { field0, field1 } => {
        //         field0.patch(patch);
        //         field1.patch(patch);
        //     }
        // }
        quote! {
           use dipa::private::#diff_n;

          match self {
             Self::#variant_name { #(#field_mut_ref_names),* } => {
                 #match_patch_tokens
             }
          }
        },
    )
}

/// #[derive(Dipa)] for an enum with one variant that has two or more tuple fields.
///
/// ```
/// # #[allow(unused)]
/// enum MyEnum {
///     OneVariant (Vec<u8>, u16)
/// }
/// ```
///
/// FIXME: Make DRY with generate_single_variant_enum_multi_struct_field_impl
pub(super) fn generate_single_variant_enum_multi_tuple_impl(
    enum_name: syn::Ident,
    variant_name: &syn::Ident,
    fields: Vec<StructOrTupleField>,
    dipa_attrs: &DipaAttrs,
) -> TokenStream {
    let field_diff_types = field_associated_diff_types(&fields);
    let field_patch_types: Vec<TokenStream2> = field_associated_patch_types(&fields);

    let diff_n = make_diff_n_ident(fields.len(), enum_name.span());

    let mut field_mut_ref_names = vec![];

    for (idx, field) in fields.iter().enumerate() {
        let span = field.span;

        let field_mut_ref_name = Ident::new(&format!("field{}_mut_ref", idx), span);
        field_mut_ref_names.push(quote! {#field_mut_ref_name});
    }

    let mut start_idents = vec![];
    let mut end_idents = vec![];

    for (idx, field) in fields.iter().enumerate() {
        let span = field.span;

        let start_ident = Ident::new(&format!("start_{}", idx), span);
        let end_ident = Ident::new(&format!("end_{}", idx), span);

        start_idents.push(start_ident);
        end_idents.push(end_ident);
    }

    let diff_ty = Type::Verbatim(quote! {#diff_n});

    let match_patch_tokens = make_match_patch_tokens(
        enum_name.span(),
        &diff_ty,
        &fields
            .iter()
            .enumerate()
            .map(|(idx, f)| {
                let end_ident = &end_idents[idx];
                StructOrTupleField {
                    name: quote! {#end_ident},
                    ty: f.ty.clone(),
                    span: f.span,
                }
            })
            .collect::<Vec<StructOrTupleField>>(),
        vec![],
        dipa_attrs,
    );

    let fields = EnumVariantFields::Tuple(fields);

    impl_dipa(
        &enum_name,
        quote! {dipa::private::#diff_n<#(#field_diff_types),*>},
        quote! {dipa::private::#diff_n<#(#field_patch_types),*>},
        make_diff_fn_tokens(
            &enum_name,
            variant_name,
            &fields,
            &start_idents,
            &end_idents,
            dipa_attrs,
        ),
        // match self {
        //     Self::OnlyVariant ( field0, field1 ) => {
        //         field0.patch(patch);
        //         field1.patch(patch);
        //     }
        // }
        quote! {
          use dipa::private::#diff_n;

          match self {
             Self::#variant_name ( #(#field_mut_ref_names),* ) => {
                 #match_patch_tokens
             }
          }
        },
    )
}

// match (self, end_state) {
//     (
//         Self::OnlyVariant ( start0, start1 ),
//         Self::OnlyVariant ( end0, end1 },
//     ) => {
//         // ...
//     }
// }
fn make_diff_fn_tokens(
    enum_name: &Ident,
    variant_name: &Ident,
    fields: &EnumVariantFields,
    start_idents: &[Ident],
    end_idents: &[Ident],
    dipa_attrs: &DipaAttrs,
) -> TokenStream2 {
    let diff_n = make_diff_n_ident(fields.len(), enum_name.span());

    let field_diff_statements = field_diff_statements(&fields, &start_idents, &end_idents);

    let match_diff_tokens = make_match_diff_tokens(
        Type::Verbatim(quote! {#diff_n}),
        "",
        enum_name.span(),
        &fields
            .iter()
            .enumerate()
            .map(|(idx, f)| {
                let start_ident = &start_idents[idx];
                StructOrTupleField {
                    name: quote! {#start_ident},
                    ty: f.ty.clone(),
                    span: f.span,
                }
            })
            .collect::<Vec<StructOrTupleField>>(),
        dipa_attrs,
    );

    let match_block_tokens = make_enum_variant_comparison_match_block(
        "start_",
        &EnumVariant {
            name: variant_name.clone(),
            fields: fields.clone(),
        },
        "end_",
        &EnumVariant {
            name: variant_name.clone(),
            fields: fields.clone(),
        },
        quote! {
         #(#field_diff_statements)*
         #match_diff_tokens

         let macro_hints = dipa::MacroOptimizationHints {
             did_change
         };

         (diff, macro_hints)
        },
    );
    let match_start_end = make_two_enums_match_statement(
        &Ident::new("self", enum_name.span()),
        &Ident::new("end_state", enum_name.span()),
        quote! {#match_block_tokens},
    );

    quote! {
     use dipa::private::#diff_n;

     #match_start_end
    }
}
