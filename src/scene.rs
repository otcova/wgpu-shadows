use crate::layers::{LigthLayer, QuadLayer};
use crate::ligth_pipeline::LigthRenderPass;
use crate::math::Vec2;
use crate::mouse::{Mouse, MouseEventHandler};
use crate::texture_atlas::TextureAtlas;
use crate::{shaders::*, shapes};
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

pub struct Scene {
    layers: SceneLayers,
    game_camera: Camera,
    frame_camera: Camera,
}

impl Scene {
    pub fn new(ctx: &WgpuContext) -> Self {
        let mut scene = Self {
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
        };

        scene.layers.background.buffer.push(QuadInstance::new(
            [0., 0.],
            3.,
            TextureAtlas::view_triangles(),
        ));

        scene.layers.ligths.add_ligth(
            ctx,
            &LigthUniform {
                pos: [0., 0., 0.2],
                ligth_color: 0x3FFFFFFF,
            },
        );

        scene
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
        self.layers.boxes.draw(pass);
        self.layers.top_particles.draw(pass);

        self.frame_camera.bind(pass);
        self.layers.frame.draw(pass);
    }
}

impl MouseEventHandler for Scene {
    fn moved(&mut self, mouse: &Mouse) {
        let game_mouse = mouse.transform(&self.game_camera);
        let pos = game_mouse.pos;

        self.layers.boxes.buffer.clear();
        self.layers.boxes.buffer.push(QuadInstance::new(
            [pos.x, pos.y],
            0.3,
            TextureAtlas::view_block_sq3(),
        ));

        self.layers.ligths.clear_shadows();

        for wind in shapes::BLOCK_SQ3.windows(2) {
            let a = [pos.x + wind[0][0] * 0.3, pos.y + wind[0][1] * 0.3];
            let b = [pos.x + wind[1][0] * 0.3, pos.y + wind[1][1] * 0.3];
            self.layers.ligths.add_shadow(ShadowInstance { a, b });
        }
    }
}
