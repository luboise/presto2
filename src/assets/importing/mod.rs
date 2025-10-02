use std::{
    io::{BufRead, Read},
    sync::Arc,
};

mod gltf;
pub use gltf::GltfImporter;

use crate::{
    assets::{ImportedPBRMaterial, ImportedTexture},
    components::transform::Transform,
    rendering::{DrawCall, RenderIndex, types::Vertex3D},
};

pub enum NodeChild {
    Mesh(ImportedMesh),
    Node(ModelNode),
}

#[derive(Debug)]
pub enum ImportedAsset {
    Mesh(ImportedMesh),
    Model(ImportedModel),
    Material(ImportedPBRMaterial),
    Texture(ImportedTexture),
}

#[derive(Debug)]
pub struct ImportError(String);

pub type ImportResult<T> = std::result::Result<T, ImportError>;

pub trait Importer {
    type ImportType;

    fn import<R: Read>(reader: &mut R) -> ImportResult<Self::ImportType>;
    fn import_raw_assets<R: BufRead>(reader: R) -> ImportResult<Vec<ImportedAsset>>;
}

#[derive(Debug)]
pub struct ImportedPrimitive {
    pub(crate) num_indices: usize,
    pub(crate) start_offset: usize,
    pub(crate) draw_type: usize,
    pub(crate) material_index: Option<RenderIndex>,
}

#[derive(Debug)]
pub struct ImportedMesh {
    pub(crate) vertices: Vec<Vertex3D>,
    pub(crate) indices: Vec<u32>,

    pub(crate) primitives: Vec<ImportedPrimitive>,
}

#[derive(Debug)]
pub struct ImportedModel {
    pub(crate) root_nodes: Vec<ModelNode>,
    pub(crate) materials: Vec<ImportedPBRMaterial>,
    pub(crate) textures: Vec<ImportedTexture>,
}

#[derive(Debug)]
pub struct ModelNode {
    children: Vec<ModelNode>,

    transform: Option<Transform>,
    mesh: Option<ImportedMesh>,
}
