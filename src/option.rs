use crate::{Diffable, MacroOptimizationHints, Patchable};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::{Debug, Formatter};

impl<'p, T: Diffable<'p, T>> Diffable<'p, Option<T>> for Option<T>
where
    T: 'p,
    <T as Diffable<'p, T>>::Delta: Serialize,
    <T as Diffable<'p, T>>::DeltaOwned: DeserializeOwned,
{
    type Delta = OptionDelta<'p, T>;
    type DeltaOwned = OptionDeltaOwned<'p, T>;

    fn create_delta_towards(
        &self,
        end_state: &'p Option<T>,
    ) -> (Self::Delta, MacroOptimizationHints) {
        let diff = match (self, end_state) {
            (None, None) => OptionDelta::NoChange,
            (None, Some(new)) => OptionDelta::OuterChange(Some(new)),
            (Some(_), None) => OptionDelta::OuterChange(None),

            (Some(old), Some(new)) => {
                let diff = old.create_delta_towards(new);

                if diff.1.did_change {
                    OptionDelta::InnerChange(diff.0)
                } else {
                    OptionDelta::NoChange
                }
            }
        };

        let did_change = match &diff {
            OptionDelta::NoChange => false,
            _ => true,
        };

        let hint = MacroOptimizationHints { did_change };

        (diff, hint)
    }
}

impl<'p, T> Patchable<<Option<T> as Diffable<'p, Option<T>>>::DeltaOwned> for Option<T>
where
    T: 'p,
    T: Diffable<'p, T>,
    T: Patchable<<T as Diffable<'p, T>>::DeltaOwned>,
    <T as Diffable<'p, T>>::Delta: Serialize,
    <T as Diffable<'p, T>>::DeltaOwned: DeserializeOwned,
{
    fn apply_patch(&mut self, patch: <Option<T> as Diffable<'p, Option<T>>>::DeltaOwned) {
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
pub enum OptionDelta<'p, T: Diffable<'p, T>>
where
    <T as Diffable<'p, T>>::Delta: Serialize,
{
    NoChange,
    InnerChange(<T as Diffable<'p, T>>::Delta),
    OuterChange(Option<&'p T>),
}

#[derive(Deserialize)]
#[allow(missing_docs)]
pub enum OptionDeltaOwned<'p, T: Diffable<'p, T>>
where
    <T as Diffable<'p, T>>::DeltaOwned: DeserializeOwned,
{
    NoChange,
    InnerChange(<T as Diffable<'p, T>>::DeltaOwned),
    OuterChange(Option<T>),
}

#[cfg(any(test, feature = "impl-tester"))]
impl<'p, T: Diffable<'p, T> + Debug> Debug for OptionDelta<'p, T>
where
    <T as Diffable<'p, T>>::Delta: Serialize,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Didn't bother adding debug impl.")?;

        Ok(())
    }
}

#[cfg(any(test, feature = "impl-tester"))]
impl<'p, T> PartialEq for OptionDelta<'p, T>
where
    <T as Diffable<'p, T>>::Delta: PartialEq,
    T: Diffable<'p, T> + PartialEq,
    <T as Diffable<'p, T>>::Delta: Serialize,
{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (OptionDelta::NoChange, OptionDelta::NoChange) => true,
            (OptionDelta::InnerChange(inner1), OptionDelta::InnerChange(inner2)) => {
                inner1.eq(inner2)
            }
            (OptionDelta::OuterChange(outer1), OptionDelta::OuterChange(outer2)) => {
                outer1.eq(outer2)
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DiffPatchTestCase;

    /// Verify that we can diff/patch an Option<T>
    #[test]
    fn dipa_option_impl() {
        DiffPatchTestCase {
            label: Some("Option<T>::None no change"),
            start: Option::<()>::None,
            end: &None,
            expected_delta: OptionDelta::NoChange,
            expected_serialized_patch_size: 1,
            expected_macro_hints: MacroOptimizationHints { did_change: false },
        }
        .test();

        DiffPatchTestCase {
            label: Some("Option<T>::Some no change"),
            start: Some(1u32),
            end: &Some(1u32),
            expected_delta: OptionDelta::NoChange,
            expected_serialized_patch_size: 1,
            expected_macro_hints: MacroOptimizationHints { did_change: false },
        }
        .test();

        DiffPatchTestCase {
            label: Some("Option<T>::Some change"),
            start: Some(1u32),
            end: &Some(5u32),
            expected_delta: OptionDelta::InnerChange(Some(5)),
            expected_serialized_patch_size: 3,
            expected_macro_hints: MacroOptimizationHints { did_change: true },
        }
        .test();

        DiffPatchTestCase {
            label: Some("Option<T> Some -> None"),
            start: Some(1u32),
            end: &None,
            expected_delta: OptionDelta::OuterChange(None),
            expected_serialized_patch_size: 2,
            expected_macro_hints: MacroOptimizationHints { did_change: true },
        }
        .test();

        DiffPatchTestCase {
            label: Some("Option<T> None -> Some"),
            start: None,
            end: &Some(1u32),
            expected_delta: OptionDelta::OuterChange(Some(&1u32)),
            expected_serialized_patch_size: 3,
            expected_macro_hints: MacroOptimizationHints { did_change: true },
        }
        .test();
    }
}
