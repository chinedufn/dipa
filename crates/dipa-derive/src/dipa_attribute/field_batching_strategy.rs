use crate::dipa_attribute::DipaContainerAttr;
use crate::{SynError, SynResult};
use quote::__private::Span;
use std::str::FromStr;
use syn::parse::ParseBuffer;
use syn::LitStr;
use syn::__private::TokenStream2;
use syn::spanned::Spanned;

/// The strategy used to encode the delta of a struct or enum variant that has 2 or more fields.
#[derive(Debug, Copy, Clone)]
pub enum FieldBatchingStrategy {
    /// Use a single enum to encode the delta.
    OneBatch,
    /// Use a struct that has one enum for every `max_fields_per_batch` fields in the original
    /// struct.
    ManyBatches,
    /// Use a struct that has one field for every field in the original struct.
    NoBatching,
}

impl DipaContainerAttr {
    pub(super) fn parse_field_batching_strategy(content: &ParseBuffer) -> SynResult<Self> {
        let strategy = content.parse::<LitStr>()?;
        let strategy = FieldBatchingStrategy::from_str(&strategy.value()).map_err(|_provided| {
            SynError::new(
                strategy.span(),
                format!(
                    r#"delta_strategy must equal "one_batch", "many_batches" or "no_batching".

See: https://chinedufn/github.io/dipa/using-derive/attributes/index.html#container-attributes
"#
                ),
            )
        })?;

        Ok(DipaContainerAttr::FieldBatchingStrategy(strategy))
    }
}

impl FieldBatchingStrategy {
    /// Verify that the field count is >= 2. If not, return a token stream that contains a compile
    /// time error.
    pub fn validate_field_count(field_count: usize, fields_span: Span) -> Result<(), TokenStream2> {
        if field_count < 2 {
            let fields_span = fields_span.span();
            let error = format!(
                r#"A struct or enum variant must have at least two fields for field delta batching
to be useful. Try removing the field_batching_strategy attribute."#
            );
            let error = quote_spanned! {fields_span=>
                compile_error!(#error);
            };
            return Err(error);
        }

        Ok(())
    }
}

impl FromStr for FieldBatchingStrategy {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "one_batch" => Ok(Self::OneBatch),
            "many_batches" => Ok(Self::ManyBatches),
            "no_batching" => Ok(Self::NoBatching),
            _ => Err(s.to_string()),
        }
    }
}
