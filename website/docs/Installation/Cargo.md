# Cargo

**Chess-tui** can be installed with cargo, the Rust package manager.


## Installation

```bash
cargo install chess-tui
```

**Then run the game with:**
```bash
chess-tui
```

The package is available on [crates.io](https://crates.io/crates/chess-tui) :tada:

## Installing without sound features

If you want to install **chess-tui** without sound support (useful for environments without audio support), you can disable the sound feature:

```bash
cargo install chess-tui --no-default-features
```

:::note
By default, **chess-tui** includes sound features. Use `--no-default-features` to install without sound support.
:::