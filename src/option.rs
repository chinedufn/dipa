use crate::{CreatedDelta, Diffable, Patchable};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::{Debug, Formatter};

impl<'s, 'e, T: Diffable<'s, 'e, T>> Diffable<'s, 'e, Option<T>> for Option<T>
where
    T: 'e,
    <T as Diffable<'s, 'e, T>>::Delta: Serialize,
    <T as Diffable<'s, 'e, T>>::DeltaOwned: DeserializeOwned,
{
    type Delta = OptionDelta<'s, 'e, T>;
    type DeltaOwned = OptionDeltaOwned<'s, 'e, T>;

    fn create_delta_towards(&'s self, end_state: &'e Option<T>) -> CreatedDelta<Self::Delta> {
        let diff = match (self, end_state) {
            (None, None) => OptionDelta::NoChange,
            (None, Some(new)) => OptionDelta::OuterChange(Some(new)),
            (Some(_), None) => OptionDelta::OuterChange(None),

            (Some(old), Some(new)) => {
                let diff = old.create_delta_towards(new);

                if diff.did_change {
                    OptionDelta::InnerChange(diff.delta)
                } else {
                    OptionDelta::NoChange
                }
            }
        };

        let did_change = match &diff {
            OptionDelta::NoChange => false,
            _ => true,
        };

        CreatedDelta {
            delta: diff,
            did_change,
        }
    }
}

impl<'s, 'e, T> Patchable<<Option<T> as Diffable<'s, 'e, Option<T>>>::DeltaOwned> for Option<T>
where
    T: 'e,
    T: Diffable<'s, 'e, T>,
    T: Patchable<<T as Diffable<'s, 'e, T>>::DeltaOwned>,
    <T as Diffable<'s, 'e, T>>::Delta: Serialize,
    <T as Diffable<'s, 'e, T>>::DeltaOwned: DeserializeOwned,
{
    fn apply_patch(&mut self, patch: <Option<T> as Diffable<'s, 'e, Option<T>>>::DeltaOwned) {
        match patch {
            OptionDeltaOwned::NoChange => {}
            OptionDeltaOwned::InnerChange(delta) => match self {
                Some(inner) => inner.apply_patch(delta),
                _ => panic!("No inner field to change"),
            },
            OptionDeltaOwned::OuterChange(outer) => {
                *self = outer;
            }
        }
    }
}

#[derive(Serialize)]
#[allow(missing_docs)]
pub enum OptionDelta<'s, 'e, T: Diffable<'s, 'e, T>>
where
    <T as Diffable<'s, 'e, T>>::Delta: Serialize,
{
    NoChange,
    InnerChange(<T as Diffable<'s, 'e, T>>::Delta),
    OuterChange(Option<&'e T>),
}

#[derive(Deserialize)]
#[allow(missing_docs)]
pub enum OptionDeltaOwned<'s, 'e, T: Diffable<'s, 'e, T>>
where
    <T as Diffable<'s, 'e, T>>::DeltaOwned: DeserializeOwned,
{
    NoChange,
    InnerChange(<T as Diffable<'s, 'e, T>>::DeltaOwned),
    OuterChange(Option<T>),
}

// Used by DipaImplTester
impl<'s, 'e, T: Diffable<'s, 'e, T>> Debug for OptionDelta<'s, 'e, T>
where
    T: Debug,
    <T as Diffable<'s, 'e, T>>::Delta: Serialize,
    <T as Diffable<'s, 'e, T>>::Delta: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OptionDelta::NoChange => {
                f.write_str("NoChange")?;
            }
            OptionDelta::InnerChange(delta) => {
                f.debug_tuple("InnerChange").field(delta).finish()?;
            }
            OptionDelta::OuterChange(outer) => {
                f.debug_tuple("Outer").field(outer).finish()?;
            }
        };

        Ok(())
    }
}

// Used by DipaImplTester
impl<'s, 'e, T: Diffable<'s, 'e, T>> PartialEq for OptionDelta<'s, 'e, T>
where
    T: PartialEq,
    <T as Diffable<'s, 'e, T>>::Delta: Serialize,
    <T as Diffable<'s, 'e, T>>::Delta: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (OptionDelta::NoChange, OptionDelta::NoChange) => true,
            (OptionDelta::InnerChange(left), OptionDelta::InnerChange(right)) => left.eq(right),
            (OptionDelta::OuterChange(left), OptionDelta::OuterChange(right)) => left.eq(right),
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DipaImplTester;

    /// Verify that we can diff/patch an Option<T>
    #[test]
    fn dipa_option_impl() {
        DipaImplTester {
            label: Some("Option<T>::None no change"),
            start: &mut Option::<()>::None,
            end: &None,
            expected_delta: OptionDelta::NoChange,
            expected_serialized_patch_size: 1,
            expected_did_change: false,
        }
        .test();

        DipaImplTester {
            label: Some("Option<T>::Some no change"),
            start: &mut Some(1u32),
            end: &Some(1u32),
            expected_delta: OptionDelta::NoChange,
            expected_serialized_patch_size: 1,
            expected_did_change: false,
        }
        .test();

        DipaImplTester {
            label: Some("Option<T>::Some change"),
            start: &mut Some(1u32),
            end: &Some(5u32),
            expected_delta: OptionDelta::InnerChange(Some(5)),
            expected_serialized_patch_size: 3,
            expected_did_change: true,
        }
        .test();

        DipaImplTester {
            label: Some("Option<T> Some -> None"),
            start: &mut Some(1u32),
            end: &None,
            expected_delta: OptionDelta::OuterChange(None),
            expected_serialized_patch_size: 2,
            expected_did_change: true,
        }
        .test();

        DipaImplTester {
            label: Some("Option<T> None -> Some"),
            start: &mut None,
            end: &Some(1u32),
            expected_delta: OptionDelta::OuterChange(Some(&1u32)),
            expected_serialized_patch_size: 3,
            expected_did_change: true,
        }
        .test();
    }
}
