mod atlas;
mod shapes;

fn main() {
    atlas::main();
    shapes::main();
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
