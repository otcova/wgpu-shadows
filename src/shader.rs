use crate::texture::Texture;
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
    pub vertex_layout: wgpu::VertexBufferLayout<'a>,
    pub output_format: wgpu::TextureFormat,
    pub blend: wgpu::BlendState,
    pub depth_stencil: Option<wgpu::DepthStencilState>,
}

impl Shader {
    pub fn new(device: &wgpu::Device, desc: ShaderDescriptor) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(desc.src),
        });

        let sampler = Texture::create_linear_sampler(&device);

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

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &entries,
            label: None,
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[&bind_group_layout],
                push_constant_ranges: &[],
            });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
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

        let bind_group = Self::new_bind_group(device, &bind_group_layout, &sampler, desc.textures);

        Self {
            pipeline,
            bind_group,
            bind_group_layout,
            sampler,
        }
    }

    fn new_bind_group(
        device: &wgpu::Device,
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

        device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout,
            entries: &entries,
            label: None,
        })
    }

    pub fn update_textures(&mut self, device: &wgpu::Device, textures: &[&wgpu::TextureView]) {
        self.bind_group =
            Self::new_bind_group(device, &self.bind_group_layout, &self.sampler, textures);
    }

    pub fn bind<'a>(&'a self, pass: &mut wgpu::RenderPass<'a>) {
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.bind_group, &[]);
    }
}
