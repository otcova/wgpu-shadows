
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
    let ligth_pos = vec3(0.5, 0.5, 0.2);
    let ligth_color = vec3(1.9, 1., 1.);
    let ligth_brigthness = 5.;
    
    let color = textureSample(color_tex, tex_sampler, in.tex_coords);
    let normal_color = textureSample(normal_tex, tex_sampler, in.tex_coords).rgb;
  
    let dist_vec = ligth_pos - vec3(in.tex_coords, 0.);
    let sq_dist = dot(dist_vec, dist_vec);
    let dist = sqrt(sq_dist);

    let normal = normalize(normal_color * 2. - 1.);
    let angle_attenuation = max(0., dot(dist_vec / dist, normal));
    
    let falloff = vec3(0.75, 3., 20.);
    let dist_attenuation = 1. / (falloff.x + falloff.y * dist + falloff.z * sq_dist);

    let final_color = color.rgb * angle_attenuation * dist_attenuation * ligth_color * ligth_brigthness;
    return vec4(final_color, color.a);
}
 