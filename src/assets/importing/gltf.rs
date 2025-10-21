use std::io::{BufRead, Read};

use gltf::{Document, Mesh, Node, buffer, image};

use crate::assets::Primitive;
use crate::assets::importing::{
    ImportError, ImportResult, ImportedMesh, ImportedModel, Importer, ModelNode,
};
use crate::core::To;
use crate::math::{Mat4, Quat, Vec3};
use crate::rendering::types::Vertex3D;

fn parse_node(ctx: &mut GltfModelContext, node: &Node) -> Option<ModelNode> {
    let transform = node.transform().to();

    let mesh = node.mesh().map_or(None, |mesh| {
        let mut primitives = vec![];

        mesh.primitives().for_each(|primitive| {
            let attributes = primitive.attributes();

            for (semantic, accessor) in attributes {
                let mut primitive = Primitive {
                    material: None,
                    index_buffer: todo!(),
                    vertex_buffer: todo!(),
                    draw_call: todo!(),
                };

                match semantic {
                    gltf::Semantic::Positions => {}
                    gltf::Semantic::Normals => todo!(),
                    gltf::Semantic::Tangents => todo!(),
                    gltf::Semantic::Colors(_) => todo!(),
                    gltf::Semantic::TexCoords(_) => todo!(),
                    gltf::Semantic::Joints(_) => todo!(),
                    gltf::Semantic::Weights(_) => todo!(),
                }

                primitives.push(primitive);
            }

            while let Some(attribute) = primitive.attributes() {}

            let mut prim = Primitive {
                material: todo!(),
                index_buffer: todo!(),
                vertex_buffer: todo!(),
                draw_call: todo!(),
            };
        });

        let vertices = mesh.primitives().for_each(|primitive| primitive.attributes);

        Some(ImportedMesh {
            vertices: todo!(),
            indices: todo!(),
            primitives: todo!(),
        })
    });

    ctx.transforms.push(transform);

    let children: Vec<ModelNode> = node
        .children()
        .filter_map(|child| parse_node(ctx, &child))
        .collect();

    ctx.transforms.pop();

    let model_node = ModelNode {
        children,
        transform: todo!(),
        mesh: todo!(),
    };
    Some(model_node)
}

#[derive(Debug)]
pub struct GltfImporter {}

impl Importer for GltfImporter {
    type ImportType = ImportedModel;

    fn import<R: Read>(reader: &mut R) -> super::ImportResult<Self::ImportType> {
        let mut bytes = Vec::with_capacity(reader.bytes().count());
        reader
            .read_to_end(&mut bytes)
            .map_err(|e| ImportError(e.to_string()))?;

        let (doc, buffers, images) = gltf::import_slice(&bytes).expect("Bad load.");

        let mut ctx = GltfModelContext::new(buffers, images);

        let scene = doc
            .default_scene()
            .ok_or(ImportError("Expected default scene.".to_string()))?;

        scene.nodes().for_each(|node| parse_node(&mut ctx, &node));

        Ok(ImportedModel {
            root_nodes: todo!(),
            materials: todo!(),
            textures: todo!(),
        })
    }

    fn import_raw_assets<R: BufRead>(reader: R) -> super::ImportResult<Vec<super::ImportedAsset>> {
        todo!()
    }
}

impl To<Mat4> for gltf::scene::Transform {
    fn to(self) -> Mat4 {
        match self {
            gltf::scene::Transform::Matrix { matrix } => Mat4::from(matrix),
            gltf::scene::Transform::Decomposed {
                translation,
                rotation,
                scale,
            } => {
                let quat = Quat::from(rotation);

                Mat4::from_translation(translation.into())
                    * Mat4::from(quat)
                    * Mat4::from_nonuniform_scale(scale[0], scale[1], scale[2])
            }
        }
    }
}

#[derive(Debug)]
pub(crate) struct GltfModelContext {
    transforms: Vec<Mat4>,
    // doc: Document,
    buffers: Vec<buffer::Data>,
    images: Vec<image::Data>,

    vertex_buffers: Vec<Vec<Vertex3D>>,
    index_buffers: Vec<Vec<u8>>,
}

impl GltfModelContext {
    fn new(buffers: Vec<buffer::Data>, images: Vec<image::Data>) -> Self {
        Self {
            transforms: vec![Mat4::from_translation(Vec3 {
                x: 0f32,
                y: 0f32,
                z: 0f32,
            })],
            buffers,
            images,
        }
    }

    fn import_model(&self) -> ImportResult<ImportedModel> {
        todo!();
    }
}
