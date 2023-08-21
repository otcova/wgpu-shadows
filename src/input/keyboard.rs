use super::*;

pub struct Keyboard {
    typed_text: String,
}

impl Keyboard {
    pub fn new() -> Self {
        Self {
            typed_text: String::new(),
        }
    }

    pub fn propagate_events<A, H>(&self, handler: &mut H, args: &mut A)
    where
        H: InputEventHandler<A>,
    {
        if !self.typed_text.is_empty() {
            handler.typed_text(&self.typed_text, args);
        }
    }

    pub fn update(&mut self) {
        self.typed_text.clear();
    }

    /// Returns true if event is used
    pub fn handle_event(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::ReceivedCharacter(c) => {
                if !c.is_ascii_control() {
                    self.typed_text.push(*c);
                }
                true
            }
            _ => false,
        }
    }
}
