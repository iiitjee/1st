# tldr

[![Clippy](https://github.com/FL03/tldr/actions/workflows/clippy.yml/badge.svg)](https://github.com/FL03/tldr/actions/workflows/clippy.yml)
[![Docker](https://github.com/FL03/tldr/actions/workflows/docker.yml/badge.svg)](https://github.com/FL03/tldr/actions/workflows/docker.yml)
[![Rust](https://github.com/FL03/tldr/actions/workflows/rust.yml/badge.svg)](https://github.com/FL03/tldr/actions/workflows/rust.yml)

***

Initially integrated with Telegram, this bot takes a given topic or URL and returns a quick summary; written in Rust.

## Getting Started

Make sure you have rust installed on your host device!

### Building from the source

Start by cloning the repository

```bash
git clone https://github.com/FL03/tldr
cd tldr
cargo build --release --workspace
```

## Usage

### Builder

```rust
cargo xtask
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

* [Apache-2.0](https://choosealicense.com/licenses/apache-2.0/)
* [MIT](https://choosealicense.com/licenses/mit/)
