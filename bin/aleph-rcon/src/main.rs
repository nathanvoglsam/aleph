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
use aleph::interfaces::make_plugin_description_for_crate;
use aleph::interfaces::plugin::{
    IInitResponse, IPlugin, IPluginRegistrar, IRegistryAccessor, PluginDescription,
};
use aleph::interfaces::schedule::{CoreStage, IScheduleProvider};
use aleph::Engine;
use serde::Deserialize;
use std::cell::{Cell, RefCell};
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream, UdpSocket};
use std::rc::Rc;
use std::str::FromStr;
use std::sync::mpsc::{Receiver, SyncSender};
use std::sync::Arc;

struct PluginGameLogic();

impl PluginGameLogic {
    pub fn new() -> Self {
        Self()
    }
}

impl IPlugin for PluginGameLogic {
    fn get_description(&self) -> PluginDescription {
        make_plugin_description_for_crate!()
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
        let remote_connection = Rc::new(Cell::new(None));

        let state = program_state.clone();
        let remote = remote_connection.clone();
        schedule.add_exclusive_at_start_system_to_stage(
            &CoreStage::Update,
            "aleph_rcon::update",
            move || {
                let mut remote_cell = remote.take();

                // Every frame we don't have a remote send a broadcast packet and check for any
                // remotes who have connected
                if remote_cell.is_none() {
                    // Queue up another broadcast packet
                    state.broadcast_sender.send(()).unwrap();

                    // Poll for a new remote
                    if let Ok(v) = state.remote_receiver.try_recv() {
                        // Store the remote channel into the slot so we stop broadcasting and
                        // polling
                        remote_cell = Some(v);

                        // Clear the log buffer in case we're starting a new session. This clears
                        // out the old log data from any previous sessions
                        state.buffer.borrow_mut().clear();
                    }
                }

                while let Ok(msg) = state.message_receiver.try_recv() {
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

                remote.set(remote_cell);
            },
        );

        let state = program_state;
        let remote = remote_connection;
        let mut command_buffer = String::new();
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
                                    log::warn!("They're trying to corner us");
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
                            egui::ScrollArea::vertical()
                                .max_height(f32::INFINITY)
                                .show(ui, |ui| {
                                    ui.add(
                                        egui::TextEdit::multiline(&mut text)
                                            .font(egui::TextStyle::Monospace)
                                            .desired_width(f32::INFINITY),
                                    );
                                });
                        });
                    egui::TopBottomPanel::bottom("input_bar").show(&egui_ctx, |ui| {
                        ui.horizontal(|ui| {
                            ui.text_edit_singleline(&mut command_buffer);
                            if ui.button("Send").clicked() && !command_buffer.is_empty() {
                                let remote_cell = remote.take();
                                if let Some(r) = remote_cell.as_ref() {
                                    r.send(command_buffer.clone()).unwrap();
                                    command_buffer.clear();
                                }
                                remote.set(remote_cell);
                            }
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
    let mut engine = Engine::builder();
    engine.default_plugins(false);
    engine.plugin(PluginGameLogic::new());
    engine.build().run()
}

struct ProgramState {
    /// Buffer that log messages are written into for display in the UI
    buffer: RefCell<String>,

    /// Channel where all received log messages are published to
    message_receiver: std::sync::mpsc::Receiver<Vec<u8>>,

    /// Channel that will trigger sending a broadcast packet when published to
    broadcast_sender: std::sync::mpsc::SyncSender<()>,

    /// Channel which will be published to when a remote connection is successfully established
    remote_receiver: std::sync::mpsc::Receiver<SyncSender<String>>,
}

impl ProgramState {
    pub fn new() -> Rc<Self> {
        let (remote_sender, remote_receiver) = std::sync::mpsc::sync_channel(16);
        let (broadcast_sender, broadcast_receiver) = std::sync::mpsc::sync_channel(256);
        let (message_sender, message_receiver) = std::sync::mpsc::sync_channel(1024);

        // Spawn persistent thread that handles sending UDP broadcast packets to announce the RCON
        // is available to connect to
        std::thread::spawn(move || broadcaster_thread(broadcast_receiver));

        // Spawn persistent thread that handles listening on a TCP socket for new connections and
        // creating threads for any remotes that connect
        std::thread::spawn(move || listener_thread(remote_sender, Arc::new(message_sender)));

        Rc::new(Self {
            buffer: RefCell::new(String::new()),
            // broadcast_thread,
            // listener_thread,
            message_receiver,
            broadcast_sender,
            remote_receiver,
        })
    }
}

#[derive(Deserialize)]
struct Message<'a> {
    r#mod: &'a str,
    r#lvl: i32,
    r#msg: String,
}

fn receiver_thread(channel: Arc<SyncSender<Vec<u8>>>, mut stream: BufReader<TcpStream>) {
    loop {
        let mut buffer = Vec::new();
        stream.read_until(b'{', &mut buffer).unwrap();
        stream.read_until(b'}', &mut buffer).unwrap();
        channel.send(buffer).unwrap();
    }
}

fn sender_thread(channel: Receiver<String>, mut stream: BufWriter<TcpStream>) {
    while let Ok(command) = channel.recv() {
        stream.write_all(&[0]).unwrap();
        stream.write_all(command.as_bytes()).unwrap();
        stream.write_all(&[0]).unwrap();
        stream.flush().unwrap();
    }
}

fn broadcaster_thread(broadcast_receiver: Receiver<()>) {
    let bind_address = SocketAddr::from_str("0.0.0.0:0").unwrap();
    let send_address = SocketAddr::from_str("255.255.255.255:42056").unwrap();
    let socket = UdpSocket::bind(bind_address).unwrap();
    socket.set_broadcast(true).unwrap();
    socket.connect(send_address).unwrap();

    while broadcast_receiver.recv().is_ok() {
        let text = "I_AM_AN_ALEPH_LOG_LISTENER_I_SWEAR";

        let bytes = socket.send(text.as_bytes()).unwrap();
        if bytes != text.len() {
            panic!("Sent wrong byte count");
        }
    }
}

fn listener_thread(
    remote_sender: SyncSender<SyncSender<String>>,
    message_sender: Arc<SyncSender<Vec<u8>>>,
) {
    let listener = TcpListener::bind("0.0.0.0:42057").unwrap();

    while let Ok((mut stream, from)) = listener.accept() {
        log::info!("New remote connected with address: {:?}", from);

        // Read the remote's handshake
        let mut shake = [0u8; 17];
        if stream.read_exact(&mut shake).is_err() {
            log::warn!("Failed to read handshake request, connection will be dropped");
            continue;
        }

        // Read the remote's
        if let Ok("I_AM_AN_ALEPH_APP") = std::str::from_utf8(&shake) {
            // Write the response
            let response = "I_AM_AN_ALEPH_LISTENER";
            stream.write_all(response.as_bytes()).unwrap();

            // Setup and spawn receiver thread
            let message_sender = message_sender.clone();
            let recv_stream = BufReader::new(stream.try_clone().unwrap());
            std::thread::spawn(move || receiver_thread(message_sender, recv_stream));

            // Setup and spawn sender thread
            let (command_sender, command_receiver) = std::sync::mpsc::sync_channel(1024);
            let send_stream = BufWriter::new(stream);
            std::thread::spawn(move || sender_thread(command_receiver, send_stream));

            remote_sender.send(command_sender).unwrap();
        } else {
            log::warn!("Handshake data is invalid, connection will be dropped");
        }
    }
}
