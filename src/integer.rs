use crate::DiffPatch;

impl DiffPatch for u8 {
    type Patch = u8;

    fn create_patch_towards(&self, end_state: &Self) -> Self::Patch {
        *end_state
    }

    fn apply_patch(&mut self, patch: Self::Patch) {
        *self = patch;
    }
}

impl DiffPatch for u16 {
    type Patch = Option<u16>;

    fn create_patch_towards(&self, end_state: &Self) -> Self::Patch {
        match *self == *end_state {
            true => None,
            false => Some(*end_state),
        }
    }

    fn apply_patch(&mut self, patch: Self::Patch) {
        if let Some(patch) = patch {
            *self = patch;
        }
    }
}

impl DiffPatch for u32 {
    type Patch = Option<u32>;

    fn create_patch_towards(&self, end_state: &Self) -> Self::Patch {
        match *self == *end_state {
            true => None,
            false => Some(*end_state),
        }
    }

    fn apply_patch(&mut self, patch: Self::Patch) {
        if let Some(patch) = patch {
            *self = patch;
        }
    }
}

impl DiffPatch for u64 {
    type Patch = Option<u64>;

    fn create_patch_towards(&self, end_state: &Self) -> Self::Patch {
        match *self == *end_state {
            true => None,
            false => Some(*end_state),
        }
    }

    fn apply_patch(&mut self, patch: Self::Patch) {
        if let Some(patch) = patch {
            *self = patch;
        }
    }
}

impl DiffPatch for u128 {
    type Patch = Option<u128>;

    fn create_patch_towards(&self, end_state: &Self) -> Self::Patch {
        match *self == *end_state {
            true => None,
            false => Some(*end_state),
        }
    }

    fn apply_patch(&mut self, patch: Self::Patch) {
        if let Some(patch) = patch {
            *self = patch;
        }
    }
}

impl DiffPatch for i8 {
    type Patch = i8;

    fn create_patch_towards(&self, end_state: &Self) -> Self::Patch {
        *end_state
    }

    fn apply_patch(&mut self, patch: Self::Patch) {
        *self = patch;
    }
}

impl DiffPatch for i16 {
    type Patch = Option<i16>;

    fn create_patch_towards(&self, end_state: &Self) -> Self::Patch {
        match *self == *end_state {
            true => None,
            false => Some(*end_state),
        }
    }

    fn apply_patch(&mut self, patch: Self::Patch) {
        if let Some(patch) = patch {
            *self = patch;
        }
    }
}

impl DiffPatch for i32 {
    type Patch = Option<i32>;

    fn create_patch_towards(&self, end_state: &Self) -> Self::Patch {
        match *self == *end_state {
            true => None,
            false => Some(*end_state),
        }
    }

    fn apply_patch(&mut self, patch: Self::Patch) {
        if let Some(patch) = patch {
            *self = patch;
        }
    }
}

impl DiffPatch for i64 {
    type Patch = Option<i64>;

    fn create_patch_towards(&self, end_state: &Self) -> Self::Patch {
        match *self == *end_state {
            true => None,
            false => Some(*end_state),
        }
    }

    fn apply_patch(&mut self, patch: Self::Patch) {
        if let Some(patch) = patch {
            *self = patch;
        }
    }
}

impl DiffPatch for i128 {
    type Patch = Option<i128>;

    fn create_patch_towards(&self, end_state: &Self) -> Self::Patch {
        match *self == *end_state {
            true => None,
            false => Some(*end_state),
        }
    }

    fn apply_patch(&mut self, patch: Self::Patch) {
        if let Some(patch) = patch {
            *self = patch;
        }
    }
}

#[cfg(test)]
mod tests_signed {
    use super::*;
    use std::fmt::Debug;

    use serde::de::DeserializeOwned;
    use serde::{Deserialize, Serialize};

    use crate::test_utils::DiffPatchTestCase;

    #[test]
    fn diff_patch_u8_same() {
        DiffPatchTestCase {
            desc: "Diff patch same u8",
            start: 0u8,
            end: 0u8,
            expected_serialized_patch_size: 1,
        }
        .test();
    }

    #[test]
    fn diff_patch_u8_different() {
        DiffPatchTestCase {
            desc: "Diff patch different u8",
            start: 0u8,
            end: 0u8,
            expected_serialized_patch_size: 1,
        }
        .test();
    }

    #[test]
    fn diff_patch_u16_same() {
        DiffPatchTestCase {
            desc: "Diff patch same u16",
            start: 0u16,
            end: 0u16,
            expected_serialized_patch_size: 1,
        }
        .test();
    }

    #[test]
    fn diff_patch_u16_different() {
        DiffPatchTestCase {
            desc: "Diff patch different u16",
            start: 0u16,
            end: 2u16,
            expected_serialized_patch_size: 3,
        }
        .test();
    }

    #[test]
    fn diff_patch_32_same() {
        DiffPatchTestCase {
            desc: "Diff patch same u32",
            start: 0u32,
            end: 0u32,
            expected_serialized_patch_size: 1,
        }
        .test();
    }

    #[test]
    fn diff_patch_32_different() {
        DiffPatchTestCase {
            desc: "Diff patch different u32s",
            start: 0u32,
            end: 1u32,
            expected_serialized_patch_size: 5,
        }
        .test();
    }

    #[test]
    fn diff_patch_64_same() {
        DiffPatchTestCase {
            desc: "Diff patch same u64",
            start: 0u64,
            end: 0u64,
            expected_serialized_patch_size: 1,
        }
        .test();
    }

    #[test]
    fn diff_patch_64_different() {
        DiffPatchTestCase {
            desc: "Diff patch different u64s",
            start: 0u64,
            end: 1u64,
            expected_serialized_patch_size: 9,
        }
        .test();
    }

    #[test]
    fn diff_patch_128_same() {
        DiffPatchTestCase {
            desc: "Diff patch same u128",
            start: 0u128,
            end: 0u128,
            expected_serialized_patch_size: 1,
        }
        .test();
    }

    #[test]
    fn diff_patch_128_different() {
        DiffPatchTestCase {
            desc: "Diff patch different u128s",
            start: 0u128,
            end: 1u128,
            expected_serialized_patch_size: 17,
        }
        .test();
    }
}

#[cfg(test)]
mod tests_unsigned {
    use super::*;
    use std::fmt::Debug;

    use serde::de::DeserializeOwned;
    use serde::{Deserialize, Serialize};

    use crate::test_utils::DiffPatchTestCase;

    #[test]
    fn diff_patch_i8_same() {
        DiffPatchTestCase {
            desc: "Diff patch same i8",
            start: 0i8,
            end: 0i8,
            expected_serialized_patch_size: 1,
        }
        .test();
    }

    #[test]
    fn diff_patch_i8_different() {
        DiffPatchTestCase {
            desc: "Diff patch different i8",
            start: 0i8,
            end: 0i8,
            expected_serialized_patch_size: 1,
        }
        .test();
    }

    #[test]
    fn diff_patch_i16_same() {
        DiffPatchTestCase {
            desc: "Diff patch same i16",
            start: 0i16,
            end: 0i16,
            expected_serialized_patch_size: 1,
        }
        .test();
    }

    #[test]
    fn diff_patch_i16_different() {
        DiffPatchTestCase {
            desc: "Diff patch different i16",
            start: 0i16,
            end: 2i16,
            expected_serialized_patch_size: 3,
        }
        .test();
    }

    #[test]
    fn diff_patch_32_same() {
        DiffPatchTestCase {
            desc: "Diff patch same i32",
            start: 0i32,
            end: 0i32,
            expected_serialized_patch_size: 1,
        }
        .test();
    }

    #[test]
    fn diff_patch_32_different() {
        DiffPatchTestCase {
            desc: "Diff patch different i32s",
            start: 0i32,
            end: 1i32,
            expected_serialized_patch_size: 5,
        }
        .test();
    }

    #[test]
    fn diff_patch_64_same() {
        DiffPatchTestCase {
            desc: "Diff patch same i64",
            start: 0i64,
            end: 0i64,
            expected_serialized_patch_size: 1,
        }
        .test();
    }

    #[test]
    fn diff_patch_64_different() {
        DiffPatchTestCase {
            desc: "Diff patch different i64s",
            start: 0i64,
            end: 1i64,
            expected_serialized_patch_size: 9,
        }
        .test();
    }

    #[test]
    fn diff_patch_128_same() {
        DiffPatchTestCase {
            desc: "Diff patch same i128",
            start: 0i128,
            end: 0i128,
            expected_serialized_patch_size: 1,
        }
        .test();
    }

    #[test]
    fn diff_patch_128_different() {
        DiffPatchTestCase {
            desc: "Diff patch different i128s",
            start: 0i128,
            end: 1i128,
            expected_serialized_patch_size: 17,
        }
        .test();
    }
}
