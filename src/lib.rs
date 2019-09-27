#[cfg(test)]
#[macro_use]
#[cfg(test)]
extern crate serde;

mod bool;
mod float;
mod integer;

pub trait DiffPatch {
    type Patch;

    /// Given some end state, generate a Patch that would convert self -> end_state
    fn create_patch_towards(&self, end_state: &Self) -> Self::Patch;

    /// Apply a patch
    fn apply_patch(&mut self, patch: Self::Patch);
}

#[cfg(test)]
mod test_utils;
