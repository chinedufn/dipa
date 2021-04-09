# Attributes

There are three categories of attributes that can be used with the derive macro. Container, variant and field
attributes.

```rust
#[derive(DiffPatch)]
#[dipa(diff_derives = "Debug, Copy, Clone")]  // <-- this is a container attribute
struct S {
    #[dipa(todo_field_attribute_here)]  // <-- this is a field attribute
    f: i32,
}

#[derive(DiffPatch)]
#[dipa(patch_derives = "Debug, Serialize")]  // <-- this is a container attribute
enum E {
    #[dipa(todo_variant_attribute_here)]  // <-- this is a variant attribute
    A(
      #[dipa(todo_field_attribute_here)]  // <-- this is a field attribute
      String
    ),
}
```

## Container Attributes

`diff_derives = "SomeDerive, AnotherDerive"`
   
Used to add #[derive(SomeDerive, AnotherDerive)]'s for the delta encoded diff type that dipa generates for your struct or enum.

This is mainly useful internally so that we can satisfy the necessary trait bounds for using our automatically generated
`Diffable::Delta`'s' with the `DipaImplTester`.

---

`patch_derives = "SomeDerive, AnotherDerive"`

Used to add #[derive(SomeDerive, AnotherDerive)]'s for the associated `Diffable::DeltaOwned` type that dipa generates for your struct or enum.

---

`field_batching_strategy = "..."`

At this time this can either be set to `one_batch`, `no_batching`. There are other batching strategies planned such as being able to use multiple enums
each responsible for a few fields, or being able to annotate individual fields in order to indicate which batch of deltas that they should belong to.

- `one_batch` - A single enum will be used as the `Diffable::Delta` type. This enum will be able to represent every possible combination of the struct's fields changing.
  By default this strategy is limited to structs that have 5 fields since as the number of fields grows the number of enum variants grows exponentially.
  The `max_fields_per_batch` attribute can be used to increase this limit on a per-struct basis.

  ```rust
  #[derive(DiffPatch)]
  #[dipa(field_batching_strategy = "one_batch")]
  struct MyStruct {
      field1: u32,
      field2: u64
  }

  // Automatically generated delta would look something like this
  enum MyStructDelta<'d> {
      NoChange,
      Change_0(<u32 as dipa::Diffable<'d>::Delta),
      Change_1(<u64 as dipa::Diffable<'d>::Delta),
      Change_0_1(
          <u32 as dipa::Diffable<'d>::Delta,
          <u64 as dipa::Diffable<'d>::Delta
      ),
  }
  ```

- `no_batching` - The `Diffable::Delta` type will be a struct with the same number of fields as your original type. This is useful when have too many fields
  for the `on_batch` strategy. Note that in the future we will introduce other strategies that are likely to better handle large numbers of fields. So this
  is more of a temporary measure.

  ```rust
  #[derive(DiffPatch)]
  #[dipa(field_batching_strategy = "no_batching")]
  struct MyStruct {
      field1: u32,
      field2: u64
  }

  // Automatically generated delta would look something like this
  struct MyStructDelta {
      field1: <u32 as dipa::Diffable<'d>::Delta,
      field2: <u64 as dipa::Diffable<'d>::Delta,
  }
  ```

---

`max_fields_per_batch = 5`

This can used when the `field_batching_strategy = "one_batch"`.

By default, the `one_batch` strategy can only be used with structs or enum that have 5 or fewer fields. The `max_fields_per_batch` allows you to increase this limit.

There is a hard cap on how high you can set `max_fields_per_batch` can be set in order to prevent you from accidentally causing unreasonable compile times. Values above
7 will lead to a compile time error. In the future we will experiment with different values to see how the compile time trade-offs look.

