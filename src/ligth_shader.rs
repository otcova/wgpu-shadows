use crate::ligth_pipeline::{LigthRenderPass, LigthTextures};
use crate::shader::*;
use crate::uniform::*;

pub struct LigthShader {
    shader: Shader,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LigthInstance {
    pub a: [f32; 2],
    pub b: [f32; 2],
}

impl LigthInstance {
    const ATTRIBS: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![
        0 => Float32x2, // pos a
        1 => Float32x2, // pos b
    ];

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<LigthInstance>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &Self::ATTRIBS,
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LigthUniform {
    pub pos: [f32; 3],
    pub ligth_index: u32,
    pub ligth_color: [f32; 3],
    pub _align: u32,
}

impl LigthShader {
    pub fn new(device: &wgpu::Device, textures: &LigthTextures) -> Self {
        let shader = Shader::new(
            device,
            ShaderDescriptor {
                src: include_str!("ligth_shader.wgsl").into(),
                textures: &[&textures.normal],
                uniforms: &[&Uniform::new_layout(
                    device,
                    wgpu::ShaderStages::VERTEX_FRAGMENT,
                )],
                vertex_layout: LigthInstance::desc(),
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

    pub fn resize(&mut self, device: &wgpu::Device, textures: &LigthTextures) {
        self.shader.update_textures(device, &[&textures.normal])
    }

    pub fn bind<'a>(&'a self, pass: &mut LigthRenderPass<'a>) {
        self.shader.bind(&mut pass.ligth);
    }
}
