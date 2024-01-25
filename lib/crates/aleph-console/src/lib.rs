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
//! This provides a [log::Log] implementation in [Logger].
//!

use std::io::{BufWriter, Write};
use std::net::TcpStream;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

use log::{Level, LevelFilter, Metadata, Record};

///
/// Provides a [log::Log] implementation that wraps an [env_logger::Logger] for local logging and
/// implements a network logging protocol for logging to a remote machine
///
pub struct Logger<T: log::Log + 'static> {
    env_logger: T,
    remote: Option<Mutex<BufWriter<TcpStream>>>,
    has_remote: AtomicBool,
}

impl<T: log::Log + 'static> Logger<T> {
    /// Consumes `self`, installing this logger as the global logger instance
    pub fn install(mut self) -> Option<TcpStream> {
        let remote = Self::listen_for_and_connect_to_remote().ok().flatten();

        let remote = if let Some(remote) = remote {
            if let Ok(cloned) = remote.try_clone() {
                // Flag that we have successfully created a remote connection
                self.has_remote.store(true, Ordering::Relaxed);

                // Construct our fully wrapper writer and add it to the logger.
                let writer = BufWriter::new(cloned);
                self.remote = Some(Mutex::new(writer));

                Some(remote)
            } else {
                None
            }
        } else {
            None
        };

        // let level = self.env_logger.filter();
        log::set_boxed_logger(Box::new(self)).expect("Attempting to install logger");
        log::set_max_level(LevelFilter::Trace);

        remote
    }

    #[cfg(feature = "remote")]
    fn listen_for_and_connect_to_remote() -> std::io::Result<Option<TcpStream>> {
        let remote = Self::find_listener()?;
        if let Some(remote) = remote {
            let socket = Self::connect_to_listener(remote)?;
            Ok(Some(socket))
        } else {
            Ok(None)
        }
    }

    #[cfg(not(feature = "remote"))]
    fn listen_for_and_connect_to_remote() -> std::io::Result<Option<TcpStream>> {
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
        stream.write_all("I_AM_AN_ALEPH_APP".as_bytes())?;

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
            if let Ok("I_AM_AN_ALEPH_LOG_LISTENER_I_SWEAR") = std::str::from_utf8(slice) {
                return Ok(Some(from));
            }
        }

        Ok(None)
    }
}

impl<T: log::Log + 'static> From<T> for Logger<T> {
    fn from(env_logger: T) -> Self {
        Self {
            env_logger,
            remote: None,
            has_remote: Default::default(),
        }
    }
}

impl<T: log::Log + 'static> log::Log for Logger<T> {
    fn enabled(&self, metadata: &Metadata) -> bool {
        // We rely on env_logger for our log filtering
        self.env_logger.enabled(metadata)
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        // First we log to the env logger
        #[cfg(not(target_vendor = "uwp"))]
        self.env_logger.log(record);

        #[cfg(target_vendor = "uwp")]
        unsafe {
            let level = match record.level() {
                Level::Error => "ERROR",
                Level::Warn => "WARN ",
                Level::Info => "INFO ",
                Level::Debug => "DEBUG",
                Level::Trace => "TRACE",
            };
            let module = record.target();

            let payload = format!("[{} {}] {}\r\n\0", level, module, record.args());

            let v = aleph_windows::core::PCSTR(payload.as_ptr());
            aleph_windows::Win32::System::Diagnostics::Debug::OutputDebugStringA(v);
        }

        // Then we log to our remote target, if we have one
        if self.has_remote.load(Ordering::Relaxed) {
            use serde::Serialize;

            #[derive(Serialize)]
            struct Message<'a> {
                r#mod: &'a str,
                r#lvl: i32,
                r#msg: &'a str,
            }

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

            // Resolve message
            let message = format!("{}", record.args());

            let payload = Message {
                r#mod: module,
                lvl: level,
                msg: &message,
            };
            let payload = serde_json::to_string(&payload).unwrap();

            if let Some(remote) = self.remote.as_ref() {
                // Lock and write our payload
                let mut lock = remote.lock().unwrap();
                let result = lock
                    .write_all(payload.as_bytes())
                    .and_then(|_| lock.flush());

                // If we encounter any error we assume we've lost the socket
                if result.is_err() {
                    self.has_remote.store(false, Ordering::Relaxed);
                }
            }
        }
    }

    fn flush(&self) {
        // First flush the env logger
        self.env_logger.flush();

        // Then flush the socket, if we have one
        if self.has_remote.load(Ordering::Relaxed) {
            if let Some(remote) = self.remote.as_ref() {
                // Lock and write our payload
                let mut lock = remote.lock().unwrap();
                let result = lock.flush();

                // If we encounter any error we assume we've lost the socket
                if result.is_err() {
                    self.has_remote.store(false, Ordering::Relaxed);
                }
            }
        }
    }
}
