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

## Tracy Profiler @ tag v0.14.1
FetchContent_Declare(
    tracy
    GIT_REPOSITORY https://github.com/wolfpld/tracy.git
    GIT_TAG        30997d5ca6bb632cc10807a1da8a6d3de0aeeb3c
    SOURCE_SUBDIR  profiler
)
FetchContent_MakeAvailable(tracy)
