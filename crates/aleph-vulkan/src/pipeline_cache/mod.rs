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

use aleph_vulkan_core::erupt::vk1_0::PipelineCacheCreateInfoBuilder;
use aleph_vulkan_core::erupt::DeviceLoader;
use aleph_vulkan_core::{erupt, DebugName, Device};
use std::ffi::CStr;
use std::fs::OpenOptions;
use std::io::{ErrorKind, Read, Write};
use std::sync::atomic::{AtomicU64, Ordering};

static PIPELINE_CACHE: AtomicU64 = AtomicU64::new(0);
static CACHE_FILE_NAME: &'static str = "shader_cache.bin";

pub struct PipelineCache {}

impl PipelineCache {
    ///
    /// Internal function for loading the pipeline cache data from disk
    ///
    fn load_file_data() -> std::io::Result<Vec<u8>> {
        aleph_log::trace!("Reading pipeline cache data from disk");

        // Open the file if it already exists to read from it
        let mut file = std::fs::OpenOptions::new()
            .create(false)
            .read(true)
            .write(false)
            .open(CACHE_FILE_NAME)?;

        //
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;

        Ok(buf)
    }

    ///
    /// Initialize the global pipeline cache object
    ///
    pub fn init(device: &Device) {
        aleph_log::trace!("Creating pipeline cache");
        let data = Self::load_file_data();

        let create_info = if let Ok(data) = data.as_ref() {
            PipelineCacheCreateInfoBuilder::new()
                .initial_data_size(data.len())
                .initial_data(data.as_ptr() as *const core::ffi::c_void)
        } else {
            PipelineCacheCreateInfoBuilder::new()
        };

        let cache = unsafe {
            device
                .create_pipeline_cache(&create_info, None, None)
                .expect("Failed to create pipeline cache")
        };
        unsafe {
            let name = erupt::cstr!(concat!(module_path!(), "::PipelineCache"));
            let name = CStr::from_ptr(name);
            cache.add_debug_name(device, name);
        }

        PIPELINE_CACHE.store(cache.0, Ordering::Relaxed);

        // TODO: Find a replacement for this
        // device.defer_destruction(|device: &DeviceLoader| {
        //     aleph_log::trace!("Destroying pipeline cache");
        //     Self::store(device);
        //
        //     unsafe {
        //         let pipeline_cache = aleph_vulkan_core::erupt::vk1_0::PipelineCache(
        //             PIPELINE_CACHE.swap(0, Ordering::Relaxed),
        //         );
        //         device
        //             .destroy_pipeline_cache(Some(pipeline_cache), None);
        //     }
        // });
    }

    ///
    /// Store the pipeline cache data to a file
    ///
    pub fn store(device: &DeviceLoader) {
        aleph_log::trace!("Storing pipeline cache data to disk");
        let data = unsafe {
            let mut data_size = 0;

            device
                .get_pipeline_cache_data(
                    Self::get(),
                    &mut data_size as *mut _,
                    std::ptr::null_mut(),
                )
                .expect("Failed to get PipelineCache data length");

            let mut data = vec![0u8; data_size];

            device
                .get_pipeline_cache_data(
                    Self::get(),
                    &mut data_size as *mut _,
                    data.as_mut_ptr() as *mut _,
                )
                .expect("Failed to get PipelineCache data");

            data
        };

        match OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(CACHE_FILE_NAME)
        {
            Ok(mut file) => {
                file.write_all(&data)
                    .expect("Failed to write pipeline cache data to file");
            }
            Err(err) => match err.kind() {
                ErrorKind::PermissionDenied => {
                    aleph_log::warn!("Failed to save pipeline cache data: PermissionDenied");
                }
                _ => {
                    panic!("Failed to save pipeline cache data");
                }
            },
        }
    }

    ///
    /// Get the global pipeline cache handle
    ///
    pub fn get() -> aleph_vulkan_core::erupt::vk1_0::PipelineCache {
        let val =
            aleph_vulkan_core::erupt::vk1_0::PipelineCache(PIPELINE_CACHE.load(Ordering::Relaxed));

        if !val.is_null() {
            val
        } else {
            panic!("Attempting to use PipelineCache before it has been loaded")
        }
    }
}
