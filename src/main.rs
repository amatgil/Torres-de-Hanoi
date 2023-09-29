use crate::structs::{World, PilaSelect};

pub mod structs;
pub mod utils;

const WINDOW_WIDTH: usize = 700;
const WINDOW_HEIGHT: usize = 600;

pub const DESPLAÇAMENT_BARRES: usize = WINDOW_HEIGHT / 4;
pub const BARRES_WIDTH: usize = 15;

pub const BLOCKS_HEIGHT: usize = 6;
pub const BLOCKS_WIDTH: usize = 20;

pub const COLORS_SEQ: [(u8, u8, u8); 5] = [
    (255,   0,   0),
    (  0, 255,   0),
    (  0,    0, 255),
    (  0, 255, 255),
    (255,   0, 255),
];

fn main() {
    let mut world = World::new(9);

    println!("{}", world);

    world.resoldre(PilaSelect::Pila1, PilaSelect::Pila3);

    println!("Estat final: {}", world);
}

