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
FetchContent_MakeAvailable(quickjs)