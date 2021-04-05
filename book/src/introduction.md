# Introduction

Dipa is focused on making it easy to delta encode Rust data structures.

Traditionally, efficient delta compression of data structures required a fair bit of
hand written code. As your data structures grew.

Dipa solves this problem by generating your delta compression code for you.

No flexibility is lost. In the most advanced cases where you need custom behavior,
Dipa exposes traits that you can implement for types that have special application specific needs.

## Use Cases

Some applications that might use Dipa include multiplayer networked games and simulations, real time data views
or any other application where you're syncing state between a server and one or more clients.

Note that Dipa itself does not know anything about networks or contain any networking related code.

Dipa is only focused on enabling the diffing and patching of Rust data structures.

## Quick Start

The key feature of Dipa is the `#[derive(DiffPatch)]` macro that allows you to implement delta compression
without the tedious and hard to maintain process of writing the necessary data structures and logic by hand.

Here's a quick look at Dipa in action.

```rust
use dipa::DiffPatch;

#[derive(DiffPatch)]
struct MyStruct {
    a_field: MyEnum,
    another_field: i32,
    one_more: Vec<MyOtherStruct>
}

#[derive(DiffPatch)]
enum MyEnum {
    A(u8),
    B(u16, Vec<f32>),
    C { set: HashSet<u128> },
}

#[derive(DiffPatch)]
struct MyOtherStruct(u64, BTreeMap<i8, MyEnum>);

fn main () {
	let old = MyStruct {
	    a_field: MyEnum::B(308, vec![1., 2.]),
	    another_field: -10,
	    one_more: vec![MyOtherStruct(7, BTreeMap::new())]
	};

	let new =  MyStruct {
	    a_field: MyEnum::A(10),
	    another_field: 650,
	    one_more: vec![MyOtherStruct(567, BTreeMap::new())]
	};

    let diff = old.create_delta_towards(&new);

    let serialized = bincode::options()
        .with_varint_encoding()
        .serialize(&diff)
        .unwrap();

    // ... Pretend we sent the bytes over a network ...

    let deserialized:
      <MyStructDiff as dipa::Diffable<'_, MyStructDiff>>::DeltaOwned =
      bincode::options()
        .with_varint_encoding()
        .deserialize(&serialized)
        .unwrap();

	let mut old = old;
    old.apply_patch(deserialized);

    // old is now equal to new.
}
```
