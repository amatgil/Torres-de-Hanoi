use crate::structs::{World, PilaSelect};

pub mod structs;
pub mod utils;

const WINDOW_WIDTH: usize = 700;
const WINDOW_HEIGHT: usize = 600;

fn main() {
    let mut world = World::new(3);

    println!("{}", world);
    //world.moure_block(PilaSelect::Pila1, PilaSelect::Pila2);

    world.resoldre(PilaSelect::Pila1, PilaSelect::Pila2);

    println!("Estat final: {}", world);
}

