$Env:ANDROID_NDK_HOME = "C:\Users\Nathan\Programs\Loose\android-ndk-r25c";
$Env:CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER = "$Env:ANDROID_NDK_HOME\toolchains\llvm\prebuilt\windows-x86_64\bin\aarch64-linux-android30-clang.cmd";
$Env:CARGO_TARGET_AARCH64_LINUX_ANDROID_AR = "$Env:ANDROID_NDK_HOME\toolchains\llvm\prebuilt\windows-x86_64\bin\aarch64-linux-android-ar";

cargo build --target=aarch64-linux-android --release
