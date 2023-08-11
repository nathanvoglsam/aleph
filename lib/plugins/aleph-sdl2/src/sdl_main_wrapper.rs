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

#[cfg(target_vendor = "uwp")]
mod uwp {
    use once_cell::sync::OnceCell;
    use std::ffi::c_void;
    use std::os::raw::{c_char, c_int};
    use std::ptr::NonNull;
    use windows::Win32::System::Threading::{
        ConvertFiberToThread, ConvertThreadToFiberEx, CreateFiberEx, DeleteFiber, SwitchToFiber,
    };

    type RTMain = unsafe extern "C" fn(c_int, *const *const c_char) -> c_int;

    extern "C" {
        fn SDL_WinRTRunApp(main: RTMain, reserved: *const c_void) -> c_int;
    }

    struct FiberPayload {
        fiber: NonNull<c_void>,
    }

    unsafe impl Send for FiberPayload {}
    unsafe impl Sync for FiberPayload {}

    static MAIN_PAYLOAD: OnceCell<FiberPayload> = OnceCell::new();
    static WINRT_PAYLOAD: OnceCell<FiberPayload> = OnceCell::new();

    unsafe extern "C" fn main_wrapper(
        _argc: std::os::raw::c_int,
        _argv: *const *const std::os::raw::c_char,
    ) -> std::os::raw::c_int {
        let winrt_fiber = ConvertThreadToFiberEx(None, 0);
        let winrt_fiber = NonNull::new(winrt_fiber).expect("Failed to convert thread to fiber");
        let payload = FiberPayload { fiber: winrt_fiber };
        if WINRT_PAYLOAD.set(payload).is_err() {
            panic!("Trying to init platform twice");
        }

        // Get our payload, panic if it isn't there
        let main_payload = MAIN_PAYLOAD.get().unwrap();

        // Now we've run the uwp init we can switch back to the original fiber
        SwitchToFiber(main_payload.fiber.as_ptr());

        // Destroy this function's fiber
        if ConvertFiberToThread() == false {
            panic!("Failed to convert fiber to thread");
        }

        // Return our exit code
        return 0;
    }

    extern "system" fn fiber_proc(_lp_parameter: *mut c_void) {
        unsafe {
            SDL_WinRTRunApp(main_wrapper, std::ptr::null());

            // Get our payload, panic if it isn't there
            let payload = MAIN_PAYLOAD.get().unwrap();

            // Switch back again as we've escaped SDL_WinRTRunApp
            SwitchToFiber(payload.fiber.as_ptr());
        }
    }

    pub struct MainCtx {
        sdl_fiber: NonNull<c_void>,
    }

    pub unsafe fn run_sdl_main() -> MainCtx {
        // Convert to fiber so we can jump back here from within the main function
        let main_fiber = ConvertThreadToFiberEx(None, 0);
        let main_fiber = NonNull::new(main_fiber).expect("Failed to convert thread to fiber");

        // Push into our global payload a pointer to the fiber to return to once SDL_WinRTRunApp
        // returns control to us
        let payload = FiberPayload { fiber: main_fiber };
        if MAIN_PAYLOAD.set(payload).is_err() {
            panic!("Trying to init platform twice");
        }

        // Create a new fiber which will drive our SDL_WinRTRunApp wrapper
        let sdl_fiber = CreateFiberEx(0, 0, 0, Some(fiber_proc), None);
        let sdl_fiber = NonNull::new(sdl_fiber).expect("Failed to crate main fiber");

        // Switch to the created fiber to run the SDL_WinRTRunApp to initialize required
        // resources. Once it hands control back to us with a callback we switch back to this
        // fiber.
        //
        // To destroy the resources later we jump back into the SDL_WinRTRunApp and then once
        // it completes it will return back to the main fiber again
        SwitchToFiber(sdl_fiber.as_ptr());

        MainCtx { sdl_fiber }
    }

    pub unsafe fn run_sdl_exit(ctx: &MainCtx) {
        let winrt = WINRT_PAYLOAD.get().unwrap();
        // Now we need to destroy the resources SDL_WinRTRunApp created. To do so we jump back
        // into the sdl fiber so it can unwind its stack. Then it will jump back here again and
        // we can continue
        SwitchToFiber(winrt.fiber.as_ptr());

        // We no longer need the other fiber so we destroy it.
        DeleteFiber(ctx.sdl_fiber.as_ptr());

        // Undo ConvertThreadToFiberEx
        if ConvertFiberToThread() == false {
            panic!("Failed to convert fiber to thread");
        }
    }
}

#[cfg(target_vendor = "uwp")]
pub use uwp::{run_sdl_exit, run_sdl_main, MainCtx};

#[cfg(not(target_vendor = "uwp"))]
mod pc {
    pub struct MainCtx();

    pub unsafe fn run_sdl_main() -> MainCtx {
        MainCtx()
    }
    pub unsafe fn run_sdl_exit(_ctx: &MainCtx) {}
}

#[cfg(not(target_vendor = "uwp"))]
pub use pc::{run_sdl_exit, run_sdl_main, MainCtx};
