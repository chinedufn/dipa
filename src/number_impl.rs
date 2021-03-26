/// Used for integer and float types excluding `u8` and `i8` since those two do not use Option
/// wrappers.
#[macro_export]
macro_rules! number_diff_impl_option_wrapped {
    ($num_ty:ty) => {
        impl<'p> crate::Diffable<'p> for $num_ty {
            type Diff = Option<$num_ty>;

            fn create_patch_towards(
                &self,
                end_state: &Self,
            ) -> CreatePatchTowardsReturn<Self::Diff> {
                let hint = MacroOptimizationHints {
                    did_change: self != end_state,
                };

                (
                    match *self == *end_state {
                        true => None,
                        false => Some(*end_state),
                    },
                    hint,
                )
            }
        }
    };
}

#[macro_export]
macro_rules! number_patch_impl_option_wrapped {
    ($num_ty:ty) => {
        impl crate::Patchable for $num_ty {
            type Patch = Option<$num_ty>;

            fn apply_patch(&mut self, patch: Self::Patch) {
                if let Some(patch) = patch {
                    *self = patch;
                }
            }
        }
    };
}

/// Used for u8 and i8 since their diffs are not wrapped in Option.
/// TODO: Rename to single_byte_impl or something, since we use this for bools
#[macro_export]
macro_rules! number_diff_impl_u8_or_i8 {
    ($num_ty:ty) => {
        impl<'p> crate::Diffable<'p> for $num_ty {
            type Diff = $num_ty;

            fn create_patch_towards(
                &self,
                end_state: &Self,
            ) -> CreatePatchTowardsReturn<Self::Diff> {
                let did_change = *self != *end_state;
                let hint = MacroOptimizationHints { did_change };

                (*end_state, hint)
            }
        }
    };
}

#[macro_export]
macro_rules! number_patch_impl_u8_or_i8 {
    ($num_ty:ty) => {
        impl crate::Patchable for $num_ty {
            type Patch = $num_ty;

            fn apply_patch(&mut self, patch: Self::Patch) {
                *self = patch;
            }
        }
    };
}
