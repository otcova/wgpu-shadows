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
    const COLOR: u32 = 0x41000AFF;
    const HOVER_COLOR: u32 = 0x752A00FF;

    pub fn new(desc: TextButtonDescriptor) -> Self {
        const MARGIN: f32 = 1.3;

        let tex_view = TextureAtlas::view_text_button();

        let back_quad = desc.layer.buffer.push(QuadInstance {
            pos: desc.pos,
            size: tex_view.aspect_ratio_x1() * desc.size * MARGIN,
            color: Self::COLOR,
            angle: 0.,
            tex_pos: tex_view.pos,
            tex_size: tex_view.size,
        });

        let text_width = desc.font.width(desc.text) * desc.size;

        let text_quads = desc
            .font
            .write(desc.text, desc.size)
            .map(|mut quad| {
                quad.pos += desc.pos;
                quad.pos.x -= text_width * 0.5;
                quad.pos.y -= desc.size * 0.37;
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
}

impl MouseEventHandler<QuadLayer> for TextButton {
    fn moved(&mut self, mouse: &Mouse, layer: &mut QuadLayer) {
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
}
