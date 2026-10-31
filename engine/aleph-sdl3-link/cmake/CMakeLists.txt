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

set(SDL_STATIC ON)
set(SDL_SHARED OFF)
set(SDL_INSTALL ON)

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