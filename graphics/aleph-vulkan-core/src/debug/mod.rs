//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use std::ffi::c_void;
use std::ffi::CStr;
use std::slice;

use aleph_log::{log, Level};
use erupt::extensions::ext_debug_utils::DebugUtilsMessageTypeFlagsEXT;
use erupt::extensions::ext_debug_utils::DebugUtilsMessengerCallbackDataEXT;
use erupt::extensions::ext_debug_utils::{
    DebugUtilsMessageSeverityFlagBitsEXT, DebugUtilsMessageSeverityFlagsEXT,
};
use erupt::vk1_0::{Bool32, FALSE};

fn message_severity_log_level(severity: DebugUtilsMessageSeverityFlagsEXT) -> Level {
    if severity == DebugUtilsMessageSeverityFlagsEXT::INFO_EXT {
        Level::Debug
    } else if severity == DebugUtilsMessageSeverityFlagsEXT::WARNING_EXT {
        Level::Warn
    } else if severity == DebugUtilsMessageSeverityFlagsEXT::ERROR_EXT {
        Level::Error
    } else if severity == DebugUtilsMessageSeverityFlagsEXT::VERBOSE_EXT {
        Level::Trace
    } else {
        Level::Debug
    }
}

fn message_type_colour(mtype: DebugUtilsMessageTypeFlagsEXT) -> console::Color {
    if mtype == DebugUtilsMessageTypeFlagsEXT::GENERAL_EXT {
        console::Color::Green
    } else if mtype == DebugUtilsMessageTypeFlagsEXT::VALIDATION_EXT {
        console::Color::Red
    } else if mtype == DebugUtilsMessageTypeFlagsEXT::PERFORMANCE_EXT {
        console::Color::Yellow
    } else {
        console::Color::Black
    }
}

fn message_type_string(mtype: DebugUtilsMessageTypeFlagsEXT) -> &'static str {
    if mtype == DebugUtilsMessageTypeFlagsEXT::GENERAL_EXT {
        "GENERAL"
    } else if mtype == DebugUtilsMessageTypeFlagsEXT::VALIDATION_EXT {
        "VALIDATION"
    } else if mtype == DebugUtilsMessageTypeFlagsEXT::PERFORMANCE_EXT {
        "PERFORMANCE"
    } else {
        "NONE"
    }
}

unsafe fn print_message(callback_data: &DebugUtilsMessengerCallbackDataEXT, level: Level) {
    let message = CStr::from_ptr(callback_data.p_message).to_str().unwrap();
    let message = console::style(message).italic();

    let message_header = console::style("Message").cyan().bold();
    log!(level, "================{}=================", message_header);
    log!(level, "{}", message);
}

unsafe fn print_call_stack(callback_data: &DebugUtilsMessengerCallbackDataEXT, level: Level) {
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
    message_severity: DebugUtilsMessageSeverityFlagBitsEXT,
    message_types: DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const DebugUtilsMessengerCallbackDataEXT,
    _p_user_data: *mut c_void,
) -> Bool32 {
    let severity_level = message_severity_log_level(message_severity.bitmask());

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

    FALSE
}
