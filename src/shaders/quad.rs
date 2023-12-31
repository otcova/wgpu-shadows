use crate::error::*;
use crate::ligth_pipeline::*;
use crate::math::*;
use crate::texture_atlas::*;
use crate::wgpu_components::*;

pub struct QuadShader {
    color: Shader,
    diffuse: Shader,
    normal: Shader,
    atlas: TextureAtlas,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::NoUninit)]
pub struct QuadInstance {
    pub color: u32,
    pub angle: f32,
    pub pos: Vec2,
    pub size: Vec2,
    pub tex_pos: Vec2,
    pub tex_size: Vec2,
}

impl QuadInstance {
    const ATTRIBS: [wgpu::VertexAttribute; 6] = wgpu::vertex_attr_array![
        0 => Unorm8x4,  // color
        1 => Float32,   // angle
        2 => Float32x2, // pos
        3 => Float32x2, // size
        4 => Float32x2, // tex_pos
        5 => Float32x2, // tex_size
    ];

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<QuadInstance>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &Self::ATTRIBS,
        }
    }

    pub fn new_tex(pos: Vec2, width: f32, texture: TextureAtlasView) -> Self {
        Self {
            pos,
            size: texture.aspect_ratio_x1() * width,
            angle: 0.,
            color: 0,
            tex_pos: texture.pos,
            tex_size: texture.size,
        }
    }

    pub fn new_color(pos: Vec2, size: Vec2, color: u32) -> Self {
        Self {
            pos,
            size,
            angle: 0.,
            color,
            tex_pos: Vec2::new(-1., 0.),
            tex_size: Vec2::zero(),
        }
    }
}

impl QuadShader {
    pub fn new(ctx: &WgpuContext, ligth_textures: &LigthTextures) -> ErrResult<Self> {
        let atlas = TextureAtlas::load(ctx).context("Unable to load texture atlas")?;

        let normal = Shader::new(
            ctx,
            ShaderDescriptor {
                src: include_str!("quad_normal.wgsl").into(),
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
                src: include_str!("quad_diffuse.wgsl").into(),
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

        let color = Shader::new(
            ctx,
            ShaderDescriptor {
                src: include_str!("quad.wgsl").into(),
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
            color,
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

    pub fn bind_ligth<'a>(&'a self, pass: &mut LigthRenderPass<'a>) {
        self.normal.bind(&mut pass.normal);
        self.diffuse.bind(&mut pass.diffuse);
    }

    pub fn bind<'a>(&'a self, pass: &mut LigthRenderPass<'a>) {
        self.color.bind(&mut pass.diffuse);
    }
}
