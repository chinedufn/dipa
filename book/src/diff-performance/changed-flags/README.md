# Changed Flags

Say you have a real-time application that delta compresses the following type.

```rust
#[derive(DiffPatch)]
struct MyStruct {
    water_droplets: Vec<WaterDroplet>
}

#[derive(DiffPatch)]
struct WaterDroplet([f32; 3]);
```

At first you were simulating the Sahara Desert and things were going smoothly.

Now, however, you're simulating a section of the River Niger and your `water_droplets` vector
can sometimes contain over `10,000` droplets.

Its currently an ice age, so these droplets don't move around very much and so your
data structure rarely changes.

Because delta encoding lists is `O(M * N)` time complexity where `M` and `N` are the lengths
of the two lists, you'd like to avoid delta encoding this data structure if possible.

```rust
use dipa::ChangeFlagged;

#[derive(DiffPatch)]
struct MyStruct {
    water_droplets: ChangeFlagged(WaterDroplet)
}

#[derive(DiffPatch)]
struct WaterDroplet([f32; 3]);
```

Now if `MyStruct.water_droplets.changed() == false` the underlying vectors will not be diffed.

_NOTE: ChangeFlagged has not been implemented yet. If you need it please open an issue._
