use crate::ligth_pipeline::LigthRenderPass;
use crate::quad_shader::QuadInstance;
use crate::vec_buffer::VecBuffer;
use crate::WgpuContext;

pub struct QuadLayer {
    buffer: VecBuffer<QuadInstance>,
}

impl QuadLayer {
    pub fn new(ctx: &WgpuContext) -> Self {
        Self {
            buffer: VecBuffer::new(ctx, wgpu::BufferUsages::VERTEX),
        }
    }

    pub fn push(&mut self, quad: QuadInstance) {
        self.buffer.push(quad);
    }

    pub fn draw<'a>(&'a mut self, pass: &mut LigthRenderPass<'a>) {
        let len = self.buffer.len() as u32;
        if let Some(quads) = self.buffer.view(pass.context) {
            pass.normal.set_vertex_buffer(0, quads);
            pass.normal.draw(0..4, 0..len);

            pass.diffuse.set_vertex_buffer(0, quads);
            pass.diffuse.draw(0..4, 0..len);
        }
    }
}
