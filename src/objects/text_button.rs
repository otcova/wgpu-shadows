use crate::font::*;
use crate::layers::*;
use crate::math::*;
use crate::mouse::*;
use crate::shaders::*;
use crate::texture_atlas::*;

pub struct TextButton {
    back_quad: usize,
    text_quads: Vec<usize>,
}

pub struct TextButtonDescriptor<'a> {
    pub layer: &'a mut QuadLayer,
    pub font: &'a Font,
    pub text: &'a str,
    pub pos: Vec2,
    pub size: f32,
}

impl TextButton {
    pub fn new(desc: TextButtonDescriptor) -> Self {
        const MARGIN: f32 = 0.04;

        let view = TextureAtlas::view_text_button();
        let back_quad = desc.layer.buffer.push(QuadInstance {
            pos: desc.pos,
            size: view.aspect_ratio_x1() * desc.size + MARGIN,
            angle: 0.,
            tex_pos: view.pos,
            tex_size: view.size,
        });

        let text_width = desc.font.width(desc.text) * desc.size;

        let text_quads = desc
            .font
            .write(desc.text, desc.size)
            .map(|mut quad| {
                quad.pos += desc.pos;
                quad.pos.x -= text_width * 0.5; // Center Horizontally
                quad.pos.y -= desc.size * 0.37; // Center Vertically
                desc.layer.buffer.push(quad)
            })
            .collect();

        Self {
            back_quad,
            text_quads,
        }
    }

    fn hitbox_check(&self, pos: Vec2, layer: &QuadInstance) -> bool {
        let half_size = layer.size * 0.5;
        !(pos.x < layer.pos.x - half_size.x
            || layer.pos.x + half_size.x < pos.x
            || pos.y < layer.pos.y - half_size.y
            || layer.pos.y + half_size.y < pos.y)
    }

    pub fn mouse_moved(&mut self, pos: Vec2, layer: &mut QuadLayer) {
        let quad = layer.buffer.get_ref(self.back_quad);

        let view = if self.hitbox_check(pos, quad) {
            TextureAtlas::view_text_button_hover()
        } else {
            TextureAtlas::view_text_button()
        };

        if quad.tex_pos.x != view.pos.x {
            let quad = layer.buffer.get_mut(self.back_quad);
            quad.tex_pos = view.pos;
        }
    }
}
