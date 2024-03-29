use crate::CreatedDelta;
use crate::{
    number_diff_impl_option_wrapped, number_diff_impl_u8_or_i8, number_patch_impl_mut_u8_or_i8,
    number_patch_impl_option_wrapped, number_patch_impl_u8_or_i8,
};

number_diff_impl_u8_or_i8!(u8, u8);
number_patch_impl_u8_or_i8!(u8, u8);
number_patch_impl_mut_u8_or_i8!(&mut u8, u8);

number_diff_impl_u8_or_i8!(i8, i8);
number_patch_impl_u8_or_i8!(i8, i8);
number_patch_impl_mut_u8_or_i8!(&mut i8, i8);

number_diff_impl_option_wrapped!(u16, u16);
number_patch_impl_option_wrapped!(u16, Option<u16>);
number_diff_impl_option_wrapped!(i16, i16);
number_patch_impl_option_wrapped!(i16, Option<i16>);

number_diff_impl_option_wrapped!(u32, u32);
number_patch_impl_option_wrapped!(u32, Option<u32>);
number_diff_impl_option_wrapped!(i32, i32);
number_patch_impl_option_wrapped!(i32, Option<i32>);

number_diff_impl_option_wrapped!(u64, u64);
number_patch_impl_option_wrapped!(u64, Option<u64>);
number_diff_impl_option_wrapped!(i64, i64);
number_patch_impl_option_wrapped!(i64, Option<i64>);

number_diff_impl_option_wrapped!(u128, u128);
number_patch_impl_option_wrapped!(u128, Option<u128>);
number_diff_impl_option_wrapped!(i128, i128);
number_patch_impl_option_wrapped!(i128, Option<i128>);

#[cfg(test)]
mod tests_signed {

    use crate::dipa_impl_tester::DipaImplTester;

    #[test]
    fn diff_patch_u8_same() {
        DipaImplTester {
            label: Some("Diff patch same u8"),
            start: &mut 0u8,
            end: &0u8,
            expected_delta: 0,
            expected_serialized_patch_size: 1,
            expected_did_change: false,
        }
        .test();
    }

    #[test]
    fn diff_patch_u8_different() {
        DipaImplTester {
            label: Some("Diff patch different u8"),
            start: &mut 0u8,
            end: &2u8,
            expected_delta: 2,
            expected_serialized_patch_size: 1,
            expected_did_change: true,
        }
        .test();
    }

    #[test]
    fn diff_patch_u16_same() {
        DipaImplTester {
            label: Some("Diff patch same u16"),
            start: &mut 0u16,
            end: &0u16,
            expected_delta: None,
            expected_serialized_patch_size: 1,
            expected_did_change: false,
        }
        .test();
    }

    #[test]
    fn diff_patch_u16_different() {
        DipaImplTester {
            label: Some("Diff patch different u16"),
            start: &mut 0u16,
            end: &2u16,
            expected_delta: Some(2),
            expected_serialized_patch_size: 2,
            expected_did_change: true,
        }
        .test();
    }

    #[test]
    fn diff_patch_32_same() {
        DipaImplTester {
            label: Some("Diff patch same u32"),
            start: &mut 0u32,
            end: &0u32,
            expected_delta: None,
            expected_serialized_patch_size: 1,
            expected_did_change: false,
        }
        .test();
    }

    #[test]
    fn diff_patch_32_different() {
        DipaImplTester {
            label: Some("Diff patch different u32s"),
            start: &mut 0u32,
            end: &1u32,
            expected_delta: Some(1),
            expected_serialized_patch_size: 2,
            expected_did_change: true,
        }
        .test();
    }

    #[test]
    fn diff_patch_64_same() {
        DipaImplTester {
            label: Some("Diff patch same u64"),
            start: &mut 0u64,
            end: &0u64,
            expected_delta: None,
            expected_serialized_patch_size: 1,
            expected_did_change: false,
        }
        .test();
    }

    #[test]
    fn diff_patch_64_different() {
        DipaImplTester {
            label: Some("Diff patch different u64s"),
            start: &mut 0u64,
            end: &1u64,
            expected_delta: Some(1),
            expected_serialized_patch_size: 2,
            expected_did_change: true,
        }
        .test();
    }

    #[test]
    fn diff_patch_128_same() {
        DipaImplTester {
            label: Some("Diff patch same u128"),
            start: &mut 0u128,
            end: &0u128,
            expected_delta: None,
            expected_serialized_patch_size: 1,
            expected_did_change: false,
        }
        .test();
    }

    #[test]
    fn diff_patch_128_different() {
        DipaImplTester {
            label: Some("Diff patch different u128s"),
            start: &mut 0u128,
            end: &1u128,
            expected_delta: Some(1),
            expected_serialized_patch_size: 2,
            expected_did_change: true,
        }
        .test();
    }
}

#[cfg(test)]
mod tests_unsigned {

    use crate::dipa_impl_tester::DipaImplTester;

    #[test]
    fn diff_patch_i8_same() {
        DipaImplTester {
            label: Some("Diff patch same i8"),
            start: &mut 0i8,
            end: &0i8,
            expected_delta: 0,
            expected_serialized_patch_size: 1,
            expected_did_change: false,
        }
        .test();
    }

    #[test]
    fn diff_patch_i8_different() {
        DipaImplTester {
            label: Some("Diff patch different i8"),
            start: &mut 0i8,
            end: &1i8,
            expected_delta: 1,
            expected_serialized_patch_size: 1,
            expected_did_change: true,
        }
        .test();
    }

    #[test]
    fn diff_patch_i16_same() {
        DipaImplTester {
            label: Some("Diff patch same i16"),
            start: &mut 0i16,
            end: &0i16,
            expected_delta: None,
            expected_serialized_patch_size: 1,
            expected_did_change: false,
        }
        .test();
    }

    #[test]
    fn diff_patch_i16_different() {
        DipaImplTester {
            label: Some("Diff patch different i16"),
            start: &mut 0i16,
            end: &2i16,
            expected_delta: Some(2),
            expected_serialized_patch_size: 2,
            expected_did_change: true,
        }
        .test();
    }

    #[test]
    fn diff_patch_32_same() {
        DipaImplTester {
            label: Some("Diff patch same i32"),
            start: &mut 0i32,
            end: &0i32,
            expected_delta: None,
            expected_serialized_patch_size: 1,
            expected_did_change: false,
        }
        .test();
    }

    #[test]
    fn diff_patch_32_different() {
        DipaImplTester {
            label: Some("Diff patch different i32s"),
            start: &mut 0i32,
            end: &1i32,
            expected_delta: Some(1),
            expected_serialized_patch_size: 2,
            expected_did_change: true,
        }
        .test();
    }

    #[test]
    fn diff_patch_64_same() {
        DipaImplTester {
            label: Some("Diff patch same i64"),
            start: &mut 0i64,
            end: &0i64,
            expected_delta: None,
            expected_serialized_patch_size: 1,
            expected_did_change: false,
        }
        .test();
    }

    #[test]
    fn diff_patch_64_different() {
        DipaImplTester {
            label: Some("Diff patch different i64s"),
            start: &mut 0i64,
            end: &1i64,
            expected_delta: Some(1),
            expected_serialized_patch_size: 2,
            expected_did_change: true,
        }
        .test();
    }

    #[test]
    fn diff_patch_128_same() {
        DipaImplTester {
            label: Some("Diff patch same i128"),
            start: &mut 0i128,
            end: &0i128,
            expected_delta: None,
            expected_serialized_patch_size: 1,
            expected_did_change: false,
        }
        .test();
    }

    #[test]
    fn diff_patch_128_different() {
        DipaImplTester {
            label: Some("Diff patch different i128s"),
            start: &mut 0i128,
            end: &1i128,
            expected_delta: Some(1),
            expected_serialized_patch_size: 2,
            expected_did_change: true,
        }
        .test();
    }
}
