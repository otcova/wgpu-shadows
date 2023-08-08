use crate::ligth_pipeline::LigthRenderPass;
use crate::uniform::Uniform;

pub struct Camera {
    uniform: Uniform,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct CameraUniform {
    pub pos: [f32; 2],
    pub size: [f32; 2],
}

impl Camera {
    pub fn new(device: &wgpu::Device) -> Self {
        Self {
            uniform: Uniform::new(
                device,
                wgpu::ShaderStages::VERTEX,
                &CameraUniform {
                    pos: [0., 0.],
                    size: [1., 1.],
                },
            ),
        }
    }

    pub fn resize(&self, queue: &wgpu::Queue, width: u32, height: u32) {
        self.uniform.update_buffer(
            queue,
            &CameraUniform {
                pos: [0., 0.],
                size: [height as f32 / width as f32, 1.],
            },
        );
    }

    pub fn bind<'a>(&'a self, pass: &mut LigthRenderPass<'a>) {
        self.uniform.bind(1, &mut pass.normal);
        self.uniform.bind(1, &mut pass.ligth);
        self.uniform.bind(1, &mut pass.diffuse);
    }
}
