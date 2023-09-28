use crate::structs::{World, PilaSelect};

pub mod structs;
pub mod utils;

const WINDOW_WIDTH: usize = 700;
const WINDOW_HEIGHT: usize = 600;

fn main() {
    let mut world = World::new(4);

    dbg!(&world);
    world.moure_block(PilaSelect::Pila1, PilaSelect::Pila2);
    dbg!(&world);
}

