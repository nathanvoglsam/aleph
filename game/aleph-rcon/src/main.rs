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

#![cfg_attr(target_vendor = "uwp", windows_subsystem = "windows")]

extern crate aleph_engine as aleph;

use aleph::egui::IEguiContextProvider;
use aleph::interfaces::plugin::{
    IInitResponse, IPlugin, IPluginRegistrar, IRegistryAccessor, PluginDescription,
};
use aleph::interfaces::schedule::{CoreStage, IScheduleProvider};
use aleph::Engine;
use serde::Deserialize;
use std::cell::RefCell;
use std::io::{BufRead, BufReader, Read};
use std::net::{SocketAddr, TcpListener, TcpStream, UdpSocket};
use std::rc::Rc;
use std::str::FromStr;
use std::sync::mpsc::SyncSender;
use std::sync::Arc;

struct PluginGameLogic();

impl PluginGameLogic {
    pub fn new() -> Self {
        Self()
    }
}

impl IPlugin for PluginGameLogic {
    fn get_description(&self) -> PluginDescription {
        PluginDescription {
            name: "PluginGameLogic".to_string(),
            description: "The game logic implementation for rcon UI".to_string(),
            major_version: env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap(),
            minor_version: env!("CARGO_PKG_VERSION_MINOR").parse().unwrap(),
            patch_version: env!("CARGO_PKG_VERSION_PATCH").parse().unwrap(),
        }
    }

    fn register(&mut self, registrar: &mut dyn IPluginRegistrar) {
        registrar.depends_on::<dyn IScheduleProvider>();
        registrar.must_init_after::<dyn IScheduleProvider>();

        registrar.depends_on::<dyn IEguiContextProvider>();
        registrar.must_init_after::<dyn IEguiContextProvider>();
    }

    fn on_init(&mut self, registry: &dyn IRegistryAccessor) -> Box<dyn IInitResponse> {
        use aleph::egui;
        let egui_provider = registry.get_interface::<dyn IEguiContextProvider>();

        let schedule_provider = registry.get_interface::<dyn IScheduleProvider>().unwrap();
        let schedule_cell = schedule_provider.get();
        let mut schedule = schedule_cell.get();

        let program_state = ProgramState::new();

        let state = program_state.clone();
        let mut has_remote = false;
        schedule.add_exclusive_at_start_system_to_stage(
            &CoreStage::Update,
            "aleph_rcon::update",
            move || {
                if !has_remote {
                    state.broadcast_queue.send(()).unwrap();
                }

                if let Ok(_) = state.remote_stream.try_recv() {
                    has_remote = true;
                    state.buffer.borrow_mut().clear();
                }

                while let Ok(msg) = state.message_stream.try_recv() {
                    use std::fmt::Write;

                    let text = std::str::from_utf8(&msg).unwrap();
                    let payload: Message = serde_json::from_str(text).unwrap();

                    let level = match payload.lvl {
                        0 => "ERROR",
                        1 => "WARN ",
                        2 => "INFO ",
                        3 => "DEBUG",
                        4 => "TRACE",
                        _ => "?????",
                    };
                    let module = payload.r#mod;
                    let message = payload.msg;

                    let mut buffer_borrow = state.buffer.borrow_mut();
                    writeln!(&mut buffer_borrow, "[{} {}] {}", level, module, message).unwrap();
                }
            },
        );

        let state = program_state.clone();
        schedule.add_exclusive_at_start_system_to_stage(
            &CoreStage::Render,
            "aleph_rcon::render",
            move || {
                if let Some(egui) = egui_provider.as_ref() {
                    let egui_ctx = egui.get_context();

                    egui::TopBottomPanel::top("menu_bar").show(&egui_ctx, |ui| {
                        egui::menu::bar(ui, |ui| {
                            ui.menu_button("File", |ui| {
                                if ui.button("Exit").clicked() {
                                    aleph::log::warn!("They're trying to corner us");
                                }
                            });
                        });
                    });

                    let fill = egui_ctx.style().visuals.extreme_bg_color;
                    let frame = egui::Frame::none().fill(fill);
                    let text_borrow = state.buffer.borrow();
                    let mut text = text_borrow.as_str();
                    egui::CentralPanel::default()
                        .frame(frame)
                        .show(&egui_ctx, |ui| {
                            egui::ScrollArea::vertical().show(ui, |ui| {
                                ui.add(
                                    egui::TextEdit::multiline(&mut text)
                                        .text_style(egui::TextStyle::Monospace)
                                        .desired_width(f32::INFINITY),
                                );
                            });
                        });
                }
            },
        );

        Box::new(Vec::new())
    }
}

aleph::any::declare_interfaces!(PluginGameLogic, [IPlugin]);

fn main() {
    let platform = aleph::target::build::target_platform();
    let headless = !platform.is_windows();

    let mut engine = Engine::builder();
    engine.default_plugins(headless);
    engine.plugin(PluginGameLogic::new());
    engine.build().run()
}

struct ProgramState {
    buffer: RefCell<String>,
    // broadcast_thread: std::thread::JoinHandle<()>,
    // listener_thread: std::thread::JoinHandle<()>,
    message_stream: std::sync::mpsc::Receiver<Vec<u8>>,
    broadcast_queue: std::sync::mpsc::SyncSender<()>,
    remote_stream: std::sync::mpsc::Receiver<()>,
}

impl ProgramState {
    pub fn new() -> Rc<Self> {
        let (remote_sender, remote_stream) = std::sync::mpsc::sync_channel(16);
        let (broadcast_queue, broadcast_stream) = std::sync::mpsc::sync_channel(256);
        let (message_sender, message_stream) = std::sync::mpsc::sync_channel(1024);
        let message_sender = Arc::new(message_sender);

        std::thread::spawn(move || {
            let bind_address = SocketAddr::from_str("0.0.0.0:0").unwrap();
            let send_address = SocketAddr::from_str("255.255.255.255:42056").unwrap();
            let socket = UdpSocket::bind(bind_address).unwrap();
            socket.set_broadcast(true).unwrap();
            socket.connect(send_address).unwrap();

            while let Ok(_) = broadcast_stream.recv() {
                let text = "I_AM_AN_ALEPH_LOG_LISTENER_I_SWEAR";

                let bytes = socket.send(text.as_bytes()).unwrap();
                if bytes != text.len() {
                    panic!("Sent wrong byte count");
                }
            }
        });

        let sender = message_sender.clone();
        std::thread::spawn(move || {
            use std::io::Write;

            let listener = TcpListener::bind("0.0.0.0:42057").unwrap();

            while let Ok((mut stream, from)) = listener.accept() {
                aleph::log::info!("New remote connected with address: {:?}", from);

                // Read the remote's handshake
                let mut shake = [0u8; 17];
                if stream.read_exact(&mut shake).is_err() {
                    aleph::log::warn!(
                        "Failed to read handshake request, connection will be dropped"
                    );
                    continue;
                }

                // Read the remote's
                if let Ok("I_AM_AN_ALEPH_APP") = std::str::from_utf8(&shake) {
                    // Write the response
                    stream
                        .write_all("I_AM_AN_ALEPH_LISTENER".as_bytes())
                        .unwrap();

                    remote_sender.send(()).unwrap();

                    let channel = sender.clone();
                    let stream = BufReader::new(stream);
                    std::thread::spawn(move || receiver_thread(channel, stream));
                } else {
                    aleph::log::warn!("Handshake data is invalid, connection will be dropped");
                }
            }
        });

        Rc::new(Self {
            buffer: RefCell::new(String::new()),
            // broadcast_thread,
            // listener_thread,
            message_stream,
            broadcast_queue,
            remote_stream,
        })
    }
}

#[derive(Deserialize)]
struct Message<'a> {
    r#mod: &'a str,
    r#lvl: i32,
    r#msg: &'a str,
}

fn receiver_thread(channel: Arc<SyncSender<Vec<u8>>>, mut stream: BufReader<TcpStream>) {
    loop {
        let mut buffer = Vec::new();
        stream.read_until('{' as u8, &mut buffer).unwrap();
        stream.read_until('}' as u8, &mut buffer).unwrap();
        channel.send(buffer).unwrap();
    }
}
