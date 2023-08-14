use crate::{
    layers::{LigthLayer, QuadLayer},
    math::Vec2,
    shaders::{QuadInstance, ShadowInstance},
    shapes::BLOCK_SQ2,
    texture_atlas::TextureAtlas,
};

pub struct BlockSq2 {
    quad_id: usize,
    shadow_id: [usize; BLOCK_SQ2.len() - 1],
}

impl BlockSq2 {
    pub fn new(layer: &mut QuadLayer, ligth_layer: &mut LigthLayer) -> Self {
        let pos = Vec2::new(0., 0.);
        let size = 0.3;

        let quad_id =
            layer
                .buffer
                .push(QuadInstance::new(pos, size, TextureAtlas::view_block_sq2()));

        let mut shadow_id = [0; BLOCK_SQ2.len() - 1];

        for i in 0..shadow_id.len() {
            shadow_id[i] = ligth_layer.add_shadow(ShadowInstance {
                a: pos + BLOCK_SQ2[i] * size,
                b: pos + BLOCK_SQ2[i + 1] * size,
            });
        }

        Self { quad_id, shadow_id }
    }

    pub fn set_pos(&self, layer: &mut QuadLayer, ligth_layer: &mut LigthLayer, pos: Vec2) {
        let quad = layer.buffer.get_mut(self.quad_id);
        for shadow_id in self.shadow_id {
            let shadow = ligth_layer.get_shadow_mut(shadow_id);
            shadow.a = shadow.a - quad.pos + pos;
            shadow.b = shadow.b - quad.pos + pos;
        }
        quad.pos = pos;
    }
}
