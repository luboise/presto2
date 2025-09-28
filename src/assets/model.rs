use std::sync::Arc;

use crate::{
    assets::{Asset, AssetBytes},
    rendering::{Draw, DrawCall, Render, RenderIndex, RendererOk},
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
    // materials: Vec<PBRMaterial>,
    bytes: Arc<AssetBytes>,
}

impl Asset for Model {
    fn from_bytes(bytes: &Arc<AssetBytes>) -> super::Result<Self> {
        let meshes = vec![Mesh {
            primitives: vec![Primitive {
                material_index: 0,
                index_buffer: 0,
                vertex_buffer: 0,
                draw_call: DrawCall {
                    num_indices: 0,
                    start_offset: 0,
                },
            }],
        }];

        Ok(Self {
            bytes: bytes.clone(),
            meshes,
        })
    }
}

impl Draw for Model {
    fn draw<R: Render>(&self, renderer: &mut R) -> RendererOk {
        for mesh in &self.meshes {
            mesh.draw(renderer)?;
        }

        Ok(())
    }
}

impl Draw for Mesh {
    fn draw<R: Render>(&self, renderer: &mut R) -> RendererOk {
        for primitive in &self.primitives {
            // TODO: Implement custom pipelines
            // renderer.bind_pipeline(primitive.shader_index);

            renderer.set_vertex_buffer(primitive.vertex_buffer)?;
            renderer.set_index_buffer(primitive.index_buffer)?;
        }

        Ok(())
    }
}
