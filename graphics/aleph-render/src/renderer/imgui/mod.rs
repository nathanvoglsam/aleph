//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use vulkan::alloc::{AllocationCreateFlag, AllocationCreateInfoBuilder, Allocator, MemoryUsage};

use imgui::DrawCmd;
use std::sync::Arc;

mod font;
mod frame;
mod global;
mod singular;

pub use font::ImguiFont;
pub use frame::ImguiFrame;
pub use global::ImguiGlobal;
pub use singular::ImguiSingular;
use vulkan::core::erupt::vk1_0::{
    Buffer, BufferCreateInfoBuilder, BufferUsageFlags, ClearColorValue, ClearValue, CommandBuffer,
    CommandBufferBeginInfoBuilder, CommandBufferUsageFlags, CommandPoolResetFlags, DependencyFlags,
    Extent2D, Fence, ImageAspectFlags, ImageLayout, ImageMemoryBarrierBuilder,
    ImageSubresourceRangeBuilder, IndexType, Offset2D, PipelineBindPoint, PipelineStageFlags,
    Rect2DBuilder, RenderPassBeginInfoBuilder, Semaphore, ShaderStageFlags, SharingMode,
    SubmitInfoBuilder, SubpassContents, ViewportBuilder, Vk10DeviceLoaderExt, WHOLE_SIZE,
};

pub struct ImguiRenderer {
    device: Arc<vulkan::core::Device>,
    allocator: Arc<Allocator>,
    frames: Vec<ImguiFrame>,
    single: ImguiSingular,
    font: ImguiFont,
    global: ImguiGlobal,
}

impl ImguiRenderer {
    pub fn new(
        fonts: imgui::FontAtlasRefMut,
        device: Arc<vulkan::core::Device>,
        allocator: Arc<Allocator>,
        swapchain: &vulkan::core::Swapchain,
    ) -> Self {
        aleph_log::trace!("Initializing ImGui Renderer");
        let global = ImguiGlobal::init(&device);

        let font = ImguiFont::init(fonts, &global, &device, &allocator);

        let single = ImguiSingular::init(&device, &global, swapchain.format().format);

        let frames = (0..swapchain.images().len())
            .into_iter()
            .map(|index| {
                ImguiFrame::init(&device, &allocator, swapchain, index, single.render_pass)
            })
            .collect();

        ImguiRenderer {
            device,
            allocator,
            frames,
            single,
            font,
            global,
        }
    }

    pub unsafe fn recreate_resources(&mut self, swapchain: &vulkan::core::Swapchain) {
        for frame in self.frames.iter() {
            frame.destroy(&self.device, &self.allocator);
        }
        self.single.destroy(&self.device);

        self.single = ImguiSingular::init(&self.device, &self.global, swapchain.format().format);

        self.frames = (0..swapchain.images().len())
            .into_iter()
            .map(|index| {
                ImguiFrame::init(
                    &self.device,
                    &self.allocator,
                    &swapchain,
                    index,
                    self.single.render_pass,
                )
            })
            .collect();
    }

    pub unsafe fn render_frame(
        &mut self,
        frame: imgui::Ui,
        swapchain: &vulkan::core::Swapchain,
        acquire_semaphore: Semaphore,
        signal_semaphore: Semaphore,
        index: usize,
    ) {
        let draw_data = frame.render();

        //
        // Free the vertex buffer if one already exists
        //
        if self.frames[index].vtx_buffer.0 != Buffer::null() {
            self.allocator.destroy_buffer(
                self.frames[index].vtx_buffer.0,
                self.frames[index].vtx_buffer.1,
            );
        }

        //
        // Free the index buffer if one already exists
        //
        if self.frames[index].idx_buffer.0 != Buffer::null() {
            self.allocator.destroy_buffer(
                self.frames[index].idx_buffer.0,
                self.frames[index].idx_buffer.1,
            );
        }

        //
        // Make sure there is actually some vertex data to upload
        //
        if draw_data.total_vtx_count != 0 {
            //
            // Allocate a vertex buffer that will only be used for one frame
            //
            let buffer_create_info = BufferCreateInfoBuilder::new()
                .usage(BufferUsageFlags::VERTEX_BUFFER)
                .sharing_mode(SharingMode::EXCLUSIVE)
                .size(
                    draw_data.total_vtx_count as u64
                        * core::mem::size_of::<imgui::DrawVert>() as u64,
                );
            let alloc_create_info = AllocationCreateInfoBuilder::new()
                .usage(MemoryUsage::CPUToGPU)
                .pool(&self.frames[index].memory_pool)
                .build();
            self.frames[index].vtx_buffer = self
                .allocator
                .create_buffer(&buffer_create_info, &alloc_create_info)
                .expect("Failed to allocate per frame vertex buffer");

            //
            // Allocate an index buffer that will only be used for one frame
            //
            let buffer_create_info = BufferCreateInfoBuilder::new()
                .usage(BufferUsageFlags::INDEX_BUFFER)
                .sharing_mode(SharingMode::EXCLUSIVE)
                .size(
                    draw_data.total_idx_count as u64
                        * core::mem::size_of::<imgui::DrawIdx>() as u64,
                );
            let allocation_create_info = AllocationCreateInfoBuilder::new()
                .flags(AllocationCreateFlag::UPPER_ADDRESS_BIT)
                .usage(MemoryUsage::CPUToGPU)
                .pool(&self.frames[index].memory_pool)
                .build();
            self.frames[index].idx_buffer = self
                .allocator
                .create_buffer(&buffer_create_info, &allocation_create_info)
                .expect("Failed to allocate per frame index buffer");

            let vtx_buffer = self.frames[index].vtx_buffer;
            let idx_buffer = self.frames[index].idx_buffer;

            //
            // Map the vertex and index buffers
            //
            let mut vptr = self
                .allocator
                .map_memory(&vtx_buffer.1)
                .expect("Failed to map vertex buffer");
            let mut iptr = self
                .allocator
                .map_memory(&idx_buffer.1)
                .expect("Failed to map index buffer");

            //
            // Copy vertex and index buffer data into the vulkan buffers
            //
            draw_data.draw_lists().for_each(|list| {
                let vslice = list.vtx_buffer();
                vptr.copy_from(
                    vslice.as_ptr() as *const _,
                    vslice.len() * core::mem::size_of::<imgui::DrawVert>(),
                );
                vptr = vptr.add(vslice.len() * core::mem::size_of::<imgui::DrawVert>());

                let islice = list.idx_buffer();
                iptr.copy_from(
                    islice.as_ptr() as *const _,
                    islice.len() * core::mem::size_of::<imgui::DrawIdx>(),
                );
                iptr = iptr.add(islice.len() * core::mem::size_of::<imgui::DrawIdx>());
            });

            //
            // Flush and unmap the vertex and index buffers
            //
            self.allocator
                .flush_allocation(&vtx_buffer.1, 0, WHOLE_SIZE);
            self.allocator
                .flush_allocation(&idx_buffer.1, 0, WHOLE_SIZE);
            self.allocator.unmap_memory(&vtx_buffer.1);
            self.allocator.unmap_memory(&idx_buffer.1);
        }

        let vtx_buffer = self.frames[index].vtx_buffer;
        let idx_buffer = self.frames[index].idx_buffer;
        let command_buffer = self.frames[index].command_buffer;
        let command_pool = self.frames[index].command_pool;

        self.device
            .loader()
            .reset_command_pool(command_pool, CommandPoolResetFlags::default())
            .expect("Failed to reset command pool");

        //
        // Begin command buffer recording
        //
        let begin_info =
            CommandBufferBeginInfoBuilder::new().flags(CommandBufferUsageFlags::ONE_TIME_SUBMIT);
        self.device
            .loader()
            .begin_command_buffer(command_buffer, &begin_info)
            .expect("Failed to begin command buffer");

        //
        // We need to special case for when imgui wants to render nothing as vertex buffers wont
        // exist so we can't bind them. We still need to transition the image though so instead we
        // just insert a pipeline barrier to do the transition.
        //
        if draw_data.total_vtx_count != 0 {
            //
            // Begin the render pass
            //
            let clear_values = [ClearValue {
                color: ClearColorValue {
                    float32: [0.0, 0.0, 0.0, 1.0],
                },
            }];
            let render_area = Rect2DBuilder::new().extent(swapchain.extents());
            let render_pass_begin = RenderPassBeginInfoBuilder::new()
                .render_pass(self.single.render_pass)
                .framebuffer(self.frames[index].framebuffer)
                .clear_values(&clear_values)
                .render_area(*render_area);
            self.device.loader().cmd_begin_render_pass(
                command_buffer,
                &render_pass_begin,
                SubpassContents::INLINE,
            );

            self.reset_render_state(
                swapchain,
                command_buffer,
                vtx_buffer.0,
                idx_buffer.0,
                draw_data,
            );

            let clip_off = draw_data.display_pos;
            let clip_scale = draw_data.framebuffer_scale;

            let mut vtx_offset = 0;
            let mut idx_offset = 0;
            draw_data.draw_lists().for_each(|list| {
                list.commands().for_each(|command| {
                    self.render_draw_command(
                        draw_data,
                        swapchain,
                        vtx_buffer.0,
                        idx_buffer.0,
                        command_buffer,
                        &clip_off,
                        &clip_scale,
                        vtx_offset,
                        idx_offset,
                        command,
                    )
                });
                vtx_offset += list.vtx_buffer().len() as i32;
                idx_offset += list.idx_buffer().len() as u32;
            });

            self.device.loader().cmd_end_render_pass(command_buffer);
        } else {
            let src = PipelineStageFlags::BOTTOM_OF_PIPE;
            let dst = PipelineStageFlags::TOP_OF_PIPE;
            let flags = DependencyFlags::from_bits(0).unwrap();
            let range = ImageSubresourceRangeBuilder::new()
                .layer_count(1)
                .level_count(1)
                .base_array_layer(0)
                .base_mip_level(0)
                .aspect_mask(ImageAspectFlags::COLOR)
                .discard();
            let memory = [];
            let buffer = [];
            let image = [ImageMemoryBarrierBuilder::new()
                .image(swapchain.images()[index].image())
                .old_layout(ImageLayout::UNDEFINED)
                .new_layout(ImageLayout::PRESENT_SRC_KHR)
                .subresource_range(range)];
            self.device.loader().cmd_pipeline_barrier(
                command_buffer,
                src,
                dst,
                flags,
                &memory,
                &buffer,
                &image,
            );
        }

        self.device
            .loader()
            .end_command_buffer(command_buffer)
            .expect("Failed to end command buffer");

        let command_buffers = [command_buffer];
        let wait_semaphores = [acquire_semaphore];
        let signal_semaphores = [signal_semaphore];
        let wait_dst_stage_mask = [PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
        let submit = SubmitInfoBuilder::new()
            .command_buffers(&command_buffers)
            .wait_semaphores(&wait_semaphores)
            .signal_semaphores(&signal_semaphores)
            .wait_dst_stage_mask(&wait_dst_stage_mask);
        self.device
            .loader()
            .queue_submit(self.device.general_queue(), &[submit], Fence::null())
            .expect("Failed to submit to queue");
    }

    unsafe fn render_draw_command(
        &mut self,
        draw_data: &imgui::DrawData,
        swapchain: &vulkan::core::Swapchain,
        vertex_buffer: Buffer,
        index_buffer: Buffer,
        command_buffer: CommandBuffer,
        clip_off: &[f32; 2],
        clip_scale: &[f32; 2],
        vtx_offset: i32,
        idx_offset: u32,
        command: DrawCmd,
    ) {
        match command {
            DrawCmd::Elements { cmd_params, count } => {
                let clip_rect = [
                    (cmd_params.clip_rect[0] - clip_off[0]) * clip_scale[0],
                    (cmd_params.clip_rect[1] - clip_off[1]) * clip_scale[1],
                    (cmd_params.clip_rect[2] - clip_off[0]) * clip_scale[0],
                    (cmd_params.clip_rect[3] - clip_off[1]) * clip_scale[1],
                ];

                let swap_extent = swapchain.extents();
                if clip_rect[0] < swap_extent.width as f32
                    && clip_rect[1] < swap_extent.height as f32
                    && clip_rect[2] > 0.0
                    && clip_rect[3] > 0.0
                {
                    let offset = Offset2D {
                        x: (clip_rect[0] as i32).max(0),
                        y: (clip_rect[1] as i32).max(0),
                    };
                    let extent = Extent2D {
                        width: (clip_rect[2] - clip_rect[0]) as u32,
                        height: (clip_rect[3] - clip_rect[1]) as u32,
                    };
                    let scissor = Rect2DBuilder::new().offset(offset).extent(extent);
                    self.device
                        .loader()
                        .cmd_set_scissor(command_buffer, 0, &[scissor]);

                    self.device.loader().cmd_draw_indexed(
                        command_buffer,
                        count as _,
                        1,
                        cmd_params.idx_offset as u32 + idx_offset,
                        cmd_params.vtx_offset as i32 + vtx_offset,
                        0,
                    )
                }
            }
            DrawCmd::ResetRenderState => {
                self.reset_render_state(
                    swapchain,
                    command_buffer,
                    vertex_buffer,
                    index_buffer,
                    draw_data,
                );
            }
            DrawCmd::RawCallback { .. } => {
                unimplemented!("Cant use callback because bindings make it hard");
            }
        }
    }

    unsafe fn reset_render_state(
        &self,
        swapchain: &vulkan::core::Swapchain,
        command_buffer: CommandBuffer,
        vertex_buffer: Buffer,
        index_buffer: Buffer,
        draw_data: &imgui::DrawData,
    ) {
        //
        // Bind the pipeline and descriptor set that we'll be rendering with
        //
        self.device.loader().cmd_bind_pipeline(
            command_buffer,
            PipelineBindPoint::GRAPHICS,
            self.single.pipeline,
        );
        self.device.loader().cmd_bind_descriptor_sets(
            command_buffer,
            PipelineBindPoint::GRAPHICS,
            self.global.pipeline_layout.pipeline_layout(),
            0,
            &[self.global.descriptor_set],
            &[],
        );

        //
        // Bind the vertex and index buffers to render with
        //
        self.device
            .loader()
            .cmd_bind_vertex_buffers(command_buffer, 0, &[vertex_buffer], &[0]);
        self.device.loader().cmd_bind_index_buffer(
            command_buffer,
            index_buffer,
            0,
            IndexType::UINT16,
        );

        //
        // Set the viewport state, we're going to be rendering to the whole frame
        //
        let viewport = ViewportBuilder::new()
            .width(swapchain.extents().width as f32)
            .height(swapchain.extents().height as f32)
            .min_depth(0.0)
            .max_depth(1.0)
            .x(0.0)
            .y(0.0);
        self.device
            .loader()
            .cmd_set_viewport(command_buffer, 0, &[viewport]);

        //
        // Push transforms via push constants
        //
        let scale = [
            2.0 / draw_data.display_size[0],
            2.0 / draw_data.display_size[1],
        ];
        self.device.loader().cmd_push_constants(
            command_buffer,
            self.global.pipeline_layout.pipeline_layout(),
            ShaderStageFlags::VERTEX,
            0,
            8,
            scale.as_ptr() as *const _,
        );
        let translate = [
            -1.0 - draw_data.display_pos[0] * scale[0],
            -1.0 - draw_data.display_pos[1] * scale[1],
        ];
        self.device.loader().cmd_push_constants(
            command_buffer,
            self.global.pipeline_layout.pipeline_layout(),
            ShaderStageFlags::VERTEX,
            8,
            8,
            translate.as_ptr() as *const _,
        );
    }
}

impl Drop for ImguiRenderer {
    fn drop(&mut self) {
        unsafe {
            for frame in self.frames.iter() {
                frame.destroy(&self.device, &self.allocator)
            }
            self.single.destroy(&self.device);
            self.font.destroy(&self.device, &self.allocator);
            self.global.destroy(&self.device);
        }
    }
}
