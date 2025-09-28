// mod vulkan;
// pub use vulkan::*;

pub mod types;

mod shader;
pub use shader::*;

pub type RenderIndex = usize;

#[derive(Debug)]
pub enum RenderError {
    MemoryError(String),
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
    fn set_shader(&mut self, shader_index: RenderIndex);
    // fn shader(&mut self) -> Shader;

    fn shader_mut(&mut self) -> &mut Shader;

    fn index_buffer(&self, buffer_index: RenderIndex) -> Option<IndexBuffer>;
    fn set_index_buffer(&mut self, buffer_index: RenderIndex) -> RendererOk;

    fn vertex_buffers(&mut self) -> Result<&[VertexBuffer], RenderError>;
    fn set_vertex_buffer(&mut self, buffer_index: RenderIndex) -> RendererOk;
    // TODO: Implement multi set
    // fn set_vertex_buffers(&mut self, buffer_index: &[RenderIndex]) -> Result<(), RenderError>;
}
