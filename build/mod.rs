mod atlas;
mod font_parser;
mod fonts;
mod math;
mod shapes;

use math::*;

fn main() {
    let atlas = atlas::main();
    shapes::main();
    fonts::main(&atlas);
}

#[allow(unused)]
macro_rules! log {
    ($($t:expr),* $(,)?) => {
        print!("cargo:warning=");
        println!($($t),*);
    };
}

#[allow(unused)]
pub(crate) use log;
