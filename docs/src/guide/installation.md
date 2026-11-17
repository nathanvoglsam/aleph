# Installation

An Aleph game engine project lives in a single directory. It contains a few key components:

1. A `Cargo.toml` file, that describes the _cargo workspace_ that the project lives in.
2. The `aleph-project.toml` file, that describes the project with information that is consumed by aleph tooling for building, running and distributing the project.
3. An `engine` directory, that contains the complete Rust source code for the game engine. 'aleph' does not publish crates to cargo.
4. A `game` directory, that contains data and code for the game itself. The most important component found here will be the root 'game' crate, which represents the executable target of the game.
5. A `.aleph/sdk` directory, that contains a suite of binary tools used by aleph tooling for building, running and distributing the project.

## Getting Started

Starting on an existing project is easy, simply clone the project's source code repository and then follow a few basic steps to bootstrap the workspace.

1. Clone the project's source code repository into a new directory.
2. Download the appropriate SDK binaries for your platform, and extract the archive into the `.aleph/sdk` directory.
3. In a terminal, under the project directory, run the `cargo install --path ./engine/aleph` command to install the 'aleph' tool for that workspace. This will compile and build the tool from the source code directly in the workspace.
4. Run `aleph genvscode` to generate a vscode workspace configuration file.
5. Run `aleph genconfigs` to compile the typescript config scripts.
6. Run `aleph shaders genproj` and `aleph shaders build` to compile the shaders.
7. Build the game using `cargo build`.
8. Run via `cargo`
