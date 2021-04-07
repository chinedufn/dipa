use crate::dipa_attribute::DipaContainerAttr;
use syn::parse::ParseBuffer;
use syn::LitInt;
use syn::{Error as SynError, Result as SynResult};

// The minimum value for the dipa(max_fields_per_batch = u8) attribute
const MIN_MAX_FIELDS_PER_BATCH: u8 = 2;

// The maximum value for the dipa(max_fields_per_batch = u8) attribute
const MAX_MAX_FIELDS_PER_BATCH: u8 = 7;

impl DipaContainerAttr {
    pub(super) fn parse_max_fields_per_batch(content: &ParseBuffer) -> SynResult<Self> {
        let path_val = content.parse::<LitInt>()?;

        let max_fields_per_batch: u8 = path_val.base10_parse()?;

        // dipa-derive-test/src/all_tests/ui/max_fields_per_batch_too_small.rs
        if max_fields_per_batch < MIN_MAX_FIELDS_PER_BATCH {
            return Err(SynError::new(
                path_val.span(),
                format!(
                    r#"The max_fields_per_batch attribute must be greater than or equal to {min}.
Consider using the `dipa(delta_strategy = "no_batching")` attribute if you are trying to
disable field batching."#,
                    min = MIN_MAX_FIELDS_PER_BATCH
                ),
            ));
        }

        // dipa-derive-test/src/all_tests/ui/max_fields_per_batch_too_large.rs
        if max_fields_per_batch > MAX_MAX_FIELDS_PER_BATCH {
            return Err(SynError::new(
                path_val.span(),
                format!(
                    r#"The max_fields_per_batch attribute must be less than or equal to {max}.
This limit is meant to prevent the use of large values, since as max_fields_per_batch grows
compile times grow exponentially."#,
                    max = MAX_MAX_FIELDS_PER_BATCH
                ),
            ));
        }

        Ok(DipaContainerAttr::MaxDeltaBatch(max_fields_per_batch))
    }
}
