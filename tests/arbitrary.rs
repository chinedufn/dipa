#[cfg(test)]
#[cfg(feature = "derive")]
pub mod with_derive {
    use bincode::Options;
    use dipa::{DiffPatch, Diffable, Patchable};
    use quickcheck::quickcheck;
    use quickcheck_derive::Arbitrary;
    use serde_derive::{Deserialize, Serialize};
    use std::collections::BTreeMap;

    #[derive(
        Clone, Eq, PartialEq, PartialOrd, Ord, Debug, DiffPatch, Serialize, Deserialize, Arbitrary,
    )]
    pub struct Basic {
        pub items: BTreeMap<u16, Vec<u8>>,
    }

    fn tester(mut a: Basic, b: Basic) -> bool {
        let bin = bincode::options().with_varint_encoding();
        let created = a.create_delta_towards(&b);

        let serialized = bin.serialize(&created.delta).unwrap();
        let deserialized: <Basic as dipa::Diffable<'_, '_, Basic>>::DeltaOwned =
            bin.deserialize(&serialized).unwrap();

        a.apply_patch(deserialized);
        return a == b;
    }

    #[test]
    fn test() {
        quickcheck(tester as fn(Basic, Basic) -> bool);
    }
}
