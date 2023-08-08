use crate::ligth_pipeline::LigthRenderPass;
use crate::quad_shader::QuadInstance;
use crate::vec_buffer::VecBuffer;

pub struct QuadLayer {
    buffer: VecBuffer<QuadInstance>,
}

impl QuadLayer {
    pub fn new(device: &wgpu::Device) -> Self {
        Self {
            buffer: VecBuffer::new(device, wgpu::BufferUsages::VERTEX),
        }
    }

    pub fn push(&mut self, quad: QuadInstance) {
        self.buffer.push(quad);
    }

    pub fn draw<'a>(
        &'a mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        pass: &mut LigthRenderPass<'a>,
    ) {
        let len = self.buffer.len() as u32;
        if let Some(quads) = self.buffer.view(device, queue) {
            pass.normal.set_vertex_buffer(0, quads);
            pass.normal.draw(0..4, 0..len);

            pass.diffuse.set_vertex_buffer(0, quads);
            pass.diffuse.draw(0..4, 0..len);
        }
    }
}
