use std::ops::{Deref, DerefMut};

use quote::__private::Span;
use syn::Ident;

mod all_combinations;
pub(crate) use self::all_combinations::make_bool_combinations;
use syn::__private::TokenStream2;

/// All of the field indices that have changed within a struct/tuple.
///
/// So 0 would mean the first field has changed, 1 is the second, etc.
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct ChangedFieldIndices(Vec<u8>);

impl ChangedFieldIndices {
    #[allow(missing_docs)]
    fn new(field_indices: Vec<u8>) -> Self {
        ChangedFieldIndices(field_indices)
    }

    ///  "Foo" ->
    ///     "FooNoChange"
    ///     "FooChange_0"
    ///     "FooChange_0_2"
    ///     ... etc   
    pub fn variant_name_ident(&self, prefix: &str, span: Span) -> Ident {
        Ident::new(&self.variant_name(prefix), span)
    }

    ///  "Foo" ->
    ///     "FooNoChange"
    ///     "FooChange_0"
    ///     "FooChange_0_2"
    ///     ... etc
    pub fn variant_name(&self, prefix: &str) -> String {
        if self.len() == 0 {
            format!("{}NoChange", prefix)
        } else {
            let mut changed = format!("{}Change", prefix);

            for idx in self.iter() {
                changed += &format!("_{}", idx);
            }

            changed
        }
    }

    /// patch0, patch2, patch5, ...
    pub fn patch_field_idents(&self, span: Span) -> Vec<Ident> {
        self.iter()
            .map(|idx| Ident::new(&format!("patch{}", idx), span))
            .collect()
    }

    /// diff0.0, diff2.0, diff5.0
    pub fn diffs(&self, span: Span) -> Vec<TokenStream2> {
        self.iter()
            .map(|idx| {
                let diff = Ident::new(&format!("diff{}", idx), span);
                
                quote! {
                    #diff.0
                }
            })
            .collect()
    }
}

impl Deref for ChangedFieldIndices {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ChangedFieldIndices {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
