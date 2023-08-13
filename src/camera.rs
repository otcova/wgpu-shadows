use crate::ligth_pipeline::LigthRenderPass;
use crate::mouse::MouseTransform;
use crate::uniform::CachedUniform;
use crate::{Vec2, WgpuContext};

pub struct Camera {
    uniform: CachedUniform<CameraUniform>,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct CameraUniform {
    pub pos: Vec2,
    pub size: Vec2,
}

impl Camera {
    pub fn new(ctx: &WgpuContext) -> Self {
        Self {
            uniform: CachedUniform::new(
                ctx,
                wgpu::ShaderStages::VERTEX,
                CameraUniform {
                    pos: Vec2::new(0., 0.),
                    size: Vec2::new(1., 1.),
                },
            ),
        }
    }

    pub fn resize(&mut self, size: Vec2) {
        self.uniform.update(CameraUniform {
            pos: Vec2::new(0., 0.),
            size: Vec2::new(size.y / size.x, 1.),
        });
    }

    pub fn update_buffers(&mut self, ctx: &WgpuContext) {
        self.uniform.update_buffers(ctx);
    }

    pub fn bind<'a>(&'a self, pass: &mut LigthRenderPass<'a>) {
        self.uniform.bind(1, &mut pass.normal);
        self.uniform.bind(1, &mut pass.ligth);
        self.uniform.bind(1, &mut pass.diffuse);
    }
}

impl MouseTransform for Camera {
    fn transform(&self, pos: Vec2) -> Vec2 {
        pos / self.uniform.data.size + self.uniform.data.pos
    }
}
