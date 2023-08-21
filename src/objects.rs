mod block;
mod text_button;
mod text_input;

pub use block::*;
pub use text_button::*;
pub use text_input::*;

use crate::math::*;
use crate::shaders::*;

const UI_SIZE: f32 = 0.08;

fn shadow_from_shape<'a>(shape: &'a [Vec2]) -> impl Iterator<Item = ShadowInstance> + 'a {
    shape
        .windows(2)
        .map(|vertices| ShadowInstance {
            a: vertices[0],
            b: vertices[1],
        })
        .chain(std::iter::once(ShadowInstance {
            a: *shape.last().unwrap(),
            b: shape[0],
        }))
}
