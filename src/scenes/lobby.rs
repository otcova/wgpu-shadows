use super::*;
use crate::font::*;
use crate::math::*;
use crate::mouse::*;
use crate::objects::*;
use crate::texture_atlas::*;

pub struct Scene {
    game_layers: GameLayers,
    frame_layers: FrameLayers,

    block: BlockSq2,
    ligth: usize,
    button: TextButton,
}

impl Scene {
    pub fn new(ctx: &WgpuContext) -> Self {
        let mut game_layers = GameLayers::new(ctx);
        let mut frame_layers = FrameLayers::new(ctx);

        game_layers.background.buffer.push(QuadInstance::new(
            Vec2::zero(),
            4.,
            TextureAtlas::view_triangles(),
        ));

        let ligth =
            game_layers
                .ligths
                .add_ligth(ctx, Vec2::zero(), LigthUniform::color(130, 130, 130));

        BlockSq3::new(&mut game_layers, Vec2::new(0., 0.4));
        let block = BlockSq2::new(&mut game_layers, Vec2::zero());

        let font = Font::parse(
            include_str!("../../fonts/Tektur-Regular.fnt"),
            TextureAtlas::view_tektur_regular(),
        );

        let button = TextButton::new(TextButtonDescriptor {
            layer: &mut frame_layers.ui,
            font: &font,
            text: "Hey!",
            pos: Vec2::new(-0.5, -0.2),
            size: 0.1,
        });

        Self {
            game_layers,
            frame_layers,

            block,
            ligth,
            button,
        }
    }

    pub fn resize(&mut self, size: Vec2) {
        self.game_layers.camera.resize(size);
        self.frame_layers.camera.resize(size);
    }

    pub fn draw<'a>(&'a mut self, pass: &mut LigthRenderPass<'a>, shaders: &'a Shaders) {
        self.game_layers.draw_game(pass, shaders);
        self.frame_layers.draw_game(pass, shaders);
    }
}

impl MouseEventHandler for Scene {
    fn moved(&mut self, mouse: &Mouse) {
        let game_mouse = mouse.transform(&self.game_layers.camera);
        let pos = game_mouse.pos;

        self.button.mouse_moved(pos, &mut self.frame_layers.ui);

        self.block.set_pos(&mut self.game_layers, pos);

        let ligth = self.game_layers.ligths.get_ligth_mut(self.ligth);
        ligth.data.pos = -pos;
        ligth.needs_update = true;
    }
}
