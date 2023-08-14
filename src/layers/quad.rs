use crate::ligth_pipeline::*;
use crate::shaders::*;
use crate::wgpu_components::*;

pub struct QuadLayer {
    pub buffer: VecBuffer<QuadInstance>,
}

impl QuadLayer {
    pub fn new(ctx: &WgpuContext) -> Self {
        Self {
            buffer: VecBuffer::new(ctx, wgpu::BufferUsages::VERTEX),
        }
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
