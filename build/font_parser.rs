use super::atlas::*;
use super::*;

pub struct Font {
    pub tex_size: Vec2,
    pub line_height: f32,
    pub glyphs: Vec<Option<Glyph>>,
    pub glyphs_start: usize,
}

#[derive(Default, Copy, Clone)]
pub struct Glyph {
    pub tex_pos: Vec2,
    pub tex_size: Vec2,
    pub pos: Vec2,
    pub size: Vec2,
    pub advance: f32,
}

impl Font {
    pub fn parse(fnt_file: &str, texture_view: AtlasView) -> Self {
        let mut font = Font {
            tex_size: Vec2::zero(),
            line_height: 0.,
            glyphs: Vec::with_capacity(128),
            glyphs_start: 0,
        };

        let mut lines = fnt_file.split('\n');
        font.parse_info(lines.next().unwrap());
        font.parse_common(lines.next().unwrap());

        for line in lines.skip(2) {
            font.parse_page_line(line);
        }
        font.glyphs.shrink_to_fit();

        if font.glyphs.len() > 300 {
            super::log!(
                "!!!!!!!!! The font has a lot of glyphs {} !!!!!!!!!",
                font.glyphs.len()
            );
        }

        font.normalize_pixels(texture_view);

        font
    }

    /// Converts all pixel coordinates to texture coordinates
    fn normalize_pixels(&mut self, texture_view: AtlasView) {
        for glyph in &mut self.glyphs {
            if let Some(glyph) = glyph {
                glyph.size = glyph.tex_size;

                glyph.tex_pos *= texture_view.size / self.tex_size;
                glyph.tex_pos += texture_view.pos;
                glyph.tex_size *= texture_view.size / self.tex_size;

                glyph.pos /= self.line_height;
                glyph.size /= self.line_height;
                glyph.advance /= self.line_height;
            }
        }
    }

    fn parse_info(&self, info: &str) {
        if !info.starts_with("info ") {
            panic!("Invalid font.fnt file (info line)");
        }
    }

    fn parse_common(&mut self, common: &str) {
        if !common.starts_with("common ") {
            panic!("Invalid font.fnt file (common line)");
        }

        let common = &common["common ".len()..];

        for var in common.split(' ') {
            let (var_name, var_value) = var.split_once('=').unwrap();

            match var_name {
                "lineHeight" => self.line_height = var_value.parse::<i32>().unwrap() as f32,
                "scaleW" => self.tex_size.x = var_value.parse::<i32>().unwrap() as f32,
                "scaleH" => self.tex_size.y = var_value.parse::<i32>().unwrap() as f32,
                _ => {}
            }
        }
    }

    fn parse_page_line(&mut self, line: &str) {
        if line.starts_with("char ") {
            self.parse_char(&line["char ".len()..]);
        } else if line.starts_with("kerning") {
            // self.parse_kerning(&line["kerning ".len()..]);
        }
    }

    fn parse_char(&mut self, char: &str) {
        let mut glyph = Glyph::default();
        let mut id: usize = 0;

        for var in char.split(' ') {
            let (var_name, var_value) = var.split_once('=').unwrap();

            match var_name {
                "id" => id = var_value.parse().unwrap(),
                "x" => glyph.tex_pos.x = var_value.parse::<i32>().unwrap() as f32,
                "y" => glyph.tex_pos.y = var_value.parse::<i32>().unwrap() as f32,
                "width" => glyph.tex_size.x = var_value.parse::<i32>().unwrap() as f32,
                "height" => glyph.tex_size.y = var_value.parse::<i32>().unwrap() as f32,
                "xoffset" => glyph.pos.x = var_value.parse::<i32>().unwrap() as f32,
                "yoffset" => glyph.pos.y = -var_value.parse::<i32>().unwrap() as f32,
                "xadvance" => glyph.advance = var_value.parse::<i32>().unwrap() as f32,
                _ => {}
            }
        }

        if self.glyphs.is_empty() {
            self.glyphs_start = id;
            self.glyphs.push(Some(glyph));
        } else {
            if self.glyphs_start > id {
                let pad = self.glyphs_start - id;
                self.glyphs.splice(0..0, std::iter::repeat(None).take(pad));
                self.glyphs_start = id;
            } else if self.glyphs.len() <= id - self.glyphs_start {
                self.glyphs.resize(id - self.glyphs_start + 1, None);
            }

            self.glyphs[id - self.glyphs_start] = Some(glyph);
        }
    }
}
