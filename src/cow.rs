use crate::{CreatedDelta, Diffable, Patchable};
use std::borrow::{Borrow, Cow};

impl<'s, 'e, 'a, T> Diffable<'s, 'e, Cow<'a, T>> for Cow<'a, T>
where
    T: ToOwned,
    T: ?Sized,
    T: Diffable<'s, 'e, T>,
{
    type Delta = <T as Diffable<'s, 'e, T>>::Delta;
    type DeltaOwned = <T as Diffable<'s, 'e, T>>::DeltaOwned;

    fn create_delta_towards(&'s self, end_state: &'e Cow<'a, T>) -> CreatedDelta<Self::Delta> {
        let inner_self: &T = self.borrow();

        inner_self.create_delta_towards(end_state.borrow())
    }
}

impl<'s, 'e, 'a, T> Patchable<<Self as Diffable<'s, 'e, Self>>::DeltaOwned> for Cow<'a, T>
where
    T: ToOwned,
    T: ?Sized,
    T: Diffable<'s, 'e, T>,
    <T as ToOwned>::Owned: Patchable<<T as Diffable<'s, 'e, T>>::DeltaOwned>,
{
    fn apply_patch(&mut self, patch: <T as Diffable<'s, 'e, T>>::DeltaOwned) {
        self.to_mut().apply_patch(patch)
    }
}

#[cfg(test)]
mod tests {
    use crate::sequence::SequenceModificationDelta;
    use crate::DipaImplTester;
    use std::borrow::Cow;

    /// Verify that we can diff and batch Cow's
    #[test]
    fn cow_impl() {
        let hello_static = "hello";
        let empty_static = "";

        DipaImplTester {
            label: Some("Cow no change Borrowed -> Owned"),
            start: &mut Cow::Borrowed(hello_static),
            end: &Cow::Owned(hello_static.to_string()),
            expected_delta: vec![],
            expected_serialized_patch_size: 1,
            expected_did_change: false,
        }
        .test();

        DipaImplTester {
            label: Some("Cow no change Owned -> Borrowed"),
            start: &mut Cow::Owned(hello_static.to_string()),
            end: &Cow::Borrowed(hello_static),
            expected_delta: vec![],
            expected_serialized_patch_size: 1,
            expected_did_change: false,
        }
        .test();

        DipaImplTester {
            label: Some("Cow no change Borrowed -> Borrowed"),
            start: &mut Cow::Borrowed(hello_static),
            end: &Cow::Borrowed(hello_static),
            expected_delta: vec![],
            expected_serialized_patch_size: 1,
            expected_did_change: false,
        }
        .test();

        DipaImplTester {
            label: Some("Cow no change Owned -> Owned"),
            start: &mut Cow::Owned::<'static, str>(hello_static.to_string()),
            end: &Cow::Owned(hello_static.to_string()),
            expected_delta: vec![],
            expected_serialized_patch_size: 1,
            expected_did_change: false,
        }
        .test();

        DipaImplTester {
            label: Some("Cow change Borrowed -> Owned"),
            start: &mut Cow::Borrowed(hello_static),
            end: &Cow::Owned(empty_static.to_string()),
            expected_delta: vec![SequenceModificationDelta::DeleteAll],
            expected_serialized_patch_size: 2,
            expected_did_change: true,
        }
        .test();

        DipaImplTester {
            label: Some("Cow change Borrowed -> Borrowed"),
            start: &mut Cow::Borrowed(hello_static),
            end: &Cow::Borrowed(empty_static),
            expected_delta: vec![SequenceModificationDelta::DeleteAll],
            expected_serialized_patch_size: 2,
            expected_did_change: true,
        }
        .test();

        DipaImplTester {
            label: Some("Cow change Owned -> Borrowed"),
            start: &mut Cow::Owned(hello_static.to_string()),
            end: &Cow::Borrowed(empty_static),
            expected_delta: vec![SequenceModificationDelta::DeleteAll],
            expected_serialized_patch_size: 2,
            expected_did_change: true,
        }
        .test();

        DipaImplTester {
            label: Some("Cow change Owned -> Owned"),
            start: &mut Cow::Owned::<'static, str>(hello_static.to_string()),
            end: &Cow::Owned(empty_static.to_string()),
            expected_delta: vec![SequenceModificationDelta::DeleteAll],
            expected_serialized_patch_size: 2,
            expected_did_change: true,
        }
        .test();
    }
}
