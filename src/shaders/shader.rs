use crate::{texture::Texture, WgpuContext};
use std::borrow::Cow;

pub struct Shader {
    pipeline: wgpu::RenderPipeline,
    bind_group: wgpu::BindGroup,
    bind_group_layout: wgpu::BindGroupLayout,
    sampler: wgpu::Sampler,
}

pub struct ShaderDescriptor<'a> {
    pub src: Cow<'a, str>,
    pub textures: &'a [&'a wgpu::TextureView],
    pub uniforms: &'a [&'a wgpu::BindGroupLayout],
    pub vertex_layout: wgpu::VertexBufferLayout<'a>,
    pub output_format: wgpu::TextureFormat,
    pub blend: wgpu::BlendState,
    pub depth_stencil: Option<wgpu::DepthStencilState>,
}

impl Shader {
    pub fn new(ctx: &WgpuContext, desc: ShaderDescriptor) -> Self {
        let shader = ctx
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(desc.src),
            });

        let sampler = Texture::create_linear_sampler(&ctx.device);

        let mut entries = Vec::with_capacity(desc.textures.len() + 1);

        entries.push(wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
            count: None,
        });

        for binding in 1..=desc.textures.len() as u32 {
            entries.push(wgpu::BindGroupLayoutEntry {
                binding,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    multisampled: false,
                    view_dimension: wgpu::TextureViewDimension::D2,
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                },
                count: None,
            });
        }

        let bind_group_layout =
            ctx.device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("Texture bind group layout"),
                    entries: &entries,
                });

        let mut bind_group_layouts = Vec::with_capacity(1 + desc.uniforms.len());
        bind_group_layouts.push(&bind_group_layout);
        bind_group_layouts.extend_from_slice(desc.uniforms);

        let render_pipeline_layout =
            ctx.device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: None,
                    bind_group_layouts: &bind_group_layouts,
                    push_constant_ranges: &[],
                });

        let pipeline = ctx
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: None,
                layout: Some(&render_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[desc.vertex_layout],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: desc.output_format,
                        blend: Some(desc.blend),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleStrip,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: None,
                    polygon_mode: wgpu::PolygonMode::Fill,
                    unclipped_depth: false,
                    conservative: false,
                },
                depth_stencil: desc.depth_stencil,
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview: None,
            });

        let bind_group = Self::new_bind_group(ctx, &bind_group_layout, &sampler, desc.textures);

        Self {
            pipeline,
            bind_group,
            bind_group_layout,
            sampler,
        }
    }

    fn new_bind_group(
        ctx: &WgpuContext,
        layout: &wgpu::BindGroupLayout,
        sampler: &wgpu::Sampler,
        textures: &[&wgpu::TextureView],
    ) -> wgpu::BindGroup {
        let mut entries = Vec::with_capacity(textures.len() + 1);
        entries.push(wgpu::BindGroupEntry {
            binding: 0,
            resource: wgpu::BindingResource::Sampler(sampler),
        });

        let mut binding = 1;
        for texture_view in textures {
            entries.push(wgpu::BindGroupEntry {
                binding,
                resource: wgpu::BindingResource::TextureView(texture_view),
            });
            binding += 1;
        }

        ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Texture bind group"),
            layout,
            entries: &entries,
        })
    }

    pub fn update_textures(&mut self, ctx: &WgpuContext, textures: &[&wgpu::TextureView]) {
        self.bind_group =
            Self::new_bind_group(ctx, &self.bind_group_layout, &self.sampler, textures);
    }

    pub fn bind<'a>(&'a self, pass: &mut wgpu::RenderPass<'a>) {
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.bind_group, &[]);
    }
}
