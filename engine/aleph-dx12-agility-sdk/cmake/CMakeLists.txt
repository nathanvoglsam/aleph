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
        d3d12_agility_sdk
        URL https://www.nuget.org/api/v2/package/Microsoft.Direct3D.D3D12/1.618.5
)
FetchContent_MakeAvailable(d3d12_agility_sdk)