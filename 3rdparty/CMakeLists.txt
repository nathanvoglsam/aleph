cmake_minimum_required(VERSION 4.1)

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

## QuickJS-NG
FetchContent_Declare(
    quickjs
    GIT_REPOSITORY https://github.com/nathanvoglsam/quickjs.git
    GIT_TAG        1beea86df0b3cf4b16d4a411fa00ab5c6b267a58
)

## SDL3
FetchContent_Declare(
        sdl3
        GIT_REPOSITORY https://github.com/libsdl-org/SDL.git
        GIT_TAG        release-3.4.0
)

## Tracy Profiler
FetchContent_Declare(
        tracy
        GIT_REPOSITORY https://github.com/wolfpld/tracy.git
        GIT_TAG        v0.13.1
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

##
## PULL DEPS
##

## QuickJS is built for all platforms
FetchContent_MakeAvailable(quickjs)

## Windows and Mac build SDL3, Linux pulls from the system
if(WIN32 OR APPLE)
    set(SDL_STATIC ON)
    set(SDL_SHARED OFF)
    set(SDL_INSTALL ON)

    FetchContent_MakeAvailable(sdl3)
endif()

## For Tracy Profiler Windows uses published binaries, other platforms must build from source
if(NOT WIN32)
    FetchContent_MakeAvailable(tracy)
endif()

## D3D12 stuff is obviously windows only
if (WIN32)
    FetchContent_MakeAvailable(d3d12_agility_sdk)
    FetchContent_MakeAvailable(win_pix_event_runtime)
endif()
