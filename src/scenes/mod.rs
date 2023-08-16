mod lobby;

pub use lobby::*;

use crate::camera::*;
use crate::layers::*;
use crate::ligth_pipeline::*;
use crate::shaders::*;
use crate::wgpu_components::*;

pub struct GameLayers {
    pub camera: Camera,
    pub ligths: LigthLayer,
    pub background: QuadLayer,
    pub bottom_particles: QuadLayer,
    pub players: QuadLayer,
    pub blocks: QuadLayer,
    pub top_particles: QuadLayer,
}

pub struct FrameLayers {
    pub camera: Camera,
    pub frame: QuadLayer,
    pub ui: QuadLayer,
}

impl GameLayers {
    pub fn new(ctx: &WgpuContext) -> Self {
        GameLayers {
            camera: Camera::new(ctx),
            ligths: LigthLayer::new(ctx),
            background: QuadLayer::new(ctx),
            bottom_particles: QuadLayer::new(ctx),
            players: QuadLayer::new(ctx),
            blocks: QuadLayer::new(ctx),
            top_particles: QuadLayer::new(ctx),
        }
    }

    pub fn draw_game<'a>(&'a mut self, pass: &mut LigthRenderPass<'a>, shaders: &'a Shaders) {
        self.camera.bind(pass);

        shaders.ligth.bind(pass);
        shaders.quad.bind_ligth(pass);

        self.ligths.draw(pass);
        self.background.draw(pass);
        self.bottom_particles.draw(pass);
        self.players.draw(pass);

        shaders.quad.bind(pass);

        self.blocks.draw(pass);
        self.top_particles.draw(pass);
    }
}

impl FrameLayers {
    pub fn new(ctx: &WgpuContext) -> Self {
        FrameLayers {
            camera: Camera::new(ctx),
            frame: QuadLayer::new(ctx),
            ui: QuadLayer::new(ctx),
        }
    }

    pub fn draw_game<'a>(&'a mut self, pass: &mut LigthRenderPass<'a>, shaders: &'a Shaders) {
        self.camera.bind(pass);

        shaders.quad.bind(pass);
        self.frame.draw(pass);
        self.ui.draw(pass);
    }
}
