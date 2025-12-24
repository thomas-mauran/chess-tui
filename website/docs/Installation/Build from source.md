# Build from source

**Chess-tui** can be directly built from the source code. 

:::warning Make sure you have [Rust](https://www.rust-lang.org/tools/install) installed on your machine.
:::

## Installation

```bash
git clone https://github.com/thomas-mauran/chess-tui
cd chess-tui
cargo build --release

./target/release/chess-tui
```

## Building without sound features

If you want to build **chess-tui** without sound support (useful for environments without audio support, like Docker containers or headless servers), you can disable the sound feature:

```bash
git clone https://github.com/thomas-mauran/chess-tui
cd chess-tui
cargo build --no-default-features

./target/debug/chess-tui
```

:::note
By default, **chess-tui** includes sound features. Use `--no-default-features` to build without sound support.
:::
