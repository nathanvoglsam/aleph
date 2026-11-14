cmake_minimum_required(VERSION 3.6)

project(${{ANDROID_GAME_NAME}})

include(FetchContent)

FetchContent_Declare(
    Corrosion
    GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
    GIT_TAG v0.6.1 # Optionally specify a commit hash, version tag or branch here
)
FetchContent_MakeAvailable(Corrosion)

# Import targets defined in a package or workspace manifest `Cargo.toml` file
corrosion_import_crate(
    MANIFEST_PATH ${{ALEPH_WORKSPACE_ROOT}}
    CRATE_TYPES cdylib
    CRATES ${{ANDROID_GAME_CRATE}}
)

## corrosion_install(TARGETS ${{ANDROID_GAME_TARGET}} EXPORT RustLibTargets)
