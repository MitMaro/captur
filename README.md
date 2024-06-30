[![Crates.io](https://img.shields.io/crates/v/captur.svg)](https://crates.io/crates/captur)
[![docs](https://docs.rs/captur/badge.svg)](https://docs.rs/captur/)
[![GitHub license](https://img.shields.io/github/license/MitMaro/captur)](https://raw.githubusercontent.com/MitMaro/captur/master/LICENSE)

# Captur

Starting in Rust 2021, Rust will no longer capture whole structs and instead will only capture a disjoint set of the fields used in a closure. In some cases, it is necessary to capture the structs to retain a particular drop order. This macro will capture the struct within the closure, ensuring the correct drop order.

## The Fix

The typical fix to this problem is to create an unused reference to the struct.

```rust
let some_struct = SomeStruct::new();
let result = || {
    // capture some_struct within the closure
    let _ = &some_struct;
    println!("{}", some_struct.y);
}
```

While this is trivial to implement in closures where capturing is required, without a comment, the meaning of the unused line is difficult to determine. This macro provides a self documenting and potentially more concise way to capture the structs.

## Installation and Usage

```toml
[dependencies]
captur = "1"
```

```rust
use captur::capture;

fn send_event_and_action(action: &Action, event: Event) {
    send(|sender| {
        capture!(action, event);
        sender.send(action.name.as_str(), event.code);
    });
}
```

# Supported Rust Versions

This project will support all Rust versions since 1.51 when Rust first supported Rust 2021.

Dropping support for a Rust version will result in a major version bump, following [Semantic Versioning](https://semver.org/).

## License

Captur is released under the ISC license. See [LICENSE](LICENSE).
