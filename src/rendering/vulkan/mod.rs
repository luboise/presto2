use std::{error::Error, sync::Arc};

use vulkano::{
    DeviceSize, ValidationError, VulkanLibrary,
    buffer::{BufferCreateInfo, BufferUsage, Subbuffer},
    command_buffer::allocator::{
        StandardCommandBufferAllocator, StandardCommandBufferAllocatorCreateInfo,
    },
    device::{
        self, Device, DeviceCreateInfo, QueueCreateInfo, QueueFlags, physical::PhysicalDevice,
    },
    format::Format,
    instance::{Instance, InstanceCreateInfo},
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
    render_pass::{RenderPass, Subpass},
    shader::ShaderModule,
    single_pass_renderpass,
};

use crate::rendering::{BufferValue, RendererRes};

use super::{CreationError, Render, RenderError, RenderIndex, RendererOk, types::Vertex3D};

pub use vs_pbr::ViewUniforms;

pub mod buffer;

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

    pbr_render_pass: Arc<RenderPass>,
    pbr_subpass: Arc<Subpass>,
}

#[derive(Debug)]
pub struct VulkanRenderer {
    vk: VulkanContext,

    pipelines: Vec<super::Pipeline>,
    current_pipeline_index: RenderIndex,

    current_vertex_buffer_index: RenderIndex,
    current_index_buffer_index: RenderIndex,

    default_texture: RenderIndex,

    pbr_pipeline: Arc<GraphicsPipeline>,
}

type VkBuffer = vulkano::buffer::Buffer;

impl Render for VulkanRenderer {
    type VertexBufferType<V: BufferValue> = buffer::VulkanVertexBuffer<V>;
    type IndexBufferType = buffer::VulkanIndexBuffer;

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

    fn index_buffer(&self, buffer_index: RenderIndex) -> Option<super::IndexBuffer> {
        todo!()
    }

    fn set_index_buffer(&mut self, buffer_index: RenderIndex) -> super::RendererOk {
        println!("Setting index buffer to index {}.", buffer_index);
        Ok(())
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

    fn create_index_buffer(&mut self) -> RendererRes<&Self::IndexBufferType> {
        todo!()
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

                let instance = Instance::new(
                    library.clone(),
                    InstanceCreateInfo::application_from_cargo_toml(),
                )?;

                // Get the first Vulkan capable device
                let physical_device = instance
                    .enumerate_physical_devices()?
                    // .expect("could not enumerate devices")
                    .next().ok_or(CreationError("no devices available".to_string()))?
                    // .expect()
                    ;

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

                let vs_3d: Arc<ShaderModule> =
                    vs_pbr::load(device.clone()).expect("Unable to compile PBR Vertex Shader.");
                let fs_3d: Arc<ShaderModule> =
                    fs_pbr::load(device.clone()).expect("Unable to compile PBR Fragment Shader.");

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

                let render_pass = single_pass_renderpass!(device.clone(),
                        attachments: {
                            color: {
                                format: Format::R8G8B8A8_UNORM,
                                samples: 1,
                                load_op: Clear, // Clear attachment at start of render pass
                                store_op: Store
                            }
                        },
                        pass: {
                            color: [color],
                            depth_stencil: {}
                        }
                )
                .map_err(|e| CreationError(format!("{:?}", e)))?;

                let subpass: Subpass = Subpass::from(render_pass.clone(), 0)
                    .ok_or(CreationError("Unable to create subpass 0.".to_string()))?;

                let pipeline = create_pipeline(&device, subpass.clone(), &vs_3d, &fs_3d)?;

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

                        pbr_render_pass: render_pass,
                        pbr_subpass: subpass.into(),
                    },
                    pipelines: vec![],
                    current_pipeline_index: 0,
                    current_vertex_buffer_index: 0,

                    current_index_buffer_index: 0,

                    default_texture,
                    pbr_pipeline: pipeline,
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
