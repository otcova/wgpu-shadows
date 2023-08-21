use super::*;

pub struct LobbyUI {
    buttons: LobbyButtons,
    username_input: TextInput,
}

pub enum LobbyButtons {
    Main(MainUI),
    Loading(LoadingUI),
}

struct MainUI {
    join_lobby: TextButton,
}

struct LoadingUI {
    spining: QuadInstance,
}

impl LobbyUI {
    pub fn new(layer: &mut QuadLayer) -> Self {
        let buttons = LobbyButtons::Main(MainUI::new(layer));
        let username_input = TextInput::new(TextInputDescriptor {
            layer,
            placeholder: "NAME",
            pos: Vec2::new(0., 0.2),
        });

        Self {
            buttons,
            username_input,
        }
    }
}

impl InputEventHandler<QuadLayer> for LobbyUI {
    fn update(&mut self, input: &InputRef, layer: &mut QuadLayer) {
        input.propagate_events(&mut self.username_input, layer);
        match &mut self.buttons {
            LobbyButtons::Main(buttons) => input.propagate_events(buttons, layer),
            LobbyButtons::Loading(_) => {}
        }
    }
}

impl MainUI {
    fn new(layer: &mut QuadLayer) -> Self {
        let join_lobby = TextButton::new(TextButtonDescriptor {
            layer,
            text: "Join Lobby",
            pos: Vec2::zero(),
        });
        Self { join_lobby }
    }
}

impl InputEventHandler<QuadLayer> for MainUI {
    fn update(&mut self, input: &InputRef, layer: &mut QuadLayer) {
        input.propagate_events(&mut self.join_lobby, layer);
    }
}
