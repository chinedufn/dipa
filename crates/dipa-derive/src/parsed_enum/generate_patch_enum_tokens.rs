use crate::dipa_attribute::DipaAttrs;
use crate::parsed_enum::ParsedEnum;
use syn::__private::TokenStream2;

impl ParsedEnum {
    /// Create a match statement patching this enum based on the patch this enum variant with another.
    ///
    /// ```
    /// # #[allow(unused)]
    /// enum MyEnum {
    ///     VariantA,
    ///     VariantB { some_field: Vec<f32>, another_field: Option<u64> },
    ///     VariantC (i16)
    /// }
    /// ```
    ///
    /// The final generate tokens will look something like:
    ///
    /// ```
    /// # use quote::quote;
    /// quote! {
    ///     match patch {
    ///         MyEnumDeltaOwned::VariantANoChange => { }
    ///         MyEnumDeltaOwned::ChangedToVariantA => {
    ///             *self = MyEnumDeltaOwned::VariantA;
    ///         }
    ///
    ///         MyEnumDeltaOwned::VariantBNoChange => { }
    ///         MyEnumDeltaOwned::ChangedToVariantB {
    ///             some_field: patch_some_field, patch_another_field
    ///         } => {
    ///             *self = MyEnumDeltaOwned::VariantB {
    ///                 some_field: patch_some_field,
    ///                 another_field: patch_another_field,
    ///             };
    ///         }
    ///         MyEnumDeltaOwned::VariantBChange_0(patch0) => {
    ///             match self {
    ///                 MyEnum::VariantB {
    ///                     some_field: patch_some_field, another_field: patch_another_field
    ///                 } => {
    ///                     some_field.apply_patch(patch0);
    ///                 }
    ///                 _ => { panic!("TODO: Return Result::Err") }
    ///             }
    ///         }
    ///         MyEnumDeltaOwned::VariantBChange_1(patch1) => {
    ///             match self {
    ///                 MyEnum::VariantB {
    ///                     some_field: patch_some_field, another_field: patch_another_field
    ///                 } => {
    ///                     another_field.apply_patch(patch1);
    ///                 }
    ///                 _ => { panic!("TODO: Return Result::Err") }
    ///             }
    ///         }
    ///         MyEnumDeltaOwned::VariantBChange_0_1(patch0, patch1) => {
    ///             match self {
    ///                 MyEnum::VariantB {
    ///                     some_field: patch_some_field, another_field: patch_another_field
    ///                 } => {
    ///                     some_field.apply_patch(patch0);
    ///                     another_field.apply_patch(patch1);
    ///                 }
    ///                 _ => { panic!("TODO: Return Result::Err") }
    ///             }
    ///         }      
    ///
    ///         MyEnumDeltaOwned::VariantCNoChange => { }
    ///         MyEnumDeltaOwned::ChangedToVariantC(patch0)  => {
    ///             *self = MyEnumDeltaOwned::VariantC(patch0);
    ///         }
    ///         MyEnumDeltaOwned::VariantCChange_0(patch0) => {
    ///             match self {
    ///                 MyEnum::VariantC(field_0) => {
    ///                     field_0.apply_patch(patch0);
    ///                 }
    ///                 _ => { panic!("TODO: Return Result::Err") }
    ///             }   
    ///         }
    ///     }
    /// };
    /// ```
    pub fn create_patch_match_stmt(&self, dipa_attrs: &DipaAttrs) -> TokenStream2 {
        let mut inner_tokens = vec![];

        for variant in self.variants.iter() {
            inner_tokens.extend_from_slice(&variant.generate_patch_blocks(&self.name, dipa_attrs));
        }

        quote! {
            #[allow(unused)] // Easier for now than prefixing unused fields with underscores
            match patch {
                #(#inner_tokens)*
            }
        }
    }
}
