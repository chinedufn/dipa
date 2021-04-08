use crate::dipa_attribute::{DipaAttrs, FieldBatchingStrategy};
use crate::multi_field_utils::ParsedFields;
use syn::Ident;
use syn::__private::TokenStream2;

mod no_batching;
mod one_batch;

impl ParsedFields {
    /// Given named or unnamed fields return a Delta type that encompasses all of those fields.
    /// Depending on the field batching strategy this might be an enum or a struct.
    pub fn generate_delta_types(&self, prefix: &str, dipa_attrs: &DipaAttrs) -> TokenStream2 {
        if self.len() < 2 {
            unreachable!(
                r#"Out logic is spread out a bit. Need to move the logic for 0 and 1 field
into this function."#
            )
        }

        match dipa_attrs
            .field_batching_strategy
            .unwrap_or(FieldBatchingStrategy::default())
        {
            FieldBatchingStrategy::OneBatch => {
                self.generate_delta_type_one_batch(prefix, dipa_attrs)
            }
            FieldBatchingStrategy::ManyBatches => {
                todo!(r#"Many batches strategy is not yet implemented."#)
            }
            FieldBatchingStrategy::NoBatching => {
                self.generate_delta_type_no_batching(prefix, dipa_attrs)
            }
        }
    }

    pub fn delta_name(&self, prefix: &str) -> Ident {
        Ident::new(&format!("{}Delta", prefix), self.span)
    }

    pub fn delta_owned_name(&self, prefix: &str) -> Ident {
        Ident::new(&format!("{}DeltaOwned", prefix), self.span)
    }
}
