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

#[cfg(any(test, feature = "dipa-impl-tester"))]
mod dipa_impl_tester;
#[cfg(feature = "dipa-impl-tester")]
pub use self::dipa_impl_tester::DiffPatchTestCase;

/// The type returned by [Diffable.create_patch_towards].
pub type CreatePatchTowardsReturn<T> = (T, MacroOptimizationHints);

/// Allows a type to be diffed with another type.
pub trait Diffable<'p> {
    /// This will typically hold references to data from the structs that are being diffed.
    type Diff;

    /// Diff self with some target end state, generating a patch that would convert
    ///  self -> end_state.
    fn create_patch_towards(&self, end_state: &'p Self) -> CreatePatchTowardsReturn<Self::Diff>;
}

/// Allows a type to be patched with a `[Diffable::Patch]`.
pub trait Patchable {
    /// Usually the same as [Diffable::Diff], but with owned data instead of references.
    ///
    /// You'll typically serialize to a [Diffable::Diff] and then deserialize to Self::Patch,
    /// then apply the Self::Patch via [Self.apply_patch].
    type Patch;

    /// Apply a patch.
    fn apply_patch(&mut self, patch: Self::Patch);
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
