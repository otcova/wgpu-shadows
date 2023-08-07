use super::ligth_shader::LigthInstance;
use crate::ligth_pipeline::LigthRenderPass;
use crate::ligth_shader::LigthUniform;
use crate::uniform::*;
use wgpu::util::DeviceExt;

pub struct LigthBatch {
    ligth_uniforms: Vec<Uniform>,
    shadows_buffer: wgpu::Buffer,
    num_instances: u32,
    t: f32,
}

const INSTANCES: &[LigthInstance] = &[
    LigthInstance {
        a: [-0.4, 0.3],
        b: [-0.2, 0.3],
    },
    LigthInstance {
        a: [-0.2, -0.1],
        b: [0.7, -0.2],
    },
    LigthInstance {
        a: [0., 0.],
        b: [0., 0.],
    },
];

impl LigthBatch {
    pub fn new(device: &wgpu::Device) -> Self {
        let num_instances = INSTANCES.len() as u32;
        let shadows_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Ligth Instances Buffer"),
            contents: bytemuck::cast_slice(INSTANCES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let ligth_uniforms = vec![
            Uniform::new(
                device,
                wgpu::ShaderStages::VERTEX_FRAGMENT,
                &LigthUniform {
                    pos: [0., 0., 0.9],
                    ligth_index: 2,
                    ligth_color: [10., 10., 10.],
                    _align: 0,
                },
            ),
            Uniform::new(
                device,
                wgpu::ShaderStages::VERTEX_FRAGMENT,
                &LigthUniform {
                    pos: [0.5, 0., 0.5],
                    ligth_index: 2,
                    ligth_color: [15., 10., 10.],
                    _align: 0,
                },
            ),
        ];

        Self {
            ligth_uniforms,
            shadows_buffer,
            num_instances,
            t: 0.,
        }
    }

    pub fn draw<'a>(&'a mut self, pass: &mut LigthRenderPass<'a>, queue: &wgpu::Queue) {
        self.t += 0.02;

        self.ligth_uniforms[1].update_buffer(
            queue,
            &LigthUniform {
                pos: [f32::sin(self.t), f32::cos(self.t), 0.5],
                ligth_index: 2,
                ligth_color: [15., 10., 10.],
                _align: 0,
            },
        );

        let buffer = self.shadows_buffer.slice(..);
        pass.ligth.set_vertex_buffer(0, buffer);

        for uniform in self.ligth_uniforms.iter() {
            uniform.bind(1, &mut pass.ligth);
            pass.ligth.draw(0..4, 0..self.num_instances);
        }
    }
}
