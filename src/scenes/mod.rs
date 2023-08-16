mod lobby;

pub use lobby::*;

use crate::layers::*;

pub struct SceneLayers {
    background: QuadLayer,
    bottom_particles: QuadLayer,
    players: QuadLayer,
    blocks: QuadLayer,
    top_particles: QuadLayer,
    frame: QuadLayer,
    ui: QuadLayer,
    ligths: LigthLayer,
}
