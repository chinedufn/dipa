use serde::Serialize;
use std::fmt::{Debug, Formatter};

#[macro_use]
mod set_impl_macro;

set_impl!(std::collections::HashSet<K>, hash_map_impl,);
set_impl!(std::collections::BTreeSet<K>, btree_map_impl, + Ord);

#[derive(Serialize)]
/// The delta between two sets.
pub enum SetDelta<'s, 'e, K> {
    /// Nothing has changed
    NoChange,
    /// Remove all elements from the HashMap
    RemoveAll,
    /// Add one entry to the set
    AddOneField(&'e K),
    /// Remove one entry from the set
    RemoveOneField(&'s K),
    /// Modify multiple entries
    ModifyMany {
        added: Vec<&'e K>,
        removed: Vec<&'s K>,
    },
}

#[derive(Deserialize)]
#[allow(missing_docs)]
/// The delta between two sets.
pub enum SetDeltaOwned<K> {
    /// Nothing has changed
    NoChange,
    /// Remove all elements from the set
    RemoveAll,
    /// Add one entry to the set
    AddOneField(K),
    /// Remove one entry from the set
    RemoveOneField(K),
    /// Modify multiple entries
    ModifyMany { added: Vec<K>, removed: Vec<K> },
}

// Used by DipaImplTester
impl<'s, 'e, K> Debug for SetDelta<'s, 'e, K>
where
    K: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SetDelta::NoChange => {
                f.write_str("NoChange")?;
            }
            SetDelta::RemoveAll => {
                f.write_str("RemoveAll")?;
            }
            SetDelta::AddOneField(k) => {
                f.debug_tuple("AddOneField").field(k).finish()?;
            }
            SetDelta::RemoveOneField(k) => {
                f.debug_tuple("RemoveOneField").field(k).finish()?;
            }
            SetDelta::ModifyMany { added, removed } => {
                f.debug_struct("ModifyMany")
                    .field("added", added)
                    .field("removed", removed)
                    .finish()?;
            }
        };

        Ok(())
    }
}

// Used by DipaImplTester
impl<'s, 'e, K> PartialEq for SetDelta<'s, 'e, K>
where
    K: PartialEq,
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
                },
                Self::ModifyMany {
                    added: right_added,
                    removed: right_removed,
                },
            ) => left_added == right_added && left_removed == right_removed,
            (Self::ModifyMany { .. }, _) => false,

            (Self::AddOneField(left_k), Self::AddOneField(right_k)) => left_k == right_k,
            (Self::AddOneField(..), _) => false,

            (Self::RemoveOneField(left_k), Self::RemoveOneField(right_k)) => left_k == right_k,
            (Self::RemoveOneField(_), _) => false,

            (Self::RemoveAll, Self::RemoveAll) => true,
            (Self::RemoveAll, _) => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DipaImplTester;
    use std::collections::{BTreeSet, HashSet};

    /// Verify that we properly handle an unchanged empty HashMap
    #[test]
    fn unchanged_empty_hashmap() {
        DipaImplTester {
            label: None,
            start: &mut HashSet::<()>::new(),
            end: &HashSet::new(),
            expected_delta: SetDelta::NoChange,
            expected_serialized_patch_size: 1,
            expected_did_change: false,
        }
        .test();
    }

    /// Verify that we properly handle an unchanged HashMap that has fields.
    #[test]
    fn unchanged_hashmap() {
        DipaImplTester {
            label: None,
            start: &mut vec![1u32].into_iter().collect::<HashSet<_>>(),
            end: &vec![1].into_iter().collect(),
            expected_delta: SetDelta::NoChange,
            expected_serialized_patch_size: 1,
            expected_did_change: false,
        }
        .test();
    }

    /// Verify that we can remove a field from the original HashMap
    #[test]
    fn all_fields_removed() {
        DipaImplTester {
            label: None,
            start: &mut vec![1u32, 22].into_iter().collect(),
            end: &HashSet::new(),
            expected_delta: SetDelta::RemoveAll,
            expected_serialized_patch_size: 1,
            expected_did_change: true,
        }
        .test();
    }

    /// Verify that we can add a field to the original HashMap
    #[test]
    fn one_field_added() {
        DipaImplTester {
            label: None,
            start: &mut HashSet::new(),
            end: &vec![1u32].into_iter().collect(),
            expected_delta: SetDelta::AddOneField(&1),
            expected_serialized_patch_size: 2,
            expected_did_change: true,
        }
        .test();
    }

    /// Verify that we can remove a field to the original HashMap
    #[test]
    fn one_field_removed() {
        DipaImplTester {
            label: None,
            start: &mut vec![1u32, 3].into_iter().collect::<HashSet<_>>(),
            end: &vec![1].into_iter().collect(),
            expected_delta: SetDelta::RemoveOneField(&3),
            expected_serialized_patch_size: 2,
            expected_did_change: true,
        }
        .test();
    }

    /// Verify that we can add multiple fields to the map.
    #[test]
    fn many_fields_added() {
        DipaImplTester {
            label: None,
            start: &mut BTreeSet::new(),
            end: &vec![1u32, 3].into_iter().collect(),
            expected_delta: SetDelta::ModifyMany {
                added: vec![&1, &3],
                removed: vec![],
            },
            expected_serialized_patch_size: 5,
            expected_did_change: true,
        }
        .test();
    }

    /// Verify that we can remove multiple entries from the map.
    #[test]
    fn many_fields_removed() {
        DipaImplTester {
            label: None,
            start: &mut vec![1u32, 3, 5].into_iter().collect::<BTreeSet<_>>(),
            end: &vec![1].into_iter().collect(),
            expected_delta: SetDelta::ModifyMany {
                added: vec![],
                removed: vec![&3, &5],
            },
            expected_serialized_patch_size: 5,
            expected_did_change: true,
        }
        .test();
    }
}
