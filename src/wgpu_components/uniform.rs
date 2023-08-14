use crate::wgpu_components::*;
use bytemuck::NoUninit;
use wgpu::util::DeviceExt;

pub struct Uniform {
    pub layout: wgpu::BindGroupLayout,
    bind_group: wgpu::BindGroup,
    buffer: wgpu::Buffer,
}

pub struct CachedUniform<T: NoUninit> {
    uniform: Uniform,
    pub data: T,
    pub needs_update: bool,
}

impl<T: NoUninit> CachedUniform<T> {
    pub fn new(ctx: &WgpuContext, visibility: wgpu::ShaderStages, data: T) -> Self {
        Self {
            uniform: Uniform::new(ctx, visibility, &data),
            data,
            needs_update: false,
        }
    }

    pub fn update(&mut self, data: T) {
        self.data = data;
        self.needs_update = true;
    }

    pub fn update_buffers(&mut self, ctx: &WgpuContext) {
        if self.needs_update {
            self.uniform.update_buffer(ctx, &self.data);
            self.needs_update = false;
        }
    }

    pub fn bind<'a>(&'a self, group: u32, pass: &mut wgpu::RenderPass<'a>) {
        self.uniform.bind(group, pass);
    }
}

impl Uniform {
    pub fn new<T: NoUninit>(ctx: &WgpuContext, visibility: wgpu::ShaderStages, data: &T) -> Self {
        let buffer = ctx
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::bytes_of(data),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });

        let layout = Self::new_layout(ctx, visibility);

        let bind_group = ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Uniform bind group"),
            layout: &layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        });

        Self {
            layout,
            bind_group,
            buffer,
        }
    }

    pub fn new_layout(ctx: &WgpuContext, visibility: wgpu::ShaderStages) -> wgpu::BindGroupLayout {
        ctx.device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Uniform bind group layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            })
    }

    pub fn update_buffer<T: NoUninit>(&self, ctx: &WgpuContext, data: &T) {
        ctx.queue
            .write_buffer(&self.buffer, 0, bytemuck::bytes_of(data));
    }

    pub fn bind<'a>(&'a self, group: u32, pass: &mut wgpu::RenderPass<'a>) {
        pass.set_bind_group(group, &self.bind_group, &[]);
    }
}
