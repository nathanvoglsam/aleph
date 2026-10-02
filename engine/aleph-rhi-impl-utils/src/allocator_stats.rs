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

use aleph_gpu_allocator::AllocatorStatsSummary;
use aleph_profile::tracy_client;
use aleph_profile::tracy_client::PlotName;

static NUM_ALLOCATIONS: PlotName = tracy_client::plot_name!("GpuAllocator::num_allocations");
static NUM_DEDICATED_ALLOCATIONS: PlotName =
    tracy_client::plot_name!("GpuAllocator::num_dedicated_allocations");
static USED_BYTES: PlotName = tracy_client::plot_name!("GpuAllocator::used_bytes");
static RESERVED_BYTES: PlotName = tracy_client::plot_name!("GpuAllocator::reserved_bytes");

pub fn setup_plots() {
    let name = NUM_ALLOCATIONS;
    let plot = tracy_client::PlotConfiguration::default()
        .format(tracy_client::PlotFormat::Number)
        .line_style(tracy_client::PlotLineStyle::Stepped);
    tracy_client::Client::start().plot_config(name, plot);
    tracy_client::Client::start().plot(name, 0.0);

    let name = NUM_DEDICATED_ALLOCATIONS;
    let plot = tracy_client::PlotConfiguration::default()
        .format(tracy_client::PlotFormat::Number)
        .line_style(tracy_client::PlotLineStyle::Stepped);
    tracy_client::Client::start().plot_config(name, plot);
    tracy_client::Client::start().plot(name, 0.0);

    let name = USED_BYTES;
    let plot = tracy_client::PlotConfiguration::default()
        .format(tracy_client::PlotFormat::Memory)
        .line_style(tracy_client::PlotLineStyle::Stepped);
    tracy_client::Client::start().plot_config(name, plot);
    tracy_client::Client::start().plot(name, 0.0);

    let name = RESERVED_BYTES;
    let plot = tracy_client::PlotConfiguration::default()
        .format(tracy_client::PlotFormat::Memory)
        .line_style(tracy_client::PlotLineStyle::Stepped);
    tracy_client::Client::start().plot_config(name, plot);
    tracy_client::Client::start().plot(name, 0.0);
}

pub fn update_plots(stats: &AllocatorStatsSummary) {
    tracy_client::plot!(
        "GpuAllocator::num_allocations",
        stats.num_allocations as f64
    );
    tracy_client::plot!(
        "GpuAllocator::num_dedicated_allocations",
        stats.num_dedicated_allocations as f64
    );
    tracy_client::plot!("GpuAllocator::used_bytes", stats.used_bytes as f64);
    tracy_client::plot!("GpuAllocator::reserved_bytes", stats.reserved_bytes as f64);
}
