use crate::dipa_attribute::{DipaAttrs, DipaContainerAttr};
use quote::ToTokens;
use syn::Ident;

impl DipaAttrs {
    pub fn parse_diff_and_patch_derives(&self) -> (Vec<Ident>, Vec<Ident>) {
        let mut diff_derives = vec![];
        let mut patch_derives = vec![];

        for attr in self.iter() {
            match attr {
                DipaContainerAttr::DiffDerives(lit) => {
                    diff_derives = parse_derives(lit);
                }
                DipaContainerAttr::PatchDerives(lit) => {
                    patch_derives = parse_derives(lit);
                }
                _ => {}
            }
        }

        (diff_derives, patch_derives)
    }
}

/// "Debug, Copy" -> vec![Debug, Copy]
fn parse_derives(lit: &syn::Lit) -> Vec<Ident> {
    let mut derive_idents = vec![];

    // "Debug, Copy"
    let derives = lit.to_token_stream().to_string();

    // Debug, Copy
    let derives = derives.trim_start_matches(r#"""#);
    let derives = derives.trim_end_matches(r#"""#);

    // [Debug, Copy]
    let derives = derives.split(",");

    for derive in derives.into_iter() {
        let derive_ident = Ident::new(derive.trim(), lit.span());
        derive_idents.push(derive_ident);
    }

    derive_idents
}
