function build {
    $OLD_ANDROID_NDK_HOME = $Env:ANDROID_NDK_HOME;
    $Env:ANDROID_NDK_HOME = "C:\Users\Nathan\Programs\Loose\android-ndk-r25c";
    $Env:CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER = "$Env:ANDROID_NDK_HOME\toolchains\llvm\prebuilt\windows-x86_64\bin\clang";
    $Env:RUSTFLAGS = "-C link-args=--target=aarch64-linux-android30";

    cargo build --target=aarch64-linux-android --release --package aleph-test --lib

    $Env:ANDROID_NDK_HOME = $OLD_ANDROID_NDK_HOME;
    $Env:CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER = "";
    $Env:RUSTFLAGS = "";
}

build;
