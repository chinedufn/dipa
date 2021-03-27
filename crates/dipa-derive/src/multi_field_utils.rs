//! Various util functions useful for generating dipa impls for structs and enum variants that
//! have multiple fields.

use quote::__private::TokenStream;
use syn::__private::{Span, TokenStream2};
use syn::spanned::Spanned;
use syn::{FieldsNamed, Ident, Type};

const FALSE_TRUE: [bool; 2] = [false, true];

pub struct StructOrTupleField<'a> {
    /// if struct field: some_name
    /// if tuple field: 0, 1, 2 etc
    pub name: TokenStream2,
    pub ty: &'a Type,
    pub span: Span,
}

pub fn fields_named_to_vec_fields(fields: &FieldsNamed) -> Vec<StructOrTupleField> {
    fields
        .named
        .iter()
        .map(|f| {
            let field_name = f.ident.as_ref().unwrap();

            StructOrTupleField {
                name: quote! {#field_name},
                ty: &f.ty,
                span: f.span(),
            }
        })
        .collect()
}

/// For making the `DiffN` in dipa::private::{Diff2, Diff3, ... etc}.
pub fn make_diff_n_ident(field_count: usize, span: Span) -> Ident {
    Ident::new(&format!("Diff{}", field_count), span)
}

/// Generate the tokens to match on the different fields within the struct to see which changed,
/// then combine the changes into one final Diff for the entire struct.
///
///
/// ```no_run
/// let diff_0 = self.some_field_name.create_patch_towards(&end_state.some_field_name);
/// let diff_1 = self.another_field_name.create_patch_towards(&end_state.another_field_name);
///
/// let diff = match (diff_0.1.did_change, diff_1.1.did_change) {
///     (false, false) => Diff2::NoChange,
///     (true, false) => Diff2::Change_0(diff_0.0),
///     (false, true) => Diff2::Change_1(diff_1.0),
///     (true, true) => Diff2::Change_0_1(diff_0.0, diff_1.0)
/// };
/// let did_change = match (diff_0.1.did_change, diff_1.1.did_change) {
///     (false, false) => false,
///     _ => true
/// };
/// ```
pub fn make_match_diff_tokens(
    struct_name: &Ident,
    fields: &[StructOrTupleField],
    start_field_prefix: TokenStream2,
    end_field_prefix: TokenStream2,
) -> TokenStream2 {
    let bool_combinations = make_bool_combinations(fields.len());

    let field_diffs_statements = field_diff_statements(
        &Ident::new("end_state", struct_name.span()),
        &fields,
        start_field_prefix,
        end_field_prefix,
    );

    // (diff_0.1.did_change, diff_1.1.did_change)
    let mut did_change_tokens = vec![];
    for (idx, f) in fields.iter().enumerate() {
        let diff_ident = Ident::new(&format!("diff_{}", idx), f.span);
        did_change_tokens.push(quote! { #diff_ident.1.did_change });
    }

    let match_diff_inner_tokens =
        make_match_diff_inner_tokens(&struct_name, &bool_combinations, &fields);

    let all_false = all_false_idents(fields.len(), struct_name.span());

    quote! {
        #(#field_diffs_statements)*

        let diff = match (#(#did_change_tokens),*) {
            #(#match_diff_inner_tokens)*
        };
        let did_change = match (#(#did_change_tokens),*) {
            (#(#all_false),*) => false,
            _ => true
        };
    }
}

/// Generate the tokens to match on the different possible Diff's and apply the appropriate patches
/// to the struct's sub fields.
///
/// let field0_mut_ref = &mut self.some_field_name;
/// let field1_mut_ref = &mut self.another_field_name;
///
/// match patch {
///     #diff_n::NoChange => {}
///     #diff_n::Change_0(field0_patch) => {
///         field0_mut_ref.apply_patch(field0_patch);
///     }
///     #diff_n::Change_1(field1_patch) => {
///         field1_mut_ref.apply_patch(field1_patch);
///     }
///     #diff_n::Change_0_1(field0_patch, field1_patch) => {
///         field0_mut_ref.apply_patch(field0_patch);
///         field1_mut_ref.apply_patch(field1_patch);
///     }
/// };
pub fn make_match_patch_tokens(struct_name: &Ident, fields: &[StructOrTupleField]) -> TokenStream2 {
    let self_ident = Ident::new("self", struct_name.span());
    let bool_combinations = make_bool_combinations(fields.len());

    // dipa::private::{Diff2, Diff3, ... etc}
    let diff_n = Ident::new(&format!("Diff{}", fields.len()), struct_name.span());

    let field_mut_refs = field_mutable_references(&self_ident, &fields);

    let match_patch_inner_tokens =
        make_match_patch_inner_tokens(&struct_name, &bool_combinations, fields.len());

    quote! {
      #(#field_mut_refs)*

      match patch {
         #diff_n::NoChange => {}
         #(#match_patch_inner_tokens)*
     };
    }
}

/// Get the Diff associated types for all of the fields.
/// i.e. vec![<u8::Diff, Option<f64>::Diff, ..]
pub fn field_associated_diff_types(fields: &[StructOrTupleField]) -> Vec<TokenStream2> {
    fields
        .iter()
        .map(|field| {
            let ty = &field.ty;

            quote! {
            <#ty as dipa::Diffable<'p, #ty>>::Diff
            }
        })
        .collect()
}

/// Get the Patch associated types for all of the fields.
/// i.e. vec![<u8::Diff, Option<f64>::Diff, ..]
pub fn field_associated_patch_types(fields: &[StructOrTupleField]) -> Vec<TokenStream2> {
    fields
        .iter()
        .map(|field| {
            let ty = &field.ty;

            quote! {
            <#ty as dipa::Diffable<'p, #ty>>::Patch
            }
        })
        .collect()
}

/// let diff_0 = self.some_field_name.create_patch_towards(&end_state.some_field_name);
/// let diff_1 = self.another_field_name.create_patch_towards(&end_state.another_field_name);
fn field_diff_statements(
    target_ident: &Ident,
    fields: &[StructOrTupleField],
    start_field_prefix: TokenStream2,
    end_field_prefix: TokenStream2,
) -> Vec<TokenStream2> {
    fields
        .iter()
        .enumerate()
        .map(|(field_idx, field)| {
            let field_name = &field.name;

            let diff_idx_ident = Ident::new(&format!("diff_{}", field_idx), field_name.span());

            quote! {
            let #diff_idx_ident = #start_field_prefix#field_name.create_patch_towards(&#end_field_prefix#field_name);
            }
        })
        .collect()
}

/// let field0_mut_ref = &mut self.some_field_name;
/// let field1_mut_ref = &mut self.another_field_name;
fn field_mutable_references(
    host_ident: &Ident,
    fields: &[StructOrTupleField],
) -> Vec<TokenStream2> {
    fields
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
        .collect()
}

fn make_bool_combinations(bool_count: usize) -> Vec<Vec<bool>> {
    match bool_count {
        2 => make_bool_combinations_2(),
        3 => make_bool_combinations_3(),
        4 => make_bool_combinations_4(),
        _ => panic!(
            r#"
TODO: Support larger structs.
"#
        ),
    }
}

fn all_false_idents(count: usize, span: Span) -> Vec<Ident> {
    vec![Ident::new("false", span); count]
}

/// Generate the inside of a match statement that returns a diff based on which fields have changed.
///
/// (false, false, false) => Diff3::NoChange,
/// (true, false, true) => Diff3::Change_0_2(diff_0.0, diff_2.0),
fn make_match_diff_inner_tokens(
    struct_name: &Ident,
    bool_combinations: &[Vec<bool>],
    fields: &[StructOrTupleField],
) -> Vec<TokenStream2> {
    let mut match_diff_inner_tokens = vec![];

    let mut all_tuple_entries_are_false = true;

    // dipa::private::{Diff2, Diff3, ... etc}
    let diff_n = Ident::new(&format!("Diff{}", fields.len()), struct_name.span());

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
    struct_name: &Ident,
    bool_combinations: &[Vec<bool>],
    field_count: usize,
) -> Vec<TokenStream> {
    // dipa::private::{Diff2, Diff3, ... etc}
    let diff_n = Ident::new(&format!("Diff{}", field_count), struct_name.span());

    let mut match_patch_inner_tokens = vec![];
    let mut all_tuple_entries_are_false = true;

    for bools in bool_combinations {
        let changed_keys = get_change_variant_name(&struct_name, &bools);

        let incoming_fields = patched_field_patch_names(&struct_name, &bools);

        let patch_expressions = get_patch_expressions(&struct_name, &bools);

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

    match_patch_inner_tokens
}

/// example if the first, third and sixth field in the struct changed.
///   Change_0_2_5
fn get_change_variant_name(struct_name: &Ident, bools: &[bool]) -> Ident {
    let mut changed_keys = "Change".to_string();

    for (idx, _bool) in bools
        .iter()
        .enumerate()
        .filter(|(_, did_change)| **did_change)
    {
        changed_keys += &format!("_{}", idx);
    }

    Ident::new(&format!("{}", changed_keys), struct_name.span())
}

/// The names of each of the new patches that we need to apply.
///
/// field0_patch, field2_patch field5_patch
fn patched_field_patch_names(struct_name: &Ident, bools: &[bool]) -> Vec<Ident> {
    let mut incoming_fields = vec![];

    for (idx, _bool) in bools
        .iter()
        .enumerate()
        .filter(|(_, did_change)| **did_change)
    {
        let patch_ident = Ident::new(&format!("field{}_patch", idx), struct_name.span());

        incoming_fields.push(patch_ident);
    }

    incoming_fields
}

/// field0_mut_ref.apply_patch(field0_patch);
/// field2_mut_ref.apply_patch(field2_patch);
/// field5_mut_ref.apply_patch(field5_patch);
fn get_patch_expressions(struct_name: &Ident, bools: &[bool]) -> Vec<TokenStream2> {
    let mut patch_expressions = vec![];

    for (idx, _bool) in bools
        .iter()
        .enumerate()
        .filter(|(_, did_change)| **did_change)
    {
        let patch_ident = Ident::new(&format!("field{}_patch", idx), struct_name.span());

        let patch_expression_ident =
            Ident::new(&format!("field{}_mut_ref", idx), struct_name.span());
        patch_expressions.push(quote! {#patch_expression_ident.apply_patch(#patch_ident);});
    }

    patch_expressions
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
