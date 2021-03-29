use std::ops::Deref;
use syn::parse::{Parse, ParseStream, Result as SynResult};
use syn::{Attribute, Ident, Lit};

/// example: #[dipa(patch_derive = "Debug, Copy", ...)]
pub fn maybe_parse_raw_dipa_attribute(attrs: Vec<Attribute>) -> Option<Attribute> {
    attrs
        .into_iter()
        .find(|a| a.path.segments.last().unwrap().ident.to_string().as_str() == "dipa")
}

/// A parsed representation of the #[dipa(...)] container attribute.
#[derive(Debug)]
pub struct DipaAttrs {
    attrs: Vec<DipaAttr>,
}

impl Parse for DipaAttrs {
    fn parse(input: ParseStream) -> SynResult<Self> {
        if input.is_empty() {
            return Ok(DipaAttrs { attrs: vec![] });
        }

        let opts =
            syn::punctuated::Punctuated::<DipaAttr, syn::token::Comma>::parse_terminated(input)?;

        Ok(DipaAttrs {
            attrs: opts.into_iter().collect(),
        })
    }
}

/// All of the supported attributes within the #[dipa(...)] container attribute.
#[derive(Debug)]
pub enum DipaAttr {
    /// Used to add #[derive(...)] to the `MyTypeDiff` type that is generated for enums and
    /// structs.
    ///
    /// example: `diff_derive = "Debug, Copy, Clone"`
    DiffDerive(Lit),
    /// Used to add #[derive(...)] to the `MyTypePatch` type that is generated for enums and
    /// structs.
    ///
    /// example: `patch_derive = "Debug, Copy, Clone"`
    PatchDerive(Lit),
}

impl Parse for DipaAttr {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let original = input.fork();

        let content;
        parenthesized!(content in input);

        let key = content.parse::<Ident>()?;
        let _equals = content.parse::<Token![=]>()?;

        // diff_derive = "Debug, Copy"
        if key == "diff_derive" {
            let path_val = content.parse::<Lit>()?;

            return Ok(DipaAttr::DiffDerive(path_val));
        }

        // patch_derive = "Debug, Copy"
        if key == "patch_derive" {
            let path_val = content.parse::<Lit>()?;

            return Ok(DipaAttr::PatchDerive(path_val));
        }

        Err(original.error("unknown attribute"))
    }
}

impl Deref for DipaAttrs {
    type Target = Vec<DipaAttr>;

    fn deref(&self) -> &Self::Target {
        &self.attrs
    }
}
