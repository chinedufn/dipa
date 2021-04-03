#[macro_use]
extern crate serde;

mod bool;
mod float;
mod integer;
mod vec;
#[macro_use]
mod number_impl;

// Exposes types meant for the dipa-derive macro.
#[doc(hidden)]
pub mod private;

#[cfg(any(test, feature = "impl-tester"))]
mod dipa_impl_tester;
#[cfg(feature = "impl-tester")]
pub use self::dipa_impl_tester::{patch_ty, DiffPatchTestCase};

/// The type returned by [Diffable.create_patch_towards].
pub type CreatePatchTowardsReturn<T> = (T, MacroOptimizationHints);

/// Allows a type to be diffed with another type.
pub trait Diffable<'p, Other> {
    /// This will typically hold references to data from the structs that are being diffed.
    ///
    /// TODO: Rename to Delta
    type Diff;

    /// This will typically be an owned version of [`Self::Diff`].
    ///
    /// TODO: Rename to DeltaOwned
    type Patch;

    /// Diff self with some target end state, generating a patch that would convert
    ///  self -> end_state.
    ///
    ///  TODO: Rename to create_delta_towards
    fn create_patch_towards(&self, end_state: &'p Other) -> CreatePatchTowardsReturn<Self::Diff>;
}

/// Allows a type to be patched.
///
/// A patch is usually the same as [Diffable::Diff], but with owned data instead of references.
///
/// You'll typically serialize to a [Diffable::Diff] and then deserialize the patch type,
/// then apply the patch via [Self.apply_patch].
pub trait Patchable<P> {
    /// Apply a patch.
    fn apply_patch(&mut self, patch: P);
}

/// Information about the diff that the derive macro can use in order to optimize the diff functions
/// that it generates.
#[derive(Debug, Copy, Clone, PartialEq)]
#[allow(missing_docs)]
pub struct MacroOptimizationHints {
    /// True if changed, false if same.
    pub did_change: bool,
}

#[cfg(test)]
mod test_utils;
