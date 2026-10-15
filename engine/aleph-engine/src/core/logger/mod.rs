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

use log::{Level, Metadata, Record};
use sdl3::log::Category;

pub struct Logger;

pub static LOGGER: Logger = Logger;

impl log::Log for Logger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        let message = if let Some(path) = record.module_path_static() {
            format!("[{}] {}", path, record.args())
        } else if let Some(path) = record.module_path() {
            format!("[{}] {}", path, record.args())
        } else {
            format!("[_] {}", record.args())
        };

        match record.level() {
            Level::Error => sdl3::log::log_error(Category::Application, &message),
            Level::Warn => sdl3::log::log_warn(Category::Application, &message),
            Level::Info => sdl3::log::log_info(Category::Application, &message),
            Level::Debug => sdl3::log::log_debug(Category::Application, &message),
            Level::Trace => sdl3::log::log_trace(Category::Application, &message),
        }
    }

    fn flush(&self) {
        // no-op
    }
}
