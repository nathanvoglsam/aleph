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

set(MI_BUILD_SHARED OFF)
set(MI_BUILD_TESTS OFF)
set(MI_OVERRIDE OFF)
set(MI_OPT_ARCH ON)
set(MI_INSTALL_TOPLEVEL ON)

##
## DEPENDENCIES DECLARATIONS
##

## mimalloc @ tag v3.5.3
FetchContent_Declare(
    mimalloc
    GIT_REPOSITORY https://github.com/microsoft/mimalloc.git
    GIT_TAG        d4881d338125e1cb7c47ba4cfb398d6f7c0c8d45
)
FetchContent_MakeAvailable(mimalloc)