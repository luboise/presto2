use std::sync::Arc;

use crate::rendering::{ImageHandle, ImageParams, Render, RendererRes};

#[derive(Debug)]
pub enum ColourFormat {
    RGB,
    RGBA,
}

impl ColourFormat {
    const fn get_pixel_stride(&self) -> usize {
        8 * self.num_channels()
    }

    const fn num_channels(&self) -> usize {
        match self {
            ColourFormat::RGB => 3,
            ColourFormat::RGBA => 4,
        }
    }
}

#[derive(Debug)]
pub struct ImportedTexture {
    pub width: usize,
    pub height: usize,
    pub colour_format: ColourFormat,
    pub data: Vec<u8>,
}

impl From<ImportedTexture> for ImageParams {
    fn from(value: ImportedTexture) -> Self {
        ImageParams {
            width: value.width,
            height: value.height,
            colour_format: value.colour_format,
            data: value.data,
        }
    }
}

#[derive(Debug)]
pub struct Texture {
    pub(crate) width: usize,
    pub(crate) height: usize,

    // TODO: Implement colour formats and such things
    // tex_type: TextureType,
    pub(crate) image_handle: Arc<ImageHandle>,
}

impl Texture {
    pub fn from_image_params<R: Render, IP: Into<ImageParams>>(
        renderer: &mut R,
        params: IP,
    ) -> RendererRes<Self> {
        let img = renderer.create_image(params.into());

        Ok(Self {
            width: img.width(),
            height: img.height(),
            image_handle: img,
        })
    }

    pub fn from_import<R: Render>(renderer: &mut R, import: ImportedTexture) -> RendererRes<Self> {
        Self::from_image_params(renderer, import)
    }
}
