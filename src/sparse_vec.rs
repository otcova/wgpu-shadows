use std::cmp::Reverse;
use std::collections::BinaryHeap;

use crate::WgpuContext;

pub trait Disable {
    fn disable(&mut self);
}

pub struct SparseVec<T: Disable> {
    data: Vec<T>,
    empty_slots: BinaryHeap<Reverse<usize>>,
}

impl<T: Disable> SparseVec<T> {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            empty_slots: BinaryHeap::new(),
        }
    }

    /// Returns the index of the inserted item
    pub fn push(&mut self, item: T) -> usize {
        if let Some(slot) = self.empty_slots.pop() {
            self.data[slot.0] = item;
            slot.0
        } else {
            self.data.push(item);
            self.data.len() - 1
        }
    }

    pub fn remove(&mut self, index: usize) {
        self.data[index].disable();
        self.empty_slots.push(Reverse(index));
    }
}

pub struct SparseBuffer<T: Disable + bytemuck::NoUninit> {
    sparse: SparseVec<T>,
    buffer: wgpu::Buffer,
    update_range: Option<Range<usize>>,
}

impl<T: Disable> SparseBuffer<T> {
    pub fn new(ctx: &WgpuContext, usage: wgpu::BufferUsages) -> Self {
        Self {
            sparse: SparseVec::new(),
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

    pub fn get_mut(&mut self, index: usize) -> &mut T {
        if let Some(range) = &mut self.update_range {
            range.start = range.start.min(index);
            range.end = range.end.max(index + 1);
        } else {
            self.update_range = Some(index..index + 1);
        }

        &mut self.sparse.data[index]
    }

    pub fn push(&mut self, item: T) -> usize {
        self.sparse.push(item)
    }

    pub fn remove(&mut self, index: usize) {
        self.remove(index);
    }
}
