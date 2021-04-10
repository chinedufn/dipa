//! Verify that we can publicly expose types that use the derive macro.
//! This essentially ensures that we mark the generated associated types as `pub`.

#[derive(DiffPatch)]
pub struct EmptyStruct;

#[derive(DiffPatch)]
pub struct Struct {}

#[derive(DiffPatch)]
pub struct StructOne {
    field: u32,
}

#[derive(DiffPatch)]
pub struct StructMulti {
    field: u32,
    field2: u64,
}

#[derive(DiffPatch)]
#[dipa(field_batching_strategy = "no_batching")]
pub struct StructNoBatching {
    a: u32,
    b: u64,
}

#[derive(DiffPatch)]
pub struct TupleStruct(u32);

#[derive(DiffPatch)]
pub struct TupleStructMulti(u32, u64);

#[derive(DiffPatch)]
pub enum MyEnum {}

#[derive(DiffPatch)]
#[allow(dead_code)]
pub enum MyEnumSingleVariant {
    One,
}

#[derive(DiffPatch)]
pub enum MyEnumMultiVariant {
    One,
    Two,
}

#[derive(DiffPatch)]
pub enum MyEnumMultiVariantWithOneField {
    One(u32),
    Two,
}

#[derive(DiffPatch)]
pub enum MyEnumMultiVariantWithFields {
    One(u32, u64),
    Two,
}
