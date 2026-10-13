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

use std::ptr::NonNull;

use aleph_alloc::BVec;
use aleph_alloc::instrumentation::{IAllocationCategory, system};
use aleph_gpu_allocator::{AllocationDesc, GpuAllocation, MemoryLocation};
use aleph_nstr::NStr;
use aleph_profile::tracy_gpu::TracyQueue;
use aleph_rhi_impl_utils::{Rhi, RhiSystem, abort_on_unwind};
use ash::vk;
use parking_lot::Mutex;

use crate::device::Device;
use crate::internal::allocation_callbacks::GLOBAL;

pub struct SharedState {
    /// A free-list where retired pool ranges are placed, ready for re-use.
    range_free_list: BVec<QueryPoolBlock, RhiSystem>,

    /// The CPU timestamp of the last calibration event.
    previous_calibration: u64,

    /// The allocation identifier for 'buffer'. Needed so we can free the buffer later.
    allocation: Option<GpuAllocation>,
}

struct MakeSendSync {
    /// CPU mapped pointer for 'buffer'.
    mapped: NonNull<u64>,
}

unsafe impl Send for MakeSendSync {}
unsafe impl Sync for MakeSendSync {}

pub struct QueueTimingQueryManager {
    /// The query pool that will be used for timing queries on one particular queue.
    pool: vk::QueryPool,

    /// Buffer that will be used to read back the query pool results from the GPU. This will be the
    /// same size as the query pool.
    buffer: vk::Buffer,

    /// The time domain of the host CPU.
    time_domain: vk::TimeDomainKHR,

    /// Upper limit of deviation. If a calibration event deviation exceeds this value it will be
    /// rejected.
    max_deviation: u64,

    /// Applied to convert VK time to the host platform's time
    vk_factor: u64,

    /// The ID assigned to this queue with the profiler.
    id: aleph_profile::tracy_gpu::QueueId,

    /// State that must be behind a lock
    shared_state: Mutex<SharedState>,

    /// Wrapper to make the type send+sync without marking the whole type send+sync
    send_sync: MakeSendSync,
}

impl QueueTimingQueryManager {
    /// Constructs a new query manager using the given device.
    ///
    /// This will
    /// - Create a VkQueryPool
    /// - Allocate a read-back buffer of equal size to the query pool
    /// - Set up the range allocator
    pub fn new(name: &NStr, device: &Device) -> Option<Self> {
        // This both fetches the value we want, but also catches platforms that we don't support.
        let vk_factor = get_vk_factor();
        let vk_factor = match vk_factor {
            Some(v) => v,
            None => {
                log::error!("QueueTimingQueryManager is not supported on this platform.");
                return None;
            }
        };

        // This both fetches the host side time domain we want, but also catches platforms that we
        // don't support.
        let time_domain = match HOST_TIME_DOMAIN {
            Some(v) => v,
            None => {
                log::error!("QueueTimingQueryManager is not supported on this platform.");
                return None;
            }
        };

        // This will
        // 1. Check if the calibrated_timestamps extension is loaded and grab the function table.
        // 2. Try to fetch the list of time domains supported by the device/host.
        // 3. Check if the required set of time domains are supported.
        //
        // If any of these steps fail then we can't create the manager and return an error.
        let (loader, _) = match device.calibrated_timestamp.as_ref() {
            Some(v) => v,
            None => {
                log::error!(
                    "Can't construct 'QueueTimingQueryManager' without VK_KHR_calibrated_timestamps"
                );
                return None;
            }
        };
        let result = Rhi::with(|| unsafe {
            loader.get_physical_device_calibrateable_time_domains(device.adapter.physical_device)
        });
        let time_domains = match result {
            Ok(v) => v,
            Err(e) => {
                log::error!("Failed to get list of supported time domains: {e:?}");
                return None;
            }
        };
        let has_device = time_domains.contains(&vk::TimeDomainKHR::DEVICE);
        let has_host = time_domains.contains(&time_domain);
        if !has_device || !has_host {
            let a = vk::TimeDomainKHR::DEVICE;
            let b = time_domain;
            log::error!("GPU and host don't support the required time domains. {a:?} and {b:?}");
            return None;
        }

        let max_deviation = Self::get_max_deviation(device, time_domain, vk_factor);
        let initial_calibration =
            match Self::try_calibrate(device, time_domain, vk_factor, max_deviation, 8192) {
                Some(v) => v,
                None => {
                    log::error!("Failed to get calibration for time domain: {time_domain:?}");
                    return None;
                }
            };

        let timestamp_period = device
            .adapter
            .device_info
            .properties_10
            .limits
            .timestamp_period;
        let id = aleph_profile::tracy_gpu::QueueId::new_queue_id(
            name.to_cstr(),
            aleph_profile::tracy_gpu::ContextType::Vulkan,
            true,
            initial_calibration.gpu,
            timestamp_period,
        );

        // Attempts to create a query pool of the largest power of two size that we can make.
        //
        // The size will always be <= u16::MAX. We retry with smaller pools until we either get a
        // success, or if the pool size is getting too small to be useful we give up.
        let (pool, pool_size) = match Self::create_query_pool(device) {
            Some(v) => v,
            None => {
                log::error!("Failed to create timing query pool.");
                return None;
            }
        };

        // Next we have a 'shadow' buffer that matches the query pool in size. We use this buffer to
        // write query results from the GPU back to the CPU so they can be read of, and use an
        // identity mapping of query pool slots into this buffer.
        let create_info = vk::BufferCreateInfo {
            size: pool_size as vk::DeviceSize * size_of::<u64>() as vk::DeviceSize,
            usage: vk::BufferUsageFlags::TRANSFER_DST,
            ..Default::default()
        };
        let alloc_info = AllocationDesc {
            location: MemoryLocation::GpuToCpu,
            strategy: Default::default(),
            desc: create_info,
        };
        let (allocation, metadata, buffer) = unsafe {
            let result = device
                .allocator
                .as_ref()
                .unwrap()
                .allocate_buffer(device, &alloc_info);
            match result {
                Some(v) => v,
                None => {
                    log::error!("Failed to allocate read-back buffer.");
                    device.device.destroy_query_pool(pool, GLOBAL);
                    return None;
                }
            }
        };
        let mapped = if let Some(v) = metadata.mapped_address {
            v.cast::<u64>()
        } else {
            log::error!("Couldn't map the read-back buffer.");
            unsafe {
                device.device.destroy_query_pool(pool, GLOBAL);
                device.device.destroy_buffer(buffer, GLOBAL);
                device
                    .allocator
                    .as_ref()
                    .unwrap_unchecked()
                    .free_allocation(device, allocation);
            }
            return None;
        };

        // Now that we know the size of the query pool we can set up our range allocator. It's as
        // basic as they come, it's just a block allocator using a free list.
        //
        // We expect command encoders to grab new blocks as they need. The blocks get attached to
        // the command list and once the RHI has observed the commands as retired the block can be
        // reclaimed. Regular calls to IDevice::garbage_collect are important for this to work.
        let chunks = pool_size / QueryPoolBlock::SIZE as usize;
        let chunks = chunks as u16;
        let mut range_free_list = BVec::with_capacity_in(chunks as usize, system());
        for i in 0..chunks {
            range_free_list.push(QueryPoolBlock {
                begin: i * QueryPoolBlock::SIZE,
            });
        }

        let out = Self {
            pool,
            buffer,
            time_domain,
            max_deviation,
            vk_factor,
            id,
            shared_state: Mutex::new(SharedState {
                range_free_list,
                previous_calibration: initial_calibration.cpu,
                allocation: Some(allocation),
            }),
            send_sync: MakeSendSync { mapped },
        };
        Some(out)
    }

    /// Destroys all allocated GPU resources associated with the query manager.
    ///
    /// It's the caller's responsibility to ensure these aren't in use anymore and are safe to free.
    pub unsafe fn free(&self, device: &Device) {
        unsafe {
            device.device.destroy_query_pool(self.pool, GLOBAL);
            device.device.destroy_buffer(self.buffer, GLOBAL);
            device
                .allocator
                .as_ref()
                .unwrap_unchecked()
                .free_allocation(device, self.shared_state.lock().allocation.take().unwrap());
        }
    }

    /// Attempts to acquire a new block of query slots from the pool. May fail if all blocks have
    /// been taken.
    pub fn allocate_block(&self) -> Option<QueryPoolBlock> {
        self.shared_state.lock().range_free_list.pop()
    }

    /// Returns the given set of blocks back to the free list.
    ///
    /// # Safety
    ///
    /// It is the caller's responsibility to ensure the block is returned to the same manager it was
    /// allocated from.
    pub unsafe fn return_blocks(&self, iter: impl ExactSizeIterator<Item = QueryPoolBlock>) {
        self.shared_state.lock().range_free_list.extend(iter);
    }

    /// Issues a barrier onto the command list that will synchronize with
    /// 'cmd_copy_query_pool_results' and make the results of that copy available to the host for
    /// reading.
    ///
    /// This should be issued once after all ranges in a submission have been flushed.
    pub unsafe fn issue_flush_to_host(&self, device: &Device, command_buffer: vk::CommandBuffer) {
        unsafe {
            let barrier = vk::BufferMemoryBarrier2 {
                src_stage_mask: vk::PipelineStageFlags2::TRANSFER,
                src_access_mask: vk::AccessFlags2::TRANSFER_WRITE,
                dst_stage_mask: vk::PipelineStageFlags2::HOST,
                dst_access_mask: vk::AccessFlags2::HOST_READ,
                buffer: self.buffer,
                offset: 0,
                size: vk::WHOLE_SIZE,
                ..Default::default()
            };
            let dependency_info = vk::DependencyInfo::default()
                .buffer_memory_barriers(std::slice::from_ref(&barrier));
            device
                .device
                .cmd_pipeline_barrier2(command_buffer, &dependency_info);
        }
    }

    pub unsafe fn issue_begin_span_alloc(
        &self,
        device: &Device,
        command_buffer: vk::CommandBuffer,
        name: &NStr,
        function: &NStr,
        file: &NStr,
        line: u32,
        query_id: u16,
    ) {
        unsafe {
            device.device.cmd_write_timestamp2(
                command_buffer,
                vk::PipelineStageFlags2::BOTTOM_OF_PIPE,
                self.pool,
                query_id as u32,
            );
            aleph_profile::tracy_gpu::QueueId::begin_span_alloc(
                name.to_cstr(),
                function.to_cstr(),
                file.to_cstr(),
                line,
                self.id,
                query_id,
            );
        }
    }

    pub unsafe fn issue_end_span(
        &self,
        device: &Device,
        command_buffer: vk::CommandBuffer,
        query_id: u16,
    ) {
        unsafe {
            device.device.cmd_write_timestamp2(
                command_buffer,
                vk::PipelineStageFlags2::BOTTOM_OF_PIPE,
                self.pool,
                query_id as u32,
            );
            aleph_profile::tracy_gpu::QueueId::end_span(self.id, query_id);
        }
    }

    pub fn issue_calibration(&self, device: &Device) {
        let result = Self::try_calibrate(
            device,
            self.time_domain,
            self.vk_factor,
            self.max_deviation,
            10,
        );
        if let Some(new_calibration) = result {
            let mut shared_state = self.shared_state.lock();
            let delta = new_calibration.cpu - shared_state.previous_calibration;
            if delta > 0 {
                shared_state.previous_calibration = new_calibration.cpu;
                aleph_profile::tracy_gpu::QueueId::time_domain_calibration(
                    self.id,
                    new_calibration.gpu,
                    delta,
                );
            }
        }
    }

    fn create_query_pool(device: &Device) -> Option<(vk::QueryPool, usize)> {
        // Safety: We only ask for valid query pools with valid info structs.
        unsafe {
            // Try a few (power of two) sizes until the pool size is too small to be useful
            let sizes = cfg_select! {
                any(target_os = "macos", target_os = "ios") => [4096, 2048],
                _ => [65536, 32768, 16384, 8192, 4096, 2048],
            };
            for count in sizes {
                let info = vk::QueryPoolCreateInfo {
                    query_type: vk::QueryType::TIMESTAMP,
                    query_count: count,
                    ..Default::default()
                };
                if let Ok(pool) = device.device.create_query_pool(&info, GLOBAL) {
                    return Some((pool, count as usize));
                }
            }
            None
        }
    }

    fn get_calibrated_timestamps(
        device: &Device,
        host_domain: vk::TimeDomainKHR,
        vk_factor: u64,
    ) -> Option<CalibrationData> {
        let info = [
            vk::CalibratedTimestampInfoKHR {
                time_domain: vk::TimeDomainKHR::DEVICE,
                ..Default::default()
            },
            vk::CalibratedTimestampInfoKHR {
                time_domain: host_domain,
                ..Default::default()
            },
        ];
        let mut timestamps = [0u64; 2];
        let mut deviation = 0u64;

        unsafe {
            let (_, loader) = device.calibrated_timestamp.as_ref().expect(
                "Can't create QueueTimingQueryManager without calibrated_timestamps extension",
            );
            let result = (loader.fp().get_calibrated_timestamps_khr)(
                loader.device(),
                1,
                info.as_ptr(),
                timestamps.as_mut_ptr(),
                &mut deviation,
            );
            if result != vk::Result::SUCCESS {
                return None;
            }
        }

        Some(CalibrationData {
            cpu: timestamps[1] * vk_factor,
            gpu: timestamps[0],
            deviation,
        })
    }

    fn try_calibrate(
        device: &Device,
        host_domain: vk::TimeDomainKHR,
        vk_factor: u64,
        max_deviation: u64,
        max_attempts: usize,
    ) -> Option<CalibrationData> {
        for _ in 0..max_attempts {
            if let Some(data) = Self::get_calibrated_timestamps(device, host_domain, vk_factor) {
                if data.deviation > max_deviation {
                    continue;
                }
                return Some(data);
            }
        }
        None
    }

    fn get_max_deviation(device: &Device, host_domain: vk::TimeDomainKHR, vk_factor: u64) -> u64 {
        let mut samples = [u64::MAX; 32];
        for sample in samples.iter_mut() {
            if let Some(data) = Self::get_calibrated_timestamps(device, host_domain, vk_factor) {
                *sample = data.deviation;
            }
        }

        let min_deviation = samples.iter().min().unwrap();

        min_deviation.saturating_mul(3) / 2 // 1.5x min_deviation
    }
}

pub struct QueryPoolBlock {
    /// The position of the first query in the block.
    begin: u16,
}

impl QueryPoolBlock {
    pub const SIZE: u16 = 128;

    /// Returns the base query index of this block.
    pub const fn base_index(&self) -> u16 {
        self.begin
    }

    /// Resets the entire range owned by 'self'.
    ///
    /// This should be called when a range has been acquired for use on a queue. This function
    /// assumes that the range that 'self' encloses is no longer in use on the GPU and will reset
    /// the entire block unconditionally.
    ///
    /// This must be called before recording timestamp queries.
    pub unsafe fn acquire_on_queue(
        &self,
        manager: &QueueTimingQueryManager,
        device: &Device,
        command_buffer: vk::CommandBuffer,
    ) {
        unsafe {
            device.device.cmd_reset_query_pool(
                command_buffer,
                manager.pool,
                self.begin as _,
                Self::SIZE as _,
            );
        }
    }

    pub unsafe fn flush_on_queue(
        &self,
        manager: &QueueTimingQueryManager,
        device: &Device,
        command_buffer: vk::CommandBuffer,
        count: u16,
    ) {
        abort_on_unwind(|| assert!(count <= Self::SIZE));
        let dst_stride = size_of::<u64>() as vk::DeviceSize;
        let dst_offset = self.begin as vk::DeviceSize * dst_stride;
        let flags = vk::QueryResultFlags::TYPE_64 | vk::QueryResultFlags::WAIT;
        unsafe {
            device.device.cmd_copy_query_pool_results(
                command_buffer,
                manager.pool,
                self.begin as _,
                count as _,
                manager.buffer,
                dst_offset,
                dst_stride,
                flags,
            );
        }
    }

    pub unsafe fn notify_profiler(&self, manager: &QueueTimingQueryManager, count: u16) {
        abort_on_unwind(|| assert!(count <= Self::SIZE));
        for i in 0..count {
            let i = self.begin + i;
            unsafe {
                let gpu_time = manager.send_sync.mapped.add(i as usize).read();
                aleph_profile::tracy_gpu::QueueId::publish_gpu_time(manager.id, i, gpu_time);
            }
        }
    }
}

struct CalibrationData {
    cpu: u64,
    gpu: u64,
    deviation: u64,
}

const HOST_TIME_DOMAIN: Option<vk::TimeDomainKHR> = const {
    if cfg!(windows) {
        Some(vk::TimeDomainKHR::QUERY_PERFORMANCE_COUNTER)
    } else if cfg!(unix) {
        Some(vk::TimeDomainKHR::CLOCK_MONOTONIC_RAW)
    } else {
        None
    }
};

cfg_select! {
    windows => {
        fn get_vk_factor() -> Option<u64> {
            unsafe {
                use aleph_windows::Win32::System::Performance::QueryPerformanceFrequency;
                let mut out = 0i64;
                QueryPerformanceFrequency(&mut out).ok()?;
                let out = out as u64;
                let out = 1000000000 / out;
                Some(out)
            }
        }
    }
    unix => {
        fn get_vk_factor() -> Option<u64> {
            Some(1)
        }
    }
    _ => {
        fn get_vk_factor() -> Option<u64> {
            None
        }
    }
}
