//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use crate::gpu::vk::Device;
use erupt::vk1_0::{PipelineCacheCreateInfoBuilder, Vk10DeviceLoaderExt};
use std::io::{ErrorKind, Read, Write};
use std::os::raw::c_void;
use std::sync::atomic::{AtomicU64, Ordering};
use std::fs::OpenOptions;

static PIPELINE_CACHE: AtomicU64 = AtomicU64::new(0);
static CACHE_FILE_NAME: &'static str = "shader_cache.bin";

pub struct PipelineCache {}

impl PipelineCache {

    ///
    /// Internal function for loading the pipeline cache data from disk
    ///
    fn load_file_data() -> Vec<u8> {
        log::trace!("Reading pipeline cache data from disk");

        // Try and open the file if it already exists to read from it
        match std::fs::OpenOptions::new()
            .create(false)
            .read(true)
            .write(false)
            .open(CACHE_FILE_NAME)
        {
            // Successfully opened the file so read the data from it
            Ok(mut file) => {
                let mut buf = Vec::new();
                file.read_to_end(&mut buf)
                    .expect("Failed to read from the pipeline cache file");
                buf
            }
            // There was an error when trying to open the file, so...
            Err(err) => match err.kind() {
                // Either the file didn't exist or we aren't allowed to open it so rather than
                // crashing we just send empty data back to the caller. This way the engine wont
                // crash if the cache hasn't been created yet or can't be created because of
                // permissions
                ErrorKind::NotFound | ErrorKind::PermissionDenied => Vec::new(),
                // Any other error is probably some odd circumstances so just panic
                _ => panic!("Failed to open the pipeline cache file"),
            },
        }
    }

    ///
    /// Initialize the global pipeline cache object
    ///
    pub fn init(device: &Device) {
        log::trace!("Creating pipeline cache");
        let initial_data_vec = Self::load_file_data();
        let initial_data: &[u8] = initial_data_vec.as_slice();
        let initial_data: &[c_void] = unsafe { std::mem::transmute(initial_data) };

        let create_info = PipelineCacheCreateInfoBuilder::new().initial_data(initial_data);

        let cache = unsafe {
            device
                .loader()
                .create_pipeline_cache(&create_info, None, None)
                .expect("Failed to create pipeline cache")
        };

        PIPELINE_CACHE.store(cache.0, Ordering::Relaxed);
    }

    ///
    /// Store the pipeline cache data to a file
    ///
    pub fn store(device: &Device) {
        log::trace!("Storing pipeline cache data to disk");
        let data = unsafe {
            let mut data_size = 0;

            device
                .loader()
                .get_pipeline_cache_data(
                    Self::get(),
                    &mut data_size as *mut _,
                    std::ptr::null_mut(),
                )
                .expect("Failed to get PipelineCache data length");

            let mut data = vec![0u8; data_size];

            device
                .loader()
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
            .open(CACHE_FILE_NAME) {
            Ok(mut file) => {
                file.write_all(&data)
                    .expect("Failed to write pipeline cache data to file");
            },
            Err(err) => {
                match err.kind() {
                    ErrorKind::PermissionDenied => {
                        log::warn!("Failed to save pipeline cache data: PermissionDenied");
                    },
                    _ => {
                        panic!("Failed to save pipeline cache data");
                    }
                }
            },
        }
    }

    ///
    /// Destroy the global pipeline cache data
    ///
    pub fn destroy(device: &Device) {
        log::trace!("Destroying pipeline cache");
        Self::store(device);

        unsafe {
            let pipeline_cache = erupt::vk1_0::PipelineCache(PIPELINE_CACHE.swap(0, Ordering::Relaxed));
            device.loader().destroy_pipeline_cache(pipeline_cache, None);
        }
    }

    ///
    /// Get the global pipeline cache handle
    ///
    pub fn get() -> erupt::vk1_0::PipelineCache {
        let val = erupt::vk1_0::PipelineCache(PIPELINE_CACHE.load(Ordering::Relaxed));

        if !val.is_null() {
            val
        } else {
            panic!("Attempting to use PipelineCache before it has been loaded")
        }
    }
}
