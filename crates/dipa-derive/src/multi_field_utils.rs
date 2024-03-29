//! Various util functions useful for generating dipa impls for structs and enum variants that
//! have multiple fields.

use quote::__private::TokenStream;
use syn::__private::{Span, TokenStream2};
use syn::spanned::Spanned;
use syn::{FieldsNamed, FieldsUnnamed, Ident, Type};

pub use self::field_changes::*;
pub use self::struct_or_tuple_field::*;
use crate::dipa_attribute::DipaAttrs;
use crate::multi_field_utils::make_bool_combinations;

mod field_changes;
mod struct_or_tuple_field;

pub fn fields_named_to_vec_fields(fields: &FieldsNamed) -> Vec<StructOrTupleField> {
    fields
        .named
        .iter()
        .map(|f| {
            let field_name = f.ident.as_ref().unwrap();

            StructOrTupleField {
                name: quote! {#field_name},
                ty: f.ty.clone(),
                span: f.span(),
            }
        })
        .collect()
}

pub fn fields_unnamed_to_vec_fields(fields: &FieldsUnnamed) -> Vec<StructOrTupleField> {
    #[rustfmt::skip]
    let tuple_field_names = [
        quote! {0}, quote! {1}, quote! {2}, quote! {3},
        quote! {4}, quote! {5}, quote! {6}, quote! {7},
        quote! {8}, quote! {9}, quote! {10}, quote! {11},
        quote! {12}, quote! {13}, quote! {14}, quote! {15},
    ];

    fields
        .unnamed
        .iter()
        .enumerate()
        .map(|(idx, f)| {
            let field_name = &tuple_field_names[idx];

            StructOrTupleField {
                name: quote_spanned! {f.span() => #field_name},
                ty: f.ty.clone(),
                span: f.span(),
            }
        })
        .collect()
}

// Generate the tokens to match on the different fields within the struct to see which changed,
// then combine the changes into one final Diff for the entire struct.
//
//
// ```no_run
// # use quote::quote;
//
// // Not included. Just here to illustrate.
// // let diff0 = self.some_field_name.create_delta_towards(&end_state.some_field_name);
// // let diff1 = self.another_field_name.create_delta_towards(&end_state.another_field_name);
// // End not included.
//
// quote! {
//     let delta = match (diff0.did_change, diff1.did_change) {
//         (false, false) => MyTypeDelta::NoChange,
//         (true, false) => MyTypeDelta::Change_0(diff0.delta),
//         (false, true) => MyTypeDelta::Change_1(diff1.delta),
//         (true, true) => MyTypeDelta::Change_0_1(diff0.delta, diff1.delta)
//     };
//     let did_change = match (diff0.did_change, diff1.did_change) {
//         (false, false) => false,
//         _ => true
//     };
// };
// ```
pub fn make_match_diff_tokens(
    diff_ty: Type,
    change_prefix: &str,
    span: Span,
    fields: &[StructOrTupleField],
    dipa_attrs: &DipaAttrs,
) -> TokenStream2 {
    let bool_combinations = make_bool_combinations(fields.len(), dipa_attrs.max_fields_per_batch);

    let match_diff_inner_tokens =
        make_match_diff_inner_tokens(diff_ty, change_prefix, span, &bool_combinations, &fields);

    // (diff0.1.did_change, diff1.1.did_change)
    let mut did_change_tokens = vec![];
    for (idx, f) in fields.iter().enumerate() {
        let diff_ident = Ident::new(&format!("diff{}", idx), f.span);
        did_change_tokens.push(quote! { #diff_ident.did_change });
    }

    let tokens = quote! {
        let delta = match (#(#did_change_tokens),*) {
            #(#match_diff_inner_tokens)*
        };
        let did_change = #(#did_change_tokens)||*;
    };

    tokens
}

// Generate the tokens to match on the different possible Diff's and apply the appropriate patches
// to the struct's sub fields.
//
// ```
// let field0_mut_ref = &mut self.some_field_name;
// let field1_mut_ref = &mut self.another_field_name;
//
// match patch {
//     #diff_n::NoChange => {}
//     #diff_n::Change_0(field0_patch) => {
//         field0_mut_ref.apply_patch(field0_patch);
//     }
//     #diff_n::Change_1(field1_patch) => {
//         field1_mut_ref.apply_patch(field1_patch);
//     }
//     #diff_n::Change_0_1(field0_patch, field1_patch) => {
//         field0_mut_ref.apply_patch(field0_patch);
//         field1_mut_ref.apply_patch(field1_patch);
//     }
// };
// ```
pub fn make_match_patch_tokens(
    span: Span,
    diff_ty: &Type,
    fields: &[StructOrTupleField],
    field_mut_refs: Vec<TokenStream2>,
    dipa_attrs: &DipaAttrs,
) -> TokenStream2 {
    let bool_combinations = make_bool_combinations(fields.len(), dipa_attrs.max_fields_per_batch);

    let match_patch_inner_tokens = make_match_patch_inner_tokens(diff_ty, span, &bool_combinations);

    quote! {
      #(#field_mut_refs)*

      match patch {
         #diff_ty::NoChange => {}
         #(#match_patch_inner_tokens)*
     };
    }
}

/// Generate the inside of a match statement that returns a diff based on which fields have changed.
///
/// (false, false, false) => Diff3::NoChange,
/// (true, false, true) => Diff3::Change_0_2(diff0.0, diff2.0),
fn make_match_diff_inner_tokens(
    diff_ty: Type,
    change_prefix: &str,
    span: Span,
    bool_combinations: &[Vec<bool>],
    fields: &[StructOrTupleField],
) -> Vec<TokenStream2> {
    let mut match_diff_inner_tokens = vec![];

    let mut all_tuple_entries_are_false = true;

    for bools in bool_combinations {
        let bools = &bools[0..fields.len()];

        let left_side: Vec<Ident> = bools
            .iter()
            .map(|true_or_false| {
                let ident = if *true_or_false { "true" } else { "false" };
                Ident::new(ident, span)
            })
            .collect();

        // example if the first, third and sixth field in the struct changed.
        //   "Change_0_2_5"
        let mut changed_keys = "Change".to_string();

        // vec![diff0.delta, diff2.delta, diff5.delta]
        let mut changed_diffs = vec![];

        for (idx, _bool) in bools
            .iter()
            .enumerate()
            .filter(|(_, did_change)| **did_change)
        {
            changed_keys += &format!("_{}", idx);

            let diff_ident = Ident::new(&format!("diff{}", idx), span);
            changed_diffs.push(quote! {#diff_ident.delta});
        }

        let changed_keys = Ident::new(&format!("{}{}", change_prefix, changed_keys), span);

        let right_side = if all_tuple_entries_are_false {
            let no_change = Ident::new(&format!("{}NoChange", change_prefix), span);
            let no_change = quote! {#no_change};

            quote! { #diff_ty::#no_change }
        } else {
            quote! { #diff_ty::#changed_keys(#(#changed_diffs),*) }
        };

        match_diff_inner_tokens.push(quote! {
        (#(#left_side),*) => #right_side,
        });

        all_tuple_entries_are_false = false;
    }

    match_diff_inner_tokens
}

/// Generate the inside of a match statement that applies a patch based on the diff.
///
/// Diff2::Change_0_1(field0_patch, field1_patch) => {
///     field0_mut_ref.apply_patch(field0_patch);
///     field1_mut_ref.apply_patch(field1_patch);
/// }
fn make_match_patch_inner_tokens(
    diff_ty: &Type,
    span: Span,
    bool_combinations: &[Vec<bool>],
) -> Vec<TokenStream> {
    let mut match_patch_inner_tokens = vec![];
    let mut all_tuple_entries_are_false = true;

    for bools in bool_combinations {
        let changed_keys = get_change_variant_name(&bools, span);

        let incoming_fields = patched_field_patch_names(span, &bools);

        let patch_expressions = get_patch_expressions(span, &bools);

        if !all_tuple_entries_are_false {
            //     Diff2::Change_0_1(field0_patch, field1_patch) => {
            //         field0_mut_ref.apply_patch(field0_patch);
            //         field1_mut_ref.apply_patch(field1_patch);
            //     }
            let match_patch_branch = quote! {
              #diff_ty::#changed_keys(#(#incoming_fields),*) => {
                  #(#patch_expressions)*
              }
            };
            match_patch_inner_tokens.push(match_patch_branch);
        }

        all_tuple_entries_are_false = false;
    }

    match_patch_inner_tokens
}

/// example if the first, third and sixth field in the struct changed.
///   Change_0_2_5
fn get_change_variant_name(bools: &[bool], span: Span) -> Ident {
    let mut changed_keys = "Change".to_string();

    for (idx, _bool) in bools
        .iter()
        .enumerate()
        .filter(|(_, did_change)| **did_change)
    {
        changed_keys += &format!("_{}", idx);
    }

    Ident::new(&format!("{}", changed_keys), span)
}

/// The names of each of the new patches that we need to apply.
///
/// field0_patch, field2_patch field5_patch
fn patched_field_patch_names(span: Span, bools: &[bool]) -> Vec<Ident> {
    let mut incoming_fields = vec![];

    for (idx, _bool) in bools
        .iter()
        .enumerate()
        .filter(|(_, did_change)| **did_change)
    {
        let patch_ident = Ident::new(&format!("field{}_patch", idx), span);

        incoming_fields.push(patch_ident);
    }

    incoming_fields
}

/// field0_mut_ref.apply_patch(field0_patch);
/// field2_mut_ref.apply_patch(field2_patch);
/// field5_mut_ref.apply_patch(field5_patch);
fn get_patch_expressions(span: Span, bools: &[bool]) -> Vec<TokenStream2> {
    let mut patch_expressions = vec![];

    for (idx, _bool) in bools
        .iter()
        .enumerate()
        .filter(|(_, did_change)| **did_change)
    {
        let patch_ident = Ident::new(&format!("field{}_patch", idx), span);

        let field_mut_ref_ident = Ident::new(&format!("field{}_mut_ref", idx), span);
        patch_expressions.push(quote! {#field_mut_ref_ident.apply_patch(#patch_ident);});
    }

    patch_expressions
}
