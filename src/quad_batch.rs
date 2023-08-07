use crate::{ligth_pipeline::LigthRenderPass, texture_atlas::TextureAtlas};

use super::quad_shader::QuadInstance;
use wgpu::util::DeviceExt;

pub struct QuadBatch {
    instances_buffer: wgpu::Buffer,
    num_instances: u32,
}

impl QuadBatch {
    pub fn new(device: &wgpu::Device) -> Self {
        let instances: &[QuadInstance] = &[
            QuadInstance {
                pos: [0., 0.],
                size: [2., 2.],
                angle: 0.,
                tex_pos: TextureAtlas::view_triangles().tex_pos,
                tex_size: TextureAtlas::view_triangles().tex_size,
            },
            QuadInstance {
                pos: [0.0, 0.5],
                size: [1., 1.],
                angle: 0.,
                tex_pos: TextureAtlas::view_arrow().tex_pos,
                tex_size: TextureAtlas::view_arrow().tex_size,
            },
            QuadInstance {
                pos: [0.0, -0.5],
                size: [0.5, 0.5],
                angle: 0.2,
                tex_pos: [0., 0.],
                tex_size: [0.1, 0.1],
            },
        ];

        let num_instances = instances.len() as u32;
        let instances_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Quad Vertex Buffer"),
            contents: bytemuck::cast_slice(instances),
            usage: wgpu::BufferUsages::VERTEX,
        });

        Self {
            instances_buffer,
            num_instances,
        }
    }

    pub fn draw<'a>(&'a self, pass: &mut LigthRenderPass<'a>) {
        let buffer = self.instances_buffer.slice(..);

        pass.diffuse.set_vertex_buffer(0, buffer);
        pass.diffuse.draw(0..4, 0..self.num_instances);

        pass.normal.set_vertex_buffer(0, buffer);
        pass.normal.draw(0..4, 0..self.num_instances);
    }
}
