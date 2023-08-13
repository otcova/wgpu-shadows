use crate::ligth_pipeline::LigthRenderPass;
use crate::shaders::{LigthUniform, ShadowInstance};
use crate::uniform::Uniform;
use crate::vec_buffer::VecBuffer;
use crate::WgpuContext;

pub struct LigthLayer {
    ligth_index: Uniform,
    ligths: Vec<Uniform>,
    shadows: VecBuffer<ShadowInstance>,
}

impl LigthLayer {
    pub fn new(ctx: &WgpuContext) -> Self {
        let mut shadows = VecBuffer::new(ctx, wgpu::BufferUsages::VERTEX);
        shadows.push(ShadowInstance::default());
        Self {
            ligth_index: Uniform::new(ctx, wgpu::ShaderStages::VERTEX, &0u32),
            ligths: Vec::new(),
            shadows,
        }
    }

    pub fn clear_shadows(&mut self) {
        self.shadows.clear();
        self.shadows.push(ShadowInstance::default());
    }

    pub fn add_shadow(&mut self, shadow: ShadowInstance) {
        *self.shadows.get_mut(self.shadows.len() - 1) = shadow;
        self.shadows.push(ShadowInstance::default());
    }

    pub fn add_ligth(&mut self, ctx: &WgpuContext, ligth: &LigthUniform) {
        self.ligths.push(Uniform::new(
            ctx,
            wgpu::ShaderStages::VERTEX_FRAGMENT,
            ligth,
        ));
    }

    pub fn draw<'a>(&'a mut self, pass: &mut LigthRenderPass<'a>) {
        let shadows_len = self.shadows.len() as u32;

        let idx = shadows_len - 1;
        self.ligth_index.update_buffer(pass.context, &idx);

        let buffer = self.shadows.view(pass.context).unwrap();

        pass.ligth.set_vertex_buffer(0, buffer);
        self.ligth_index.bind(2, &mut pass.ligth);

        for uniform in self.ligths.iter() {
            uniform.bind(3, &mut pass.ligth);
            pass.ligth.draw(0..4, 0..shadows_len);
        }
    }
}
