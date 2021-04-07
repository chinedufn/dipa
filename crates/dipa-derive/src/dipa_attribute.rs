use syn::parse::{Parse, ParseStream, Result as SynResult};
use syn::{Attribute, Ident, Lit};

pub use self::field_batching_strategy::*;
use crate::dipa_attribute::generated_delta_type_derives::parse_derives;

mod field_batching_strategy;
mod generated_delta_type_derives;
mod max_delta_batch;

/// example: #[dipa(patch_derive = "Debug, Copy", ...)]
pub fn maybe_parse_raw_dipa_attribute(attrs: Vec<Attribute>) -> Option<Attribute> {
    attrs
        .into_iter()
        .find(|a| a.path.segments.last().unwrap().ident.to_string().as_str() == "dipa")
}

/// A parsed representation of the #[dipa(...)] container attribute.
#[derive(Debug, Default)]
pub struct DipaAttrs {
    pub diff_derives: Vec<Ident>,
    pub patch_derives: Vec<Ident>,
    pub max_delta_batch: Option<u8>,
    pub field_batching_strategy: Option<FieldBatchingStrategy>,
}

impl Parse for DipaAttrs {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let mut dipa_attrs = DipaAttrs::default();

        if input.is_empty() {
            return Ok(dipa_attrs);
        }

        let opts =
            syn::punctuated::Punctuated::<DipaContainerAttr, syn::token::Comma>::parse_terminated(
                input,
            )?;

        for dipa_attr in opts.into_iter() {
            match dipa_attr {
                DipaContainerAttr::DiffDerives(lit) => {
                    dipa_attrs.diff_derives = parse_derives(&lit);
                }
                DipaContainerAttr::PatchDerives(lit) => {
                    dipa_attrs.patch_derives = parse_derives(&lit);
                }
                DipaContainerAttr::MaxDeltaBatch(max) => {
                    dipa_attrs.max_delta_batch = Some(max);
                }
                DipaContainerAttr::FieldBatchingStrategy(f) => {
                    dipa_attrs.field_batching_strategy = Some(f);
                }
            };
        }

        Ok(dipa_attrs)
    }
}

/// All of the supported attributes within the #[dipa(...)] container attribute.
#[derive(Debug)]
pub enum DipaContainerAttr {
    /// Used to add #[derive(...)] to the `MyTypeDiff` type that is generated for enums and
    /// structs.
    ///
    /// example: `dipa(diff_derive = "Debug, Copy, Clone")`
    DiffDerives(Lit),
    /// Used to add #[derive(...)] to the `MyTypePatch` type that is generated for enums and
    /// structs.
    ///
    /// example: `dipa(patch_derive = "Debug, Copy, Clone")`
    PatchDerives(Lit),
    /// Used enable larger enums to be used to batch a struct's fields into Delta types.
    /// Larger batch sizes allow for even smaller diffs at the cost of some compile time.
    ///
    /// example: `dipa(max_delta_batch = 6)`
    MaxDeltaBatch(u8),
    /// Controls how fields with a struct are batched when generating the delta type.
    FieldBatchingStrategy(FieldBatchingStrategy),
}

impl Parse for DipaContainerAttr {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let original = input.fork();

        let content;
        parenthesized!(content in input);

        let key = content.parse::<Ident>()?;
        let _equals = content.parse::<Token![=]>()?;

        // diff_derives = "Debug, Copy"
        if key == "diff_derives" {
            let path_val = content.parse::<Lit>()?;

            return Ok(DipaContainerAttr::DiffDerives(path_val));
        }

        // patch_derives = "Debug, Copy"
        if key == "patch_derives" {
            let path_val = content.parse::<Lit>()?;

            return Ok(DipaContainerAttr::PatchDerives(path_val));
        }

        // max_delta_batch = 6
        if key == "max_delta_batch" {
            return Self::parse_max_delta_batch(&content);
        }

        // field_batching_strategy = "no_batching"
        if key == "field_batching_strategy" {
            return Self::parse_field_batching_strategy(&content);
        }

        Err(original.error("unknown attribute"))
    }
}
