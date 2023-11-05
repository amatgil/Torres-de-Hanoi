use crate::structs::{World, StackSelect};

pub mod structs;
pub mod utils;

pub const BARS_WIDTH: usize = 30;
pub const BARS_HEIGHT: usize = 666;

pub const BLOCKS_HEIGHT: usize = 10;
pub const BLOCKS_WIDTH: usize = 30;

pub const BACKGROUND_COL: (u8, u8, u8) = (73, 77, 100);
pub const BARS_COL: (u8, u8, u8) = (184, 192, 224);

pub const FRAMERATE: usize = 60; // Defined in the justfile, given to ffmpeg

pub const COLORS_SEQ: [(u8, u8, u8); 7] = [
    (245, 189, 230),
    (198, 160, 246),
    (237, 137, 150),
    (245, 169, 127),
    (166, 218, 149),
    (139, 213, 202),
    (125, 196, 228),
];

fn main() {
    let n = std::env::args()
        .nth(1).expect("ERROR: Forgot to provide 'n'")
        .parse().expect("Remember that 'n' must be a positive natural **NUMBER** (n >= 1)");

    let frames = 2usize.pow(n as u32) - 1;
    println!("Generating with {n} blocks, meaning ~{} frames == ~{} seconds.", frames, frames / FRAMERATE );
    let mut world = World::new(n);

    println!("{}", world);

    world.resoldre(StackSelect::Stack1, StackSelect::Stack3);

    println!("Estat final: {}", world);
}

