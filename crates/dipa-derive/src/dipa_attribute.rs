use std::ops::Deref;

use syn::parse::{Parse, ParseStream, Result as SynResult};
use syn::{Attribute, Ident, Lit};

pub use self::field_batching_strategy::*;

mod field_batching_strategy;
mod max_delta_batch;

/// example: #[dipa(patch_derive = "Debug, Copy", ...)]
pub fn maybe_parse_raw_dipa_attribute(attrs: Vec<Attribute>) -> Option<Attribute> {
    attrs
        .into_iter()
        .find(|a| a.path.segments.last().unwrap().ident.to_string().as_str() == "dipa")
}

/// A parsed representation of the #[dipa(...)] container attribute.
#[derive(Debug)]
pub struct DipaAttrs {
    attrs: Vec<DipaContainerAttr>,
}

impl Parse for DipaAttrs {
    fn parse(input: ParseStream) -> SynResult<Self> {
        if input.is_empty() {
            return Ok(DipaAttrs { attrs: vec![] });
        }

        let opts =
            syn::punctuated::Punctuated::<DipaContainerAttr, syn::token::Comma>::parse_terminated(
                input,
            )?;

        Ok(DipaAttrs {
            attrs: opts.into_iter().collect(),
        })
    }
}

/// All of the supported attributes within the #[dipa(...)] container attribute.
#[derive(Debug)]
pub enum DipaContainerAttr {
    /// Used to add #[derive(...)] to the `MyTypeDiff` type that is generated for enums and
    /// structs.
    ///
    /// example: `dipa(diff_derive = "Debug, Copy, Clone")`
    DiffDerive(Lit),
    /// Used to add #[derive(...)] to the `MyTypePatch` type that is generated for enums and
    /// structs.
    ///
    /// example: `dipa(patch_derive = "Debug, Copy, Clone")`
    PatchDerive(Lit),
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

            return Ok(DipaContainerAttr::DiffDerive(path_val));
        }

        // patch_derives = "Debug, Copy"
        if key == "patch_derives" {
            let path_val = content.parse::<Lit>()?;

            return Ok(DipaContainerAttr::PatchDerive(path_val));
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

impl Deref for DipaAttrs {
    type Target = Vec<DipaContainerAttr>;

    fn deref(&self) -> &Self::Target {
        &self.attrs
    }
}
