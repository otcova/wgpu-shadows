use crate::layers::{LigthLayer, QuadLayer};
use crate::ligth_pipeline::LigthRenderPass;
use crate::math::Vec2;
use crate::mouse::{Mouse, MouseEventHandler};
use crate::objects::BlockSq2;
use crate::texture_atlas::TextureAtlas;
use crate::{shaders::*, shapes};
use crate::{Camera, WgpuContext};

struct SceneLayers {
    background: QuadLayer,
    bottom_particles: QuadLayer,
    players: QuadLayer,
    blocks: QuadLayer,
    top_particles: QuadLayer,
    frame: QuadLayer,
    ligths: LigthLayer,
}

pub struct Scene {
    layers: SceneLayers,
    game_camera: Camera,
    frame_camera: Camera,

    block: BlockSq2,
    ligth: usize,
}

impl Scene {
    pub fn new(ctx: &WgpuContext) -> Self {
        let mut layers = SceneLayers {
            background: QuadLayer::new(ctx),
            bottom_particles: QuadLayer::new(ctx),
            players: QuadLayer::new(ctx),
            blocks: QuadLayer::new(ctx),
            top_particles: QuadLayer::new(ctx),
            frame: QuadLayer::new(ctx),
            ligths: LigthLayer::new(ctx),
        };
        let game_camera = Camera::new(ctx);
        let frame_camera = Camera::new(ctx);

        layers.background.buffer.push(QuadInstance::new(
            Vec2::zero(),
            4.,
            TextureAtlas::view_triangles(),
        ));

        let ligth = layers
            .ligths
            .add_ligth(ctx, LigthUniform::new(Vec2::zero(), 0.2, 10, 50, 130));

        let block = BlockSq2::new(&mut layers.blocks, &mut layers.ligths);

        Self {
            layers,
            game_camera,
            frame_camera,

            block,
            ligth,
        }
    }

    pub fn resize(&mut self, size: Vec2) {
        self.game_camera.resize(size);
        self.frame_camera.resize(size);
    }

    pub fn draw<'a>(&'a mut self, pass: &mut LigthRenderPass<'a>, shaders: &'a Shaders) {
        self.game_camera.update_buffers(pass.context);
        self.frame_camera.update_buffers(pass.context);

        shaders.ligth.bind(pass);

        shaders.quad.bind_ligth(pass);
        self.frame_camera.bind(pass);
        self.layers.background.draw(pass);

        self.game_camera.bind(pass);
        self.layers.ligths.draw(pass);
        self.layers.bottom_particles.draw(pass);
        self.layers.players.draw(pass);

        shaders.quad.bind(pass);
        self.layers.blocks.draw(pass);
        self.layers.top_particles.draw(pass);

        self.frame_camera.bind(pass);
        self.layers.frame.draw(pass);
    }
}

impl MouseEventHandler for Scene {
    fn moved(&mut self, mouse: &Mouse) {
        let game_mouse = mouse.transform(&self.game_camera);
        let pos = game_mouse.pos;

        self.block
            .set_pos(&mut self.layers.blocks, &mut self.layers.ligths, pos);

        let ligth = self.layers.ligths.get_ligth_mut(self.ligth);
        ligth.data.pos = -pos;
        ligth.needs_update = true;
    }
}
