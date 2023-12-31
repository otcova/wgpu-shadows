use super::*;
use image::{DynamicImage, EncodableLayout, ImageBuffer};
use indoc::*;
use std::collections::HashMap;
use std::{ffi::OsStr, fs};

use texture_packer::{
    exporter::ImageExporter, importer::ImageImporter, texture::Texture, MultiTexturePacker,
    TexturePackerConfig,
};

const ATLAS_CONFIG: TexturePackerConfig = TexturePackerConfig {
    max_width: 1 << 12,
    max_height: 1 << 12,
    allow_rotation: false,
    texture_outlines: false,
    border_padding: 0,
    texture_padding: 1,
    texture_extrusion: 0,
    trim: true,
};

#[derive(Clone, Copy)]
pub struct AtlasView {
    pub pos: Vec2,
    pub size: Vec2,
}

#[derive(Default)]
pub struct Atlas {
    pub images: HashMap<String, AtlasView>,
}

fn pack_images<'a>(
    diffuse_pack: &mut MultiTexturePacker<'a, DynamicImage, String>,
    normal_pack: &mut MultiTexturePacker<'a, DynamicImage, String>,
) {
    for entry in fs::read_dir("assets").unwrap() {
        let Ok(entry) = entry else {
            continue;
        };
        let Ok(file_type) = entry.file_type() else {
            continue;
        };
        if !file_type.is_file() {
            continue;
        }

        let mut path = entry.path();
        if path.extension() != Some(OsStr::new("webp")) {
            continue;
        }

        let name = path.file_stem().unwrap().to_str().unwrap().to_string();
        if name.ends_with("_norm") {
            continue;
        }

        let image = ImageImporter::import_from_file(&path).unwrap();

        path.set_file_name(format!("{name}_norm.webp"));
        let norm_image = match ImageImporter::import_from_file(&path) {
            Ok(norm_image) => norm_image,
            // Create a normal default normal image
            Err(err) if err.starts_with("The system cannot find") => DynamicImage::ImageRgba8(
                ImageBuffer::from_vec(
                    image.width(),
                    image.height(),
                    image
                        .clone()
                        .into_rgba8()
                        .as_raw()
                        .chunks_exact(4)
                        .flat_map(|data| [128, 128, 255, data[3]])
                        .collect::<Vec<u8>>(),
                )
                .unwrap(),
            ),
            Err(err) => panic!("{:?}", err),
        };
        normal_pack.pack_own(name.clone(), norm_image).unwrap();

        diffuse_pack.pack_own(name, image).unwrap();
    }
}

fn export_textures<'a>(
    name: &str,
    diffuse_pack: &mut MultiTexturePacker<'a, DynamicImage, String>,
) {
    for (i, page) in diffuse_pack.get_pages().iter().enumerate() {
        let exporter = ImageExporter::export(page).unwrap();

        let encoder = webp::Encoder::from_image(&exporter).unwrap();
        let encoded_webp: webp::WebPMemory = encoder.encode_simple(true, 100.).unwrap();

        let path = format!("atlas/{}-{}.webp", name, i);
        fs::write(path, encoded_webp.as_bytes()).unwrap();
    }
}

fn generate_code<'a>(
    diffuse_pack: &mut MultiTexturePacker<'a, DynamicImage, String>,
    atlas: &mut Atlas,
) {
    let pages_count = diffuse_pack.get_pages().len();

    let mut texture_views = String::with_capacity(64 * pages_count);
    let mut load_diffuse_textures = String::with_capacity(64 * pages_count);
    let mut load_normal_textures = String::with_capacity(64 * pages_count);

    for (page_i, page) in diffuse_pack.get_pages().iter().enumerate() {
        let page_w = page.width() as f32;
        let page_h = page.height() as f32;
        let page_offset = page_i as f32;

        let mut page: Vec<_> = page.get_frames().into_iter().collect();
        page.sort_unstable_by_key(|(name, _)| *name);

        for (name, frame) in page {
            let x = (frame.frame.x as f32) / page_w + page_offset;
            let y = (frame.frame.y as f32) / page_h + page_offset;

            let pixel_w = frame.frame.w as f32;
            let pixel_h = frame.frame.h as f32;

            let w = pixel_w / page_w;
            let h = pixel_h / page_h;

            atlas.images.insert(
                name.clone(),
                AtlasView {
                    pos: Vec2::new(x, y),
                    size: Vec2::new(w, h),
                },
            );

            texture_views += &formatdoc! {"
                #[allow(dead_code)]
                pub fn view_{name}() -> TextureAtlasView {{
                    TextureAtlasView {{
                        pos: Vec2::new({x}f32, {y}f32),
                        size: Vec2::new({w}f32, {h}f32),
                        ratio: {}f32,
                    }}
                }}
            /", pixel_w / pixel_h};
            texture_views.pop();
        }

        load_diffuse_textures += &formatdoc! {r#"
            {indent}Texture::from_bytes(
            {indent}    ctx,
            {indent}    include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/atlas/diffuse-{page_i}.webp")),
            {indent}    "Diffuse Texture {page_i}",
            {indent})?,
            "#,
            indent = "                ",
        };
        load_normal_textures += &formatdoc! {r#"
            {indent}Texture::from_bytes(
            {indent}    ctx,
            {indent}    include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/atlas/normal-{page_i}.webp")),
            {indent}    "Normal Texture {page_i}",
            {indent})?,
            "#,
            indent = "                ",
        };
    }

    texture_views.pop();
    load_diffuse_textures.pop();
    load_normal_textures.pop();

    let texture_atlas_src = formatdoc! {"
        // THIS CODE IS GENERATED BY THE BUILD SCRIPT.
        // ANY CHANGE WILL BE OVERWRITTEN.
        
        use crate::error::*;
        use crate::math::*;
        use crate::wgpu_components::*;
        
        pub struct TextureAtlas {{
            pub diffuse_textures: [Texture; {pages_count}],
            pub normal_textures: [Texture; {pages_count}],
        }}

        #[derive(Copy, Clone)]
        pub struct TextureAtlasView {{
            pub pos: Vec2,
            pub size: Vec2,
            ratio: f32,
        }}

        impl TextureAtlas {{
            pub fn load(ctx: &WgpuContext) -> ErrResult<Self> {{
                Ok(Self {{
                    diffuse_textures: [
        {load_diffuse_textures}
                    ],
                    normal_textures: [
        {load_normal_textures}
                    ],
                }})
            }}
        
        {texture_views}
        }}

        impl TextureAtlasView {{
            /// Returns: Vec2 {{ x: ratio, y: 1.0 }}
            pub fn aspect_ratio_x1(&self) -> Vec2 {{
                Vec2::new(self.ratio, 1.)
            }}
        }}
    "};

    fs::write("src/texture_atlas.rs", texture_atlas_src).unwrap();
}

pub fn main() -> Atlas {
    let mut atlas = Atlas::default();

    let mut diffuse = MultiTexturePacker::new_skyline(ATLAS_CONFIG);
    let mut normal = MultiTexturePacker::new_skyline(ATLAS_CONFIG);

    pack_images(&mut diffuse, &mut normal);
    export_textures("diffuse", &mut diffuse);
    export_textures("normal", &mut normal);
    generate_code(&mut diffuse, &mut atlas);

    println!("cargo:rerun-if-changed=assets,atlas");

    atlas
}
