use crate::dipa_attribute::DipaAttrs;
use crate::impl_dipa;
use crate::parsed_enum::{delta_owned_type_name, delta_type_name, ParsedEnum};
use syn::__private::TokenStream2;

mod generate_one_batch_apply_patch_tokens;
mod generate_one_batch_create_delta_tokens;

impl ParsedEnum {
    /// Generate an implementation of Diffable and Patchable for an enum that has one variant where that
    /// variant has 2 or more fields.
    pub fn generate_single_variant_multi_field_dipa_impl(
        &self,
        dipa_attrs: &DipaAttrs,
    ) -> TokenStream2 {
        let delta_ty = delta_type_name(&self.name);
        let delta_owned_ty = delta_owned_type_name(&self.name);

        let delta_type_definitions = self.variants[0]
            .fields
            .unwrap_parsed_fields()
            .generate_delta_types(&self.name.to_string(), dipa_attrs);

        let dipa_impl = impl_dipa(
            &self.name,
            quote! {#delta_ty<'s, 'e>},
            quote! {#delta_owned_ty},
            self.generate_single_variant_multi_field_one_batch_create_delta_tokens(dipa_attrs),
            self.generate_single_variant_multi_field_one_batch_apply_patch_tokens(dipa_attrs),
        );

        quote! {
            #delta_type_definitions
            #dipa_impl
        }
    }
}
