#[cfg(test)]
#[macro_use]
#[cfg(test)]
extern crate serde;

mod bool;
mod float;
mod integer;
mod vec;

pub trait DiffPatch<'p> {
    /// This will typically hold references to data from the structs that are being diffed.
    type Diff;

    /// Should be the same as [Self::Diff], but with owned data instead of references.
    ///
    /// You'll typically serialize to a [Self::Diff] and then deserialize to Self::OwnedDiff, then
    /// apply the Self::OwnedDiff via [Self.apply_patch].
    type OwnedDiff;

    /// Given some end state, generate a Patch that would convert self -> end_state
    fn create_patch_towards(&self, end_state: &'p Self) -> Self::Diff;

    /// Apply a patch
    fn apply_patch(&mut self, patch: Self::OwnedDiff);
}

#[cfg(test)]
mod test_utils;
