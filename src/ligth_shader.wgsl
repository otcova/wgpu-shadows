
struct VertexInput {
    @builtin(vertex_index) vertex_index: u32,
    @builtin(instance_index) instance_index: u32,
    @location(0) a: vec2<f32>,
    @location(1) b: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_pos: vec4<f32>,
    @location(0) ligth: f32,
    @location(1) pos: vec2<f32>,
};

fn quad_mesh(i: u32) -> vec2<f32> {
    return vec2(f32((i & 1u) << 1u), f32(i & 2u));
}

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    let camera_pos = vec2(0.);
    let z_offset = 0.;
    let ligth_index = 2u;
    
    var out: VertexOutput;

    if model.instance_index == ligth_index {
        out.ligth = 1.;
        out.pos = quad_mesh(model.vertex_index) - 1.;
    } else  {
        let shadow_size = 10.;
               
        out.pos = select(model.a, model.b, vec2<bool>((model.vertex_index & 1u) == 0u));
        
        if (model.vertex_index & 2u) != 0u {
            out.pos += (out.pos - camera_pos) * shadow_size;
        }
    }
    
    out.clip_pos = vec4<f32>(out.pos, z_offset, 1.);
    return out;
}

@group(0) @binding(0)
var tex_sampler: sampler;

@group(0) @binding(1)
var normal_tex: texture_2d<f32>;
 
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let normal_color = textureSample(normal_tex, tex_sampler, in.pos * vec2(0.5, -0.5) + 0.5).rgb;
        
    if in.ligth > 0. {
         let ligth_pos = vec3(0., 0., 0.2);
         let ligth_color = vec3(1., 1., 1.);
         let ligth_brigthness = 10.;

         let dist_vec = ligth_pos - vec3(in.pos, 0.);
         let sq_dist = dot(dist_vec, dist_vec);
         let dist = sqrt(sq_dist);

         let normal = normalize(normal_color * 2. - 1.);
         let angle_attenuation = max(0., dot(dist_vec / dist, normal));

         let falloff = vec3(0.75, 3., 20.);
         let dist_attenuation = 1. / (falloff.x + falloff.y * dist + falloff.z * sq_dist);

         let final_color = angle_attenuation * dist_attenuation * ligth_color * ligth_brigthness;
         return vec4(final_color, 1.);
    }
    return vec4(0.);
}

