// THIS CODE IS GENERATED BY THE BUILD SCRIPT.
// ANY CHANGE WILL BE OVERWRITTEN.

use crate::error::*;
use crate::math::*;
use crate::wgpu_components::*;

pub struct TextureAtlas {
    pub diffuse_textures: [Texture; 1],
    pub normal_textures: [Texture; 1],
}

#[derive(Copy, Clone)]
pub struct TextureAtlasView {
    pub pos: Vec2,
    pub size: Vec2,
    ratio: f32,
}

impl TextureAtlas {
    pub fn load(ctx: &WgpuContext) -> ErrResult<Self> {
        Ok(Self {
            diffuse_textures: [
                Texture::from_bytes(
                    ctx,
                    include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/atlas/diffuse-0.webp")),
                    "Diffuse Texture 0",
                )?,
            ],
            normal_textures: [
                Texture::from_bytes(
                    ctx,
                    include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/atlas/normal-0.webp")),
                    "Normal Texture 0",
                )?,
            ],
        })
    }

    #[allow(dead_code)]
    pub fn view_arrow() -> TextureAtlasView {
        TextureAtlasView {
            pos: Vec2::new(0f32, 0f32),
            size: Vec2::new(0.045027625f32, 0.031583104f32),
            ratio: 2.0375f32,
        }
    }
    #[allow(dead_code)]
    pub fn view_block_sq2() -> TextureAtlasView {
        TextureAtlasView {
            pos: Vec2::new(0.045303866f32, 0f32),
            size: Vec2::new(0.10359116f32, 0.1480458f32),
            ratio: 1f32,
        }
    }
    #[allow(dead_code)]
    pub fn view_block_sq3() -> TextureAtlasView {
        TextureAtlasView {
            pos: Vec2::new(0.14917128f32, 0f32),
            size: Vec2::new(0.10359116f32, 0.1480458f32),
            ratio: 1f32,
        }
    }
    #[allow(dead_code)]
    pub fn view_block_sq4() -> TextureAtlasView {
        TextureAtlasView {
            pos: Vec2::new(0.25303867f32, 0f32),
            size: Vec2::new(0.10359116f32, 0.1480458f32),
            ratio: 1f32,
        }
    }
    #[allow(dead_code)]
    pub fn view_bow() -> TextureAtlasView {
        TextureAtlasView {
            pos: Vec2::new(0.3569061f32, 0f32),
            size: Vec2::new(0.047513813f32, 0.106592976f32),
            ratio: 0.63703704f32,
        }
    }
    #[allow(dead_code)]
    pub fn view_bow_charge_0() -> TextureAtlasView {
        TextureAtlasView {
            pos: Vec2::new(0.40469614f32, 0f32),
            size: Vec2::new(0.029281767f32, 0.13067509f32),
            ratio: 0.3202417f32,
        }
    }
    #[allow(dead_code)]
    pub fn view_bow_charge_1() -> TextureAtlasView {
        TextureAtlasView {
            pos: Vec2::new(0.43425414f32, 0f32),
            size: Vec2::new(0.020718232f32, 0.15752073f32),
            ratio: 0.18796992f32,
        }
    }
    #[allow(dead_code)]
    pub fn view_tektur_regular() -> TextureAtlasView {
        TextureAtlasView {
            pos: Vec2::new(0.45524862f32, 0f32),
            size: Vec2::new(0.13425414f32, 0.19107777f32),
            ratio: 1.0041323f32,
        }
    }
    #[allow(dead_code)]
    pub fn view_text_button() -> TextureAtlasView {
        TextureAtlasView {
            pos: Vec2::new(0.589779f32, 0f32),
            size: Vec2::new(0.18342541f32, 0.04500592f32),
            ratio: 5.8245616f32,
        }
    }
    #[allow(dead_code)]
    pub fn view_text_button_hover() -> TextureAtlasView {
        TextureAtlasView {
            pos: Vec2::new(0.77348065f32, 0f32),
            size: Vec2::new(0.18342541f32, 0.04500592f32),
            ratio: 5.8245616f32,
        }
    }
    #[allow(dead_code)]
    pub fn view_triangles() -> TextureAtlasView {
        TextureAtlasView {
            pos: Vec2::new(0.43425414f32, 0.19147256f32),
            size: Vec2::new(0.56574583f32, 0.8085274f32),
            ratio: 1f32,
        }
    }
}

impl TextureAtlasView {
    /// Returns: Vec2 { x: ratio, y: 1.0 }
    pub fn aspect_ratio_x1(&self) -> Vec2 {
        Vec2::new(self.ratio, 1.)
    }
}
