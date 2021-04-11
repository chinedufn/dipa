#[macro_export]
macro_rules! map_impl {
    ($map_ty:ty, $module:ident, $($additional_key_bounds:tt)*) => {
        mod $module {
            use super::{MapDelta, MapDeltaOwned};
            use crate::{CreatedDelta, Diffable, Patchable};
            use serde::{Serialize, de::DeserializeOwned};
            use std::hash::Hash;

            type MapAssociatedDeltaOwned<'s, 'e, K, V> =
                <$map_ty as Diffable<'s, 'e, $map_ty>>::DeltaOwned;

            impl<'s, 'e, K, V> Diffable<'s, 'e, $map_ty> for $map_ty
            where
                K: 's + 'e + Eq + Hash $($additional_key_bounds)*,
                V: 'e + Diffable<'s, 'e, V>,
                <V as Diffable<'s,'e,V>>::Delta: Serialize,
                <V as Diffable<'s,'e,V>>::DeltaOwned: DeserializeOwned
            {
                type Delta = MapDelta<'s, 'e, K, V>;
                type DeltaOwned = MapDeltaOwned<'s, 'e, K, V>;

                fn create_delta_towards(
                    &'s self,
                    end_state: &'e $map_ty,
                ) -> CreatedDelta<Self::Delta> {
                    let mut did_change = false;

                    if end_state.len() == 0 && self.len() > 0 {
                        let delta = MapDelta::RemoveAll;
                        return CreatedDelta {
                            delta,
                            did_change: true
                        };
                    }

                    let mut delta = MapDelta::NoChange;

                    let mut fields_to_add = Vec::new();
                    let mut fields_to_remove = Vec::new();
                    let mut fields_changed = Vec::new();

                    for (key, start) in self.iter() {
                        match end_state.get(key) {
                            None => {
                                did_change = true;
                                fields_to_remove.push(key);
                            }
                            Some(end) => {
                                let CreatedDelta {delta, did_change: changed} = start.create_delta_towards(end);

                                if changed {
                                    did_change = true;
                                    fields_changed.push((key, delta));
                                }
                            }
                        }
                    }

                    for (key, val) in end_state.iter() {
                        if !self.contains_key(key) {
                            did_change = true;
                            fields_to_add.push((key, val));
                        }
                    }

                    let many_fields_to_add = fields_to_add.len() > 1;
                    let many_fields_to_remove = fields_to_remove.len() > 1;
                    let many_fields_changed = fields_changed.len() > 1;

                    let one_field_to_add = fields_to_add.len() == 1;
                    let one_field_to_remove = fields_to_remove.len() == 1;
                    let one_field_changed = fields_changed.len() == 1;

                    let encode_using_many_variant =
                        many_fields_to_add || many_fields_to_remove || many_fields_changed;

                    if encode_using_many_variant {
                        delta = MapDelta::ModifyMany {
                            added: fields_to_add,
                            removed: fields_to_remove,
                            changed: fields_changed,
                        }
                    } else if one_field_to_add {
                        delta = MapDelta::AddOneField(fields_to_add[0].0, fields_to_add[0].1);
                    } else if one_field_to_remove {
                        delta = MapDelta::RemoveOneField(fields_to_remove[0]);
                    } else if one_field_changed {
                        let change = fields_changed.remove(0);
                        delta = MapDelta::ChangeOneField(change.0, change.1);
                    }

                    CreatedDelta {
                        delta,
                        did_change
                    }
                }
            }

            impl<'s, 'e, K, V> Patchable<MapAssociatedDeltaOwned<'s, 'e, K, V>> for $map_ty
            where
                K: 's + 'e + Eq + Hash $($additional_key_bounds)*,
                V: 'e + Diffable<'s, 'e, V>,
                V: Patchable<<V as Diffable<'s, 'e, V>>::DeltaOwned>,
                <V as Diffable<'s,'e,V>>::Delta: Serialize,
                <V as Diffable<'s,'e,V>>::DeltaOwned: DeserializeOwned
            {
                fn apply_patch(&mut self, patch: MapAssociatedDeltaOwned<'s, 'e, K, V>) {
                    match patch {
                        MapDeltaOwned::NoChange => {}
                        MapDeltaOwned::RemoveAll => self.clear(),
                        MapDeltaOwned::AddOneField(k, v) => {
                            self.insert(k, v);
                        }
                        MapDeltaOwned::RemoveOneField(k) => {
                            self.remove(&k);
                        }
                        MapDeltaOwned::ChangeOneField(k, delta) => {
                            self.get_mut(&k).unwrap().apply_patch(delta);
                        }
                        MapDeltaOwned::ModifyMany {
                            added,
                            removed,
                            changed,
                        } => {
                            for add in added {
                                self.insert(add.0, add.1);
                            }

                            for remove in removed {
                                self.remove(&remove);
                            }

                            for change in changed {
                                self.get_mut(&change.0).unwrap().apply_patch(change.1);
                            }
                        }
                    }
                }
            }
        }
    };
}
