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

use std::sync::mpsc;
use std::thread::JoinHandle;

pub struct AsyncLoader<C> {
    queue: AsyncLoaderHandle<C>,
    worker: Option<JoinHandle<()>>,
}

impl<C> Drop for AsyncLoader<C> {
    fn drop(&mut self) {
        self.queue.queue.send(LoaderMessage::Exit).unwrap();
        self.worker.take().unwrap().join().unwrap();
    }
}

impl<C: Send + 'static> AsyncLoader<C> {
    pub fn new<T: Send + 'static>(context: T, handler: fn(&mut T, &C)) -> Self {
        let (sender, receiver) = mpsc::channel();

        let mut worker = LoaderWorker2::new(receiver, context);
        let worker = std::thread::Builder::new()
            .name("AsyncLoaderThread".to_string())
            .spawn(move || {
                aleph_profile::register_thread!("AsyncLoaderThread");
                worker.run(handler)
            })
            .unwrap();

        Self {
            queue: AsyncLoaderHandle::new(sender),
            worker: Some(worker),
        }
    }

    pub fn load(&self, command: C) {
        self.queue.load(command)
    }

    pub fn handle(&self) -> AsyncLoaderHandle<C> {
        AsyncLoaderHandle::new(self.queue.queue.clone())
    }
}

pub struct AsyncLoaderHandle<C> {
    queue: mpsc::Sender<LoaderMessage<C>>,
}

impl<C: Send> AsyncLoaderHandle<C> {
    fn new(v: mpsc::Sender<LoaderMessage<C>>) -> Self {
        Self { queue: v }
    }

    pub fn load(&self, command: C) {
        let message = LoaderMessage::Command(command);
        self.queue.send(message).ok().unwrap();
    }
}

struct LoaderWorker2<T, C> {
    queue: mpsc::Receiver<LoaderMessage<C>>,
    context: T,
}

impl<T, C> LoaderWorker2<T, C> {
    fn new(receiver: mpsc::Receiver<LoaderMessage<C>>, context: T) -> Self {
        Self {
            queue: receiver,
            context,
        }
    }

    fn run(&mut self, handler: fn(&mut T, &C)) {
        loop {
            match self.queue.recv() {
                Ok(LoaderMessage::Exit) => {
                    return;
                }
                Ok(LoaderMessage::Command(cmd)) => handler(&mut self.context, &cmd),
                Err(_e) => {
                    // TODO: log the error then return
                    return;
                }
            }
        }
    }
}

enum LoaderMessage<T> {
    Exit,
    Command(T),
}
