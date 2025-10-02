use std::sync::Arc;

use crate::assets::{ImportedTexture, Texture};
use crate::math::Vec4;
use crate::rendering::{Pipeline, RenderError};
use crate::rendering::{RenderIndex, RendererRes};

pub trait Material {
    fn write_to_shader(&self, shader: &mut Pipeline) -> Result<(), RenderError>;
}

#[derive(Debug)]
pub struct ImportedPBRMaterial {
    pub colour_texture_index: Option<RenderIndex>,
    pub base_colour: Option<Vec4>,
    pub metallic: Option<f32>,
    pub roughness: Option<f32>,
    // TODO: metallic_roughness_texture: MetallicRoughnessTexture
}

#[derive(Debug)]
pub struct PBRMaterial {
    pub colour_texture: Option<Arc<Texture>>,
    pub base_colour: Option<Vec4>,
    pub metallic: Option<f32>,
    pub roughness: Option<f32>,
    // TODO: metallic_roughness_texture: MetallicRoughnessTexture
}

impl PBRMaterial {
    pub fn from_import(
        import: ImportedPBRMaterial,
        texture_list: &Vec<Arc<Texture>>,
    ) -> RendererRes<Self> {
        let colour_texture: Option<Arc<Texture>> = match import.colour_texture_index {
            Some(index) => {
                if let Some(tex) = texture_list.get(index) {
                    Some(tex.clone())
                } else {
                    return Err(RenderError::ResourceMissing(format!(
                        "Texture at index {} does not exist in the texture list used to create a PBRMaterial. (Only {} available)",
                        index,
                        texture_list.len()
                    )));
                }
            }
            None => None,
        };

        Ok(Self {
            colour_texture,
            base_colour: import.base_colour,
            metallic: import.metallic,
            roughness: import.roughness,
        })
    }
}

/*
impl Material for PBRMaterial {
    fn write_to_shader(&self, shader: &mut Pipeline) -> Result<(), RenderError> {
        todo!();
        /*
        shader.set_texture(0, self.diffuse_texture)?;
        shader.set_sampler(1, self.diffuse_texture_sampler)?;
        // shader.set::<vec4>(2, self.base_colour)?;
        shader.set::<f32>(3, self.metallic)?;
        shader.set::<f32>(4, self.roughness)?;
        */
    }
}
*/
