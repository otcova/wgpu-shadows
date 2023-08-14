use super::shadow_from_shape;
use crate::layers::*;
use crate::math::*;
use crate::shaders::*;
use crate::shapes::*;
use crate::texture_atlas::*;

macro_rules! block_object {
    ($Struct:ident, $SHAPE:ident, $image:ident) => {
        pub struct $Struct {
            quad_id: usize,
            shadow_id: [usize; $SHAPE.len()],
        }

        impl $Struct {
            pub fn new(layer: &mut QuadLayer, ligth_layer: &mut LigthLayer, pos: Vec2) -> Self {
                let size = 0.3;

                let quad_id =
                    layer
                        .buffer
                        .push(QuadInstance::new(pos, size, TextureAtlas::$image()));

                let mut shadow_id = [0; $SHAPE.len()];

                for (i, mut shadow) in shadow_from_shape(&$SHAPE).enumerate() {
                    shadow.a = shadow.a * size + pos;
                    shadow.b = shadow.b * size + pos;
                    shadow_id[i] = ligth_layer.add_shadow(shadow);
                }

                Self { quad_id, shadow_id }
            }

            #[allow(unused)]
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
    };
}

block_object!(BlockSq2, BLOCK_SQ2, view_block_sq2);
block_object!(BlockSq3, BLOCK_SQ3, view_block_sq3);
