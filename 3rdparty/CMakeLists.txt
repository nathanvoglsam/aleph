cmake_minimum_required(VERSION 3.16)

project(AlephDeps)

add_subdirectory("quickjs")

if(WIN32 OR APPLE)
    if (APPLE)
        set(SDL_STATIC ON)
        set(SDL_SHARED OFF)
    endif()

    if (WIN32)
        set(SDL_STATIC OFF)
        set(SDL_SHARED ON)
    endif()

    set(SDL_INSTALL ON)
    add_subdirectory("SDL")
endif()
