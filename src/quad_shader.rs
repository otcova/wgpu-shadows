use crate::ligth_pipeline::{LigthRenderPass, LigthTextures};
use crate::shader::{Shader, ShaderDescriptor};
use crate::texture_atlas::TextureAtlas;
use crate::uniform::Uniform;
use crate::{error::*, WgpuContext};

pub struct QuadShader {
    diffuse: Shader,
    normal: Shader,
    atlas: TextureAtlas,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct QuadInstance {
    pub pos: [f32; 2],
    pub size: [f32; 2],
    pub angle: f32,
    pub tex_pos: [f32; 2],
    pub tex_size: [f32; 2],
}

impl QuadInstance {
    const ATTRIBS: [wgpu::VertexAttribute; 5] = wgpu::vertex_attr_array![
        0 => Float32x2, // pos
        1 => Float32x2, // size
        2 => Float32,   // angle
        3 => Float32x2, // tex_pos
        4 => Float32x2, // tex_size
    ];

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<QuadInstance>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &Self::ATTRIBS,
        }
    }
}

impl QuadShader {
    pub fn new(ctx: &WgpuContext, ligth_textures: &LigthTextures) -> ErrResult<Self> {
        let atlas = TextureAtlas::load(ctx).context("Unable to load texture atlas")?;

        let normal = Shader::new(
            ctx,
            ShaderDescriptor {
                src: include_str!("normal_shader.wgsl").into(),
                textures: &[&atlas.normal_textures[0].view],
                uniforms: &[&Uniform::new_layout(ctx, wgpu::ShaderStages::VERTEX)],
                vertex_layout: QuadInstance::desc(),
                output_format: wgpu::TextureFormat::Rgb10a2Unorm,
                blend: wgpu::BlendState::ALPHA_BLENDING,
                depth_stencil: None,
            },
        );

        let diffuse = Shader::new(
            ctx,
            ShaderDescriptor {
                src: include_str!("diffuse_shader.wgsl").into(),
                textures: &[
                    &ligth_textures.ligth,
                    &atlas.diffuse_textures[0].view,
                    &atlas.normal_textures[0].view,
                ],
                uniforms: &[&Uniform::new_layout(ctx, wgpu::ShaderStages::VERTEX)],
                vertex_layout: QuadInstance::desc(),
                output_format: wgpu::TextureFormat::Bgra8Unorm,
                blend: wgpu::BlendState::ALPHA_BLENDING,
                depth_stencil: None,
            },
        );

        Ok(Self {
            normal,
            diffuse,
            atlas,
        })
    }

    pub fn resize(&mut self, ctx: &WgpuContext, textures: &LigthTextures) {
        self.diffuse.update_textures(
            ctx,
            &[
                &textures.ligth,
                &self.atlas.diffuse_textures[0].view,
                &self.atlas.normal_textures[0].view,
            ],
        );
    }

    pub fn bind<'a>(&'a self, pass: &mut LigthRenderPass<'a>) {
        self.normal.bind(&mut pass.normal);
        self.diffuse.bind(&mut pass.diffuse);
    }
}
