use super::ligth_shader::LigthInstance;
use crate::ligth_pipeline::LigthRenderPass;
use wgpu::util::DeviceExt;

pub struct LigthBatch {
    vertices_buffer: wgpu::Buffer,
    num_instances: u32,
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
        let vertices_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Ligth Instances Buffer"),
            contents: bytemuck::cast_slice(INSTANCES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        Self {
            vertices_buffer,
            num_instances,
        }
    }

    pub fn draw<'a>(&'a self, pass: &mut LigthRenderPass<'a>) {
        let buffer = self.vertices_buffer.slice(..);
        pass.ligth.set_vertex_buffer(0, buffer);
        pass.ligth.draw(0..4, 0..self.num_instances);
    }
}
