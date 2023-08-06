
struct VertexInput {
    @builtin(vertex_index) vertex_index: u32,
    @location(0) pos: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    
    out.tex_coords = model.pos * vec2(0.5, -0.5) + 0.5;
    out.clip_position = vec4<f32>(model.pos, 1., 1.);
    
    return out;
}


@group(0) @binding(0)
var tex_sampler: sampler;

@group(0) @binding(1)
var color_tex: texture_2d<f32>;

@group(0) @binding(2)
var normal_tex: texture_2d<f32>;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let origin = vec2<f32>();
    
    let color = textureSample(color_tex, tex_sampler, in.tex_coords);
    let normal = textureSample(normal_tex, tex_sampler, in.tex_coords);// * 2. - 1.;
    
    //let color = trunc(normal_color);
    //normal_color = (normal_color - color);
    
    // let dist_vec = in.tex_coords - origin;
    // let sq_dist = dot(dist_vec, dist_vec);
    // let dist = sqrt(sq_dist);
    
    // let falloff = vec3(0.75, 3., 20.);
    // let attenuation = 1. / (falloff * vec3(1., dist, sq_dist));

    // return vec4(1., 0., 0., 1.);
    return color;
}
 