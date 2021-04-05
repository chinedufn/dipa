use crate::enum_utils::{diff_type_name, patch_type_name, ParsedEnum};
use quote::ToTokens;
use quote::__private::TokenStream;
use syn::Ident;
use syn::__private::TokenStream2;

impl ParsedEnum {
    /// Generate the tokens for the diff or patch type for an enum that has at least one variant
    /// that has one or more named or unnamed fields.
    ///
    /// ```
    /// # #[allow(unused)]
    /// enum MyEnum {
    ///     One(u8),
    ///     Two
    /// }
    ///
    /// # #[allow(unused)]
    /// enum MyEnumDiff<'p> {
    ///     OneNoChange,
    ///     ChangedToVariantOne(&'p u8),
    ///     OneChange_0(<u8 as Diffable<'p, u8>>::Delta),
    ///     TwoNoChange,
    ///     ChangedToVariantTwo,
    /// }
    /// ```
    pub fn create_associated_type_for_enum_with_fields(
        &self,
        associated_type: DipaAssociatedType,
    ) -> TokenStream2 {
        let enum_associated_ty = self.ty_name(associated_type);

        let mut diff_ty_variants = vec![];

        for variant in self.variants.iter() {
            diff_ty_variants.extend_from_slice(&variant.diff_type_variants(associated_type));
        }

        let maybe_lifetime = associated_type.maybe_lifetime();

        quote! {
            enum #enum_associated_ty#maybe_lifetime {
                #(#diff_ty_variants),*,
            }
        }
    }

    fn ty_name(&self, associated_type: DipaAssociatedType) -> Ident {
        match associated_type {
            DipaAssociatedType::Delta => diff_type_name(&self.name),
            DipaAssociatedType::DeltaOwned => patch_type_name(&self.name),
        }
    }
}

#[derive(Copy, Clone)]
pub enum DipaAssociatedType {
    Delta,
    DeltaOwned,
}

impl ToTokens for DipaAssociatedType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            DipaAssociatedType::Delta => tokens.extend(quote! { Delta }),
            DipaAssociatedType::DeltaOwned => tokens.extend(quote! { DeltaOwned }),
        }
    }
}

impl DipaAssociatedType {
    fn maybe_lifetime(&self) -> TokenStream2 {
        if self.has_lifetime() {
            quote! { <'p> }
        } else {
            quote! {}
        }
    }

    pub fn has_lifetime(&self) -> bool {
        match self {
            DipaAssociatedType::Delta => true,
            DipaAssociatedType::DeltaOwned => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::multi_field_utils::StructOrTupleField;
    use crate::test_utils::{assert_starts_with_tokens, assert_tokens_eq};
    use quote::__private::{Ident, Span};
    use syn::Type;

    /// Verify that we properly generate an enum's diff type
    #[test]
    fn generates_diff_type() {
        let parsed_enum = ParsedEnum::new_test_two_variants_one_field();

        let tokens =
            parsed_enum.create_associated_type_for_enum_with_fields(DipaAssociatedType::Delta);

        let expected = quote! {
            enum MyEnumDiff<'p> {
                OneNoChange,
                ChangedToVariantOne(&'p u16),
                OneChange_0(<u16 as dipa::Diffable<'p, u16>>::Delta),
                TwoNoChange,
                ChangedToVariantTwo,
            }
        };

        assert_tokens_eq(&tokens, &expected);
    }

    /// Verify that we properly generate an enum's diff type
    #[test]
    fn generates_patch_type() {
        let parsed_enum = ParsedEnum::new_test_two_variants_one_field();

        let tokens =
            parsed_enum.create_associated_type_for_enum_with_fields(DipaAssociatedType::DeltaOwned);

        let expected = quote! {
            enum MyEnumPatch {
                OneNoChange,
                ChangedToVariantOne(u16),
                OneChange_0(<u16 as dipa::Diffable<'_, u16>>::DeltaOwned),
                TwoNoChange,
                ChangedToVariantTwo,
            }
        };

        assert_tokens_eq(&tokens, &expected);
    }
}
