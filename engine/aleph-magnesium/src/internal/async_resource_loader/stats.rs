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

use aleph_profile::tracy_client;

pub struct Stats {}

impl Stats {
    pub fn new() -> Self {
        let name = tracy_client::plot_name!("AsyncResourceLoader::queued_bytes");
        let plot = tracy_client::PlotConfiguration::default()
            .format(tracy_client::PlotFormat::Memory)
            .line_style(tracy_client::PlotLineStyle::Stepped);
        tracy_client::Client::start().plot_config(name, plot);
        tracy_client::Client::start().plot(name, 0.0);

        let name = tracy_client::plot_name!("AsyncResourceLoader::buffers_live");
        let plot = tracy_client::PlotConfiguration::default()
            .format(tracy_client::PlotFormat::Number)
            .line_style(tracy_client::PlotLineStyle::Stepped);
        tracy_client::Client::start().plot_config(name, plot);
        tracy_client::Client::start().plot(name, 0.0);

        let name = tracy_client::plot_name!("AsyncResourceLoader::textures_live");
        let plot = tracy_client::PlotConfiguration::default()
            .format(tracy_client::PlotFormat::Number)
            .line_style(tracy_client::PlotLineStyle::Stepped);
        tracy_client::Client::start().plot_config(name, plot);
        tracy_client::Client::start().plot(name, 0.0);

        let name = tracy_client::plot_name!("AsyncResourceLoader::live_submissions");
        let plot = tracy_client::PlotConfiguration::default()
            .format(tracy_client::PlotFormat::Number)
            .line_style(tracy_client::PlotLineStyle::Stepped);
        tracy_client::Client::start().plot_config(name, plot);
        tracy_client::Client::start().plot(name, 0.0);

        Self {}
    }

    pub fn update_queued_bytes(&self, v: u64) {
        tracy_client::plot!("AsyncResourceLoader::queued_bytes", v as f64)
    }

    pub fn update_buffers_live(&self, v: usize) {
        tracy_client::plot!("AsyncResourceLoader::buffers_live", v as f64)
    }

    pub fn update_textures_live(&self, v: usize) {
        tracy_client::plot!("AsyncResourceLoader::textures_live", v as f64)
    }

    pub fn update_live_submissions(&self, v: usize) {
        tracy_client::plot!("AsyncResourceLoader::live_submissions", v as f64)
    }
}
