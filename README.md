# bevy_oneshot

[![Crates.io](https://img.shields.io/crates/v/bevy_oheshot)](https://crates.io/crates/bevy_oneshot)
![Crates.io](https://img.shields.io/crates/l/bevy_oneshot)
![docs.rs](https://img.shields.io/docsrs/bevy_oneshot)

Naive implementation of one-shot systems for Bevy

## Why?

The library is made mainly for the convenience of working with the GUI. Remember that feeling when you created a system that creates GUI with a couple of input parameters? Nothing foretold trouble, but then the first button appears in the interface, the processing of which adds a few arguments to the system. The second button needed a resource, the third needed a query. After the fourth button you get a clippy::too_many_arguments warn... Pretty frustrating, huh?

With bevy_oneshot you can put all the arguments a particular button needs into a separate system, or even not write a system at all, by making a lambda, which greatly reduces the amount of unnecessary stuff in the system scope.

I really hope oneshot systems will be introduced in Bevy soon and this crate will become irrelevant.

## Installation

```bash
cargo add bevy_oneshot
```

or add this to your `Cargo.toml`

```toml
[dependencies]
bevy_oheshot = "0.1"
```

## Usage

```rust
use bevy::prelude::*;
use bevy_oneshot::CommandRunOnce;

fn some_system(mut commands: Commands) {
    // run once without arguments
    commands.run_once(|_query: Query<&Window>| {
        // your code here
    });

    // run once with input arguments
    commands.run_once_with(
        |input: In<u32>| {
            println!("{}", input.0);
        },
        42,
    );
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(some_system)
        .run();
}
```

## Contributing

PRs are very welcome.
