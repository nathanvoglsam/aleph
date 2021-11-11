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

project(vma LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)

add_library(vma STATIC vk_mem_alloc.cpp)

if (NOT MSVC)
    target_compile_options(vma PUBLIC "-Wno-missing-field-initializers")
    target_compile_options(vma PUBLIC "-Wno-unused-variable")
    target_compile_options(vma PUBLIC "-Wno-unused-parameter")
    target_compile_options(vma PUBLIC "-Wno-reorder")
    target_compile_options(vma PUBLIC "-Wno-type-limits")
    target_compile_options(vma PUBLIC "-Wno-class-memaccess")
endif()

if (FEATURE_VMA_DEBUG_DETECT_CORRUPTION)
target_compile_definitions(vma PUBLIC VMA_DEBUG_DETECT_CORRUPTION=1)
endif()

target_compile_definitions(vma PUBLIC VMA_STATIC_VULKAN_FUNCTIONS=0)

target_include_directories(vma PUBLIC "../../../../submodules/Vulkan-Headers/include")
target_include_directories(vma PUBLIC "../../../../submodules/VulkanMemoryAllocator/src")

install(
        TARGETS vma
        RUNTIME DESTINATION ./
        LIBRARY DESTINATION ./
        ARCHIVE DESTINATION ./
)