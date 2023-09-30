use crate::structs::{World, PilaSelect};

pub mod structs;
pub mod utils;

//const WINDOW_WIDTH: usize = 1700;
//const WINDOW_HEIGHT: usize = 700;

//pub const DESPLAÇAMENT_BARRES: usize = WINDOW_HEIGHT / 6;
pub const BARRES_WIDTH: usize = 30;
pub const BARRES_HEIGHT: usize = 666;

pub const BLOCKS_HEIGHT: usize = 10;
pub const BLOCKS_WIDTH: usize = 30;

pub const BACKGROUND_COL: (u8, u8, u8) = (73, 77, 100);
pub const BARRA_COL: (u8, u8, u8) = (184, 192, 224);

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
    //let args: Vec<String> = env::args().collect();
    let n = std::env::args()
        .nth(1).expect("Te n'has oblidat de donar 'n'")
        .parse().expect("Recorda que 'n' ha de ser un enter positiu n >= 1");

    let frames = 2usize.pow(n as u32) - 1;
    println!("Generant amb {n} blocks. Hi haurà {} frames = {} segons.", frames, frames / 20 );
    let mut world = World::new(n);

    println!("{}", world);

    world.resoldre(PilaSelect::Pila1, PilaSelect::Pila3);

    println!("Estat final: {}", world);
}

