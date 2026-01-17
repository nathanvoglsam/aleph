cmake_minimum_required(VERSION 3.16)

project(AlephDeps)

add_subdirectory("quickjs")

if(WIN32 OR APPLE)
    set(SDL_STATIC OFF)
    set(SDL_SHARED ON)
    set(SDL_INSTALL ON)
    add_subdirectory("SDL")
endif()
