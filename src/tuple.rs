use crate::delta_n::{Delta2, DeltaOwned2};
use crate::{Diffable, MacroOptimizationHints, Patchable};

// TODO: 3-tuple and 4-tuple implementations. Similar to 2-tuple just with more fields.
//  We already generate code like this in dipa-derive so we can probably look to re-use that
//  in dipa's build script in order to generate the tuple-2 .. tuple-n implementations that we
//  need.

impl<'s, 'e, A: Diffable<'s, 'e, A>, B: Diffable<'s, 'e, B>> Diffable<'s, 'e, (A, B)> for (A, B) {
    type Delta = Delta2<<A as Diffable<'s, 'e, A>>::Delta, <B as Diffable<'s, 'e, B>>::Delta>;
    type DeltaOwned =
        DeltaOwned2<<A as Diffable<'s, 'e, A>>::DeltaOwned, <B as Diffable<'s, 'e, B>>::DeltaOwned>;

    fn create_delta_towards(
        &'s self,
        end_state: &'e (A, B),
    ) -> (Self::Delta, MacroOptimizationHints) {
        let diff0 = self.0.create_delta_towards(&end_state.0);
        let diff1 = self.1.create_delta_towards(&end_state.1);

        let did_change = diff0.1.did_change || diff1.1.did_change;
        let hints = MacroOptimizationHints { did_change };

        let diff = match (diff0.1.did_change, diff1.1.did_change) {
            (false, false) => Delta2::NoChange,
            (true, false) => Delta2::Change_0(diff0.0),
            (false, true) => Delta2::Change_1(diff1.0),
            (true, true) => Delta2::Change_0_1(diff0.0, diff1.0),
        };

        (diff, hints)
    }
}

impl<
        's,
        'e,
        A: Diffable<'s, 'e, A> + Patchable<<A as Diffable<'s, 'e, A>>::DeltaOwned>,
        B: Diffable<'s, 'e, B> + Patchable<<B as Diffable<'s, 'e, B>>::DeltaOwned>,
    >
    Patchable<
        DeltaOwned2<<A as Diffable<'s, 'e, A>>::DeltaOwned, <B as Diffable<'s, 'e, B>>::DeltaOwned>,
    > for (A, B)
{
    fn apply_patch(
        &mut self,
        patch: DeltaOwned2<
            <A as Diffable<'s, 'e, A>>::DeltaOwned,
            <B as Diffable<'s, 'e, B>>::DeltaOwned,
        >,
    ) {
        match patch {
            DeltaOwned2::NoChange => {}
            DeltaOwned2::Change_0(patch0) => {
                self.0.apply_patch(patch0);
            }
            DeltaOwned2::Change_1(patch1) => {
                self.1.apply_patch(patch1);
            }
            DeltaOwned2::Change_0_1(patch0, patch1) => {
                self.0.apply_patch(patch0);
                self.1.apply_patch(patch1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DipaImplTester;

    /// Verify that we can diff and patch a 2-tuple
    #[test]
    fn two_tuple() {
        DipaImplTester {
            label: Some("2 tuple no change"),
            start: &mut (1u16, 2u32),
            end: &(1u16, 2u32),
            expected_delta: Delta2::NoChange,
            expected_serialized_patch_size: 1,
            expected_macro_hints: MacroOptimizationHints { did_change: false },
        }
        .test();

        DipaImplTester {
            label: Some("2 tuple Change_1"),
            start: &mut (1u16, 2u32),
            end: &(5u16, 2u32),
            expected_delta: Delta2::Change_0(Some(5)),
            expected_serialized_patch_size: 3,
            expected_macro_hints: MacroOptimizationHints { did_change: true },
        }
        .test();

        DipaImplTester {
            label: Some("2 tuple Change_1"),
            start: &mut (1u16, 2u32),
            end: &(1u16, 6u32),
            expected_delta: Delta2::Change_1(Some(6)),
            expected_serialized_patch_size: 3,
            expected_macro_hints: MacroOptimizationHints { did_change: true },
        }
        .test();

        DipaImplTester {
            label: Some("2 tuple Change_0_1"),
            start: &mut (1u16, 2u32),
            end: &(5u16, 6u32),
            expected_delta: Delta2::Change_0_1(Some(5), Some(6)),
            expected_serialized_patch_size: 5,
            expected_macro_hints: MacroOptimizationHints { did_change: true },
        }
        .test();
    }
}
