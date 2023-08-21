mod ui;

use self::ui::LobbyUI;

use super::*;
use crate::input::*;
use crate::math::*;
use crate::objects::*;
use crate::texture_atlas::*;

pub struct Lobby {
    game_layers: GameLayers,
    frame_layers: FrameLayers,
    ui: LobbyUI,

    block: BlockSq2,
    ligth: usize,
}

impl Lobby {
    pub fn new(ctx: &WgpuContext) -> Self {
        let mut game_layers = GameLayers::new(ctx);
        let mut frame_layers = FrameLayers::new(ctx);

        game_layers.background.buffer.push(QuadInstance::new_tex(
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

        let ui = LobbyUI::new(&mut frame_layers.ui);

        Self {
            game_layers,
            frame_layers,
            ui,

            block,
            ligth,
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

impl InputEventHandler<()> for Lobby {
    fn update(&mut self, input: &InputRef, _: &mut ()) {
        input.propagate_transformed_events(
            &self.frame_layers.camera,
            &mut self.ui,
            &mut self.frame_layers.ui,
        );
    }

    fn mouse_moved(&mut self, mouse: &Mouse, _: &mut ()) {
        let game_mouse = mouse.transform(&self.game_layers.camera);
        let pos = game_mouse.pos;

        self.block.set_pos(&mut self.game_layers, pos);

        let ligth = self.game_layers.ligths.get_ligth_mut(self.ligth);
        ligth.data.pos = -pos;
        ligth.needs_update = true;
    }
}
