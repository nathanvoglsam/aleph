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

#include "windows.h"
#include "pix3.h"

#include <stdint.h>

extern "C" {
    void SHIM_PIXBeginEvent_NW(uint64_t color, const uint16_t* formatString) {
        PIXBeginEvent((UINT64)color, (PCWSTR)formatString);
    }

    void SHIM_PIXBeginEvent_NC(uint64_t color, const char* formatString) {
        PIXBeginEvent((UINT64)color, (PCSTR)formatString);
    }

    void SHIM_PIXSetMarker_NW(uint64_t color, const uint16_t* formatString) {
        PIXSetMarker((UINT64)color, (PCWSTR)formatString);
    }

    void SHIM_PIXSetMarker_NC(uint64_t color, const char* formatString) {
        PIXSetMarker((UINT64)color, (PCSTR)formatString);
    }

    void SHIM_PIXBeginEvent_CW(void* context, uint64_t color, const uint16_t* formatString) {
        PIXBeginEvent(context, (UINT64)color, (PCWSTR)formatString);
    }

    void SHIM_PIXBeginEvent_CC(void* context, uint64_t color, const char* formatString) {
        PIXBeginEvent(context, (UINT64)color, (PCSTR)formatString);
    }

    void SHIM_PIXSetMarker_CW(void* context, uint64_t color, const uint16_t* formatString) {
        PIXSetMarker(context, (UINT64)color, (PCWSTR)formatString);
    }

    void SHIM_PIXSetMarker_CC(void* context, uint64_t color, const char* formatString) {
        PIXSetMarker(context, (UINT64)color, (PCSTR)formatString);
    }

    void SHIM_PIXEndEvent_N() {
        PIXEndEvent();
    }

    void SHIM_PIXEndEvent_C(void* context) {
        PIXEndEvent(context);
    }
}