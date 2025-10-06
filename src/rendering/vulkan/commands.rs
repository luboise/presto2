use crate::rendering::{BufferValue, CommandSubmit, DrawCall, Render, RendererOk, VulkanRenderer};

#[derive(Debug)]
pub struct VulkanCommandsCtx {}

/*
pub trait CommandSubmit<VB, IB> where VB: VertexBuffer, IB: IndexBuffer {
    fn set_index_buffer(&mut self, buffer_index: RenderIndex) -> RendererOk;


    fn vertex_buffers(&mut self) -> RendererRes<&[VertexBuffer]>;
    fn set_vertex_buffer(&mut self, buffer_index: RenderIndex) -> RendererOk;



    fn set_vertex_buffers(&mut self, vertex_buffers: &[VB]);

    fn index_buffer(&self, buffer_index: RenderIndex) -> Option<IndexBuffer>;

    fn set_index_buffer(&mut self, index_buffer: &IB);

    fn draw(&mut self, DrawCall{ num_indices, start_offset })

}

*/

impl CommandSubmit<VulkanRenderer> for VulkanCommandsCtx {
    fn set_vertex_buffer<V: BufferValue>(
        &mut self,
        buffer: &<VulkanRenderer as Render>::VertexBufferType<V>,
    ) -> RendererOk {
        println!("Setting vertex buffer.");

        Ok(())
    }

    fn set_index_buffer(
        &mut self,
        buffer: &<VulkanRenderer as Render>::IndexBufferType,
    ) -> RendererOk {
        println!("Setting index buffer.");
        Ok(())
    }

    fn draw(&mut self, draw_call: DrawCall) -> RendererOk {
        println!("Drawing one thing.");
        Ok(())
    }
}
