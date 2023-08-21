use super::*;
use crate::font::*;
use crate::input::*;
use crate::layers::*;

pub struct TextInput {
    back_quad: usize,
    text_quads: Vec<usize>,
    pos: Vec2,
    text: String,
    placeholder: String,
}

pub struct TextInputDescriptor<'a> {
    pub layer: &'a mut QuadLayer,
    pub placeholder: &'a str,
    pub pos: Vec2,
}

impl TextInput {
    const COLOR: u32 = 0x2B1100FF;
    const HOVER_COLOR: u32 = 0x752A00FF;

    pub fn new(desc: TextInputDescriptor) -> Self {
        const MARGIN: f32 = 1.3;

        let back_quad = desc.layer.buffer.push(QuadInstance::new_color(
            desc.pos,
            Vec2::new(7., 1.) * UI_SIZE * MARGIN,
            Self::COLOR,
        ));

        let mut text_input = Self {
            pos: desc.pos,
            back_quad,
            text_quads: Vec::new(),
            text: String::new(),
            placeholder: String::from(desc.placeholder),
        };
        text_input.update_text_quads(desc.layer);
        text_input
    }

    fn hitbox_check(&self, pos: Vec2, layer: &QuadInstance) -> bool {
        let half_size = layer.size * 0.5;
        !(pos.x < layer.pos.x - half_size.x
            || layer.pos.x + half_size.x < pos.x
            || pos.y < layer.pos.y - half_size.y
            || layer.pos.y + half_size.y < pos.y)
    }

    fn update_text_quads(&mut self, layer: &mut QuadLayer) {
        let (text, color) = if self.text.is_empty() {
            (&self.placeholder, 0x888888FF)
        } else {
            (&self.text, 0)
        };

        let text_width = FONT.width(text) * UI_SIZE;
        let quads_it = FONT.write(text, UI_SIZE);

        let mut i = 0;
        for mut quad in quads_it {
            quad.color = color;
            quad.pos += self.pos;
            quad.pos.x -= text_width * 0.5; // Center Horizontally
            quad.pos.y -= UI_SIZE * 0.37; // Center Vertically

            if let Some(quad_index) = self.text_quads.get(i) {
                *layer.buffer.get_mut(*quad_index) = quad;
            } else {
                let quad_index = layer.buffer.push(quad);
                self.text_quads.push(quad_index);
            }
            i += 1;
        }

        // Make the remaining quads not visible
        for i in i..self.text_quads.len() {
            let quad_index = self.text_quads[i];
            layer.buffer.get_mut(quad_index).size.x = 0.;
        }
    }
}

impl InputEventHandler<QuadLayer> for TextInput {
    fn mouse_moved(&mut self, mouse: &Mouse, layer: &mut QuadLayer) {
        let quad = layer.buffer.get_ref(self.back_quad);

        let color = if self.hitbox_check(mouse.pos, quad) {
            Self::HOVER_COLOR
        } else {
            Self::COLOR
        };

        if quad.color != color {
            let quad = layer.buffer.get_mut(self.back_quad);
            quad.color = color;
        }
    }

    fn typed_text(&mut self, text: &str, layer: &mut QuadLayer) {
        self.text += text;
        self.update_text_quads(layer);
    }
}
