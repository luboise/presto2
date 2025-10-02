use vulkano::{buffer::BufferContents, pipeline::graphics::vertex_input::Vertex};

use crate::rendering::BufferValue;

pub type Vec2 = [f32; 2];
pub type Vec3 = [f32; 3];
pub type Vec4 = [f32; 4];

pub type Mat2 = [Vec2; 2];
pub type Mat3 = [Vec3; 3];
pub type Mat4 = [Vec4; 4];

#[derive(Debug, Clone, BufferContents, Vertex)]
#[repr(C)]
pub struct Vertex3D {
    #[name("position", "a_position")]
    #[format(R32G32B32_SFLOAT)]
    pub position: Vec3,

    #[name("colour", "a_colour")]
    #[format(R32G32B32_SFLOAT)]
    pub colour: Vec3,

    #[name("normal", "a_normal")]
    #[format(R32G32B32_SFLOAT)]
    pub normal: Vec3,

    #[name("texcoords", "a_texcoords")]
    #[format(R32G32_SFLOAT)]
    pub tex_coords: Vec2,
}

impl Default for Vertex3D {
    fn default() -> Self {
        Self {
            position: [0f32, 0f32, 0f32],
            colour: [0f32, 0f32, 0f32],
            normal: Default::default(),
            tex_coords: Default::default(),
        }
    }
}

impl BufferValue for Vertex3D {}

/*
#[derive(Debug)]
pub struct RenderNode {
    // material_index: Arc<dyn Material>,

    // Renderer specific
    index_buffer: RenderIndex,
    vertex_buffer: RenderIndex,

    draw_call: DrawCall,
}

impl RenderNode {
    pub fn draw_recursive<R: Render>(&self, renderer: &mut R, transform: &Transform) -> RendererOk {
        renderer.bind_pipeline;
    }
}

impl Draw for RenderNode {
    fn draw<R: Render>(&self, renderer: &mut R) -> RendererOk {
        self.draw_recursive(renderer);
    }
}
*/
