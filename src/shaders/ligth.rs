use crate::ligth_pipeline::*;
use crate::math::*;
use crate::wgpu_components::*;

pub struct LigthShader {
    shader: Shader,
}

#[repr(C)]
#[derive(Default, Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ShadowInstance {
    pub a: Vec2,
    pub b: Vec2,
}

impl ShadowInstance {
    const ATTRIBS: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![
        0 => Float32x2, // pos a
        1 => Float32x2, // pos b
    ];

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<ShadowInstance>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &Self::ATTRIBS,
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LigthUniform {
    pub pos: Vec2,
    pub z_index: f32,
    pub color: u32,
}

impl LigthUniform {
    pub fn color(r: u16, g: u16, b: u16) -> u32 {
        let r = (r & 0x3FF) as u32;
        let g = (g & 0x3FF) as u32;
        let b = (b & 0x3FF) as u32;
        (r << 20) | (g << 10) | b
    }
}

impl LigthShader {
    pub fn new(ctx: &WgpuContext, textures: &LigthTextures) -> Self {
        let shader = Shader::new(
            ctx,
            ShaderDescriptor {
                src: include_str!("ligth.wgsl").into(),
                textures: &[&textures.normal],
                uniforms: &[
                    &Uniform::new_layout(ctx, wgpu::ShaderStages::VERTEX),
                    &Uniform::new_layout(ctx, wgpu::ShaderStages::VERTEX),
                    &Uniform::new_layout(ctx, wgpu::ShaderStages::VERTEX_FRAGMENT),
                ],
                vertex_layout: ShadowInstance::desc(),
                output_format: wgpu::TextureFormat::Rgb10a2Unorm,
                blend: wgpu::BlendState {
                    color: wgpu::BlendComponent {
                        src_factor: wgpu::BlendFactor::One,
                        dst_factor: wgpu::BlendFactor::One,
                        operation: wgpu::BlendOperation::Add,
                    },
                    alpha: wgpu::BlendComponent::REPLACE,
                },
                depth_stencil: Some(wgpu::DepthStencilState {
                    format: wgpu::TextureFormat::Depth32Float,
                    depth_write_enabled: true,
                    depth_compare: wgpu::CompareFunction::Less,
                    stencil: wgpu::StencilState::default(),
                    bias: wgpu::DepthBiasState::default(),
                }),
            },
        );
        Self { shader }
    }

    pub fn resize(&mut self, ctx: &WgpuContext, textures: &LigthTextures) {
        self.shader.update_textures(ctx, &[&textures.normal])
    }

    pub fn bind<'a>(&'a self, pass: &mut LigthRenderPass<'a>) {
        self.shader.bind(&mut pass.ligth);
    }
}
