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

use crate::adapter::Adapter;
use crate::context::Context;
use crate::internal::queues::Queues;
use crate::shader::Shader;
use byteorder::{ByteOrder, NativeEndian};
use erupt::vk;
use interfaces::any::{declare_interfaces, AnyArc, AnyWeak};
use interfaces::anyhow::anyhow;
use interfaces::gpu::{
    BackendAPI, BufferCreateError, BufferDesc, CommandListSubmitError, CommandPoolCreateError,
    IAcquiredTexture, IBuffer, ICommandPool, IDevice, IGeneralCommandList, INamedObject, ISampler,
    IShader, ITexture, QueuePresentError, SamplerDesc, ShaderBinary, ShaderCreateError,
    ShaderOptions, TextureCreateError, TextureDesc,
};
use std::ffi::CString;

pub struct Device {
    pub(crate) this: AnyWeak<Self>,
    pub(crate) context: AnyArc<Context>,
    pub(crate) adapter: AnyArc<Adapter>,
    pub(crate) device_loader: erupt::DeviceLoader,
    pub(crate) queues: Queues,
}

declare_interfaces!(Device, [IDevice, IDeviceExt]);

impl IDevice for Device {
    fn upgrade(&self) -> AnyArc<dyn IDevice> {
        self.this.upgrade().unwrap().query_interface().unwrap()
    }

    fn garbage_collect(&self) {
        todo!()
    }

    fn wait_idle(&self) {
        todo!()
    }

    fn create_shader(
        &self,
        options: &ShaderOptions,
    ) -> Result<AnyArc<dyn IShader>, ShaderCreateError> {
        if let ShaderBinary::Spirv(data) = options.data {
            // Vulkan shaders must always have a buffer length that is a multiple of 4. SPIR-V's binary
            // representation is a sequence of u32 values.
            if data.len() % 4 != 0 || data.is_empty() {
                return Err(ShaderCreateError::InvalidInputSize(data.len()));
            }

            // We need to copy the data into a u32 buffer to satisfy alignment requirements
            let data: Vec<u32> = data.chunks_exact(4).map(NativeEndian::read_u32).collect();

            let module = unsafe {
                let create_info = vk::ShaderModuleCreateInfoBuilder::new().code(&data);
                self.device_loader
                    .create_shader_module(&create_info, None)
                    .map_err(|v| anyhow!(v))?
            };

            let shader = AnyArc::new_cyclic(move |v| Shader {
                this: v.clone(),
                device: self.this.upgrade().unwrap(),
                shader_type: options.shader_type,
                module,
                entry_point: options.entry_point.to_string(),
            });
            Ok(shader.query_interface().unwrap())
        } else {
            Err(ShaderCreateError::UnsupportedShaderFormat)
        }
    }

    fn create_buffer(&self, _desc: &BufferDesc) -> Result<AnyArc<dyn IBuffer>, BufferCreateError> {
        todo!()
    }

    fn create_texture(
        &self,
        _desc: &TextureDesc,
    ) -> Result<AnyArc<dyn ITexture>, TextureCreateError> {
        todo!()
    }

    fn create_sampler(&self, desc: &SamplerDesc) -> Result<AnyArc<dyn ISampler>, ()> {
        todo!()
    }

    fn create_command_pool(&self) -> Result<AnyArc<dyn ICommandPool>, CommandPoolCreateError> {
        todo!()
    }

    unsafe fn general_queue_submit_list(
        &self,
        _command_list: Box<dyn IGeneralCommandList>,
    ) -> Result<(), CommandListSubmitError> {
        todo!()
    }

    unsafe fn general_queue_submit_lists(
        &self,
        _command_lists: &mut dyn Iterator<Item = Box<dyn IGeneralCommandList>>,
    ) -> Result<(), CommandListSubmitError> {
        todo!()
    }

    unsafe fn general_queue_present(
        &self,
        image: Box<dyn IAcquiredTexture>,
    ) -> Result<(), QueuePresentError> {
        todo!()
    }

    fn get_backend_api(&self) -> BackendAPI {
        BackendAPI::Vulkan
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe {
            self.device_loader.destroy_device(None);
        }
    }
}

pub trait IDeviceExt: IDevice {
    fn get_raw_handle(&self) -> &erupt::DeviceLoader;
    fn get_raw_general_queue(&self) -> Option<vk::Queue>;
    fn get_raw_compute_queue(&self) -> Option<vk::Queue>;
    fn get_raw_transfer_queue(&self) -> Option<vk::Queue>;
}

impl IDeviceExt for Device {
    fn get_raw_handle(&self) -> &erupt::DeviceLoader {
        &self.device_loader
    }

    fn get_raw_general_queue(&self) -> Option<vk::Queue> {
        self.queues.general.as_ref().map(|v| v.queue)
    }

    fn get_raw_compute_queue(&self) -> Option<vk::Queue> {
        self.queues.compute.as_ref().map(|v| v.queue)
    }

    fn get_raw_transfer_queue(&self) -> Option<vk::Queue> {
        self.queues.transfer.as_ref().map(|v| v.queue)
    }
}

impl INamedObject for Device {
    fn set_name(&self, name: &str) {
        let loader = &self.device_loader;
        if let Some(func) = loader.set_debug_utils_object_name_ext {
            let name = CString::new(name).unwrap();
            let info = vk::DebugUtilsObjectNameInfoEXTBuilder::new()
                .object_type(vk::ObjectType::DEVICE)
                .object_handle(self.device_loader.handle.object_handle())
                .object_name(&name);
            unsafe {
                (func)(loader.handle, &info.build());
            }
        }
    }
}
