use crate::DiffPatch;

impl<'p> DiffPatch<'p> for u8 {
    // TODO: &u8
    type Diff = u8;
    type OwnedDiff = Self::Diff;

    fn create_patch_towards(&self, end_state: &Self) -> Self::Diff {
        *end_state
    }

    fn apply_patch(&mut self, patch: Self::Diff) {
        *self = patch;
    }
}

impl<'p> DiffPatch<'p> for u16 {
    // TODO: &u16
    type Diff = Option<u16>;
    type OwnedDiff = Self::Diff;

    fn create_patch_towards(&self, end_state: &Self) -> Self::Diff {
        match *self == *end_state {
            true => None,
            false => Some(*end_state),
        }
    }

    fn apply_patch(&mut self, patch: Self::Diff) {
        if let Some(patch) = patch {
            *self = patch;
        }
    }
}

impl<'p> DiffPatch<'p> for u32 {
    // TODO: &u16
    type Diff = Option<u32>;
    type OwnedDiff = Self::Diff;

    fn create_patch_towards(&self, end_state: &Self) -> Self::Diff {
        match *self == *end_state {
            true => None,
            false => Some(*end_state),
        }
    }

    fn apply_patch(&mut self, patch: Self::Diff) {
        if let Some(patch) = patch {
            *self = patch;
        }
    }
}

impl<'p> DiffPatch<'p> for u64 {
    // TODO: &u16
    type Diff = Option<u64>;
    type OwnedDiff = Self::Diff;

    fn create_patch_towards(&self, end_state: &Self) -> Self::Diff {
        match *self == *end_state {
            true => None,
            false => Some(*end_state),
        }
    }

    fn apply_patch(&mut self, patch: Self::Diff) {
        if let Some(patch) = patch {
            *self = patch;
        }
    }
}

impl<'p> DiffPatch<'p> for u128 {
    // TODO: &u16
    type Diff = Option<u128>;
    type OwnedDiff = Self::Diff;

    fn create_patch_towards(&self, end_state: &Self) -> Self::Diff {
        match *self == *end_state {
            true => None,
            false => Some(*end_state),
        }
    }

    fn apply_patch(&mut self, patch: Self::Diff) {
        if let Some(patch) = patch {
            *self = patch;
        }
    }
}

impl<'p> DiffPatch<'p> for i8 {
    // TODO: &u16
    type Diff = i8;
    type OwnedDiff = Self::Diff;

    fn create_patch_towards(&self, end_state: &Self) -> Self::Diff {
        *end_state
    }

    fn apply_patch(&mut self, patch: Self::Diff) {
        *self = patch;
    }
}

impl<'p> DiffPatch<'p> for i16 {
    // TODO: &u16
    type Diff = Option<i16>;
    type OwnedDiff = Self::Diff;

    fn create_patch_towards(&self, end_state: &Self) -> Self::Diff {
        match *self == *end_state {
            true => None,
            false => Some(*end_state),
        }
    }

    fn apply_patch(&mut self, patch: Self::Diff) {
        if let Some(patch) = patch {
            *self = patch;
        }
    }
}

impl<'p> DiffPatch<'p> for i32 {
    // TODO: &u16
    type Diff = Option<i32>;
    type OwnedDiff = Self::Diff;

    fn create_patch_towards(&self, end_state: &Self) -> Self::Diff {
        match *self == *end_state {
            true => None,
            false => Some(*end_state),
        }
    }

    fn apply_patch(&mut self, patch: Self::Diff) {
        if let Some(patch) = patch {
            *self = patch;
        }
    }
}

impl<'p> DiffPatch<'p> for i64 {
    // TODO: &u16
    type Diff = Option<i64>;
    type OwnedDiff = Self::Diff;

    fn create_patch_towards(&self, end_state: &Self) -> Self::Diff {
        match *self == *end_state {
            true => None,
            false => Some(*end_state),
        }
    }

    fn apply_patch(&mut self, patch: Self::Diff) {
        if let Some(patch) = patch {
            *self = patch;
        }
    }
}

impl<'p> DiffPatch<'p> for i128 {
    // TODO: &u16
    type Diff = Option<i128>;
    type OwnedDiff = Self::Diff;

    fn create_patch_towards(&self, end_state: &Self) -> Self::Diff {
        match *self == *end_state {
            true => None,
            false => Some(*end_state),
        }
    }

    fn apply_patch(&mut self, patch: Self::Diff) {
        if let Some(patch) = patch {
            *self = patch;
        }
    }
}

#[cfg(test)]
mod tests_signed {

    use crate::test_utils::DiffPatchTestCase;

    #[test]
    fn diff_patch_u8_same() {
        DiffPatchTestCase {
            label: Some("Diff patch same u8"),
            start: 0u8,
            end: &0u8,
            expected_diff: 0,
            expected_serialized_patch_size: 1,
        }
        .test();
    }

    #[test]
    fn diff_patch_u8_different() {
        DiffPatchTestCase {
            label: Some("Diff patch different u8"),
            start: 0u8,
            end: &2u8,
            expected_diff: 2,
            expected_serialized_patch_size: 1,
        }
        .test();
    }

    #[test]
    fn diff_patch_u16_same() {
        DiffPatchTestCase {
            label: Some("Diff patch same u16"),
            start: 0u16,
            end: &0u16,
            expected_diff: None,
            expected_serialized_patch_size: 1,
        }
        .test();
    }

    #[test]
    fn diff_patch_u16_different() {
        DiffPatchTestCase {
            label: Some("Diff patch different u16"),
            start: 0u16,
            end: &2u16,
            expected_diff: Some(2),
            expected_serialized_patch_size: 3,
        }
        .test();
    }

    #[test]
    fn diff_patch_32_same() {
        DiffPatchTestCase {
            label: Some("Diff patch same u32"),
            start: 0u32,
            end: &0u32,
            expected_diff: None,
            expected_serialized_patch_size: 1,
        }
        .test();
    }

    #[test]
    fn diff_patch_32_different() {
        DiffPatchTestCase {
            label: Some("Diff patch different u32s"),
            start: 0u32,
            end: &1u32,
            expected_diff: Some(1),
            expected_serialized_patch_size: 5,
        }
        .test();
    }

    #[test]
    fn diff_patch_64_same() {
        DiffPatchTestCase {
            label: Some("Diff patch same u64"),
            start: 0u64,
            end: &0u64,
            expected_diff: None,
            expected_serialized_patch_size: 1,
        }
        .test();
    }

    #[test]
    fn diff_patch_64_different() {
        DiffPatchTestCase {
            label: Some("Diff patch different u64s"),
            start: 0u64,
            end: &1u64,
            expected_diff: Some(1),
            expected_serialized_patch_size: 9,
        }
        .test();
    }

    #[test]
    fn diff_patch_128_same() {
        DiffPatchTestCase {
            label: Some("Diff patch same u128"),
            start: 0u128,
            end: &0u128,
            expected_diff: None,
            expected_serialized_patch_size: 1,
        }
        .test();
    }

    #[test]
    fn diff_patch_128_different() {
        DiffPatchTestCase {
            label: Some("Diff patch different u128s"),
            start: 0u128,
            end: &1u128,
            expected_diff: Some(1),
            expected_serialized_patch_size: 17,
        }
        .test();
    }
}

#[cfg(test)]
mod tests_unsigned {

    use crate::test_utils::DiffPatchTestCase;

    #[test]
    fn diff_patch_i8_same() {
        DiffPatchTestCase {
            label: Some("Diff patch same i8"),
            start: 0i8,
            end: &0i8,
            expected_diff: 0,
            expected_serialized_patch_size: 1,
        }
        .test();
    }

    #[test]
    fn diff_patch_i8_different() {
        DiffPatchTestCase {
            label: Some("Diff patch different i8"),
            start: 0i8,
            end: &1i8,
            expected_diff: 1,
            expected_serialized_patch_size: 1,
        }
        .test();
    }

    #[test]
    fn diff_patch_i16_same() {
        DiffPatchTestCase {
            label: Some("Diff patch same i16"),
            start: 0i16,
            end: &0i16,
            expected_diff: None,
            expected_serialized_patch_size: 1,
        }
        .test();
    }

    #[test]
    fn diff_patch_i16_different() {
        DiffPatchTestCase {
            label: Some("Diff patch different i16"),
            start: 0i16,
            end: &2i16,
            expected_diff: Some(2),
            expected_serialized_patch_size: 3,
        }
        .test();
    }

    #[test]
    fn diff_patch_32_same() {
        DiffPatchTestCase {
            label: Some("Diff patch same i32"),
            start: 0i32,
            end: &0i32,
            expected_diff: None,
            expected_serialized_patch_size: 1,
        }
        .test();
    }

    #[test]
    fn diff_patch_32_different() {
        DiffPatchTestCase {
            label: Some("Diff patch different i32s"),
            start: 0i32,
            end: &1i32,
            expected_diff: Some(1),
            expected_serialized_patch_size: 5,
        }
        .test();
    }

    #[test]
    fn diff_patch_64_same() {
        DiffPatchTestCase {
            label: Some("Diff patch same i64"),
            start: 0i64,
            end: &0i64,
            expected_diff: None,
            expected_serialized_patch_size: 1,
        }
        .test();
    }

    #[test]
    fn diff_patch_64_different() {
        DiffPatchTestCase {
            label: Some("Diff patch different i64s"),
            start: 0i64,
            end: &1i64,
            expected_diff: Some(1),
            expected_serialized_patch_size: 9,
        }
        .test();
    }

    #[test]
    fn diff_patch_128_same() {
        DiffPatchTestCase {
            label: Some("Diff patch same i128"),
            start: 0i128,
            end: &0i128,
            expected_diff: None,
            expected_serialized_patch_size: 1,
        }
        .test();
    }

    #[test]
    fn diff_patch_128_different() {
        DiffPatchTestCase {
            label: Some("Diff patch different i128s"),
            start: 0i128,
            end: &1i128,
            expected_diff: Some(1),
            expected_serialized_patch_size: 17,
        }
        .test();
    }
}
