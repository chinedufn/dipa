# Introduction

> dipa makes it easy to efficiently delta encode large Rust data structures.

In some applications, the data that you are sending to the client is often almost exactly the same as the data
that you last sent to them.

Rather than repeatedly sending nearly identical state objects to a client, an application might calculate
what has changed since the last time data was sent and then only send down those changes.

This approach can dramatically reduce both your and your users' bandwidth requirements and network traffic
costs.

The process of determining what has changed between two instances of a data structure is known as delta encoding.

Historically, delta encoding code would become more and more difficult to maintain as your application's
data structures grew more and more complex.

This made it a tedious optimization reserved for only the most bandwidth sensitive applications, such as networked
games.

dipa eliminates the maintainability challenges of efficient delta encoding code by generating all of the code for you.

dipa is designed to generate very tiny diffs by default. In the most sensitive cases where you have application specific
knowledge about your data structures that could help you generate even tinier diffs, you can implement the traits
for that type yourself and let dipa's derive macro take care of the rest.

_Note that **dipa does not know anything about networks and has no networking code**.
It is only focused on encoding deltas, not transmitting them._

## Use Cases

You might make use of dipa as the underlying delta compression machinery in any application where
you want to reduce the network traffic required to keep clients up to date with state from a server such as:

- Multiplayer networked games and simulations

- Real time client side views into server side data

_Note that **dipa does not know anything about networks and has no networking code**.
It is only focused on encoding deltas, not transmitting them._

## Quick Start

One key feature of dipa is the `#[derive(DiffPatch)]` macro that allows you to implement delta compression
without the tedious and hard to maintain process of writing the necessary data structures and logic by hand.

Here's a quick look at dipa in action.

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
