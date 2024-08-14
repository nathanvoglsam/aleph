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

#[cfg(any(target_vendor = "uwp", target_os = "ios"))]
mod common {
    use std::ffi::{c_char, c_int};

    use parking_lot::Mutex;

    pub type MainFn = unsafe extern "C" fn(c_int, *const *const c_char) -> c_int;

    #[derive(Copy, Clone)]
    pub enum PayloadState {
        /// The payload has not yet been written. Reading in this state is invalid.
        None,

        /// The payload has been written and is valid to read from exactly once.
        Present(PayloadHack),

        /// The payload has been read from and is now invalid for both reads and writes.
        Used,
    }

    impl PayloadState {
        pub fn write(&mut self, payload: PayloadHack) {
            assert!(
                matches!(*self, PayloadState::None),
                "Trying to init platform twice"
            );
            *self = Self::Present(payload);
        }

        pub fn read(&mut self) -> PayloadHack {
            match *self {
                Self::None => panic!("How the fuck did you get here?"),
                Self::Present(p) => {
                    *self = Self::Used;
                    p
                }
                Self::Used => {
                    panic!("Trying to init platform twice");
                }
            }
        }
    }

    /// A disgusting hack to work around the restriction that static types must be 'Sync'. We
    /// guarantee we don't do any Send or Sync stuff (all on the main thread) in our implementation.
    #[derive(Copy, Clone)]
    pub struct PayloadHack(pub *mut (dyn FnOnce() + 'static));
    unsafe impl Send for PayloadHack {}
    unsafe impl Sync for PayloadHack {}

    impl PayloadHack {
        pub fn from_continuation(continuation: impl FnOnce()) -> Self {
            let payload: Box<dyn FnOnce()> = Box::new(continuation);
            let payload = Box::leak(payload) as *mut dyn FnOnce();
            PayloadHack(payload as *mut _)
        }
    }

    static MAIN_PAYLOAD: Mutex<PayloadState> = Mutex::new(PayloadState::None);

    pub fn send_payload(p: PayloadHack) {
        let mut lock = MAIN_PAYLOAD.lock();
        lock.write(p);
    }

    fn recv_payload() -> PayloadHack {
        let mut lock = MAIN_PAYLOAD.lock();
        lock.read()
    }

    pub unsafe extern "C" fn main_wrapper(_argc: c_int, _argv: *const *const c_char) -> c_int {
        let payload = recv_payload();
        let payload = Box::from_raw(payload.0);
        payload();

        return 0;
    }
}

#[cfg(target_vendor = "uwp")]
mod uwp {
    use crate::sdl_main_wrapper::common::{main_wrapper, send_payload, MainFn, PayloadHack};
    use std::ffi::{c_int, c_void};

    extern "C" {
        fn SDL_WinRTRunApp(main: MainFn, reserved: *const c_void) -> c_int;
    }

    pub fn intercept_main(continuation: impl FnOnce()) {
        let payload = PayloadHack::from_continuation(continuation);
        send_payload(payload);

        // Safety: We have no choice but to trust this function
        unsafe {
            let _exit_code = SDL_WinRTRunApp(main_wrapper, std::ptr::null());
        }
    }
}

#[cfg(target_vendor = "uwp")]
pub use uwp::intercept_main;

#[cfg(target_os = "ios")]
mod ios {
    use crate::sdl_main_wrapper::common::{main_wrapper, send_payload, MainFn, PayloadHack};
    use std::ffi::{c_char, c_int};

    extern "C" {
        fn SDL_UIKitRunApp(argc: c_int, argv: *const *const c_char, main: MainFn) -> c_int;
    }

    pub fn intercept_main(continuation: impl FnOnce()) {
        let payload = PayloadHack::from_continuation(continuation);
        send_payload(payload);

        // Cook up some dummy arguments to pass in. We never use these but SDL would probably get a
        // little sad if we gave it argc = 0 and a nullptr here.
        let dummy_arg_0 = "DeadBeef\0".as_ptr() as *const c_char;
        let dummy_args = [dummy_arg_0, std::ptr::null()];

        // Safety: We have no choice but to trust this function
        unsafe {
            let _exit_code = SDL_UIKitRunApp(1, dummy_args.as_ptr(), main_wrapper);
        }
    }
}

#[cfg(target_os = "ios")]
pub use ios::intercept_main;

#[cfg(not(any(target_vendor = "uwp", target_os = "ios")))]
mod pc {
    pub fn intercept_main(continuation: impl FnOnce()) {
        continuation()
    }
}

#[cfg(not(any(target_vendor = "uwp", target_os = "ios")))]
pub use pc::intercept_main;
