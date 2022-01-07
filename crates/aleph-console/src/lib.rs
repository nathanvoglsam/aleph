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

use log::{Level, Metadata, Record};
use rhai::RegisterNativeFunction;
use smartstring::{LazyCompact, SmartString};
use std::cell::RefCell;
use std::io::Write;
use std::net::TcpStream;
use std::rc::Rc;
use std::sync::Mutex;

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
    remote: Option<Mutex<TcpStream>>,
}

impl Logger {
    /// Consumes `self`, installing this logger as the global logger instance
    pub fn install(mut self) {
        let remote = Self::listen_for_and_connect_to_remote().ok().flatten();

        // Add our remote, if we found one
        self.remote = remote;

        let level = self.env_logger.filter();
        log::set_boxed_logger(Box::new(self)).expect("Attempting to install logger");
        log::set_max_level(level);
    }

    #[cfg(feature = "remote")]
    fn listen_for_and_connect_to_remote() -> std::io::Result<Option<Mutex<TcpStream>>> {
        let remote = Self::find_listener()?;
        if let Some(remote) = remote {
            let socket = Self::connect_to_listener(remote)?;
            Ok(Some(Mutex::new(socket)))
        } else {
            Ok(None)
        }
    }

    #[cfg(not(feature = "remote"))]
    fn listen_for_and_connect_to_remote() -> std::io::Result<Option<Mutex<TcpStream>>> {
        Ok(None)
    }

    #[cfg(feature = "remote")]
    fn connect_to_listener(mut addr: std::net::SocketAddr) -> std::io::Result<TcpStream> {
        use std::io::{Error, ErrorKind, Read};
        use std::time::Duration;

        // The convention is to use the port immediately after the advertisement port
        addr.set_port(42057);

        // Connect to the remote that advertised itself as available
        let mut stream = TcpStream::connect_timeout(&addr, Duration::from_secs(2))?;

        // We can't wait for the socket indefinitely while reading here so set a timeout
        stream.set_read_timeout(Some(Duration::from_secs(2)))?;

        // Write a magic string so the remote can identify us as a valid client
        stream.write("I_AM_AN_ALEPH_APP".as_bytes())?;

        // Try to read the exact bytes for the expected handshake response
        let mut buffer = [0u8; 22];
        stream.read_exact(&mut buffer)?;

        // Verify the handshake response is correct, if not return an error
        let text =
            std::str::from_utf8(&buffer).map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
        if text == "I_AM_AN_ALEPH_LISTENER" {
            // Remove the timeout
            stream.set_read_timeout(None)?;
            Ok(stream)
        } else {
            Err(Error::new(
                ErrorKind::InvalidData,
                "Remote sent incorrect handshake response",
            ))
        }
    }

    #[cfg(feature = "remote")]
    fn find_listener() -> std::io::Result<Option<std::net::SocketAddr>> {
        use std::io::ErrorKind;
        use std::net::UdpSocket;
        use std::time::{Duration, Instant};

        // Bind our socket and listen for UDP packets on port 42056
        let socket = UdpSocket::bind("0.0.0.0:42056")?;

        // Enforce a read time out so we don't wait forever
        socket.set_read_timeout(Duration::from_secs(2).into())?;

        // Listen for remote log outputs for 2 seconds
        let start_time = Instant::now();
        while Instant::now().duration_since(start_time).as_secs() < 2 {
            // Wait for packets and write the contents into the buffer
            let mut buffer = [0u8; 128];
            let result = socket.recv_from(&mut buffer);

            // Handle errors and unwrap our socket read
            let (bytes, from) = match result {
                Ok(v) => v,
                Err(e) => {
                    if matches!(e.kind(), ErrorKind::TimedOut) {
                        // A timeout is a valid error here so we just restart the loop
                        continue;
                    } else {
                        // Any other error is actually an error so we yield the error
                        return Err(e);
                    }
                }
            };

            // Get a slice of just the bytes written in the packet and check if it matches our magic
            // string
            let slice = &buffer[0..bytes];
            if let Some("I_AM_AN_ALEPH_LOG_LISTENER_I_SWEAR") = std::str::from_utf8(slice).ok() {
                return Ok(Some(from));
            }
        }

        Ok(None)
    }
}

impl From<env_logger::Logger> for Logger {
    fn from(env_logger: env_logger::Logger) -> Self {
        Self {
            env_logger,
            remote: None,
        }
    }
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        // We rely on env_logger for our log filtering
        self.env_logger.enabled(metadata)
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        // First we log to the env logger
        self.env_logger.log(record);

        // Then we log to our remote target, if we have one
        if let Some(remote) = self.remote.as_ref() {
            // Convert error level to integer
            let level = match record.level() {
                Level::Error => 0,
                Level::Warn => 1,
                Level::Info => 2,
                Level::Debug => 3,
                Level::Trace => 4,
            };

            // Get log target
            let module = record.target();
            let payload = format!(
                "{{\"mod\":\"{}\",\"lvl\":{},\"msg\":\"{}\"}}",
                module,
                level,
                record.args()
            );

            // TODO: Handle remote disconnects
            remote
                .lock()
                .unwrap()
                .write_all(payload.as_bytes())
                .unwrap();
        }
    }

    fn flush(&self) {
        // First flush the env logger
        self.env_logger.flush();

        // Then flush the socket, if we have one
        if let Some(remote) = self.remote.as_ref() {
            // TODO: Handle remote disconnects
            remote.lock().unwrap().flush().unwrap();
        }
    }
}
