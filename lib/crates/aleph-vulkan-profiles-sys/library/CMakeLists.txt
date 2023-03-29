##
##
## This file is a part of NovaEngine
## https://gitlab.com/MindSpunk/NovaEngine
##
##
## MIT License
##
## Copyright (c) 2020 Nathan Voglsam
##
## Permission is hereby granted, free of charge, to any person obtaining a copy
## of this software and associated documentation files (the "Software"), to deal
## in the Software without restriction, including without limitation the rights
## to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
## copies of the Software, and to permit persons to whom the Software is
## furnished to do so, subject to the following conditions:
##
## The above copyright notice and this permission notice shall be included in all
## copies or substantial portions of the Software.
##
## THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
## IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
## FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
## AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
## LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
## OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
## SOFTWARE.
##


cmake_minimum_required(VERSION 3.6)

project(vulkan_profiles LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)

if (ALEPH_VULKAN_PROFILE_DEBUG)
    add_library(vulkan_profiles STATIC source/debug/vulkan_profiles.cpp source/debug_callback_slot.cpp)
    target_compile_definitions(vulkan_profiles PUBLIC VP_DEBUG_MESSAGE_CALLBACK=vpAlephDebugMessageTrampoline)
else()
    add_library(vulkan_profiles STATIC source/vulkan_profiles.cpp source/debug_callback_slot.cpp)
endif()

target_include_directories(vulkan_profiles PUBLIC "../../../../submodules/Vulkan-Headers/include")
target_include_directories(vulkan_profiles PUBLIC "include")
target_compile_definitions(vulkan_profiles PUBLIC VK_NO_PROTOTYPES)

install(
        TARGETS vulkan_profiles
        RUNTIME DESTINATION ./
        LIBRARY DESTINATION ./
        ARCHIVE DESTINATION ./
)
