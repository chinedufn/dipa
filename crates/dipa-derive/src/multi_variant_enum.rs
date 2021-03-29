use crate::dipa_attribute::{DipaAttr, DipaAttrs};
use crate::enum_utils::{
    diff_type_name, make_two_enums_match_statement, patch_type_name, DipaAssociatedType,
    EnumVariant, EnumVariantFields, ParsedEnum,
};
use crate::impl_dipa;
use crate::multi_field_utils::{fields_named_to_vec_fields, fields_unnamed_to_vec_fields};
use syn::__private::{ToTokens, TokenStream2};
use syn::punctuated::Punctuated;
use syn::{Fields, Ident, Type, TypeReference, Variant};

pub fn generate_multi_variant_enum_impl(
    enum_name: syn::Ident,
    variants: Punctuated<Variant, Token![,]>,
    dipa_attrs: Option<DipaAttrs>,
) -> TokenStream2 {
    let mut enum_variants = vec![];

    let mut all_variants_unit = true;

    for idx in 0..variants.len() {
        let variant = &variants[idx];
        let fields = match &variant.fields {
            Fields::Named(named) => {
                all_variants_unit = false;
                EnumVariantFields::Struct(fields_named_to_vec_fields(named))
            }
            Fields::Unnamed(unnamed) => {
                all_variants_unit = false;
                EnumVariantFields::Tuple(fields_unnamed_to_vec_fields(unnamed))
            }
            Fields::Unit => EnumVariantFields::Unit,
        };

        enum_variants.push(EnumVariant {
            name: variant.ident.clone(),
            fields,
        })
    }

    if all_variants_unit {
        generate_multi_variant_enum_no_data_impl(enum_name, enum_variants)
    } else {
        generate_multi_variant_enum_with_data_impl(enum_name, enum_variants, dipa_attrs)
    }
}

/// #[derive(Dipa)] for an enum with two or more variants, none of which contain any data.
///
/// ```
/// # #[allow(missing_docs)]
/// enum MyEnum {
///     One,
///     Two
/// }
/// ```
fn generate_multi_variant_enum_no_data_impl(
    enum_name: syn::Ident,
    variants: Vec<EnumVariant>,
) -> TokenStream2 {
    impl_dipa(
        &enum_name,
        quote! {
        #enum_name
        },
        quote! {
        #enum_name
        },
        no_data_diff_match(&enum_name, &variants),
        quote! {*self = patch;},
    )
}

/// #[derive(Dipa)] for an enum with two or more variants, where at least one variant contains data.
///
/// ```
/// # #[allow(missing_docs)]
/// enum MyEnum {
///     One(u8),
///     Two
/// }
/// ```
fn generate_multi_variant_enum_with_data_impl(
    enum_name: syn::Ident,
    variants: Vec<EnumVariant>,
    dipa_attrs: Option<DipaAttrs>,
) -> TokenStream2 {
    let parsed_enum = ParsedEnum {
        name: enum_name.clone(),
        variants,
    };

    let diff_ty = diff_type_name(&enum_name);
    let patch_ty = patch_type_name(&enum_name);

    let mut diff_variants = vec![];

    let mut needs_lifetime = false;

    for variant in parsed_enum.variants.iter() {
        let no_change = Ident::new(&format!("{}NoChange", variant.name,), variant.name.span());

        let changed_to_variant = Ident::new(
            &format!("ChangedToVariant{}", variant.name,),
            variant.name.span(),
        );

        // An enum has changed values within the same variant.
        // i.e. MyEnum::SomeVal(5) -> MyEnum::SomeVal(30)
        let changed_same_variant =
            Ident::new(&format!("{}Change_0", variant.name), variant.name.span());

        let mut diff_tys = quote! {};
        let mut ref_tys = quote! {};

        if variant.fields.len() > 0 {
            let tys: Vec<Type> = variant
                .fields
                .iter()
                .map(|f| {
                    needs_lifetime = true;

                    let ref_ty = Type::Reference(TypeReference {
                        and_token: syn::token::And::default(),
                        lifetime: Some(syn::Lifetime::new("'p", f.span)),
                        mutability: None,
                        elem: Box::new(f.ty.clone()),
                    });
                    Type::Verbatim(quote! {#ref_ty})
                })
                .collect();

            // example: (&u8, &Vec<f32>)
            ref_tys = quote! {( #(#tys),* )};
        };

        if variant.fields.len() > 0 {
            let tys: Vec<Type> = variant
                .fields
                .iter()
                .map(|f| {
                    let ty = &f.ty;

                    Type::Verbatim(quote! { <#ty as dipa::Diffable<'p, #ty>>::Diff })
                })
                .collect();

            // example: (u8, Vec<f32>)
            diff_tys = quote! {( #(#tys),* )};
        }

        diff_variants.push(quote! {#no_change});
        diff_variants.push(quote! {#changed_to_variant #ref_tys});
        diff_variants.push(quote! {#changed_same_variant #diff_tys});
    }

    let mut patch_variants = vec![];
    for variant in parsed_enum.variants.iter() {
        let no_change = Ident::new(&format!("{}NoChange", variant.name,), variant.name.span());
        patch_variants.push(quote! {#no_change});
    }

    let (diff_derives, patch_derives) = parse_diff_and_patch_derives(dipa_attrs.as_ref());

    let maybe_lifetime = if needs_lifetime {
        let lifetime = syn::Lifetime::new("'p", enum_name.span());
        quote! {<#lifetime>}
    } else {
        quote! {}
    };

    let diff_type_definition =
        parsed_enum.create_associated_type_for_enum_with_fields(DipaAssociatedType::Diff);
    let diff_type_definition = quote! {
        #[derive(serde::Serialize, #(#patch_derives),*)]
        #diff_type_definition
    };

    let patch_type_definition =
        parsed_enum.create_associated_type_for_enum_with_fields(DipaAssociatedType::Patch);
    let patch_type_definition = quote! {
        #[derive(serde::Deserialize, #(#patch_derives),*)]
        #patch_type_definition
    };

    let diff_tokens = diff_match_with_data(&enum_name, &parsed_enum.variants);

    let patch_tokens = parsed_enum.create_patch_match_stmt();

    let dipa_impl = impl_dipa(
        &enum_name,
        quote! {
        #diff_ty#maybe_lifetime
        },
        quote! {
        #patch_ty
        },
        quote! { #diff_tokens },
        quote! { #patch_tokens },
    );

    let tokens = quote! {
       #diff_type_definition
       #patch_type_definition

       #dipa_impl
    };

    // panic!("{}", tokens.to_string());

    tokens
}

/// ```
/// # #[allow(unused)]
/// enum MyEnum {
///     VariantOne,
///     VariantTwo,
/// }
/// ```
fn no_data_diff_match(enum_name: &syn::Ident, variants: &[EnumVariant]) -> TokenStream2 {
    let mut diff_match_branches = vec![];

    for (idx1, variant1) in variants.iter().enumerate() {
        for (idx2, variant2) in variants.iter().enumerate() {
            let variant_name_1 = &variant1.name;
            let variant_name_2 = &variant2.name;

            let did_change = idx1 != idx2;
            let did_change = Ident::new(&format!("{}", did_change), enum_name.span());

            diff_match_branches.push(quote! {
                (Self::#variant_name_1, Self::#variant_name_2) => {
                    let hints = dipa::MacroOptimizationHints {
                        did_change: #did_change
                    };
                    (Self::#variant_name_2, hints)
                }
            });
        }
    }

    quote! {
      match (self, end_state) {
          #(#diff_match_branches)*
      }
    }
}

/// Generate a match statement to diff an enum where at least one variant contains data.
///
/// ```
/// # #[allow(unused)]
/// enum MyEnum {
///     VariantOne,
///     VariantTwo(SomeData),
/// }
/// ```
fn diff_match_with_data(enum_name: &syn::Ident, variants: &[EnumVariant]) -> TokenStream2 {
    let mut match_stmt_branches = vec![];

    let diff_ty = diff_type_name(&enum_name);

    for (idx1, variant1) in variants.iter().enumerate() {
        for (idx2, variant2) in variants.iter().enumerate() {
            let variant_name_1 = &variant1.name;
            let variant_name_2 = &variant2.name;

            let match_block = variant1.diff_match_block_one_or_more_data(variant2, enum_name);
            match_stmt_branches.push(match_block);
        }
    }

    make_two_enums_match_statement(
        &Ident::new("self", enum_name.span()),
        &Ident::new("end_state", enum_name.span()),
        quote! { #(#match_stmt_branches)* },
    )
}

fn parse_diff_and_patch_derives(dipa_attrs: Option<&DipaAttrs>) -> (Vec<Ident>, Vec<Ident>) {
    let mut diff_derives = vec![];
    let mut patch_derives = vec![];

    if let Some(attrs) = dipa_attrs {
        for attr in attrs.iter() {
            match attr {
                DipaAttr::DiffDerive(lit) => {
                    diff_derives = parse_derives(lit);
                }
                DipaAttr::PatchDerive(lit) => {
                    patch_derives = parse_derives(lit);
                }
            }
        }
    }

    (diff_derives, patch_derives)
}

/// "Debug, Copy" -> vec![Debug, Copy]
fn parse_derives(lit: &syn::Lit) -> Vec<Ident> {
    let mut derive_idents = vec![];

    // "Debug, Copy"
    let derives = lit.to_token_stream().to_string();

    // Debug, Copy
    let derives = derives.trim_start_matches(r#"""#);
    let derives = derives.trim_end_matches(r#"""#);

    // [Debug, Copy]
    let derives = derives.split(",");

    for derive in derives.into_iter() {
        let derive_ident = Ident::new(derive.trim(), lit.span());
        derive_idents.push(derive_ident);
    }

    derive_idents
}