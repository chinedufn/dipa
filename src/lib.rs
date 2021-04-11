//! dipa makes it easy to efficiently delta encode large Rust data structures.
//!
//! # Documentation
//!
//! The Dipa Book introduces you to dipa and teaches you how to use it.
//!
//! It is available online at https://chinedufn.github.io/dipa/
//!
//! You can also view the book offline:
//!
//! ```sh,no_run,ignore
//! # Do this once while online
//! git clone git@github.com:chinedufn/dipa.git && cd dipa
//! cargo install mdbook
//!
//! # This works offline
//! ./bin/serve-book.sh
//! ```

#![deny(missing_docs)]

#[macro_use]
extern crate serde;

#[cfg(feature = "derive")]
pub use dipa_derive::DiffPatch;

mod sequence;

mod bool;
mod cow;
mod float;
mod integer;
mod map;
mod null;
mod option;
mod set;
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
///
/// FIXME: Rename to `CreatedDelta {delta: T, did_change: bool}` and delete MacroOptimizationHints.
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

/// Modifies a type using n a patch.
///
/// You would typically create this patch using [`Diffable.create_delta_towards`].
///
/// You'll typically serialize to a [`Diffable::Delta`], send it over a network and then deserialize
/// to [`Diffable::DeltaOwned`]. Then you would use that owned delta as the patch to apply via
/// [`Patchable.apply_patch`].
///
/// FIXME: This should return a result since it is possible to, for example, accidentally apply a
///  a patch to the wrong data structure do to a logical bug.
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
