cmake_minimum_required(VERSION 3.16)

project(AlephDeps)

add_subdirectory("quickjs")

if(WIN32 OR APPLE)
    set(SDL_STATIC ON)
    set(SDL_SHARED OFF)
    set(SDL_INSTALL ON)
    add_subdirectory("SDL")
endif()

if(NOT (WIN32 OR APPLE))
    add_subdirectory("tracy/profiler")
endif()
