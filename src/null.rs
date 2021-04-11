use crate::{CreatedDelta, Diffable, Patchable};

impl<'s, 'e> Diffable<'s, 'e, ()> for () {
    type Delta = ();
    type DeltaOwned = ();

    fn create_delta_towards(&self, _end_state: &()) -> CreatedDelta<Self::Delta> {
        CreatedDelta {
            delta: (),
            did_change: false,
        }
    }
}

impl Patchable<()> for () {
    fn apply_patch(&mut self, _patch: ()) {}
}
