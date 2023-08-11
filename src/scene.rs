use crate::layers::{LigthLayer, QuadLayer};
use crate::ligth_pipeline::LigthRenderPass;
use crate::{Camera, WgpuContext};

struct SceneLayers {
    background: QuadLayer,
    bottom_particles: QuadLayer,
    players: QuadLayer,
    boxes: QuadLayer,
    top_particles: QuadLayer,
    frame: QuadLayer,
    ligths: LigthLayer,
}

struct Scene {
    layers: SceneLayers,
    game_camera: Camera,
    frame_camera: Camera,
}

impl Scene {
    pub fn new(ctx: &WgpuContext) -> Self {
        Self {
            layers: SceneLayers {
                background: QuadLayer::new(ctx),
                bottom_particles: QuadLayer::new(ctx),
                players: QuadLayer::new(ctx),
                boxes: QuadLayer::new(ctx),
                top_particles: QuadLayer::new(ctx),
                frame: QuadLayer::new(ctx),
                ligths: LigthLayer::new(ctx),
            },
            game_camera: Camera::new(ctx),
            frame_camera: Camera::new(ctx),
        }
    }

    pub fn draw<'a>(&'a mut self, pass: &mut LigthRenderPass<'a>) {
        self.frame_camera.bind(pass);
        self.layers.background.draw(pass);

        self.game_camera.bind(pass);
        self.layers.bottom_particles.draw(pass);
        self.layers.players.draw(pass);
        self.layers.boxes.draw(pass);
        self.layers.top_particles.draw(pass);

        self.layers.ligths.draw(pass);

        self.frame_camera.bind(pass);
        self.layers.frame.draw(pass);
    }
}
