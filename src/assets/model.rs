use std::sync::Arc;

use crate::{
    assets::{Asset, AssetBytes, PBRMaterial},
    rendering::{Draw, DrawCall, Render, RenderIndex},
};

#[derive(Debug)]
pub struct Primitive {
    material_index: RenderIndex,

    // Renderer specific
    index_buffer: RenderIndex,
    vertex_buffer: RenderIndex,

    draw_call: DrawCall,
}

#[derive(Debug)]
pub struct Mesh {
    primitives: Vec<Primitive>,
}

#[derive(Debug)]
pub struct Model {
    meshes: Vec<Mesh>,
    materials: Vec<PBRMaterial>,

    bytes: Arc<AssetBytes>,
}

impl Asset for Model {
    fn from_bytes(bytes: &Arc<AssetBytes>) -> super::Result<Self> {
        Ok(Self {
            bytes: bytes.clone(),
        })
    }
}

impl Draw for Mesh {
    fn draw<R: Render>(&self, renderer: &mut R) {
        for primitive in self.primitives {
            renderer.bind_pipeline(primitive.shader_index);

            renderer.set_vertex_buffer(primitive.vertex_buffer);
            renderer.set_index_buffer(primitive.index_buffer);
        }
    }
}
