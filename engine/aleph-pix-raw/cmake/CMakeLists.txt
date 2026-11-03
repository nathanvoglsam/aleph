cmake_minimum_required(VERSION 3.30)

##
## PROJECT
##
project(AlephDeps LANGUAGES C CXX)

##
## MODULES
##
include(FetchContent)
include(GNUInstallDirs)

##
## DEPENDENCIES DECLARATIONS
##
FetchContent_Declare(
        win_pix_event_runtime
        URL https://www.nuget.org/api/v2/package/WinPixEventRuntime/1.0.240308001
)
FetchContent_MakeAvailable(win_pix_event_runtime)

##
## TARGETS
##
# The WinPixEventRuntime package ships binaries in a per-architecture subfolder of bin/
string(TOLOWER "${CMAKE_CXX_COMPILER_ARCHITECTURE_ID}" PIX_TARGET_ARCH_ID)
if (PIX_TARGET_ARCH_ID STREQUAL "x64" OR PIX_TARGET_ARCH_ID STREQUAL "x86_64")
    set(PIX_BIN_ARCH_DIR "x64")
elseif (PIX_TARGET_ARCH_ID STREQUAL "arm64" OR PIX_TARGET_ARCH_ID STREQUAL "aarch64")
    set(PIX_BIN_ARCH_DIR "ARM64")
else ()
    message(FATAL_ERROR "Unsupported target architecture for WinPixEventRuntime: '${CMAKE_CXX_COMPILER_ARCHITECTURE_ID}'")
endif ()

add_library(aleph_pix_shim SHARED shim.cpp)

# shim.cpp includes "pix3.h" directly, so the WinPixEventRuntime header directory itself must be on the include path
target_include_directories(aleph_pix_shim PRIVATE
        "${win_pix_event_runtime_SOURCE_DIR}/Include/WinPixEventRuntime"
)

# The import library for WinPixEventRuntime.dll matching the target architecture
target_link_libraries(aleph_pix_shim PRIVATE
        "${win_pix_event_runtime_SOURCE_DIR}/bin/${PIX_BIN_ARCH_DIR}/WinPixEventRuntime.lib"
)

##
## INSTALL
##
install(TARGETS aleph_pix_shim
        RUNTIME DESTINATION "${CMAKE_INSTALL_BINDIR}"
        ARCHIVE DESTINATION "${CMAKE_INSTALL_LIBDIR}"
)