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

use std::ffi::{c_void, CStr};
use std::slice;

use ash::vk;
use log::{log, Level};

fn message_severity_log_level(severity: vk::DebugUtilsMessageSeverityFlagsEXT) -> Level {
    if severity == vk::DebugUtilsMessageSeverityFlagsEXT::INFO {
        Level::Debug
    } else if severity == vk::DebugUtilsMessageSeverityFlagsEXT::WARNING {
        Level::Warn
    } else if severity == vk::DebugUtilsMessageSeverityFlagsEXT::ERROR {
        Level::Error
    } else if severity == vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE {
        Level::Trace
    } else {
        Level::Debug
    }
}

fn message_type_colour(mtype: vk::DebugUtilsMessageTypeFlagsEXT) -> console::Color {
    if mtype == vk::DebugUtilsMessageTypeFlagsEXT::GENERAL {
        console::Color::Green
    } else if mtype == vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION {
        console::Color::Red
    } else if mtype == vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE {
        console::Color::Yellow
    } else {
        console::Color::Black
    }
}

fn message_type_string(mtype: vk::DebugUtilsMessageTypeFlagsEXT) -> &'static str {
    if mtype == vk::DebugUtilsMessageTypeFlagsEXT::GENERAL {
        "GENERAL"
    } else if mtype == vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION {
        "VALIDATION"
    } else if mtype == vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE {
        "PERFORMANCE"
    } else {
        "NONE"
    }
}

unsafe fn print_message(callback_data: &vk::DebugUtilsMessengerCallbackDataEXT, level: Level) {
    let message = CStr::from_ptr(callback_data.p_message).to_str().unwrap();
    let message = console::style(message).italic();

    let message_header = console::style("Message").cyan().bold();
    log!(level, "================{}=================", message_header);
    log!(level, "{}", message);
}

unsafe fn print_call_stack(callback_data: &vk::DebugUtilsMessengerCallbackDataEXT, level: Level) {
    let queue_labels = slice::from_raw_parts(
        callback_data.p_queue_labels,
        callback_data.queue_label_count as usize,
    );
    let cmd_labels = slice::from_raw_parts(
        callback_data.p_cmd_buf_labels,
        callback_data.cmd_buf_label_count as usize,
    );

    // If we have a call stack to print
    if !cmd_labels.is_empty() || !queue_labels.is_empty() {
        // Current indentation level
        let mut indent = 0;
        // String buffer
        let mut label_stack = String::with_capacity(1024);

        // Default text
        label_stack.push_str("QUEUE:\n");

        for queue_label in queue_labels.iter() {
            // Push indent
            for _ in 0..indent {
                label_stack.push(' ');
            }

            // Push name
            let name = CStr::from_ptr(queue_label.p_label_name);
            let name = name.to_str().unwrap();
            label_stack.push_str(name);
            label_stack.push('\n');

            // Increase indent for next iteration
            indent += 2;
        }

        // Push indent
        for _ in 0..indent {
            label_stack.push(' ');
        }

        // Default text
        label_stack.push_str("COMMAND BUFFER:\n");

        for cmd_label in cmd_labels.iter() {
            // Push indent
            for _ in 0..indent {
                label_stack.push(' ');
            }

            // Push message
            let name = CStr::from_ptr(cmd_label.p_label_name);
            let name = name.to_str().unwrap();
            label_stack.push_str(name);
            label_stack.push('\n');

            // Increase indent for next iteration
            indent += 2;
        }

        let labels_header = console::style("Call Stack").cyan().bold();
        log!(level, "==============={}===============", labels_header);
        log!(level, "{}", label_stack);
    }
}

pub unsafe extern "system" fn vulkan_debug_messenger(
    message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    message_types: vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _p_user_data: *mut c_void,
) -> vk::Bool32 {
    let severity_level = message_severity_log_level(message_severity);

    let mtype_colour = message_type_colour(message_types);
    let mtype_string = message_type_string(message_types);

    let mtype = console::style(mtype_string).fg(mtype_colour).bold();

    let callback_data = p_callback_data.as_ref().expect("Nullptr for callback data");

    let main_header = console::style("Vulkan Debug Message").cyan().bold();

    log!(severity_level, "=========={}==========", main_header);
    log!(severity_level, "Type     : {}", mtype);
    print_message(callback_data, severity_level);
    print_call_stack(callback_data, severity_level);
    log!(severity_level, "========================================");
    log!(severity_level, "");

    // Break on debugger, if one is attached (assuming the platform supports the behavior)
    if !message_types.intersects(vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE) {
        debug_break();
    }

    vk::FALSE
}

#[cfg(target_os = "windows")]
#[inline(always)]
fn debug_break() {
    unsafe {
        use aleph_windows::Win32::System::Diagnostics::Debug::{DebugBreak, IsDebuggerPresent};

        let debugger_present: bool = IsDebuggerPresent().as_bool();
        if debugger_present {
            DebugBreak();
        }
    }
}

#[cfg(not(target_os = "windows"))]
#[inline(always)]
fn debug_break() {}
