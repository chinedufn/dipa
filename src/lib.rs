#[macro_use]
extern crate serde;

mod sequence;

mod bool;
mod float;
mod integer;
mod map;
mod null;
mod option;
mod string;
mod tuple;

#[macro_use]
mod number_impl;

mod delta_n;

#[cfg(any(test, feature = "impl-tester"))]
mod dipa_impl_tester;
#[cfg(any(test, feature = "impl-tester"))]
pub use self::dipa_impl_tester::DipaImplTester;

/// The type returned by [Diffable.create_delta_towards].
pub type CreatePatchTowardsReturn<T> = (T, MacroOptimizationHints);

/// Allows a type to be diffed with another type.
pub trait Diffable<'s, 'e, Other: ?Sized> {
    /// This will typically hold references to data from the structs that are being diffed.
    type Delta;

    /// This will typically be an owned version of [`Self::Delta`].
    type DeltaOwned;

    /// Diff self with some target end state, generating a patch that would convert
    ///  self -> end_state.
    fn create_delta_towards(
        &'s self,
        end_state: &'e Other,
    ) -> CreatePatchTowardsReturn<Self::Delta>;
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
///
/// TODO: This is actually used outside of macros. For example, the HashMap implementation uses
///  this. So we should just return `did_change` as its own separate field. So have the
///  `create_delta_towards` method return `-> SomeStruct { delta: Delta, did_change: bool }`
#[derive(Debug, Copy, Clone, PartialEq)]
#[allow(missing_docs)]
pub struct MacroOptimizationHints {
    /// True if changed, false if same.
    pub did_change: bool,
}

#[cfg(test)]
mod test_utils;
