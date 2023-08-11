use wgpu::{util::DeviceExt, BufferAddress};

use crate::WgpuContext;

pub struct VecBuffer<T: bytemuck::NoUninit> {
    data: Vec<T>,
    buffer: wgpu::Buffer,
    update_index: Option<usize>,
}

impl<T: bytemuck::NoUninit> VecBuffer<T> {
    pub fn new(ctx: &WgpuContext, usage: wgpu::BufferUsages) -> Self {
        Self {
            data: vec![],
            buffer: ctx
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Vec buffer"),
                    contents: &[],
                    usage,
                }),
            update_index: None,
        }
    }

    pub fn push(&mut self, quad: T) {
        if self.update_index.is_none() {
            self.update_index = Some(self.data.len());
        }
        self.data.push(quad);
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn view(&mut self, ctx: &WgpuContext) -> Option<wgpu::BufferSlice> {
        if self.data.is_empty() {
            return None;
        }

        let data_size = (self.data.len() * std::mem::size_of::<T>()) as BufferAddress;

        if let Some(update_index) = self.update_index {
            if self.buffer.size() < data_size {
                // Realocate Buffer
                self.buffer = ctx
                    .device
                    .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some("Vec buffer"),
                        contents: bytemuck::cast_slice(&self.data[..]),
                        usage: self.buffer.usage(),
                    });
            } else {
                // Update Buffer
                ctx.queue.write_buffer(
                    &self.buffer,
                    (update_index * std::mem::size_of::<T>()) as BufferAddress,
                    bytemuck::cast_slice(&self.data[update_index..]),
                );
            }
            self.update_index = None;
        }

        Some(self.buffer.slice(0..data_size))
    }
}
