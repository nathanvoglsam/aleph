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
use interfaces::anyhow::anyhow;
use interfaces::gpu::{
    BackendAPI, BufferCreateError, BufferDesc, CommandListSubmitError, CommandPoolCreateError,
    IBuffer, ICommandPool, IDevice, IGeneralCommandList, INamedObject, IShader, ITexture,
    ShaderBinary, ShaderCreateError, ShaderOptions, TextureCreateError, TextureDesc,
};
use interfaces::ref_ptr::{ref_ptr_init, ref_ptr_object, RefPtr, RefPtrObject};
use std::ffi::CString;

ref_ptr_object! {
    pub struct Device: IDevice, IDeviceExt {
        pub(crate) device_loader: erupt::DeviceLoader,
        pub(crate) queues: Queues,
        pub(crate) adapter: RefPtr<Adapter>,
        pub(crate) context: RefPtr<Context>,
    }
}

impl IDevice for Device {
    fn garbage_collect(&self) {
        todo!()
    }

    fn create_shader(
        &self,
        options: &ShaderOptions,
    ) -> Result<RefPtr<dyn IShader>, ShaderCreateError> {
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

            let shader = ref_ptr_init! {
                Shader {
                    device: self.as_ref_ptr(),
                    shader_type: options.shader_type,
                    module: module,
                    entry_point: options.entry_point.to_string(),
                }
            };
            let shader: RefPtr<Shader> = RefPtr::new(shader);

            Ok(shader.query_interface().unwrap())
        } else {
            Err(ShaderCreateError::UnsupportedShaderFormat)
        }
    }

    fn create_buffer(&self, _desc: &BufferDesc) -> Result<RefPtr<dyn IBuffer>, BufferCreateError> {
        todo!()
    }

    fn create_texture(
        &self,
        _desc: &TextureDesc,
    ) -> Result<RefPtr<dyn ITexture>, TextureCreateError> {
        todo!()
    }

    fn create_command_pool(&self) -> Result<RefPtr<dyn ICommandPool>, CommandPoolCreateError> {
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
