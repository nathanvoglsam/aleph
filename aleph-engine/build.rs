//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

extern crate aleph_compile as compile;
extern crate aleph_target as target;

use cmake;
use target::{Architecture, Platform};

///
/// Gets the name of the dll/so file that will need to be copied around
///
fn dll_name() -> &'static str {
    match target::build::target_platform() {
        Platform::WindowsGNU | Platform::WindowsMSVC => "SDL2.dll",
        Platform::Linux | Platform::Android => "libSDL2.so",
    }
}

///
/// Gets the name of the ndk-build driver
///
fn get_ndk_build_file() -> String {
    let ndk_build = std::env::var("ANDROID_HOME").unwrap();
    match target::build::host_platform() {
        Platform::WindowsGNU | Platform::WindowsMSVC => {
            format!("{}\\ndk-bundle\\ndk-build.cmd", &ndk_build)
        }
        Platform::Linux => format!("{}/ndk-bundle/ndk-build", &ndk_build),
        Platform::Android => panic!("Unsupported host"),
    }
}

///
/// Driver for compiling SDL2 for android, handles all the pain of dealing with ndk-build and
/// the different architectures
///
fn android_compile_sdl2(arch: Architecture) {
    let ndk_build_dir = get_ndk_build_file();
    let mut ndk_build = std::process::Command::new(&ndk_build_dir);

    let out_dir = compile::artifacts_dir();

    let mut obj_dir = out_dir.clone();
    obj_dir.push("obj");
    let obj_dir = format!("NDK_OUT={}", obj_dir.display());

    let mut lib_dir = out_dir.clone();
    lib_dir.push("lib");
    let lib_dir = format!("NDK_LIBS_OUT={}", lib_dir.display());

    ndk_build.arg("NDK_PROJECT_PATH=null");
    ndk_build.arg("APP_BUILD_SCRIPT=Android.mk");
    ndk_build.arg("APP_PLATFORM=android-24");
    ndk_build.arg("APP_STL=c++_shared");
    ndk_build.arg("APP_MODULES=SDL2 SDL2_main");
    ndk_build.arg(&obj_dir);
    ndk_build.arg(&lib_dir);

    let abi = format!("APP_ABI={}", arch.ndk_name());
    ndk_build.arg(&abi);

    ndk_build.current_dir("./thirdparty/sdl2");

    ndk_build.stdout(std::process::Stdio::inherit());
    ndk_build.stderr(std::process::Stdio::inherit());

    println!("ndk-build: {}", &ndk_build_dir);
    let exit_status = ndk_build
        .spawn()
        .expect("Failed to start ndk-build")
        .wait()
        .expect("ndk-build failed unexpectedly");

    if !exit_status.success() {
        panic!("ndk-build failed");
    }
}

///
/// Main driver for compiling SDL2, handles switching between w/e implementation is needed for the
/// target platforms.
///
fn main() {
    match target::build::target_platform() {
        Platform::WindowsGNU | Platform::WindowsMSVC => {
            // If we're building for windows we need to compile SDL2 ourselves. We already have the
            // source for android builds so we may as well build for windows as well rather than
            // try to bundle binaries

            // Perform the cmake build with the ninja generator, ninja will need to be in the system
            // path for this to work
            let mut build = cmake::Config::new("thirdparty/sdl2");
            build.generator("Ninja");

            // When compiling for MSVC we need to include vcruntime as some symbols are missing
            // if we don't link in this lib
            if target::build::target_platform().is_msvc() {
                build.define("EXTRA_LIBS", "vcruntime");
            }

            // Having this enabled is going to cause problems for linking so get rid of it
            build.define("SDL_CMAKE_DEBUG_POSTFIX", "");

            let out_dir = build.build();

            // We're going to need the output lib and bin dir
            let lib_dir = out_dir.join("lib");
            let bin_dir = out_dir.join("bin");

            // Give rustc the directory of where to find the lib files to link to
            println!("cargo:rustc-link-search=all={}", &lib_dir.display());

            // Give rustc the directory of where to find the lib files to link to
            println!("cargo:rustc-link-search=all={}", &bin_dir.display());

            // Copy the output dll file to the artifacts dir
            let source = bin_dir.join(dll_name());
            compile::copy_file_to_artifacts_dir(&source)
                .expect("Failed to copy SDL2 dll/so to artifacts dir");

            // Copy the output dll file to the target dir
            compile::copy_file_to_target_dir(&source)
                .expect("Failed to copy SDL2 dll/so to target dir");
        }
        Platform::Linux => {
            // Nothing has to be done on linux as the most sane choice is to use the system provided
            // SDL2 lest we wake the horrible demons of distributing your own libraries on linux.
            // If it's in the distro repository, get it from there as it will probably play much
            // nicer than compiling our own.
        }
        Platform::Android => {
            // On android we need to compile with ndk-build so it will play nicely with all the
            // wacky things android has done for building.

            // Driver function for doing the android compile
            android_compile_sdl2(target::build::target_architecture());

            // Get the location of the link files and add it to rustc's search path
            let mut lib_dir = compile::artifacts_dir();
            lib_dir.push("obj");
            lib_dir.push("local");
            lib_dir.push(target::build::target_architecture().ndk_name());
            println!("cargo:rustc-link-search=all={}", lib_dir.display());
        }
    }
}
