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

//!
//! This crate provides a debug console like object for use with game engines that want a simple way
//! to dynamically call functions from user input.
//!
//! This crate also provides a [log::Log] implementation in [Logger].
//!
//! The [DebugConsole] provides the interface for the debug console. It just wraps an `rhai`
//! [rhai::Engine] into a constrained interface that only allows adding functions and evaluating
//! expressions.
//!

extern crate aleph_log as log;
extern crate env_logger;
extern crate rhai;
extern crate smartstring;

use log::{Metadata, Record};
use rhai::RegisterNativeFunction;
use smartstring::{LazyCompact, SmartString};
use std::cell::RefCell;
use std::rc::Rc;

/// A ref-counted handle to a debug console.
#[derive(Clone)]
pub struct DebugConsole {
    inner: Rc<RefCell<rhai::Engine>>,
}

impl DebugConsole {
    /// Constructs a new debug console instance and returns a ref-counted handle to it.
    ///
    /// [DebugConsole] implements [Clone] and internally wraps an [Rc] so can be shared around like
    /// an [Rc]. Only once all [DebugConsole] handles pointing to a console instance are dropped
    /// will the console be dropped.
    pub fn new() -> Self {
        let engine = rhai::Engine::new();
        let out = Self {
            inner: Rc::new(RefCell::new(engine)),
        };
        out
    }

    /// Register a custom function with the [`DebugConsole`].
    ///
    /// # Example
    ///
    /// ```
    /// use aleph_console::DebugConsole;
    ///
    /// // Normal function
    /// fn add(x: i64, y: i64) -> i64 {
    ///     x + y
    /// }
    ///
    /// let mut console = DebugConsole::new();
    ///
    /// // Register a function
    /// console.register_fn("add", add);
    ///
    /// // You can also register a closure.
    /// console.register_fn("sub", |x: i64, y: i64| x - y );
    ///
    /// // Logs the result of the expressions
    /// console.eval("add(40, 2)");
    /// console.eval("sub(44, 2)");
    /// ```
    pub fn register_fn<N, A, F>(&self, name: N, func: F)
    where
        N: AsRef<str> + Into<SmartString<LazyCompact>>,
        F: RegisterNativeFunction<A, ()>,
    {
        self.inner.borrow_mut().register_fn(name, func);
    }

    /// Evaluate a string containing an expression.
    ///
    /// # Example
    ///
    /// ```
    /// use aleph_console::DebugConsole;
    ///
    /// let console = DebugConsole::new();
    ///
    /// console.eval("40 + 2");
    /// ```
    pub fn eval(&self, expr: &str) {
        let v: Result<rhai::Dynamic, Box<rhai::EvalAltResult>> =
            self.inner.borrow().eval_expression::<rhai::Dynamic>(expr);

        match v {
            Ok(v) => {
                log::info!("{} = {}", expr, v);
            }
            Err(v) => {
                log::info!("{} = {}", expr, v);
            }
        }
    }
}

///
/// Provides a [log::Log] implementation that wraps an [env_logger::Logger] for local logging and
/// implements a network logging protocol for logging to a remote machine
///
pub struct Logger {
    env_logger: env_logger::Logger,
}

impl Logger {
    /// Consumes `self`, installing this logger as the global logger instance
    pub fn install(self) {
        let level = self.env_logger.filter();
        log::set_boxed_logger(Box::new(self)).expect("Attempting to install logger");
        log::set_max_level(level);
    }
}

impl From<env_logger::Logger> for Logger {
    fn from(env_logger: env_logger::Logger) -> Self {
        Self { env_logger }
    }
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        self.env_logger.enabled(metadata)
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        self.env_logger.log(record);
    }

    fn flush(&self) {
        self.env_logger.flush();
    }
}
