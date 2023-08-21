mod keyboard;
mod mouse;

pub use keyboard::*;
pub use mouse::*;

use crate::math::*;
use winit::event::WindowEvent;

pub trait InputEventHandler<A> {
    #[allow(unused_variables)]
    fn update(&mut self, input: &InputRef, args: &mut A) {}

    #[allow(unused_variables)]
    fn mouse_moved(&mut self, mouse: &Mouse, args: &mut A) {}

    #[allow(unused_variables)]
    fn typed_text(&mut self, text: &str, args: &mut A) {}
}

pub struct Input {
    mouse: Mouse,
    keyboard: Keyboard,
}

pub struct InputRef<'a> {
    mouse: Mouse,
    keyboard: &'a Keyboard,
}

impl Input {
    pub fn new() -> Self {
        Self {
            mouse: Mouse::new(),
            keyboard: Keyboard::new(),
        }
    }

    // pub fn propagate_transformed_events<A, H, T>(&self, trans: &T, handler: &mut H, args: &mut A)
    // where
    //     H: InputEventHandler<A>,
    //     T: MouseTransform,
    // {
    //     let input = InputRef {
    //         mouse: self.mouse.transform(trans),
    //         keyboard: &self.keyboard,
    //     };

    //     handler.update(&input, args);
    //     input.mouse.propagate_events(handler, args);
    //     input.keyboard.propagate_events(handler, args);
    // }

    pub fn propagate_events<A, H: InputEventHandler<A>>(&self, handler: &mut H, args: &mut A) {
        let input = InputRef {
            mouse: self.mouse,
            keyboard: &self.keyboard,
        };
        input.propagate_events(handler, args);
        // self.propagate_transformed_events(&(), handler, args);
    }

    /// Returns true if event is used
    pub fn handle_event(&mut self, event: &WindowEvent) -> bool {
        self.mouse.handle_event(event) || self.keyboard.handle_event(event)
    }

    pub fn update(&mut self) {
        self.mouse.update();
        self.keyboard.update();
    }

    pub fn resize(&mut self, size: Vec2) {
        self.mouse.resize(size);
    }
}

impl<'a> InputRef<'a> {
    pub fn propagate_transformed_events<A, H, T>(&self, trans: &T, handler: &mut H, args: &mut A)
    where
        H: InputEventHandler<A>,
        T: MouseTransform,
    {
        let input = InputRef {
            mouse: self.mouse.transform(trans),
            keyboard: self.keyboard,
        };

        handler.update(&input, args);
        input.mouse.propagate_events(handler, args);
        input.keyboard.propagate_events(handler, args);
    }

    pub fn propagate_events<A, H: InputEventHandler<A>>(&self, handler: &mut H, args: &mut A) {
        self.propagate_transformed_events(&(), handler, args);
    }
}
