use std::sync::Arc;

use crate::assets::importing::{ImportedMesh, ImportedModel, ImportedPrimitive};
use crate::assets::{PBRMaterial, Texture};
use crate::rendering::{Draw, DrawCall, Render, RenderError, RenderIndex, RendererOk, RendererRes};

/*
#[derive(Debug)]
pub struct Primitive {
    pub(crate) material: Option<Arc<PBRMaterial>>,

    // Renderer specific
    pub(crate) index_buffer: RenderIndex,
    pub(crate) vertex_buffer: RenderIndex,

    pub(crate) draw_call: DrawCall,
}

impl Primitive {
    pub fn from_import(
        import: ImportedPrimitive,
        materials: &Vec<Arc<PBRMaterial>>,
    ) -> RendererRes<Self> {
        let material = match import.material_index {
            Some(index) =>  Some(materials.get(index).ok_or(RenderError::ResourceMissing(format!("Material index {} out of bounds for material list when instantiating a Primitive. ({} materials available)",index, materials.len() )))?),
            None => None,
        };

        Self {
            material: todo!(),
            index_buffer: todo!(),
            vertex_buffer: todo!(),
            draw_call: todo!(),
        }
    }
}
*/

#[derive(Debug)]
pub struct Mesh {
    primitives: Vec<Primitive>,
}

impl Mesh {
    pub fn new<R: Render>() {}

    /*
    pub fn from_import<R: Render>(
        import: ImportedMesh,
        renderer: &mut R,
        materials: &Vec<Arc<PBRMaterial>>,
    ) -> RendererRes<Self> {
        let vertex_buffer_index: RenderIndex = renderer
            .create_vertex_buffer()
            .expect("Unable to create vertex buffer.");
        let index_buffer_index: RenderIndex = renderer
            .create_index_buffer()
            .expect("Unable to create index buffer.");

        let primitives: Vec<Primitive> = import
            .primitives
            .into_iter()
            .map(|imported_primitive| Primitive::from_import(imported_primitive, materials))
            .collect()?;

        Ok(Self { primitives })
    }
    */
}

#[derive(Debug)]
pub struct Model {
    vertex_buffer_index: usize,
    index_buffer_index: usize,

    meshes: Vec<Mesh>,
    // materials: Vec<PBRMaterial>,
    // bytes: Arc<AssetBytes>,
}

impl Model {
    fn from_import<R: Render>(
        imported_model: &ImportedModel,
        renderer: &mut R,
    ) -> Result<Self, String> {
        let textures: Vec<Arc<Texture>> = imported_model
            .textures
            .into_iter()
            .map(|imported_texture| -> RendererRes<Texture> {
                Texture::from_import(renderer, imported_texture)
            })
            .collect()?;

        let materials: Vec<Arc<PBRMaterial>> = imported_model
            .materials
            .into_iter()
            .map(|imported_pbrmaterial| -> RendererRes<PBRMaterial> {
                PBRMaterial::from_import(imported_pbrmaterial, &textures)
            })
            .collect()?;

        Ok(Model {
            vertex_buffer_index: todo!(),
            index_buffer_index: todo!(),
            meshes: todo!(),
            bytes: todo!(),
        })
    }
}

/*
impl Asset for Model {
    fn from_entry(entry: &AssetEntry) -> super::Result<Self> {
        let bytes_arc = entry
            .bytes
            .upgrade()
            .expect("Failed to upgrade previously checked AssetEntry to Arc.");

        let bytes = bytes_arc.as_ref().as_slice();

        let imported_model: ImportedModel = GltfImporter::import(bytes.into())
            .map_err(|e| AssetError::ReadError(format!("{:?}", e)))?;

        /*
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
        */

        Ok(Self {
            bytes: entry.bytes,
            meshes,
        })
    }
}
*/

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
