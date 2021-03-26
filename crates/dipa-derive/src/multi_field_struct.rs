use crate::impl_dipa;
use syn::__private::{Span, TokenStream2};
use syn::spanned::Spanned;
use syn::{Ident, Type};

const FALSE_TRUE: [bool; 2] = [false, true];

pub struct StructField<'a> {
    pub name: TokenStream2,
    pub ty: &'a Type,
    pub span: Span,
}

/// Generate an implementation of Diffable for a struct with 2 or more fields.
///
/// TODO: Internals should be split up into smaller functions. Just stuffing things together as we
///  get the test suite passing, then we can clean up later.
pub(super) fn generate_multi_field_struct_impl(
    struct_name: &syn::Ident,
    fields: Vec<StructField>,
) -> TokenStream2 {
    // i.e. u8::Diff, Option<f64>::Diff, ...
    let field_diff_types: Vec<TokenStream2> = fields
        .iter()
        .map(|field| {
            let ty = &field.ty;

            quote! {
            <#ty as dipa::Diffable<'p>>::Diff
            }
        })
        .collect();

    // i.e. u8::OwnedDiff, Option<f64>::OwnedDiff, ...
    let field_owned_diff_types: Vec<TokenStream2> = fields
        .iter()
        .map(|field| {
            let ty = &field.ty;

            quote! {
            <#ty as dipa::Diffable<'p>>::OwnedDiff
            }
        })
        .collect();

    // let diff_0 = self.some_field_name.create_patch_towards(&end_state.some_field_name);
    // let diff_1 = self.another_field_name.create_patch_towards(&end_state.another_field_name);
    let field_diffs: Vec<TokenStream2> = fields
        .iter()
        .enumerate()
        .map(|(field_idx, field)| {
            let field_name = &field.name;

            let diff_idx_ident = Ident::new(&format!("diff_{}", field_idx), field_name.span());

            quote! {
            let #diff_idx_ident = self.#field_name.create_patch_towards(&end_state.#field_name);
            }
        })
        .collect();

    // let field0_mut_ref = &mut self.some_field_name;
    // let field1_mut_ref = &mut self.another_field_name;
    let field_mut_refs: Vec<TokenStream2> = fields
        .iter()
        .enumerate()
        .map(|(field_idx, field)| {
            let field_name = &field.name;

            let mut_ref_ident =
                Ident::new(&format!("field{}_mut_ref", field_idx), field_name.span());

            quote! {
            let #mut_ref_ident = &mut self.#field_name;
            }
        })
        .collect();

    // dipa::private::{Diff2, Diff3, ... etc}
    let diff_n = Ident::new(&format!("Diff{}", fields.len()), struct_name.span());

    let bool_combinations = match fields.len() {
        2 => make_bool_combinations_2(),
        3 => make_bool_combinations_3(),
        4 => make_bool_combinations_4(),
        _ => panic!(
            r#"
TODO: Support larger structs.
"#
        ),
    };

    let mut match_diff_inner_tokens = vec![];
    let mut match_patch_inner_tokens = vec![];

    let mut all_tuple_entries_are_false = true;

    // (false, false, false) => Diff3::NoChange,
    // (true, false, true) => Diff3::Change_0_2(diff_0.0, diff_2.0),
    for bools in bool_combinations {
        let bools = &bools[0..fields.len()];

        let left_side: Vec<Ident> = bools
            .iter()
            .map(|true_or_false| {
                let ident = if *true_or_false { "true" } else { "false" };
                Ident::new(ident, struct_name.span())
            })
            .collect();

        // example if the first, third and sixth field in the struct changed.
        //   "Change_0_2_5"
        let mut changed_keys = "Change".to_string();

        // vec![diff_0.0, diff_2.0, diff_5.0]
        let mut changed_diffs = vec![];

        // field0_mut_ref.apply_patch(field0_patch);
        // field2_mut_ref.apply_patch(field2_patch);
        // field5_mut_ref.apply_patch(field5_patch);
        let mut patch_expressions = vec![];

        // field0_patch, field2_patch field5_patch
        let mut incoming_fields = vec![];

        for (idx, _bool) in bools
            .iter()
            .enumerate()
            .filter(|(_, did_change)| **did_change)
        {
            changed_keys += &format!("_{}", idx);

            let diff_ident = Ident::new(&format!("diff_{}", idx), struct_name.span());
            changed_diffs.push(quote! {#diff_ident.0});

            let patch_expression_ident =
                Ident::new(&format!("field{}_mut_ref", idx), struct_name.span());
            let patch_ident = Ident::new(&format!("field{}_patch", idx), struct_name.span());
            patch_expressions.push(quote! {#patch_expression_ident.apply_patch(#patch_ident);});

            incoming_fields.push(patch_ident);
        }

        let changed_keys = Ident::new(&format!("{}", changed_keys), struct_name.span());

        let right_side = if all_tuple_entries_are_false {
            quote! { #diff_n::NoChange }
        } else {
            quote! { #diff_n::#changed_keys(#(#changed_diffs),*) }
        };

        match_diff_inner_tokens.push(quote! {
        (#(#left_side),*) => #right_side,
        });

        if !all_tuple_entries_are_false {
            //     Diff2::Change_0_1(field0_patch, field1_patch) => {
            //         field0_mut_ref.apply_patch(field0_patch);
            //         field1_mut_ref.apply_patch(field1_patch);
            //     }
            let match_patch_branch = quote! {
              #diff_n::#changed_keys(#(#incoming_fields),*) => {
                  #(#patch_expressions)*
              }
            };
            match_patch_inner_tokens.push(match_patch_branch);
        }

        all_tuple_entries_are_false = false;
    }

    // false, false, false
    let mut all_false = vec![];
    for _ in 0..fields.len() {
        all_false.push(Ident::new("false", struct_name.span()));
    }

    // (diff_0.1.did_change, diff_1.1.did_change)
    let mut did_change_tokens = vec![];
    for (idx, f) in fields.iter().enumerate() {
        let diff_ident = Ident::new(&format!("diff_{}", idx), f.span);
        did_change_tokens.push(quote! { #diff_ident.1.did_change });
    }

    // let diff = match (diff_0.1.did_change, diff_1.1.did_change) {
    //     (false, false) => #diff_n::NoChange,
    //     (true, false) => #diff_n::Change_0(diff_0.0),
    //     (false, true) => #diff_n::Change_1(diff_1.0),
    //     (true, true) => #diff_n::Change_0_1(diff_0.0, diff_1.0)
    // };
    // let did_change = match (diff_0.1.did_change, diff_1.1.did_change) {
    //     (false, false) => false,
    //     _ => true
    // };
    let match_diff_tokens = quote! {
        let diff = match (#(#did_change_tokens),*) {
            #(#match_diff_inner_tokens)*
        };
        let did_change = match (#(#did_change_tokens),*) {
            (#(#all_false),*) => false,
            _ => true
        };
    };

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
    let match_patch_tokens = quote! {
            match patch {
               #diff_n::NoChange => {}
               #(#match_patch_inner_tokens)*
           };
    };

    impl_dipa(
        struct_name,
        quote! {dipa::private::#diff_n<#(#field_diff_types),*>},
        quote! {dipa::private::#diff_n<#(#field_owned_diff_types),*>},
        quote! {
           use dipa::private::#diff_n;
           use dipa::MacroOptimizationHints;

           #(#field_diffs)*

           #match_diff_tokens

           let macro_hints = MacroOptimizationHints {
               did_change
           };

          (diff, macro_hints)
        },
        quote! {
           use dipa::private::#diff_n;

           #(#field_mut_refs)*
           #match_patch_tokens
        },
    )
}

fn make_bool_combinations_2() -> Vec<Vec<bool>> {
    let mut bool_combinations = Vec::with_capacity(4);

    for field0 in FALSE_TRUE.iter() {
        for field1 in FALSE_TRUE.iter() {
            let bools = vec![*field0, *field1];
            bool_combinations.push(bools);
        }
    }

    bool_combinations
}

fn make_bool_combinations_3() -> Vec<Vec<bool>> {
    let mut bool_combinations = Vec::with_capacity(8);

    for field0 in FALSE_TRUE.iter() {
        for field1 in FALSE_TRUE.iter() {
            for field2 in FALSE_TRUE.iter() {
                let bools = vec![*field0, *field1, *field2];
                bool_combinations.push(bools);
            }
        }
    }

    bool_combinations
}

fn make_bool_combinations_4() -> Vec<Vec<bool>> {
    let mut bool_combinations = Vec::with_capacity(8);

    for field0 in FALSE_TRUE.iter() {
        for field1 in FALSE_TRUE.iter() {
            for field2 in FALSE_TRUE.iter() {
                for field3 in FALSE_TRUE.iter() {
                    let bools = vec![*field0, *field1, *field2, *field3];
                    bool_combinations.push(bools);
                }
            }
        }
    }

    bool_combinations
}
