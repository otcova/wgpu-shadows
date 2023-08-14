mod block;

pub use block::*;

use crate::math::*;
use crate::shaders::*;

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
