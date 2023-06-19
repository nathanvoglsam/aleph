function build {
    $OLD_ANDROID_NDK_HOME = $Env:ANDROID_NDK_HOME;
    $Env:ANDROID_NDK_HOME = "C:\Users\Nathan\Programs\Loose\android-ndk-r25c";

    $Env:ALEPH_BUILD_TYPE="Retail";
    cargo ndk -t arm64-v8a -p 30 build --profile=retail --target=aarch64-linux-android --package aleph-test --lib

    $Env:ANDROID_NDK_HOME = $OLD_ANDROID_NDK_HOME;
    $Env:ALEPH_BUILD_TYPE="";
}

build;
