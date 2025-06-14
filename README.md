# rsdiff

[![crates.io](https://img.shields.io/crates/v/rsdiff?style=for-the-badge&logo=rust)](https://crates.io/crates/rsdiff)
[![docs.rs](https://img.shields.io/docsrs/rsdiff?style=for-the-badge&logo=docs.rs)](https://docs.rs/rsdiff)
[![GitHub License](https://img.shields.io/github/license/FL03/rsdiff?style=for-the-badge&logo=github)](https://github.com/FL03/rsdiff/blob/main/LICENSE)

***

_**Warning: The library is currently in the early stages of development and is not yet ready for production use.**_

`rsdiff` is a Rust library enabling the automatic differentiation of functions using various techniques

## Background

Autodifferentiation is a powerful technique used in machine learning and optimization, allowing for efficient computation of gradients. The `rsdiff` library aims to provide a robust foundation for building autodifferentiation systems by leveraging hypergraphs to represent complex relationships between variables.

## Features

- [x] `hash_graph` - A hash-based hypergraph implementation.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies.rsdiff]
features = [
    "hash_graph",
    "macros",
]
version = "0.1.x"
```

### Examples

For more detailed examples, please refer to the [examples directory](https://github.com/FL03/rsdiff/blob/main/rsdiff/examples).

#### _Example #1:_ Basic Usage

```rust
    extern crate rsdiff;

    fn main() -> rsdiff::Result<()> {
        // initialize a new instance of a hypergraph
        let mut graph: HashGraph<usize, usize> = HashGraph::new();
        // use the macro to insert nodes into the graph
        rsdiff::hypernode! {
            graph {
                let v0;
                let v1 = 2;
                let v2 = 3;
                let v3 = 4;
            }
        }
        // Add some hyperedges
        let e1 = graph.insert_edge(vec![v0, v1, v2])?;
        println!("Added hyperedge {e1}: {:?}", [v0, v1, v2]);

        let e2 = graph.insert_edge(vec![v1, v2, v3])?;
        println!("Added hyperedge {e2}: {:?}", [v1, v2, v3]);

        // Get neighbors of vertex v1
        let neighbors = graph.neighbors(&v1)?;
        println!("Neighbors of {}: {:?}", v1, neighbors);

        // Get degree of vertex v1
        let degree = graph.get_degree_of_node(&v1);
        println!("Degree of {v1}: {degree}");

        // Remove a vertex
        graph.remove_vertex(&v2)?;
        println!("Removed vertex {v2}");

        println!("---------\nFinal graph state: {:?}", graph);
        Ok(())
    }

```

## Getting Started

### Prerequisites

Ensure you have the latest version of Rust installed. You can install Rust using [rustup](https://rustup.rs/).

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

After installation, I always recommend ensuring that rustup is updated to the latest version:

```bash
rustup update
```

And to add the latest nightly toolchain, which is often useful for development:

```bash
rustup toolchain install nightly
```

Additionally, you may wish to install the `cargo-binstall` utility to streamline the installation of Rust binaries:

```bash
cargo install cargo-binstall
```

If necessary, add the `wasm32-*` target(s) if you plan to compile for WebAssembly:

```bash
rustup target add wasm32-unknown-unknown wasm32-p1 wasm32-p2
```

### Building from the source

Start by cloning the repository

```bash
git clone https://github.com/FL03/rsdiff.git -b main --depth 1
```

Then, navigate to the project directory:

```bash
cd rsdiff
```

Once you're in the project directory, you can build the project using `cargo`:

```bash
cargo build --workspace --release --all-features
```

Or, if you want to run the tests, you can use:

```bash
cargo test --workspace --release --all-features
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

Please make sure to update tests as appropriate.
