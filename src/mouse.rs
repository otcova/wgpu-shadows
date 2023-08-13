use crate::Vec2;
use winit::event::WindowEvent;

pub struct Mouse {
    pub pos: Vec2,
    pub past_pos: Vec2,
    screen_size: Vec2,
}

pub trait MouseEventHandler {
    fn moved(&mut self, _mouse: &Mouse) {}
    // fn down(&mut self, mouse: &Mouse);
    // fn up(&mut self, mouse: &Mouse);
}

pub trait MouseTransform {
    fn transform(&self, pos: Vec2) -> Vec2;
}

impl Mouse {
    pub fn new() -> Self {
        Self {
            pos: Vec2::new(0., 0.),
            past_pos: Vec2::new(0., 0.),
            screen_size: Vec2::new(1., 1.),
        }
    }

    pub fn transform<T: MouseTransform>(&self, trans: &T) -> Self {
        Mouse {
            pos: trans.transform(self.pos),
            past_pos: trans.transform(self.past_pos),
            screen_size: self.screen_size,
        }
    }

    pub fn propagate_events<H: MouseEventHandler>(&self, handler: &mut H) {
        if self.pos != self.past_pos {
            handler.moved(self);
        }
    }

    /// Updates past values (like past_pos)
    pub fn update(&mut self) {
        self.past_pos = self.pos;
    }

    pub fn resize(&mut self, mut size: Vec2) {
        size.x = size.x * 0.5;
        size.y = size.y * 0.5;

        self.pos = size * self.pos / self.screen_size;
        self.past_pos = size * self.past_pos / self.screen_size;
        self.screen_size = size;
    }

    /// Returns true if event is used
    pub fn handle_event(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::CursorMoved { position, .. } => {
                self.pos = Vec2::new(
                    position.x as f32 - self.screen_size.x,
                    self.screen_size.y - position.y as f32,
                ) / self.screen_size;
                true
            }
            _ => false,
        }
    }
}
