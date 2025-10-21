use std::{error::Error, sync::Arc};

use vulkano::{
    DeviceSize, Validated, ValidationError, VulkanError, VulkanLibrary,
    buffer::{BufferCreateInfo, BufferUsage, Subbuffer},
    command_buffer::{
        AutoCommandBufferBuilder, CommandBufferUsage, RenderPassBeginInfo, SubpassBeginInfo,
        SubpassContents, SubpassEndInfo,
        allocator::{StandardCommandBufferAllocator, StandardCommandBufferAllocatorCreateInfo},
    },
    device::{
        self, Device, DeviceCreateInfo, DeviceExtensions, QueueCreateInfo, QueueFlags,
        physical::PhysicalDevice,
    },
    image::{ImageUsage, view::ImageView},
    instance::{Instance, InstanceCreateInfo, InstanceExtensions},
    memory::allocator::{
        AllocationCreateInfo, FreeListAllocator, GenericMemoryAllocator, MemoryTypeFilter,
        StandardMemoryAllocator,
    },
    pipeline::{
        GraphicsPipeline, PipelineLayout, PipelineShaderStageCreateInfo,
        graphics::{
            GraphicsPipelineCreateInfo,
            color_blend::{ColorBlendAttachmentState, ColorBlendState},
            input_assembly::InputAssemblyState,
            multisample::MultisampleState,
            rasterization::RasterizationState,
            vertex_input::{Vertex, VertexDefinition},
            viewport::{Viewport, ViewportState},
        },
        layout::PipelineDescriptorSetLayoutCreateInfo,
    },
    render_pass::{Framebuffer, FramebufferCreateInfo, RenderPass, Subpass},
    shader::ShaderModule,
    swapchain::{self, Surface, Swapchain, SwapchainCreateInfo, SwapchainPresentInfo},
    sync::{self, GpuFuture},
};

use crate::{
    event::window::Window,
    rendering::{BufferValue, Index, RendererRes, vulkan::commands::VulkanCommandsCtx},
};

use super::{CreationError, Render, RenderError, RenderIndex, RendererOk, types::Vertex3D};

pub use vs_pbr::ViewUniforms;

pub mod buffer;

mod commands;

#[derive(Debug)]
pub(super) struct VulkanContext {
    // Instances
    library: Arc<VulkanLibrary>,
    instance: Arc<Instance>,
    physical_device: Arc<PhysicalDevice>,
    device: Arc<Device>,
    // Queues
    graphics_queue: Arc<device::Queue>,
    // Allocators
    memory_allocator: Arc<GenericMemoryAllocator<FreeListAllocator>>,
    command_buffer_allocator: Arc<StandardCommandBufferAllocator>,
    // pbr_render_pass: Arc<RenderPass>,
    // pbr_subpass: Arc<Subpass>,
}

/*
#[derive(Debug)]
pub struct VulkanRenderContext {
    render_pass: Arc<RenderPass>,
    command_buffers: Vec<CommandBuffer>
}
*/

#[derive(Debug)]
pub struct VulkanSurfaceContext {
    recreate_swapchain: bool,

    surface: Arc<Surface>,
    swapchain: Arc<Swapchain>,
    images: Vec<Arc<vulkano::image::Image>>,

    render_pass: Arc<RenderPass>,
    subpass: Subpass,

    pbr_pipeline: Arc<GraphicsPipeline>,

    framebuffers: Vec<Arc<Framebuffer>>,
    // render_contexts: Vec<VulkanRenderContext>
}

fn get_render_pass(
    device: &Arc<Device>,
    swapchain: &Arc<Swapchain>,
) -> RendererRes<Arc<RenderPass>> {
    vulkano::single_pass_renderpass!(
        device.clone(),
        attachments: {
            color: {
                // Set the format the same as the swapchain.
                format: swapchain.image_format(),
                samples: 1,
                load_op: Clear,
                store_op: Store,
            },
        },
        pass: {
            color: [color],
            depth_stencil: {},
        },
    )
    .map_err(|_| RenderError::Creation("Failed to create render pass.".into()))
}

fn get_framebuffers(
    images: &[Arc<vulkano::image::Image>],
    render_pass: &Arc<RenderPass>,
) -> Vec<Arc<Framebuffer>> {
    images
        .iter()
        .map(|image| {
            let view = ImageView::new_default(image.clone()).unwrap();

            Framebuffer::new(
                render_pass.clone(),
                FramebufferCreateInfo {
                    attachments: vec![view],
                    ..Default::default()
                },
            )
            .unwrap()
        })
        .collect::<Vec<_>>()
}

#[derive(Debug)]
pub struct VulkanRenderer {
    vk: VulkanContext,

    surface_ctx: Option<VulkanSurfaceContext>,

    pipelines: Vec<super::Pipeline>,
    current_pipeline_index: RenderIndex,

    current_vertex_buffer_index: RenderIndex,
    current_index_buffer_index: RenderIndex,

    default_texture: RenderIndex,
    // pbr_pipeline: Arc<GraphicsPipeline>,
}

type VkBuffer = vulkano::buffer::Buffer;

impl Render for VulkanRenderer {
    type VertexBufferType<V: BufferValue> = buffer::VulkanVertexBuffer<V>;
    type IndexBufferType = buffer::VulkanIndexBuffer;
    type CommandsCtx = VulkanCommandsCtx;

    fn set_window(&mut self, window: &mut Window) -> RendererOk {
        if self.surface_ctx.is_some() {
            return Err(RenderError::Creation(
                "Unable to bind new surface to renderer without unbinding the existing one.".into(),
            ));
        }

        let surface =
            unsafe { Surface::from_window_ref(self.vk.instance.clone(), &window.handle()) }
                .map_err(|e| RenderError::Creation(e.to_string()))?;

        let caps = self
            .vk
            .physical_device
            .surface_capabilities(&surface, Default::default())
            .expect("failed to get surface capabilities");

        let (width, height) = window.handle().get_size();

        let composite_alpha = caps.supported_composite_alpha.into_iter().next().unwrap();
        let image_format = self
            .vk
            .physical_device
            .surface_formats(&surface, Default::default())
            .unwrap()[0]
            .0;

        let (swapchain, images) = Swapchain::new(
            self.vk.device.clone(),
            surface.clone(),
            SwapchainCreateInfo {
                min_image_count: caps.min_image_count + 1,
                image_format,
                image_extent: [width as u32, height as u32],
                image_usage: ImageUsage::COLOR_ATTACHMENT,
                composite_alpha,
                ..Default::default()
            },
        )
        .unwrap();

        let render_pass = get_render_pass(&self.vk.device, &swapchain)?;
        let subpass: Subpass = Subpass::from(render_pass.clone(), 0)
            .ok_or(CreationError("Unable to create subpass 0.".to_string()))?;

        let framebuffers = get_framebuffers(&images, &render_pass);

        let vs_3d: Arc<ShaderModule> =
            vs_pbr::load(self.vk.device.clone()).expect("Unable to compile PBR Vertex Shader.");
        let fs_3d: Arc<ShaderModule> =
            fs_pbr::load(self.vk.device.clone()).expect("Unable to compile PBR Fragment Shader.");

        let pbr_pipeline =
            create_pipeline(&self.vk.device.clone(), subpass.clone(), &vs_3d, &fs_3d)?;

        self.surface_ctx = Some(VulkanSurfaceContext {
            surface,
            swapchain,
            images,
            recreate_swapchain: false,
            render_pass,
            framebuffers,
            subpass,
            pbr_pipeline,
        });

        Ok(())
    }

    fn begin_frame(&mut self) -> RendererOk {
        Ok(())
    }

    fn end_frame(&mut self) -> RendererOk {
        Ok(())
    }

    fn bind_pipeline(&mut self, pipeline_index: RenderIndex) -> RendererOk {
        todo!()
    }

    fn pipeline(&self) -> &super::Pipeline {
        &self.pipelines[self.current_pipeline_index]
    }

    fn pipeline_mut(&mut self) -> &mut super::Pipeline {
        &mut self.pipelines[self.current_pipeline_index]
    }

    fn pipelines(&self) -> &[super::Pipeline] {
        todo!()
    }

    fn create_vertex_buffer<V: BufferValue>(
        &mut self,
        capacity: usize,
    ) -> RendererRes<Self::VertexBufferType<V>> {
        let vb: Subbuffer<[V]> = vulkano::buffer::Buffer::new_slice::<V>(
            self.vk.memory_allocator.clone(),
            BufferCreateInfo {
                usage: BufferUsage::VERTEX_BUFFER,
                ..Default::default()
            },
            AllocationCreateInfo {
                // TODO: Make this prefer device, and add a staging buffer
                memory_type_filter: MemoryTypeFilter::PREFER_HOST
                    | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
            },
            capacity as DeviceSize,
        )?;

        Ok(Self::VertexBufferType::<V> { subbuffer: vb })
    }

    fn create_index_buffer(&mut self, capacity: usize) -> RendererRes<Self::IndexBufferType> {
        let ib: Subbuffer<[Index]> = vulkano::buffer::Buffer::new_slice::<Index>(
            self.vk.memory_allocator.clone(),
            BufferCreateInfo {
                usage: BufferUsage::INDEX_BUFFER,
                ..Default::default()
            },
            AllocationCreateInfo {
                // TODO: Make this prefer device, and add a staging buffer
                memory_type_filter: MemoryTypeFilter::PREFER_HOST
                    | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
            },
            capacity as DeviceSize,
        )?;

        Ok(Self::IndexBufferType { subbuffer: ib })
    }

    fn set_view_uniforms(&mut self, view_uniforms: vs_pbr::ViewUniforms) {
        println!("Setting the view uniforms.");
    }

    fn create_image(&mut self, params: super::ImageParams) -> Arc<super::ImageHandle> {
        todo!()
    }

    fn default_texture(&self) -> RenderIndex {
        self.default_texture
    }

    fn run_commands<F>(&mut self, mut f: F) -> RendererOk
    where
        F: FnMut(&mut Self::CommandsCtx) -> RendererOk,
    {
        let surface_ctx = self.surface_ctx.as_mut().ok_or(RenderError::Draw(
            "No surface to draw onto registered in Vulkan renderer.".into(),
        ))?;

        let (image_index, suboptimal, acquire_future) =
            match swapchain::acquire_next_image(surface_ctx.swapchain.clone(), None)
                .map_err(Validated::unwrap)
            {
                Ok(r) => r,
                Err(VulkanError::OutOfDate) => {
                    surface_ctx.recreate_swapchain = true;
                    return Ok(());
                }
                Err(e) => panic!("Unexpected error: {}", e),
            };

        if suboptimal {
            surface_ctx.recreate_swapchain = true
        }

        let mut ctx = VulkanCommandsCtx {
            builder: AutoCommandBufferBuilder::primary(
                self.vk.command_buffer_allocator.clone(),
                self.vk.graphics_queue.queue_family_index(),
                CommandBufferUsage::OneTimeSubmit,
            )
            .map_err(|_| RenderError::Draw("Failed to create command buffer.".to_string()))?,
        };

        ctx.builder
            .begin_render_pass(
                RenderPassBeginInfo {
                    clear_values: vec![Some([0.0, 0.0, 1.0, 1.0].into())],
                    ..RenderPassBeginInfo::framebuffer(
                        surface_ctx.framebuffers[image_index as usize].clone(),
                    )
                },
                SubpassBeginInfo {
                    contents: SubpassContents::Inline,
                    ..Default::default()
                },
            )
            .map_err(|_| RenderError::Draw("Failed to begin render pass.".to_string()))?;

        f(&mut ctx)?;

        let VulkanCommandsCtx { mut builder } = ctx;

        builder
            .end_render_pass(SubpassEndInfo::default())
            .map_err(|_| RenderError::Draw("Unable to end render pass".to_string()))?;

        let command_buffer = builder
            .build()
            .map_err(|_| RenderError::Draw("Unable to build command buffer.".to_string()))?;

        let execution = sync::now(self.vk.device.clone())
            .join(acquire_future)
            .then_execute(self.vk.graphics_queue.clone(), command_buffer.clone())
            .unwrap()
            .then_swapchain_present(
                self.vk.graphics_queue.clone(),
                SwapchainPresentInfo::swapchain_image_index(
                    surface_ctx.swapchain.clone(),
                    image_index,
                ),
            )
            .then_signal_fence_and_flush();

        match execution.map_err(Validated::unwrap) {
            Ok(future) => {
                // Wait for the GPU to finish.
                future.wait(None).unwrap();
            }
            Err(VulkanError::OutOfDate) => {
                surface_ctx.recreate_swapchain = true;
            }
            Err(e) => {
                println!("failed to flush future: {e}");
            }
        };

        /*

        let future = sync::now(self.vk.device.clone())
            .then_execute(self.vk.graphics_queue.clone(), command_buffer)
            .unwrap()
            .then_signal_fence_and_flush()
            .unwrap();
        future.wait(None).unwrap();
            */

        Ok(())
    }

    /*
    fn set_vertex_buffer(&mut self, buffer_index: RenderIndex) -> super::RendererOk {
        println!("Setting vertex buffer to index {}.", buffer_index);
        Ok(())
    }

    fn vertex_buffers(&mut self) -> Result<&[super::VertexBuffer], super::RenderError> {
        todo!()
    }
    */
}

impl From<ValidationError> for CreationError {
    fn from(value: ValidationError) -> Self {
        CreationError(format!("{:?}", value))
    }
}

mod vs_pbr {
    vulkano_shaders::shader! {
        ty: "vertex",
        // path: "src/rendering/default_shaders/default_3d.vert"
        path: "src/rendering/default_shaders/default_3d.vert"
    }
}

mod fs_pbr {
    vulkano_shaders::shader! {
        ty: "fragment",
        path: "src/rendering/default_shaders/default_3d.frag"
    }
}

impl VulkanRenderer {
    pub fn new() -> RendererRes<VulkanRenderer> {
        (|| -> Result<Self, Box<dyn Error>> {
            {
                let library = VulkanLibrary::new()?;

                // TODO: Make this try X11 if wayland fails
                let extensions = InstanceExtensions {
                    khr_surface: true,
                    khr_wayland_surface: true,
                    ..InstanceExtensions::empty()
                };

                let instance = Instance::new(
                    library.clone(),
                    InstanceCreateInfo {
                        enabled_extensions: extensions,
                        ..InstanceCreateInfo::application_from_cargo_toml()
                    },
                )?;

                let device_extensions = DeviceExtensions {
                    khr_swapchain: true,
                    ..DeviceExtensions::empty()
                };

                // Get the first Vulkan capable device
                let physical_device = instance
                    .enumerate_physical_devices()?
                    .find(|pd| pd.supported_extensions().contains(&device_extensions))
                    .ok_or(CreationError("no devices available".to_string()))?;

                for family in physical_device.queue_family_properties() {
                    println!(
                        "Found a queue family with {:?} queue(s)",
                        family.queue_count
                    );
                }

                let queue_family_index = physical_device
                    .queue_family_properties()
                    .iter()
                    .enumerate()
                    .position(|(_queue_family_index, queue_family_properties)| {
                        queue_family_properties
                            .queue_flags
                            .contains(QueueFlags::GRAPHICS)
                    })
                    .expect("couldn't find a graphical queue family")
                    as u32;

                let (device, mut queues) = Device::new(
                    physical_device.clone(),
                    DeviceCreateInfo {
                        // here we pass the desired queue family to use by index
                        queue_create_infos: vec![QueueCreateInfo {
                            queue_family_index,
                            ..Default::default()
                        }],
                        enabled_extensions: device_extensions,
                        ..Default::default()
                    },
                )
                .expect("failed to create device");

                // Assumed only a single queue, the graphics queue
                let queue = queues.next().ok_or(CreationError(
                    "Unable to get first device queue.".to_string(),
                ))?;

                let memory_allocator =
                    Arc::new(StandardMemoryAllocator::new_default(device.clone()));
                let command_buffer_allocator = Arc::new(StandardCommandBufferAllocator::new(
                    device.clone(),
                    StandardCommandBufferAllocatorCreateInfo::default(),
                ));

                /*
                let render_pass = RenderPass::new(
                    device.clone(),
                    RenderPassCreateInfo {
                        subpasses: vec![SubpassDescription {
                            color_attachments: vec![Some(AttachmentReference {
                                attachment: todo!(),
                                layout: todo!(),
                                stencil_layout: todo!(),
                                aspects: todo!(),
                                _ne: todo!(),
                            })],
                            ..Default::default()
                        }],
                        ..Default::default()
                    },
                )?;
                */

                // TODO: Implement an actual default texture here
                let default_texture = 0;

                Ok(VulkanRenderer {
                    vk: VulkanContext {
                        library,
                        instance,
                        physical_device,
                        device,
                        graphics_queue: queue,
                        memory_allocator,
                        command_buffer_allocator,
                        // pbr_render_pass: render_pass,
                        // pbr_subpass: subpass.into(),
                    },
                    surface_ctx: None,
                    pipelines: vec![],
                    current_pipeline_index: 0,

                    current_vertex_buffer_index: 0,

                    current_index_buffer_index: 0,
                    default_texture,
                    // pbr_pipeline: pipeline,
                })
            }
        })()
        .map_err(|e| RenderError::Creation(format!("{:?}", e)))
    }
}

fn create_pipeline(
    device: &Arc<Device>,
    subpass: Subpass,
    vertex_shader: &Arc<ShaderModule>,
    fragment_shader: &Arc<ShaderModule>,
) -> Result<Arc<GraphicsPipeline>, CreationError> {
    let pbr_vs = vertex_shader.entry_point("main").ok_or(CreationError(
        "Failed to get entry point main of PBR vertex shader.".to_string(),
    ))?;

    let pbr_fs = fragment_shader.entry_point("main").ok_or(CreationError(
        "Failed to get entry point main of PBR fragment shader.".to_string(),
    ))?;

    let pbr_vertex_input = Vertex3D::per_vertex().definition(&pbr_vs).map_err(|e| {
        CreationError(format!(
            "Unable to get vertex definition for PBR vertex shader. Error: {}",
            e
        ))
    })?;

    let stages = [
        PipelineShaderStageCreateInfo::new(pbr_vs),
        PipelineShaderStageCreateInfo::new(pbr_fs),
    ];

    let layout = PipelineLayout::new(
        device.clone(),
        PipelineDescriptorSetLayoutCreateInfo::from_stages(&stages)
            .into_pipeline_layout_create_info(device.clone())
            .map_err(|e| CreationError(e.to_string()))?,
    )
    .map_err(|e| CreationError(e.to_string()))?;

    let viewport = Viewport {
        offset: [0f32, 0f32],
        extent: [1024.0, 1024.0],
        depth_range: 0.0..=1.0,
    };

    GraphicsPipeline::new(
        device.clone(),
        None,
        GraphicsPipelineCreateInfo {
            stages: stages.into_iter().collect(),
            vertex_input_state: Some(pbr_vertex_input),
            // Can manually specify draw type
            input_assembly_state: Some(InputAssemblyState::default()),

            viewport_state: Some(ViewportState {
                viewports: [viewport].into_iter().collect(),
                ..Default::default()
            }),

            rasterization_state: Some(RasterizationState::default()),
            multisample_state: Some(MultisampleState::default()),
            color_blend_state: Some(ColorBlendState::with_attachment_states(
                subpass.num_color_attachments(),
                ColorBlendAttachmentState::default(),
            )),

            subpass: Some(subpass.into()),
            ..GraphicsPipelineCreateInfo::layout(layout)
        },
    )
    .map_err(|e| CreationError(format!("{:?}", e)))
}
