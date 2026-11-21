# Compilation

To build the game executable, cargo is the primary build tool. Typical `cargo build` workflows can be used directly. The `cargo run` command can also be used.

## Cross compilation

Cross compilation workflows are more complex, and depend on the target platform.

### Android

The Android target does not use cargo (directly). Instead, an Android Studio project must be generated. Builds are driven by Android studio (and ultimately gradle). Cargo is invoked to build the Rust code, but in general cargo can not be invoked directly to build for Android.

Run the following to generate an Android Studio project
```
aleph genrpoj android
```

The project can be found under `local/android`.

### Windows

We support cross compiling for a Windows target via MinGW-w64 from a Linux host. See [this](./01-installation.md) page for details on what tools must be installed.


```
cargo build --target x86_64-pc-windows-gnu -p aleph-test
```

The Windows executable is written to `local/rs/bin/x86_64-pc-windows-gnu/debug/aleph-test.exe`.

The CLI also accepts the Windows platform on a Linux host:

```
aleph build --platform windows
```
