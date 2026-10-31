cmake_minimum_required(VERSION 3.30)

##
## MODULES
##
include(FetchContent)

##
## PROJECT
##
project(AlephDeps LANGUAGES C CXX)

##
## DEPENDENCIES DECLARATIONS
##
FetchContent_Declare(
        win_pix_event_runtime
        URL https://www.nuget.org/api/v2/package/WinPixEventRuntime/1.0.240308001
)
FetchContent_MakeAvailable(win_pix_event_runtime)