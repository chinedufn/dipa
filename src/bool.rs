use crate::CreatedDelta;
use crate::{number_diff_impl_u8_or_i8, number_patch_impl_u8_or_i8};

number_diff_impl_u8_or_i8!(bool, bool);
number_patch_impl_u8_or_i8!(bool, bool);

#[cfg(test)]
mod tests {
    use crate::dipa_impl_tester::DipaImplTester;

    #[test]
    fn bool_unchanged() {
        DipaImplTester {
            label: Some("Diff patch same bool"),
            start: &mut true,
            end: &true,
            expected_delta: true,
            expected_serialized_patch_size: 1,
            expected_did_change: false,
        }
        .test();
    }

    #[test]
    fn bool_changed() {
        DipaImplTester {
            label: Some("Diff patch different bool"),
            start: &mut true,
            end: &false,
            expected_delta: false,
            expected_serialized_patch_size: 1,
            expected_did_change: true,
        }
        .test();
    }
}
