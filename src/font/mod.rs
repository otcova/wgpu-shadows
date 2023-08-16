mod parser;

pub use parser::*;

use crate::math::*;
use crate::shaders::*;

pub struct Font {
    tex_size: Vec2,
    line_height: f32,
    glyphs: Vec<Option<Glyph>>,
    glyphs_start: usize,
}

#[derive(Default, Copy, Clone)]
struct Glyph {
    tex_pos: Vec2,
    tex_size: Vec2,
    pos: Vec2,
    size: Vec2,
    advance: f32,
}

impl Font {
    /// Calculates the width of the text with size 1.0
    pub fn width<'a>(&'a self, text: &'a str) -> f32 {
        let mut width = 0.;
        let mut last_glyph = None;

        for char_id in text.chars() {
            if let Some(glyph) = self.get_glyph(char_id) {
                width += glyph.advance;
                last_glyph = Some(glyph);
            }
        }

        if let Some(last_glyph) = last_glyph {
            width - last_glyph.advance + last_glyph.pos.x + last_glyph.size.x
        } else {
            width
        }
    }

    pub fn write<'a>(
        &'a self,
        text: &'a str,
        scale: f32,
    ) -> impl Iterator<Item = QuadInstance> + 'a {
        let mut pos = Vec2::new(0., scale);

        text.chars().flat_map(move |char_id| {
            if let Some(glyph) = self.get_glyph(char_id) {
                let quad_size = glyph.size * scale;

                let quad = QuadInstance {
                    pos: pos + glyph.pos * scale + quad_size * Vec2::new(0.5, -0.5),
                    size: quad_size,
                    angle: 0.,
                    tex_pos: glyph.tex_pos,
                    tex_size: glyph.tex_size,
                };

                pos.x += glyph.advance * scale;

                Some(quad)
            } else {
                None
            }
        })
    }

    fn get_glyph(&self, char_id: char) -> Option<&Glyph> {
        if self.glyphs_start > char_id as usize {
            None
        } else {
            match self.glyphs.get(char_id as usize - self.glyphs_start)? {
                Some(glyph) => Some(glyph),
                None => None,
            }
        }
    }
}
