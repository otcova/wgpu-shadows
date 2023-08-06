use crate::ligth_pipeline::LigthRenderPass;

use super::ligth_shader::LigthVertex;
use wgpu::util::DeviceExt;

pub struct LigthBatch {
    vertices_buffer: wgpu::Buffer,
    num_vertices: u32,
}

const VERTICES: &[LigthVertex] = &[
    LigthVertex { pos: [-1., -1.] },
    LigthVertex { pos: [-1., 2.] },
    LigthVertex { pos: [2., -1.] },
];

impl LigthBatch {
    pub fn new(device: &wgpu::Device) -> Self {
        let num_vertices = VERTICES.len() as u32;
        let vertices_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        Self {
            vertices_buffer,
            num_vertices,
        }
    }

    pub fn draw<'a>(&'a self, pass: &mut LigthRenderPass<'a>) {
        let buffer = self.vertices_buffer.slice(..);
        pass.ligth.set_vertex_buffer(0, buffer);
        pass.ligth.draw(0..self.num_vertices, 0..1);
    }
}
