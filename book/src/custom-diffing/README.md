# Custom Delta Encoding

In most cases, you'll start by using `#[derive(DiffPatch)]` on your data structure in order to get yourself up and running quickly.

For complex applications where every byte matters you will eventually run into situations where you can use your knowledge of how your
application works in order to generate even smaller deltas for your types.

In this case you can turn to the community to see if there is already a custom implementation that handles what you are after,
or simply implement `Diffable` and `Patchable` yourself.

```rust
#[derive(DiffPatch)]
struct MyStruct {
   field_a: Vec<f32>,
   custom: CustomStruct
}

// You know things about how you're using CustomStruct that dipa does not,
// and you want to use that knowledge to control how it gets delta encoded.
// So, you implement Diffable and Patchable yourself.
struct CustomStruct { name: String }

impl Diffable<'d, CustomStruct> for CustomStruct {
  type Delta = MyDelta;
  type DeltaOwned = MyDeltaOwned;

   // ...
}

impl Patchable<MyDeltaOwned> for CustomStruct {
   // ...
}

#[derive(Serialize)]
struct MyDelta<'a>(&'a u128, &'a [u8]);

#[derive(Deerialize)]
struct MyDeltaOwned(u128, Vec<u8>);
```
