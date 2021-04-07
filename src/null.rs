use crate::{Diffable, MacroOptimizationHints, Patchable};

impl<'d> Diffable<'d, ()> for () {
    type Delta = ();
    type DeltaOwned = ();

    fn create_delta_towards(&self, _end_state: &'d ()) -> (Self::Delta, MacroOptimizationHints) {
        ((), MacroOptimizationHints { did_change: false })
    }
}

impl Patchable<()> for () {
    fn apply_patch(&mut self, _patch: ()) {}
}
