use crate::{
    math::Vec4,
    rendering::{Pipeline, RenderError},
};

pub trait Material: Sized {
    fn write_to_shader(&self, shader: &mut Pipeline) -> Result<(), RenderError>;
}

// impl MaterialLayout for PBRMaterial {}

#[derive(Debug)]
pub struct PBRMaterial {
    diffuse_texture: BoundTexture,
    diffuse_texture_sampler: Sampler2D,
    base_colour: Vec4,
    metallic: f32,
    roughness: f32,
}

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
