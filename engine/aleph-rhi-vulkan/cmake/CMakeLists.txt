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
FetchContent_Declare(
        moltenvk_ios
        URL https://github.com/KhronosGroup/MoltenVK/releases/download/v1.4.2/MoltenVK-ios.tar
)
FetchContent_MakeAvailable(moltenvk_ios)