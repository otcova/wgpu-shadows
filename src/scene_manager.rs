use winit::event::WindowEvent;

use crate::assets::Assets;
use crate::ligth_pipeline::LigthPipeline;
use crate::math::Vec2;
use crate::mouse::Mouse;
use crate::scenes::*;
use crate::shaders::Shaders;
use crate::ErrResult;
use crate::WgpuContext;

pub struct SceneManager {
    scene: Lobby,

    assets: Assets,
    shaders: Shaders,
    pipeline: LigthPipeline,
    mouse: Mouse,
}

impl SceneManager {
    pub fn new(ctx: &WgpuContext, width: u32, height: u32) -> ErrResult<Self> {
        let pipeline = LigthPipeline::new(ctx, width, height);
        let shaders = Shaders::new(ctx, &pipeline.textures)?;
        let assets = Assets::load();
        let mouse = Mouse::new();

        Ok(Self {
            scene: Lobby::new(ctx, &assets),
            assets,
            shaders,
            pipeline,
            mouse,
        })
    }

    /// Returns true if event is used
    pub fn handle_event(&mut self, event: &WindowEvent) -> bool {
        self.mouse.handle_event(event)
    }

    pub fn draw(&mut self, ctx: &WgpuContext, target: &wgpu::TextureView) {
        self.mouse.propagate_events(&mut self.scene, &mut ());
        self.mouse.update();

        let mut ligth_frame = self.pipeline.start_frame(&ctx, target);
        let mut ligth_pass = ligth_frame.create_render_pass();

        self.scene.draw(&mut ligth_pass, &self.shaders);

        drop(ligth_pass);
        ligth_frame.resolve();
    }

    pub fn resize(&mut self, ctx: &WgpuContext, width: u32, height: u32) {
        self.pipeline.resize(ctx, width, height);
        self.shaders.resize(ctx, &self.pipeline.textures);

        let size = Vec2::new(width as f32, height as f32);
        self.scene.resize(size);
        self.mouse.resize(size);
    }
}
