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

# Dependencies

Aleph assumes a few things are installed into the host system by default. Those are

- A rust toolchain, installed via rustup.
- A `cmake` build tool, available in the path.
- The `ninja` build tool, available in the path.
- A `C/C++` toolchain, available in the path.
    - On Linux this will typically be installed via your package manager.
    - On MacOS this will typically installed as part of the Xcode command line tools.
    - On Windows we prefer the `msvc` family of build targets, which requires the `msvc` build tools from Microsoft. Typically these are installed from Visual Studio. `rustc` requires them for the linker, but we require the C/C++ toolchain too.
    - On Windows, it is possible to use the mingw-w64 project and compile using the `gnu` family of targets. This workflow is not covered here, on Windows at least.
- On Linux hosts, we require some dependencies to be installed on the host via the package manager for native builds.
    - On Fedora, those are `SDL3-devel`

# Extra Dependencies

Some workflows require extra dependencies to be installed on the host system. These workflows are outside the common case and are only needed in limited circumstances.

## Cross Compilation - Android

Android cross compilation requires Android Studio. Our Android build support relies on gradle, and Android Studio to make the Android SDK and Android NDK available to the build environment.

## Cross Compilation - Windows

We do support cross compiling for the `gnu` family of windows targets via mingw-w64 from a Linux host. We do not typically recommend using the `gnu` targets when running on a Windows host. There are useful workflows made available to Linux hosts via this build path however.

This workflow requires some additional tools:

- A mingw-w64 toolchain, typically installed from the system package manager.
    - On Ubuntu this is the `mingw-w64` package.
    - On Fedora this is covered by the `mingw64-gcc`, and `mingw64-gcc-c++` packages.
- The `x86_64-pc-windows-gnu` target must be installed for the active rust toolchain.
    - This can be installed using rustup with the `rustup target add x86_64-pc-windows-gnu` command

## Optional Tools - Linux

The tracy profiler is built from source for the host system. The build depends on some dependencies being available from the system package manager.

On Fedora, those are: `libxkbcommon-devel`, `wayland-devel`, `wayland-protocols-devel`, `libglvnd-devel`, and `dbus-devel`. 
