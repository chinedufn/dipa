# dipa [![Actions Status](https://github.com/chinedufn/dipa/workflows/ci/badge.svg)](https://github.com/chinedufn/dipa/actions) [![docs](https://docs.rs/dipa/badge.svg)](https://docs.rs/dipa)

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

## [The dipa Book][book]

[The dipa Book][book] will introduce you to the library and teach you how to use it.

It is also available offline:

```sh
# Do this once while online
git clone git@github.com:chinedufn/dipa.git && cd dipa
cargo install mdbook

# This works offline
./bin/serve-book.sh
```

## Quickstart

The easiest way to get started with dipa is by using the `#[derive(DiffPatch)]` macro. Here's a quick peek.

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
use std::borrow::Cow;

#[derive(DiffPatch)]
struct MyClientState {
    id: u32,
    friends: Option<u8>,
    position: Position,
    notifications: Vec<Cow<&'static, str>>,
	emotional_state: EmotionalState
}

#[derive(DiffPatch)]
struct Position {
    x: f32,
    y: f32,
    z: f32
}

#[derive(DiffPatch)]
enum EmotionalState {
    Peace { score: u128 },
    Love(u64),
    Courage(u32),
}

fn main() {
    let old_client_state = MyClientState {
        id: 308,
        friends: None,
        position: Position { x: 1., y: 2., z: 3. }
        notifications: vec![Cow::Borrowed("let"), Cow::Owned("go".to_string())],
        emotional_state: EmotionalState::Love(100),
    };

    let new_client_state = MyClientState {
        id: 308,
        friends: Some(1),
        position: Position { x: 4., y: 2., z: 3. }
        notifications: vec![Cow::Borrowed("free")]
        emotional_state: EmotionalState::Peace { score: 10_000 },
    };

    let delta = old_client_state.create_delta_towards(&new_client_state);

    let bin = bincode::options().with_varint_encoding();

    // Consider using bincode to serialize your diffs on the server side.
    // You can then send them over the wire and deserialize them on the client side.
    //
    // For the tiniest diffs, be sure to use variable integer encoding.
    let serialized = bin.serialize(&delta).unwrap();

    // ... Pretend you send the data to the client ...

    let deserialized: <MyClientState as dipa::Diffable<'_, '_, MyClientState>::DeltaOwned = 
        bin.deserialize(&serialized).unwrap();

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

However, whenever the client's hair length changed there would be up to an additional 17\* bytes in the payload to variable integer
encode the new `u128` value.

But, what if you knew that it was impossible for a client's hair length to ever change by more than `100` units in between state updates?

And, your application requirements mean that saving every byte matters and so it is worth your time to customize your hair
length delta encoding.

In this case, you could go for something like:

```rust
use dipa::{CreatedDelta, Diffable, Patchable};

#[derive(DiffPatch)]
struct ClientState {
    hair_length: DeltaWithI8(u128)
}

struct DeltaWithI8(u128);

impl<'s, 'e> Diffable<'s, 'e, u128> for DeltaWithI8 {
    type Delta = i8;
    type DeltaOwned = Self::Delta;

    fn create_delta_towards(&self, end_state: &u128) -> CreatedDelta<Self::Delta> {
		CreatedDelta {
		    delta: self.0 - *end_state,
		    did_change: self.0 != *end_state,
		}
    }
}

impl Patchable<i8> for OnlySmallChanges {
    fn apply_patch(&mut self, patch: i8) {
        self.0 += patch;
    }
}
```

This approach would reduce hair length delta from 17 bytes down to just a single byte.

\* - _17, not 16, since integers larger than `u8/i8` are wrapped in `Option` by their default `DiffPatch` implementation. This optimizes for the case when the integer does not change since `None` serializes to 1 byte._

## Questions

If you have a question that you can't find the answer to within five minutes then this is considered a documentation bug.

Please [open an issue](https://github.com/chinedufn/dipa/issues/new) with your question.

Or, even better, a work-in-progress pull request with a skeleton of a code example,
API documentation or area in the book where your question could have been answered.

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

## License

dipa is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE][apache] or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT][mit] or http://opensource.org/licenses/MIT)

[book]: https://chinedufn.github.io/dipa
[apache]: ./LICENSE-APACHE
[mit]: ./LICENSE-MIT
