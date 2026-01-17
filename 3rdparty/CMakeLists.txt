cmake_minimum_required(VERSION 3.16)

project(AlephDeps)

set(SDL_STATIC OFF)
set(SDL_SHARED ON)
set(SDL_INSTALL ON)

add_subdirectory("quickjs")
add_subdirectory("SDL")
