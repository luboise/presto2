use vulkano::{buffer::BufferContents, pipeline::graphics::vertex_input::Vertex};

type Vec2 = [f32; 2];
type Vec3 = [f32; 3];
type Vec4 = [f32; 4];

type Mat2 = [Vec2; 2];
type Mat3 = [Vec3; 3];
type Mat4 = [Vec4; 4];

#[derive(Debug, BufferContents, Vertex)]
#[repr(C)]
pub struct Vertex3D {
    #[format(R32G32B32_SFLOAT)]
    position: Vec3,

    #[format(R32G32B32_SFLOAT)]
    colour: Vec3,

    #[format(R32G32B32_SFLOAT)]
    normal: Vec3,

    #[format(R32G32_SFLOAT)]
    tex_coords: Vec2,
}
