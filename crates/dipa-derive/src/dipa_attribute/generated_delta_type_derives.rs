use quote::ToTokens;
use syn::Ident;

/// "Debug, Copy" -> vec![Debug, Copy]
pub fn parse_derives(lit: &syn::Lit) -> Vec<Ident> {
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
