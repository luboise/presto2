mod vulkan;
use std::{error::Error, fmt::Display};

pub use vulkan::*;

pub mod types;

mod pipeline;
pub use pipeline::*;

pub type RenderIndex = usize;

#[derive(Debug)]
pub enum RenderError {
    MemoryError(String),
    CreationError(String),
}

impl Display for RenderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                RenderError::MemoryError(s) => s,
                RenderError::CreationError(s) => s,
            }
        )
    }
}

#[derive(Debug)]
pub struct CreationError(String);

impl Error for CreationError {}

impl Display for CreationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl From<CreationError> for RenderError {
    fn from(value: CreationError) -> Self {
        RenderError::CreationError(value.0)
    }
}

pub type RendererRes<T> = Result<T, RenderError>;
pub type RendererOk = RendererRes<()>;

pub trait Draw {
    fn draw<R: Render>(&self, renderer: &mut R);
}

// renderer.set_shader(self.shader_index);
// self.constants.iter().for_each(|constant|{
// renderer.set_constant(index, value);
// renderer.draw();
// });
// renderer.(self.shader_index);

#[derive(Debug)]
pub struct DrawCall {
    num_indices: usize,
    start_offset: usize,
}

pub trait Buffer {
    fn len(&self) -> usize;

    fn write(&mut self, bytes: &[u8], start_offset: usize) -> RendererOk;
}

pub struct VertexBuffer {}

pub struct IndexBuffer {}

pub trait Render {
    fn bind_pipeline(&mut self, pipeline_index: RenderIndex) -> RendererOk;
    // fn shader(&mut self) -> Shader;

    fn pipeline(&self) -> &Pipeline;
    fn pipeline_mut(&mut self) -> &mut Pipeline;

    fn pipelines(&self) -> &[Pipeline];

    fn index_buffer(&self, buffer_index: RenderIndex) -> Option<IndexBuffer>;
    fn set_index_buffer(&mut self, buffer_index: RenderIndex) -> RendererOk;

    fn vertex_buffers(&mut self) -> Result<&[VertexBuffer], RenderError>;
    fn set_vertex_buffer(&mut self, buffer_index: RenderIndex) -> RendererOk;
    // TODO: Implement multi set
    // fn set_vertex_buffers(&mut self, buffer_index: &[RenderIndex]) -> Result<(), RenderError>;
}
