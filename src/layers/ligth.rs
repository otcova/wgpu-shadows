use crate::ligth_pipeline::*;
use crate::shaders::*;
use crate::wgpu_components::*;

pub struct LigthLayer {
    ligth_index: Uniform,
    ligths: Vec<CachedUniform<LigthUniform>>,
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

    pub fn get_shadow_mut(&mut self, index: usize) -> &mut ShadowInstance {
        self.shadows.get_mut(index)
    }

    pub fn add_shadow(&mut self, shadow: ShadowInstance) -> usize {
        let index = self.shadows.len() - 1;
        *self.shadows.get_mut(index) = shadow;
        self.shadows.push(ShadowInstance::default());
        index
    }

    pub fn get_ligth_mut(&mut self, index: usize) -> &mut CachedUniform<LigthUniform> {
        &mut self.ligths[index]
    }

    pub fn add_ligth(&mut self, ctx: &WgpuContext, ligth: LigthUniform) -> usize {
        self.ligths.push(CachedUniform::new(
            ctx,
            wgpu::ShaderStages::VERTEX_FRAGMENT,
            ligth,
        ));
        self.ligths.len() - 1
    }

    pub fn draw<'a>(&'a mut self, pass: &mut LigthRenderPass<'a>) {
        let shadows_len = self.shadows.len() as u32;

        let idx = shadows_len - 1;
        self.ligth_index.update_buffer(pass.context, &idx);

        let buffer = self.shadows.view(pass.context).unwrap();

        pass.ligth.set_vertex_buffer(0, buffer);
        self.ligth_index.bind(2, &mut pass.ligth);

        for uniform in self.ligths.iter_mut() {
            uniform.update_buffers(pass.context);
            uniform.bind(3, &mut pass.ligth);
            pass.ligth.draw(0..4, 0..shadows_len);
        }
    }
}
