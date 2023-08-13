mod ligth;
mod quad;
mod shader;

pub use ligth::*;
pub use quad::*;

use crate::{error::ErrResult, ligth_pipeline::LigthTextures, WgpuContext};
use shader::*;

pub struct Shaders {
    pub quad: QuadShader,
    pub ligth: LigthShader,
}
impl Shaders {
    pub fn new(ctx: &WgpuContext, textures: &LigthTextures) -> ErrResult<Self> {
        Ok(Shaders {
            ligth: LigthShader::new(ctx, textures),
            quad: QuadShader::new(ctx, textures)?,
        })
    }
    pub fn resize(&mut self, ctx: &WgpuContext, textures: &LigthTextures) {
        self.ligth.resize(ctx, textures);
        self.quad.resize(ctx, textures);
    }
}
