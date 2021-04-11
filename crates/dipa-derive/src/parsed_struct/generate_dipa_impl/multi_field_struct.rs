use crate::dipa_attribute::{DipaAttrs, FieldBatchingStrategy};
use crate::impl_dipa;
use crate::multi_field_utils::{
    make_match_diff_tokens, make_match_patch_tokens, StructOrTupleField,
};
use crate::parsed_struct::ParsedStruct;
use syn::__private::TokenStream2;
use syn::spanned::Spanned;
use syn::{Ident, Type};

mod generate_no_batching_apply_patch_tokens;
mod generate_no_batching_create_delta_tokens;

impl ParsedStruct {
    /// Generate an implementation of Diffable for a struct with 2 or more fields.
    pub fn generate_multi_field_struct_impl(&self, dipa_attrs: &DipaAttrs) -> TokenStream2 {
        let struct_name = &self.name;

        let prefix = self.name.to_string();
        let prefix = &prefix;
        let delta_name = self.fields.delta_name(prefix);
        let delta_owned_name = self.fields.delta_owned_name(prefix);

        let diff_ty = Type::Verbatim(quote! {#delta_name});
        let patch_ty = Type::Verbatim(quote! {#delta_owned_name});

        let field_diffs_statements = field_diff_statements(&self.fields);

        let (calculate_delta_tokens, apply_patch_tokens) = match dipa_attrs
            .field_batching_strategy
            .unwrap_or(FieldBatchingStrategy::default())
        {
            FieldBatchingStrategy::OneBatch => {
                let field_mut_refs = field_mutable_references(&self.fields);

                (
                    make_match_diff_tokens(
                        diff_ty,
                        "",
                        struct_name.span(),
                        &self.fields,
                        dipa_attrs,
                    ),
                    make_match_patch_tokens(
                        struct_name.span(),
                        &patch_ty,
                        &self.fields,
                        field_mut_refs,
                        dipa_attrs,
                    ),
                )
            }
            FieldBatchingStrategy::ManyBatches => {
                todo!("Implement many batches")
            }
            FieldBatchingStrategy::NoBatching => (
                self.generate_no_batching_create_delta_tokens(),
                self.generate_no_batching_apply_patch_tokens(),
            ),
        };

        let delta_tys = self.fields.generate_delta_types(prefix, dipa_attrs);

        let dipa_impl = impl_dipa(
            struct_name,
            quote! {#delta_name<'s, 'e>},
            quote! {#delta_owned_name},
            quote! {
               #field_diffs_statements
               #calculate_delta_tokens

                CreatedDelta {
                    delta,
                    did_change,
                }
            },
            quote! {
               #apply_patch_tokens
            },
        );

        let tokens = quote! {
            #delta_tys

            #dipa_impl
        };

        // panic!("{}", tokens.to_string());

        tokens
    }
}

/// let field0_mut_ref = &mut self.some_field_name;
/// let field1_mut_ref = &mut self.another_field_name;
fn field_mutable_references(fields: &[StructOrTupleField]) -> Vec<TokenStream2> {
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

/// let diff0 = self.some_field_name.create_delta_towards(&end_state.some_field_name);
/// let diff1 = self.another_field_name.create_delta_towards(&end_state.another_field_name);
/// let did_change = diff0.1.did_change || diff1.1.did_change;
pub fn field_diff_statements(fields: &[StructOrTupleField]) -> TokenStream2 {
    let diffs = field_diff_calculations(fields);

    let did_change: Vec<TokenStream2> = fields
        .iter()
        .enumerate()
        .map(|(field_idx, field)| {
            let field_name = &field.name;

            let diff_idx_ident = Ident::new(&format!("diff{}", field_idx), field_name.span());

            quote! {
            #diff_idx_ident.1.did_change
            }
        })
        .collect();

    quote! {
        #(#diffs)*
        let did_change = #(#did_change)||*;
    }
}

/// let diff0 = self.some_field_name.create_delta_towards(&end_state.some_field_name);
/// let diff1 = self.another_field_name.create_delta_towards(&end_state.another_field_name);
fn field_diff_calculations(fields: &[StructOrTupleField]) -> Vec<TokenStream2> {
    fields
        .iter()
        .enumerate()
        .map(|(field_idx, field)| {
            let field_name = &field.name;

            let diff_idx_ident = Ident::new(&format!("diff{}", field_idx), field_name.span());

            quote! {
            let #diff_idx_ident = self.#field_name.create_delta_towards(&end_state.#field_name);
            }
        })
        .collect()
}
