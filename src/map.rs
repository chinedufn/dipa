use crate::Diffable;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::{Debug, Formatter};

#[macro_use]
mod map_impl_macro;

map_impl!(std::collections::HashMap<K,V>, hash_map_impl, );
map_impl!(std::collections::BTreeMap<K,V>, btree_map_impl, + Ord);

#[derive(Serialize)]
/// The delta between two maps.
pub enum MapDelta<'s, 'e, K, V: Diffable<'s, 'e, V>>
where
    <V as Diffable<'s, 'e, V>>::Delta: Serialize,
{
    /// Nothing has changed
    NoChange,
    /// Remove all elements from the HashMap
    RemoveAll,
    /// Add one field to the HashMap
    AddOneField(&'e K, &'e V),
    /// Remove one field to the HashMap
    RemoveOneField(&'s K),
    /// Change one field to the HashMap
    ChangeOneField(&'s K, <V as Diffable<'s, 'e, V>>::Delta),
    /// Modify multiple entries
    ModifyMany {
        added: Vec<(&'e K, &'e V)>,
        removed: Vec<&'s K>,
        changed: Vec<(&'s K, <V as Diffable<'s, 'e, V>>::Delta)>,
    },
}

#[derive(Deserialize)]
#[allow(missing_docs)]
/// The delta between two maps.
pub enum MapDeltaOwned<'s, 'e, K, V: Diffable<'s, 'e, V>>
where
    <V as Diffable<'s, 'e, V>>::DeltaOwned: DeserializeOwned,
{
    /// Nothing has changed
    NoChange,
    /// Remove all elements from the HashMap
    RemoveAll,
    /// Add one field to the HashMap
    AddOneField(K, V),
    /// Remove one field to the HashMap
    RemoveOneField(K),
    /// Change one field to the HashMap
    ChangeOneField(K, <V as Diffable<'s, 'e, V>>::DeltaOwned),
    /// Modify multiple entries
    ModifyMany {
        added: Vec<(K, V)>,
        removed: Vec<K>,
        changed: Vec<(K, <V as Diffable<'s, 'e, V>>::DeltaOwned)>,
    },
}

// Used by DipaImplTester
impl<'s, 'e, K, V> Debug for MapDelta<'s, 'e, K, V>
where
    V: Diffable<'s, 'e, V>,
    <V as Diffable<'s, 'e, V>>::Delta: Debug,
    <V as Diffable<'s, 'e, V>>::Delta: Serialize,
    K: Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MapDelta::NoChange => {
                f.write_str("NoChange")?;
            }
            MapDelta::RemoveAll => {
                f.write_str("RemoveAll")?;
            }
            MapDelta::AddOneField(k, v) => {
                f.debug_tuple("AddOneField").field(k).field(v).finish()?;
            }
            MapDelta::RemoveOneField(k) => {
                f.debug_tuple("RemoveOneField").field(k).finish()?;
            }
            MapDelta::ChangeOneField(k, diff) => {
                f.debug_tuple("ChangeOneField")
                    .field(k)
                    .field(diff)
                    .finish()?;
            }
            MapDelta::ModifyMany {
                added,
                removed,
                changed,
            } => {
                f.debug_struct("ModifyMany")
                    .field("added", added)
                    .field("removed", removed)
                    .field("changed", changed)
                    .finish()?;
            }
        };

        Ok(())
    }
}

// Used by DipaImplTester
impl<'s, 'e, K, V> PartialEq for MapDelta<'s, 'e, K, V>
where
    V: Diffable<'s, 'e, V>,
    <V as Diffable<'s, 'e, V>>::Delta: PartialEq,
    <V as Diffable<'s, 'e, V>>::Delta: Serialize,
    K: PartialEq,
    V: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        // Matched exhaustive so that we remember to add new variants.
        match (self, other) {
            (Self::NoChange, Self::NoChange) => true,
            (Self::NoChange, _) => false,

            (
                Self::ModifyMany {
                    added: left_added,
                    removed: left_removed,
                    changed: left_changed,
                },
                Self::ModifyMany {
                    added: right_added,
                    removed: right_removed,
                    changed: right_changed,
                },
            ) => {
                left_added == right_added
                    && left_removed == right_removed
                    && left_changed == right_changed
            }
            (Self::ModifyMany { .. }, _) => false,

            (
                Self::ChangeOneField(left_k, left_diff),
                Self::ChangeOneField(right_k, right_diff),
            ) => left_k == right_k && left_diff == right_diff,
            (Self::ChangeOneField(..), _) => false,

            (Self::RemoveOneField(left_k), Self::RemoveOneField(right_k)) => left_k == right_k,
            (Self::RemoveOneField(_), _) => false,

            (Self::AddOneField(left_k, left_v), Self::AddOneField(right_k, right_v)) => {
                left_k == right_k && left_v == right_v
            }
            (Self::AddOneField(..), _) => false,

            (Self::RemoveAll, Self::RemoveAll) => true,
            (Self::RemoveAll, _) => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DipaImplTester, MacroOptimizationHints};
    use std::collections::{BTreeMap, HashMap};

    /// Verify that we properly handle an unchanged empty HashMap
    #[test]
    fn unchanged_empty_hashmap() {
        DipaImplTester {
            label: None,
            start: &mut HashMap::<(), ()>::new(),
            end: &HashMap::new(),
            expected_delta: MapDelta::NoChange,
            expected_serialized_patch_size: 1,
            expected_macro_hints: MacroOptimizationHints { did_change: false },
        }
        .test();
    }

    /// Verify that we properly handle an unchanged HashMap that has fields.
    #[test]
    fn unchanged_hashmap() {
        DipaImplTester {
            label: None,
            start: &mut vec![(1u32, 2u64)].into_iter().collect::<HashMap<_, _>>(),
            end: &vec![(1, 2)].into_iter().collect(),
            expected_delta: MapDelta::NoChange,
            expected_serialized_patch_size: 1,
            expected_macro_hints: MacroOptimizationHints { did_change: false },
        }
        .test();
    }

    /// Verify that we can remove a field from the original HashMap
    #[test]
    fn all_fields_removed() {
        DipaImplTester {
            label: None,
            start: &mut vec![(1u32, 2u64)].into_iter().collect(),
            end: &HashMap::new(),
            expected_delta: MapDelta::RemoveAll,
            expected_serialized_patch_size: 1,
            expected_macro_hints: MacroOptimizationHints { did_change: true },
        }
        .test();
    }

    /// Verify that we can add a field to the original HashMap
    #[test]
    fn one_field_added() {
        DipaImplTester {
            label: None,
            start: &mut HashMap::new(),
            end: &vec![(1u32, 2u64)].into_iter().collect(),
            expected_delta: MapDelta::AddOneField(&1, &2),
            expected_serialized_patch_size: 3,
            expected_macro_hints: MacroOptimizationHints { did_change: true },
        }
        .test();
    }

    /// Verify that we can remove a field to the original HashMap
    #[test]
    fn one_field_removed() {
        DipaImplTester {
            label: None,
            start: &mut vec![(1u32, 2u64), (3, 4)]
                .into_iter()
                .collect::<HashMap<_, _>>(),
            end: &vec![(1, 2)].into_iter().collect(),
            expected_delta: MapDelta::RemoveOneField(&3),
            expected_serialized_patch_size: 2,
            expected_macro_hints: MacroOptimizationHints { did_change: true },
        }
        .test();
    }

    /// Verify that we can remove a field to the original HashMap
    #[test]
    fn one_field_changed() {
        DipaImplTester {
            label: None,
            start: &mut vec![(1u32, 2u64), (3, 4)]
                .into_iter()
                .collect::<HashMap<_, _>>(),
            end: &vec![(1, 2), (3, 9)].into_iter().collect(),
            expected_delta: MapDelta::ChangeOneField(&3, Some(9)),
            expected_serialized_patch_size: 4,
            expected_macro_hints: MacroOptimizationHints { did_change: true },
        }
        .test();
    }

    /// Verify that we can add multiple fields to the map.
    #[test]
    fn many_fields_added() {
        DipaImplTester {
            label: None,
            start: &mut BTreeMap::new(),
            end: &vec![(1u32, 2u64), (3, 4)].into_iter().collect(),
            expected_delta: MapDelta::ModifyMany {
                added: vec![(&1, &2), (&3, &4)],
                removed: vec![],
                changed: vec![],
            },
            expected_serialized_patch_size: 8,
            expected_macro_hints: MacroOptimizationHints { did_change: true },
        }
        .test();
    }

    /// Verify that we can remove multiple entries from the map.
    #[test]
    fn many_fields_removed() {
        DipaImplTester {
            label: None,
            start: &mut vec![(1u32, 2u64), (3, 4), (5, 6)]
                .into_iter()
                .collect::<BTreeMap<_, _>>(),
            end: &vec![(1, 2)].into_iter().collect(),
            expected_delta: MapDelta::ModifyMany {
                added: vec![],
                removed: vec![&3, &5],
                changed: vec![],
            },
            expected_serialized_patch_size: 6,
            expected_macro_hints: MacroOptimizationHints { did_change: true },
        }
        .test();
    }

    /// Verify that we can change multiple entries within the map.
    #[test]
    fn many_fields_changed() {
        DipaImplTester {
            label: None,
            start: &mut vec![(1u32, 2u64), (3, 4)]
                .into_iter()
                .collect::<BTreeMap<_, _>>(),
            end: &vec![(1, 15), (3, 16)].into_iter().collect(),
            expected_delta: MapDelta::ModifyMany {
                added: vec![],
                removed: vec![],
                changed: vec![(&1, Some(15)), (&3, Some(16))],
            },
            expected_serialized_patch_size: 10,
            expected_macro_hints: MacroOptimizationHints { did_change: true },
        }
        .test();
    }
}
