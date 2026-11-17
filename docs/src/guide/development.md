# Development

For the most part, Aleph is a Rust project built around cargo. Cargo workflows are used to build and run the game. Our custom tooling integrates with cargo's dependency management to handle the compilation of shaders and config scripts.

## Basic Workflows

### Tests

We use cargo's test harness to run out tests. Simply use `cargo test` to run all tests. More advanced usage is the same as any other cargo project.

### Code Formatting

We use `rustfmt` to format the code. We use _nightly_ rustfmt options in our rustfmt.toml file. You must install the nightly toolchain, and use `cargo +nightly fmt` to format the code.


