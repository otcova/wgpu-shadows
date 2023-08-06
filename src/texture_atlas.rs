// THIS CODE IS GENERATED BY THE BUILD SCRIPT.
// ANY CHANGE WILL BE OVERWRITTEN.

use crate::error::ErrResult;
use crate::texture::Texture;

pub struct TextureAtlas {
    pub textures: [Texture; 2],
}

#[derive(Copy, Clone)]
pub struct TextureAtlasView {
    tex_pos: [f32; 2],
    tex_size: [f32; 2],
}

impl TextureAtlas {
    pub fn load(device: &wgpu::Device, queue: &wgpu::Queue) -> ErrResult<Self> {
        Ok(Self {
            textures: [
                Texture::from_bytes(
                    device,
                    queue,
                    include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/atlas/diffuse-0.webp")),
                    "Diffuse Texture 0",
                )?,
                Texture::from_bytes(
                    device,
                    queue,
                    include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/atlas/normal-0.webp")),
                    "Normal Texture 0",
                )?,
            ],
        })
    }

    fn view_triangles() -> TextureAtlasView {
        TextureAtlasView {
            tex_pos: [0.5326335f32, 0f32],
            tex_size: [0.4673665f32, 1f32],
        }
    }
    fn view_bow_charge_0() -> TextureAtlasView {
        TextureAtlasView {
            tex_pos: [0.19853948f32, 0f32],
            tex_size: [0.15107258f32, 0.20703125f32],
        }
    }
    fn view_arrow() -> TextureAtlasView {
        TextureAtlasView {
            tex_pos: [0f32, 0f32],
            tex_size: [0.074395254f32, 0.15625f32],
        }
    }
    fn view_bow() -> TextureAtlasView {
        TextureAtlasView {
            tex_pos: [0.07485167f32, 0f32],
            tex_size: [0.1232314f32, 0.3359375f32],
        }
    }
    fn view_bow_charge_1() -> TextureAtlasView {
        TextureAtlasView {
            tex_pos: [0.35006845f32, 0f32],
            tex_size: [0.18210863f32, 0.14648438f32],
        }
    }
}
