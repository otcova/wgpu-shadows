struct CameraUniform {
    pos: vec2<f32>,
    size: vec2<f32>,
}

@group(1) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexInput {
    @builtin(vertex_index) vertex_index: u32,
    @location(0) pos: vec2<f32>,
    @location(1) size: vec2<f32>,
    @location(2) angle: f32,
    @location(3) tex_pos: vec2<f32>,
    @location(4) tex_size: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) pos: vec2<f32>,
};

fn quad_mesh(i: u32) -> vec2<f32> {
    return vec2(f32(i & 1u), f32((i & 2u) >> 1u));
}

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;

    let coord = quad_mesh(model.vertex_index);

    out.tex_coords = vec2(coord.x, 1. - coord.y) * model.tex_size + model.tex_pos;
    
    let c = cos(model.angle);
    let s = sin(model.angle);
    let rotation_matrix = mat2x2<f32>(c, -s, s, c);
    
    out.pos = rotation_matrix * (coord - 0.5) * model.size + model.pos;
    out.pos += camera.pos;
    out.pos *= camera.size;
    
    out.clip_position = vec4<f32>(out.pos, 0., 1.);
    out.pos = out.pos * vec2(0.5, -0.5) + 0.5;
    
    return out;
}



@group(0) @binding(0)
var tex_sampler: sampler;

@group(0) @binding(2)
var atlas_tex: texture_2d<f32>;

fn grayscale(color: vec3<f32>) -> f32 {
    return dot(color, vec3(0.2126, 0.7152, 0.0722));
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(atlas_tex, tex_sampler, in.tex_coords);
}
 