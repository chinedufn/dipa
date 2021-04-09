//! Validate the usage of different dipa attributes on a type's container, variants and
//! fields.

use crate::dipa_attribute::{DipaAttrs, FieldBatchingStrategy};
use crate::parsed_struct::ParsedStruct;
use syn::__private::TokenStream2;

impl ParsedStruct {
    // Validate `#[dipa(...)]` attributes on a struct. Certain attributes or combinations of
    // attributes might not be allowed based on information about the struct.
    //
    // If any issues are found tokens containing a compile time error are returned.
    //
    // ```
    // #[derive(DiffPatch)]
    // #[dipa(...)] // <-- These are being validated.
    // struct Foo {}
    // ```
    pub fn validate_struct_container_attributes(
        &self,
        attributes: &DipaAttrs,
    ) -> Result<(), TokenStream2> {
        let mut errs = vec![];

        if let Some(_field_batching_strategy) = attributes.field_batching_strategy {
            if let Err(err) =
                FieldBatchingStrategy::validate_field_count(self.fields.len(), self.fields.span)
            {
                errs.push(err);
            }
        }

        if errs.len() == 0 {
            Ok(())
        } else {
            let errs = quote! {
                #(#errs)*
            };

            Err(errs)
        }
    }
}
