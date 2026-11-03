//
//
// This file is a part of Aleph
//
// https://github.com/nathanvoglsam/aleph
//
// MIT License
//
// Copyright (c) 2020 Aleph Engine
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

#define USE_PIX
#include "d3d12.h"
#include "pix3.h"

#include <stdint.h>

#if defined(_MSC_VER) || defined(__MINGW32__)
#define SHIM_API __declspec(dllexport)
#else
#define SHIM_API
#endif

extern "C" {
    SHIM_API void SHIM_PIXBeginEventOnThread(uint64_t color, const char* string) {
        PIXBeginEvent((UINT64)color, "%s", (PCSTR)string);
    }

    SHIM_API void SHIM_PIXSetMarkerOnThread(uint64_t color, const char* string) {
        PIXSetMarker((UINT64)color, "%s", (PCSTR)string);
    }

    SHIM_API void SHIM_PIXBeginEventDataOnThread(uint64_t color, const char* string, const char* data) {
        PIXBeginEvent((UINT64)color, "%s - %s", (PCSTR)string, (PCSTR)data);
    }

    SHIM_API void SHIM_PIXSetMarkerDataOnThread(uint64_t color, const char* string, const char* data) {
        PIXSetMarker((UINT64)color, "%s - %s", (PCSTR)string, (PCSTR)data);
    }

    SHIM_API void SHIM_PIXBeginEventOnCommandList(void* context, uint64_t color, const char* string) {
        PIXBeginEvent((ID3D12GraphicsCommandList*)context, (UINT64)color, "%s", (PCSTR)string);
    }

    SHIM_API void SHIM_PIXBeginEventOnCommandQueue(void* context, uint64_t color, const char* string) {
        PIXBeginEvent((ID3D12CommandQueue*)context, (UINT64)color, "%s", (PCSTR)string);
    }

    SHIM_API void SHIM_PIXBeginEventDataOnCommandList(void* context, uint64_t color, const char* string, const char* data) {
        PIXBeginEvent((ID3D12GraphicsCommandList*)context, (UINT64)color, "%s - %s", (PCSTR)string, (PCSTR)data);
    }

    SHIM_API void SHIM_PIXBeginEventDataOnCommandQueue(void* context, uint64_t color, const char* string, const char* data) {
        PIXBeginEvent((ID3D12CommandQueue*)context, (UINT64)color, "%s - %s", (PCSTR)string, (PCSTR)data);
    }

    SHIM_API void SHIM_PIXSetMarkerOnCommandList(void* context, uint64_t color, const char* string) {
        PIXSetMarker((ID3D12GraphicsCommandList*)context, (UINT64)color, "%s", (PCSTR)string);
    }

    SHIM_API void SHIM_PIXSetMarkerOnCommandQueue(void* context, uint64_t color, const char* string) {
        PIXSetMarker((ID3D12CommandQueue*)context, (UINT64)color, "%s", (PCSTR)string);
    }

    SHIM_API void SHIM_PIXSetMarkerDataOnCommandList(void* context, uint64_t color, const char* string, const char* data) {
        PIXSetMarker((ID3D12GraphicsCommandList*)context, (UINT64)color, "%s - %s", (PCSTR)string, (PCSTR)data);
    }

    SHIM_API void SHIM_PIXSetMarkerDataOnCommandQueue(void* context, uint64_t color, const char* string, const char* data) {
        PIXSetMarker((ID3D12CommandQueue*)context, (UINT64)color, "%s - %s", (PCSTR)string, (PCSTR)data);
    }

    SHIM_API void SHIM_PIXEndEventOnThread() {
        PIXEndEvent();
    }

    SHIM_API void SHIM_PIXEndEventOnCommandList(void* context) {
        PIXEndEvent((ID3D12GraphicsCommandList*)context);
    }

    SHIM_API void SHIM_PIXEndEventOnCommandQueue(void* context) {
        PIXEndEvent((ID3D12CommandQueue*)context);
    }
}
