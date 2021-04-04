# dipa [![Actions Status](https://github.com/chinedufn/dipa/workflows/test/badge.svg)](https://github.com/chinedufn/dipa/actions) [![docs](https://docs.rs/dipa/badge.svg)](https://docs.rs/dipa)

> dipa makes it easy to efficiently delta encode Rust data structures.

dipa's code generation makes it possible to create very tiny diffs between very large data structures
without the burden of writing and maintaining delta compression code by hand.

You can annotate your types with `#[derive(DiffPatch)]` in order to automatically generate
highly space optimized diffing and patching types and functions, or in the most sensitive cases
where you need custom behavior you can instead implement the `Diffable` and `Patchable` traits yourself.

You might make use of dipa as the underlying delta compression machinery in any application where
clients need to be kept up to date with state from a server such as:

- Multiplayer networked games

- Real time databases

_Note that **dipa does not know anything about networks and has no networking code**.
It is only focused on delta encoding._

## [The Dipa Book][book]

[The Dipa Book][book] will introduce you to the library and teach you how to use it.

## Quickstart

<details>
<summary>
Click to show Cargo.toml.
</summary>

```toml
[dependencies]

bincode = "1"
dipa = { version = "0.1", features = ["derive"] }
serde = { version = "1", features = ["derive"] }
```
</details>
<p></p>

```rust
use dipa::{DiffPatch};
use serde::{Serialize, Deserialize};

#[derive(DiffPatch, Serialize, Deserialize)]
struct MyClientState {
    id: u32,
    friends: Option<u8>,
    position: Position,
    notifications: Vec<String>
}

#[derive(DiffPatch, Serialize, Deserialize)]
struct Position {
    x: f32,
    y: f32,
    z: f32
}

fn main() {
    let old_client_state = MyClientState {
        id: 308,
        friends: None,
        position: Position { x: 1., y: 2., z: 3. }
        notifications: vec!["courage".to_string(), "love".to_string()]
    };

    let new_client_state = MyClientState {
        id: 308,
        friends: Some(1),
        position: Position { x: 4., y: 2., z: 3. }
        notifications: vec!["peace".to_string()]
    };

    let patch = old_client_state.create_patch_towards(&new_client_state);

    // Consider using bincode to serialize your diffs on the server side.
    // You can then send them over the wire and deserialize them on the client side.
    //
    // For the tiniest diffs, be sure to use variable integer encoding.
    let serialized = bincode::options().with_varint_encoding().serialize(&patch).unwrap();
    let deserialized: <MyClientState as dipa::Diffable<'_, MyClientState'>::Patch = bincode::options()
        .with_varint_encoding()
        .deserialize(&serialized)
        .unwrap();

    old_client_state.apply_patch(deserialized);

    // All of the fields are now equal.
    assert_eq!(
      old_client_state.notifications,
      new_client_state.notifications
    );
}
```

[See the full API Documentation](https://docs.rs/dipa)

## Advanced Usage

For applications where incredibly small payloads are a top priority, you may wish to take advantage of knowledge about how your application works in order to 
generate even smaller diffs.

For example, say you have the following client state data structure.

```rust
#[derive(DiffPatch)]
struct ClientState {
    hair_length: u128
}
```

If the hair length hasn't changed the diff will be a single byte.

However, whenever the client's hair length changed there would be an additional 17\* bytes in the payload to encode the new `u128` value.

But, what if you that it was impossible for a client's hair length to ever change by more than `100` units in between state updates?

And, hair length changes in between almost every time you send new state out to clients.

And, your application requirements mean that saving every byte matters.

In this case, you could go for something like:

```rust
#[derive(DiffPatch)]
struct ClientState {
    // TODO: dipa should add attributes such as
    // #[dipa(diff_with = "only_small_changes", patch_with = "only_small_changes")]
    // In order to enable custom diffing/patching without needing to clutter your data
    // structues with wrapper types.
    hair_length: OnlySmallChanges(u128)
}

struct OnlySmallChanges(u128);

impl<'p> Diffable<'p, u128> for OnlySmallChanges {
    type Delta = i8;
    type DeltaOwned = Self::Diff;

    fn create_patch_towards(&self, end_state: &u128) -> dipa::CreatePatchTowardsReturn<Self::Diff> {
        let hints = MacroOptimizationHints {
            did_change: self.0 != *end_state,
        };

        (
            self.0 - *end_state,
            hints,
        )
    }

    fn apply_patch(&mut self, patch: Self::Diff) {
        self.0 += patch;
    }
}
```

This approach would reduce your changed value payload from 17 bytes down to just 1.

\* - _17, not 16, since integers larger than `u8/i8` are wrapped in `Option` by their default `DiffPatch` implementation. This optimizes for the case when the integer does not change since `None` serializes to 1 byte._

## Contributing

If you have a use case that isn't supported, a question, a patch, or anything else, go right ahead and open an issue or submit a pull request.

## To Test

To run the test suite.

```sh
# Clone the repository
git clone git@github.com:chinedufn/dipa.git
cd dipa

# Run tests
cargo test --all
```

[book]: https://chinedufn.github.io/dipa
