mod shader;
mod texture;
mod uniform;
mod vec_buffer;

pub use shader::*;
pub use texture::*;
pub use uniform::*;
pub use vec_buffer::*;

pub struct WgpuContext {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}
