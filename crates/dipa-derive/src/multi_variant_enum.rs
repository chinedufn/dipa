use crate::impl_dipa;
use crate::multi_field_utils::{
    fields_named_to_vec_fields, fields_unnamed_to_vec_fields, StructOrTupleField,
};
use std::collections::HashSet;
use syn::__private::TokenStream2;
use syn::punctuated::Punctuated;
use syn::{Fields, Ident, Variant};

struct EnumVariant<'a> {
    name: &'a Ident,
    fields: Vec<StructOrTupleField<'a>>,
}

pub fn generate_multi_variant_enum_impl(
    enum_name: syn::Ident,
    variants: Punctuated<Variant, Token![,]>,
) -> TokenStream2 {
    let mut enum_variants = vec![];
    for idx in 0..variants.len() {
        let variant = &variants[idx];
        let fields = match &variant.fields {
            Fields::Named(named) => fields_named_to_vec_fields(named),
            Fields::Unnamed(unnamed) => fields_unnamed_to_vec_fields(unnamed),
            Fields::Unit => vec![],
        };

        enum_variants.push(EnumVariant {
            name: &variant.ident,
            fields: vec![],
        })
    }

    generate_multi_variant_enum_no_data_impl(enum_name, enum_variants)
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
