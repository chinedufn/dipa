#[macro_export]
macro_rules! set_impl {
    ($map_ty:ty, $module:ident, $($additional_key_bounds:tt)*) => {
        mod $module {
            use super::{SetDelta, SetDeltaOwned};
            use crate::{CreatedDelta, Diffable, Patchable};
            use std::hash::Hash;

            type SetAssociatedDeltaOwned<'s, 'e, K> =
                <$map_ty as Diffable<'s, 'e, $map_ty>>::DeltaOwned;

            impl<'s, 'e, K> Diffable<'s, 'e, $map_ty> for $map_ty
            where
                K: 's + 'e + Eq + Hash $($additional_key_bounds)*,
            {
                type Delta = SetDelta<'s, 'e, K>;
                type DeltaOwned = SetDeltaOwned<K>;

                fn create_delta_towards(
                    &'s self,
                    end_state: &'e $map_ty,
                ) -> CreatedDelta<Self::Delta> {
                    let mut did_change = false;

                    if end_state.len() == 0 && self.len() > 0 {
                        let delta = SetDelta::RemoveAll;
                        return CreatedDelta {
                            delta,
                            did_change: true,
                        };
                    }

                    let mut delta = SetDelta::NoChange;

                    let mut fields_to_add = Vec::new();
                    let mut fields_to_remove = Vec::new();

                    for key in self.iter() {
                        if !end_state.contains(key) {
                            did_change = true;
                            fields_to_remove.push(key);
                        }
                    }

                    for key in end_state.iter() {
                        if !self.contains(key) {
                            did_change = true;
                            fields_to_add.push(key);
                        }
                    }

                    let many_fields_to_add = fields_to_add.len() > 1;
                    let many_fields_to_remove = fields_to_remove.len() > 1;

                    let one_field_to_add = fields_to_add.len() == 1;
                    let one_field_to_remove = fields_to_remove.len() == 1;

                    let encode_using_many_variant =
                        many_fields_to_add || many_fields_to_remove ;

                    if encode_using_many_variant {
                        delta = SetDelta::ModifyMany {
                            added: fields_to_add,
                            removed: fields_to_remove,
                        }
                    } else if one_field_to_add {
                        delta = SetDelta::AddOneField(fields_to_add[0]);
                    } else if one_field_to_remove {
                        delta = SetDelta::RemoveOneField(fields_to_remove[0]);
                    }

                    CreatedDelta {
                        delta,
                        did_change,
                    }
                }
            }

            impl<'s, 'e, K> Patchable<SetAssociatedDeltaOwned<'s, 'e, K>> for $map_ty
            where
                K: 's + 'e + Eq + Hash $($additional_key_bounds)*,
            {
                fn apply_patch(&mut self, patch: SetAssociatedDeltaOwned<'s, 'e, K>) {
                    match patch {
                        SetDeltaOwned::NoChange => {}
                        SetDeltaOwned::RemoveAll => self.clear(),
                        SetDeltaOwned::AddOneField(k) => {
                            self.insert(k);
                        }
                        SetDeltaOwned::RemoveOneField(k) => {
                            self.remove(&k);
                        }
                        SetDeltaOwned::ModifyMany {
                            added,
                            removed,
                        } => {
                            for add in added {
                                self.insert(add);
                            }

                            for remove in removed {
                                self.remove(&remove);
                            }
                        }
                    }
                }
            }
        }
    };
}
