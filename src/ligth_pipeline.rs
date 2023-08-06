pub struct LigthTextures {
    pub diffuse: wgpu::TextureView,
    pub normal: wgpu::TextureView,
}

pub struct LigthPipeline {
    pub textures: LigthTextures,
}

struct Encoders {
    diffuse: wgpu::CommandEncoder,
    normal: wgpu::CommandEncoder,
    ligth: wgpu::CommandEncoder,
}

pub struct LigthFrame<'a> {
    encoders: Encoders,

    output_view: &'a wgpu::TextureView,
    queue: &'a wgpu::Queue,
    ligth_pipeline: &'a mut LigthPipeline,
}

pub struct LigthRenderPass<'a> {
    pub diffuse: wgpu::RenderPass<'a>,
    pub normal: wgpu::RenderPass<'a>,
    pub ligth: wgpu::RenderPass<'a>,
}

impl<'a> LigthFrame<'a> {
    pub fn create_render_pass<'b>(&'b mut self) -> LigthRenderPass<'b> {
        LigthRenderPass {
            diffuse: self
                .encoders
                .diffuse
                .begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Quad render pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &self.ligth_pipeline.textures.diffuse,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 0.1,
                                g: 0.2,
                                b: 0.3,
                                a: 1.0,
                            }),
                            store: true,
                        },
                    })],
                    depth_stencil_attachment: None,
                }),
            normal: self
                .encoders
                .normal
                .begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Quad render pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &self.ligth_pipeline.textures.normal,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 0.5,
                                g: 0.5,
                                b: 1.0,
                                a: 1.0,
                            }),
                            store: true,
                        },
                    })],
                    depth_stencil_attachment: None,
                }),
            ligth: self
                .encoders
                .ligth
                .begin_render_pass(&wgpu::RenderPassDescriptor {
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: self.output_view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                            store: true,
                        },
                    })],
                    depth_stencil_attachment: None,
                    label: Some("Ligth render pass"),
                }),
        }
    }

    pub fn resolve(self) {
        self.encoders.finish(self.queue);
    }
}

impl Encoders {
    fn new(device: &wgpu::Device) -> Self {
        Self {
            diffuse: device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Diffuse command encoder"),
            }),
            normal: device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Normal command encoder"),
            }),
            ligth: device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Ligth command encoder"),
            }),
        }
    }

    fn finish(self, queue: &wgpu::Queue) {
        queue.submit([self.diffuse.finish(), self.normal.finish()]);
        queue.submit([self.ligth.finish()]);
    }
}

impl LigthTextures {
    fn new(device: &wgpu::Device, width: u32, height: u32) -> Self {
        let size = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };
        let texture_desc = wgpu::TextureDescriptor {
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            label: Some("Ligth target texture description"),
            view_formats: &[],
        };

        Self {
            diffuse: device.create_texture(&texture_desc).create_view(
                &wgpu::TextureViewDescriptor {
                    label: Some("Ligth diffuse target texture"),
                    ..Default::default()
                },
            ),
            normal: device.create_texture(&texture_desc).create_view(
                &wgpu::TextureViewDescriptor {
                    label: Some("Ligth normal target texture"),
                    ..Default::default()
                },
            ),
        }
    }
}

impl LigthPipeline {
    pub fn new(device: &wgpu::Device, width: u32, height: u32) -> Self {
        let textures = LigthTextures::new(device, width, height);
        Self { textures }
    }

    pub fn resize(&mut self, device: &wgpu::Device, width: u32, height: u32) {
        self.textures = LigthTextures::new(device, width, height);
    }

    pub fn start_frame<'a>(
        &'a mut self,
        device: &'a wgpu::Device,
        queue: &'a wgpu::Queue,
        output_view: &'a wgpu::TextureView,
    ) -> LigthFrame<'a> {
        LigthFrame {
            encoders: Encoders::new(device),
            queue,
            output_view,
            ligth_pipeline: self,
        }
    }
}
