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

use aleph_rhi_api::{CommandListDesc, QueueSubmitDesc, QueueType};
use aleph_rhi_tests::swap_chain::acquire_next_image;
use aleph_rhi_tests::*;

fn main() {
    let options = HarnessOptions {
        swap_chain: Some(SwapChainRequest::default()),
        ..Default::default()
    };
    let harness = TestHarness::new(&options);

    let window = harness.window_state.as_ref().unwrap();
    let swap_chain = harness.swap_chain().unwrap();
    let config = swap_chain.get_config();

    if config.width == 0 || config.height == 0 {
        panic!("swap chain has zero size");
    }

    let device = harness.device.clone();
    let queue = device
        .get_queue(QueueType::General)
        .expect("No general queue available");

    let mut event_pump = window.platform.event_pump.borrow_mut();
    let mut needs_rebuild = false;
    for _ in 0..100 {
        if event_pump
            .poll_iter()
            .any(|event| matches!(event, sdl3::event::Event::Quit { .. }))
        {
            break;
        }

        device.garbage_collect().expect("Failed to garbage collect");

        let image = unsafe { acquire_next_image(swap_chain.as_ref(), &mut needs_rebuild) };
        let mut command_list = device
            .create_command_list(&CommandListDesc {
                queue_type: QueueType::General,
                name: None,
            })
            .expect("Failed to create command list");
        let encoder = command_list
            .begin_general()
            .expect("Failed to begin command list");
        unsafe {
            encoder.close().expect("Failed to close empty command list");
        }

        let mut command_lists = [Some(command_list)];
        unsafe {
            queue
                .submit(QueueSubmitDesc {
                    command_lists: &mut command_lists,
                    swap_image: Some(image.as_ref()),
                    ..Default::default()
                })
                .expect("Failed to submit empty command list");
        }
        drop(image);
    }
}
