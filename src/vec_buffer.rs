use crate::WgpuContext;
use std::ops::Range;
use wgpu::{util::DeviceExt, BufferAddress};

pub struct VecBuffer<T: bytemuck::NoUninit> {
    data: Vec<T>,
    buffer: wgpu::Buffer,
    update_range: Option<Range<usize>>,
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
                    usage: usage | wgpu::BufferUsages::COPY_DST,
                }),
            update_range: None,
        }
    }

    pub fn get_mut(&mut self, idx: usize) -> &mut T {
        if let Some(range) = &mut self.update_range {
            range.start = range.start.min(idx);
            range.end = range.end.max(idx + 1);
        } else {
            self.update_range = Some(idx..idx + 1);
        }

        &mut self.data[idx]
    }

    pub fn push(&mut self, quad: T) {
        self.data.push(quad);

        let end = self.data.len();
        if let Some(range) = &mut self.update_range {
            range.end = end;
        } else {
            self.update_range = Some(end - 1..end);
        }
    }

    pub fn clear(&mut self) {
        self.data.clear();
        self.update_range = Some(0..1);
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn view(&mut self, ctx: &WgpuContext) -> Option<wgpu::BufferSlice> {
        if self.data.is_empty() {
            return None;
        }

        let item_bytes: usize = std::mem::size_of::<T>();
        let data_size = (self.data.len() * item_bytes) as BufferAddress;

        if let Some(update_range) = self.update_range.clone() {
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
                    (update_range.start * item_bytes) as BufferAddress,
                    bytemuck::cast_slice(&self.data[update_range]),
                );
            }
            self.update_range = None;
        }

        Some(self.buffer.slice(0..data_size))
    }
}
