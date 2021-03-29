use crate::enum_utils::{DipaAssociatedType, EnumVariant};
use crate::multi_field_utils::ChangedFieldIndices;
use quote::ToTokens;
use syn::__private::TokenStream2;
use syn::{Type, TypeReference};

impl EnumVariant {
    /// All of the variants in the MyEnumDiff type that relate to this enum variant.
    ///
    /// ```
    /// # use quote::quote;
    ///
    /// # #[allow(unused)]
    /// enum MyEnum {
    ///     One(u16),
    ///     Two
    /// }
    ///
    /// // For variant `One` we would generate
    /// vec![
    ///     quote!{ OneNoChange },
    ///     quote!{ ChangedToVariantOne(&'p u16) },
    ///     quote!{ OneChange_0(<u16 as Diffable<'p, u16>::Diff) }
    /// ];
    /// ```
    pub fn diff_type_variants(&self, associated_type: DipaAssociatedType) -> Vec<TokenStream2> {
        let mut diff_ty_variants = vec![];

        let no_change = self.variant_no_change();
        diff_ty_variants.push(quote! { #no_change });

        if self.fields.len() > 0 {
            diff_ty_variants.push(self.changed_to_with_ref_fields(associated_type));
            diff_ty_variants.extend_from_slice(&self.change_same_variant(associated_type));
        } else {
            let changed_to = self.changed_to_variant();
            diff_ty_variants.push(quote! { #changed_to });
        }

        diff_ty_variants
    }

    /// quote![ChangedToVariantOne(&'p u16)
    fn changed_to_with_ref_fields(&self, associated_type: DipaAssociatedType) -> TokenStream2 {
        let tys: Vec<Type> = self
            .fields
            .iter()
            .map(|f| {
                let ty = if associated_type.has_lifetime() {
                    let ref_ty = Type::Reference(TypeReference {
                        and_token: syn::token::And::default(),
                        lifetime: Some(syn::Lifetime::new("'p", f.span)),
                        mutability: None,
                        elem: Box::new(f.ty.clone()),
                    });
                    Type::Verbatim(quote! {#ref_ty})
                } else {
                    let owned_ty = &f.ty;
                    Type::Verbatim(quote! {#owned_ty})
                };

                ty
            })
            .collect();

        // example: (&u8, &Vec<f32>)
        let ref_tys = quote! {( #(#tys),* )};

        let changed_to = self.changed_to_variant();

        quote! {
            #changed_to#ref_tys
        }
    }

    /// quote!(OneChanged_0(<u16 as dipa::Diffable<'p, u16>::Diff))
    fn change_same_variant(&self, associated_type: DipaAssociatedType) -> Vec<TokenStream2> {
        let change_combinations =
            ChangedFieldIndices::all_changed_index_combinations(self.fields.len());

        let mut tokens = vec![];

        let associated_type_tokens = associated_type.to_token_stream();

        for change_combination in change_combinations {
            let variant =
                change_combination.variant_name_ident(&self.name.to_string(), self.name.span());

            let mut tys = Vec::with_capacity(change_combination.len());

            for field_idx in change_combination.iter() {
                let field = &self.fields[*field_idx as usize];
                let ty = &field.ty;

                let lifetime = match associated_type {
                    DipaAssociatedType::Diff => {
                        quote! {'p}
                    }
                    DipaAssociatedType::Patch => {
                        quote! {'static}
                    }
                };

                tys.push(Type::Verbatim(
                    quote! { <#ty as dipa::Diffable<#lifetime, #ty>>::#associated_type_tokens },
                ));
            }

            // example: (Option<u16>, Option<u32>)
            let diff_tys = quote! {( #(#tys),* )};

            let change_variant = quote! {
                #variant#diff_tys
            };
            tokens.push(change_variant);
        }

        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::assert_tokens_eq;

    /// Verify that we return the proper tokens for an enum variant that has no fields.
    #[test]
    fn no_fields_variant() {
        let variant = EnumVariant::no_field_variant();

        let diff_variants = variant.diff_type_variants(DipaAssociatedType::Diff);
        let expected = vec![quote! {TwoNoChange}, quote! {ChangedToVariantTwo}];

        assert_eq!(diff_variants.len(), expected.len());

        for (idx, tokens) in diff_variants.into_iter().enumerate() {
            assert_tokens_eq(&tokens, &expected[idx]);
        }
    }

    /// Verify that we return the proper tokens for an enum variant that has one field.
    #[test]
    fn one_field_variant() {
        let variant = EnumVariant::one_field_variant();

        let diff_variants = variant.diff_type_variants(DipaAssociatedType::Diff);
        let expected = vec![
            quote! {OneNoChange},
            quote! {
            ChangedToVariantOne(&'p u16)},
            quote! { OneChange_0(<u16 as dipa::Diffable<'p, u16>>::Diff) },
        ];

        assert_eq!(diff_variants.len(), expected.len());

        for (idx, tokens) in diff_variants.into_iter().enumerate() {
            assert_tokens_eq(&tokens, &expected[idx]);
        }
    }

    /// Verify that we return the proper tokens for an enum variant that has two fields.
    #[test]
    fn two_field_variant() {
        let variant = EnumVariant::two_fields_variant();

        let diff_variants = variant.diff_type_variants(DipaAssociatedType::Diff);
        let expected = vec![
            quote! {TwoNoChange},
            quote! {
            ChangedToVariantTwo(&'p u16, &'p u32)},
            quote! { TwoChange_0(<u16 as dipa::Diffable<'p, u16>>::Diff) },
            quote! { TwoChange_1(<u32 as dipa::Diffable<'p, u32>>::Diff) },
            quote! {
                TwoChange_0_1(
                    <u16 as dipa::Diffable<'p, u16>>::Diff,
                    <u32 as dipa::Diffable<'p, u32>>::Diff
                )
            },
        ];

        assert_eq!(diff_variants.len(), expected.len());

        for (idx, tokens) in diff_variants.into_iter().enumerate() {
            assert_tokens_eq(&tokens, &expected[idx]);
        }
    }
}
