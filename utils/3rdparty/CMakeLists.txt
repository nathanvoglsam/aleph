cmake_minimum_required(VERSION 3.30)

##
## MODULES
##
include(FetchContent)

##
## PROJECT
##
project(AlephDeps LANGUAGES C CXX)

if(APPLE)
    enable_language(OBJC)
endif()

##
## DEPENDENCIES DECLARATIONS
##

## QuickJS-NG @ tag v0.17.0
FetchContent_Declare(
    quickjs
    GIT_REPOSITORY https://github.com/quickjs-ng/quickjs.git
    GIT_TAG        6d46d07d04041b40f4f49eaa7fdebe44c314c699
)

## SDL3 @ tag release-3.4.18
FetchContent_Declare(
    sdl3
    GIT_REPOSITORY https://github.com/libsdl-org/SDL.git
    GIT_TAG        829a65d769d935c4852f8159e964312c0957260a
)

## mimalloc @ tag v3.5.3
FetchContent_Declare(
    mimalloc
    GIT_REPOSITORY https://github.com/microsoft/mimalloc.git
    GIT_TAG        d4881d338125e1cb7c47ba4cfb398d6f7c0c8d45
)

## Tracy Profiler @ tag v0.14.1
FetchContent_Declare(
    tracy
    GIT_REPOSITORY https://github.com/wolfpld/tracy.git
    GIT_TAG        30997d5ca6bb632cc10807a1da8a6d3de0aeeb3c
    SOURCE_SUBDIR  profiler
)

## D3D12 Agility SDK
FetchContent_Declare(
    d3d12_agility_sdk
    URL https://www.nuget.org/api/v2/package/Microsoft.Direct3D.D3D12/1.618.5
)

## WinPixEventRuntime
FetchContent_Declare(
    win_pix_event_runtime
    URL https://www.nuget.org/api/v2/package/WinPixEventRuntime/1.0.240308001
)

## MoltenVK for iOS
FetchContent_Declare(
    moltenvk_ios
    URL https://github.com/KhronosGroup/MoltenVK/releases/download/v1.4.2/MoltenVK-ios.tar
)

##
## PULL DEPS
##

## QuickJS is built for all platforms
FetchContent_MakeAvailable(quickjs)

## mimalloc is built for all platforms
set(MI_BUILD_SHARED OFF)
set(MI_BUILD_TESTS OFF)
set(MI_OVERRIDE OFF)
set(MI_OPT_ARCH ON)
set(MI_INSTALL_TOPLEVEL ON)
FetchContent_MakeAvailable(mimalloc)

## We always grab this because cross compilation
FetchContent_MakeAvailable(moltenvk_ios)

## Windows and Mac build SDL3, Linux pulls from the system
if(WIN32 OR APPLE)
    set(SDL_STATIC ON)
    set(SDL_SHARED OFF)
    set(SDL_INSTALL ON)

    FetchContent_MakeAvailable(sdl3)
endif()

## For Tracy Profiler Windows uses published binaries, other platforms must build from source
if(NOT WIN32 AND NOT ALEPH_NO_TRACY)
    FetchContent_MakeAvailable(tracy)
endif()

## D3D12 stuff is obviously windows only
if (WIN32)
    FetchContent_MakeAvailable(d3d12_agility_sdk)
    FetchContent_MakeAvailable(win_pix_event_runtime)
endif()
