cmake_minimum_required(VERSION 3.6)

project(${{ANDROID_GAME_NAME}} C CXX)

include(FetchContent)

set(CORROSION_CARGO_TARGET_DIR "${{ALEPH_WORKSPACE_ROOT}}/build/andrs" CACHE PATH "Cargo target directory" FORCE)

FetchContent_Declare(
    Corrosion
    GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
    GIT_TAG 8babc44f331941d76ab93a238377cfdac43a23ec
)
FetchContent_MakeAvailable(Corrosion)

set(SDL_STATIC OFF)
set(SDL_SHARED ON)

##
## DEPENDENCIES DECLARATIONS
##

## SDL3 @ tag release-3.4.18
FetchContent_Declare(
    sdl3
    GIT_REPOSITORY https://github.com/libsdl-org/SDL.git
    GIT_TAG        829a65d769d935c4852f8159e964312c0957260a
)
FetchContent_MakeAvailable(sdl3)

# Import targets defined in a package or workspace manifest `Cargo.toml` file
corrosion_import_crate(
    MANIFEST_PATH ${{ALEPH_WORKSPACE_ROOT}}/Cargo.toml
    CRATE_TYPES cdylib
    CRATES ${{ANDROID_GAME_CRATE}}
)

if (DEFINED ANDROID_NDK_HOME)
    corrosion_set_env_vars(aleph_test_lib ANDROID_NDK_HOME=${ANDROID_NDK_HOME})
endif()
if (DEFINED ANDROID_NDK_ROOT)
    corrosion_set_env_vars(aleph_test_lib ANDROID_NDK_ROOT=${ANDROID_NDK_ROOT})
endif()
if (DEFINED ANDROID_NDK)
    corrosion_set_env_vars(aleph_test_lib ANDROID_NDK=${ANDROID_NDK})
endif()
if (DEFINED CARGO_NDK_ANDROID_PLATFORM)
    corrosion_set_env_vars(aleph_test_lib CARGO_NDK_ANDROID_PLATFORM=${CARGO_NDK_ANDROID_PLATFORM})
endif()
if (DEFINED ANDROID_PLATFORM)
    corrosion_set_env_vars(aleph_test_lib ANDROID_PLATFORM=${ANDROID_PLATFORM})
endif()
if (DEFINED ANDROID_STL)
    corrosion_set_env_vars(aleph_test_lib ANDROID_STL=${ANDROID_STL})
endif()

corrosion_link_libraries(aleph_test_lib SDL3::SDL3)

add_library(main SHARED
    main.cpp
)
target_link_libraries(main PRIVATE SDL3::SDL3 aleph_test_lib)
