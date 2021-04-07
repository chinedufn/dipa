//! Validate the usage of different dipa attributes on a type's container, variants and
//! fields.

use crate::dipa_attribute::{DipaAttrs, DipaContainerAttr, FieldBatchingStrategy};
use crate::parsed_struct::ParsedStruct;
use syn::__private::TokenStream2;

impl ParsedStruct {
    /// Validate `#[dipa(...)]` attributes on a struct. Certain attributes or combinations of
    /// attributes might not be allowed based on information about the struct.
    ///
    /// If any issues are found tokens containing a compile time error are returned.
    ///
    /// ```no_run
    /// #[derive(DiffPatch)]
    /// #[dipa(...)] // <-- These are being validated.
    /// struct Foo {}
    /// ```
    pub fn validate_struct_container_attributes(
        &self,
        attributes: &DipaAttrs,
    ) -> Result<(), TokenStream2> {
        let mut errs = vec![];

        for attribute in attributes.iter() {
            match attribute {
                DipaContainerAttr::DiffDerive(_) => {}
                DipaContainerAttr::PatchDerive(_) => {}
                DipaContainerAttr::MaxDeltaBatch(_) => {}
                DipaContainerAttr::FieldBatchingStrategy(_field_batching_strat) => {
                    if let Err(err) = FieldBatchingStrategy::validate_field_count(
                        self.fields.len(),
                        self.fields_span,
                    ) {
                        errs.push(err);
                    }
                }
            };
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
