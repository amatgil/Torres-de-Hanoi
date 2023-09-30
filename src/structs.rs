use std::{error::Error, fs::File, io::Write, fmt::Display};

use crate::{utils::{rgb_to_str, draw_box}, BARRES_WIDTH, COLORS_SEQ, BLOCKS_WIDTH, BLOCKS_HEIGHT, BACKGROUND_COL, BARRA_COL, BARRES_HEIGHT};

#[derive(Debug)]
pub struct World {
    width: usize,
    height: usize,
    n: usize,
    pila1: Pila,
    pila2: Pila,
    pila3: Pila,
}

pub struct Generacio<'a> (&'a mut usize);

impl Generacio<'_> {
    pub fn inc(&mut self) {
        *self.0 += 1;
    }
}

impl Display for Generacio<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:0>6}", self.0)
    }
}

impl World {
    pub fn new(n: usize) -> Self {
        let mut color_idx = 0;

        let blocks = {
            let mut v = Vec::new();
            for i in (1..=n).rev() {
                let color = COLORS_SEQ[color_idx];
                color_idx = (color_idx + 1) %  COLORS_SEQ.len();
                v.push(Block{val: i, color});
            }
            v
        };
        let s = World {
            n,
            width: n * BLOCKS_WIDTH * 5, 
            height: n * BLOCKS_HEIGHT * 4,
            pila1: Pila::new(blocks),
            pila2: Pila::default(),
            pila3: Pila::default(),
        };
        s
    }

    pub fn save_to_file(&self, gen: &mut Generacio) -> Result<(), Box<dyn Error>> {
        let file_name = format!("output/frame_{}.ppm", gen);
        gen.inc();

        print!("\rGuardant: '{}'", file_name);
        std::io::stdout().flush()?;


        let mut file = File::create(file_name)?;
        let mut buffer: Vec<String> = Vec::new(); 

        for _ in 0..self.width * self.height {
            buffer.push(rgb_to_str(BACKGROUND_COL.0, BACKGROUND_COL.1, BACKGROUND_COL.2).to_string());
        }

        let eighth = self.width / 8;
        let horizontal_positions = [
            1 * eighth,
            4 * eighth,
            7 * eighth];

        /*
        for x in horizontal_positions { // Les barres
            for dx in -(BARRES_WIDTH as isize) / 2..BARRES_WIDTH as isize / 2 {
                draw_box(&mut buffer,
                         ((x as isize - dx) as usize, 3 * self.height / 4),
                         ((x as isize + dx) as usize,     self.height / 3),
                         BARRA_COL,
                         self.width
                );
            }
        }
        */

        let parelles = [
            (&self.pila1.0, horizontal_positions[2]),
            (&self.pila2.0, horizontal_positions[1]),
            (&self.pila3.0, horizontal_positions[0]),
        ];
        let y_base = self.height / 3;

        for (pila, x_base) in parelles {
            for (i, bloc) in pila.iter().enumerate() {
                let width = bloc.val * BLOCKS_WIDTH;
                let delta_altura = i * BLOCKS_HEIGHT;

                let top_left = (x_base - width / 2, y_base + delta_altura + BLOCKS_HEIGHT / 2 );
                let bottom_right = (x_base + width / 2, y_base + delta_altura - BLOCKS_HEIGHT / 2 );

                draw_box(&mut buffer, top_left, bottom_right, bloc.color, self.width);
            }
            
        }

        // ppm spec al reves pq el girarè
        buffer.push("255\n".to_string());                                          // Max color val
        buffer.push(format!("{} {}\n", self.width, self.height));                  // Dimensions
        buffer.push("P3\n".to_string());                                           // Color
                                                                                  
        let bytes: Vec<u8> = buffer.iter() // Vec<String>
            .rev()
            .map(|s| s.as_bytes()) // Vec<[u8]>
            .flatten() // Vec<&u8>
            .map(|n| *n)
            .collect(); // Vec<&u8>

        file.write(&bytes)?;
        Ok(())
    }

    pub fn resoldre(&mut self, origin: PilaSelect, destinacio: PilaSelect) {
        let mut g = 0;
        let mut gen = Generacio(&mut g);
        for _ in 0..80 { self.save_to_file(&mut gen).ok(); }
        self.moure_pila(self.n , origin, destinacio, &mut gen);
        for _ in 0..120 { self.save_to_file(&mut gen).ok(); }
        println!("Fet!");
    }

    pub fn moure_pila(&mut self, n: usize, origin: PilaSelect, destinacio: PilaSelect, t: &mut Generacio ) -> Option<()> {
        use PilaSelect as Sel;

        if n == 1 {
            self.moure_block(origin, destinacio, t).unwrap();
        } else {
            let temp_stack = match (origin, destinacio) {
                (Sel::Pila1, Sel::Pila2) => Sel::Pila3,
                (Sel::Pila1, Sel::Pila3) => Sel::Pila2,
                (Sel::Pila2, Sel::Pila1) => Sel::Pila3,
                (Sel::Pila2, Sel::Pila3) => Sel::Pila1,
                (Sel::Pila3, Sel::Pila1) => Sel::Pila2,
                (Sel::Pila3, Sel::Pila2) => Sel::Pila1,
                _ => unreachable!(),
            };

            self.moure_pila(n - 1, origin, temp_stack, t)?;
            self.moure_block(origin, destinacio, t)?;
            self.moure_pila(n - 1, temp_stack, destinacio, t)?;
        }

        Some(())
    }
    pub fn moure_block(&mut self, origin: PilaSelect, destinacio: PilaSelect, gen: &mut Generacio) -> Option<()>{
        if origin == destinacio { 
            eprintln!("S'ha intentat moure a la mateixa pila ({origin}-{destinacio})");
            return None; 
        }

        self.save_to_file(gen).ok()?;

        let block_origin: Option<&Block> = match origin {
            PilaSelect::Pila1 => self.pila1.0.last(),
            PilaSelect::Pila2 => self.pila2.0.last(),
            PilaSelect::Pila3 => self.pila3.0.last(),
        };

        let block_destin: Option<&Block> = match destinacio {
            PilaSelect::Pila1 => self.pila1.0.last(),
            PilaSelect::Pila2 => self.pila2.0.last(),
            PilaSelect::Pila3 => self.pila3.0.last(),
        };

        if let (Some(orig_block), Some(destin_block)) = (block_origin, block_destin) {
                if orig_block.val > destin_block.val {
                    eprintln!("S'ha intentat moure trencant la Una Norma ({orig_block}>{destin_block})");
                    return None; 
                }
        } 

        //println!("{}", &self);
        let elem = match origin {
            PilaSelect::Pila1 => self.pila1.0.pop(),
            PilaSelect::Pila2 => self.pila2.0.pop(),
            PilaSelect::Pila3 => self.pila3.0.pop(),
        }.expect(&format!("Error agafant element de la pila: {origin} - {destinacio}"));

        match destinacio {
            PilaSelect::Pila1 => self.pila1.0.push(elem),
            PilaSelect::Pila2 => self.pila2.0.push(elem),
            PilaSelect::Pila3 => self.pila3.0.push(elem),
        }
        Some(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PilaSelect {
    Pila1,
    Pila2,
    Pila3
}

#[derive(Debug, Clone)]
pub struct Pila (Vec<Block>); // End is top

impl Default for Pila {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}
impl Pila {
    pub fn new(v: Vec<Block>) -> Self {
        Self(v)
    }
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Clone)]
pub struct Block {
    val: usize,
    color: (u8, u8, u8),
}


impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.val)
    }
}
impl Display for PilaSelect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let o = match self {
            PilaSelect::Pila1 => "Pila A",
            PilaSelect::Pila2 => "Pila B",
            PilaSelect::Pila3 => "Pila C",
        };
        write!(f, "{}", o)
    }
}
impl Display for Pila {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = {
            let mut s = String::new();
            for t in self.0.iter() {
                s.push_str(&format!("{}, ", t));
            }
            s
        };

        write!(f, "{}", out)
    }
}
impl Display for World {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = {
            let mut s = String::new();
            s.push_str(&format!("[n: {}], ", self.n));
            s.push_str(&format!("[A: {}]", self.pila1));
            s.push_str(&format!("[B: {}]", self.pila2));
            s.push_str(&format!("[C: {}]", self.pila3));

            s
        };

        write!(f, "{}", out)
    }
}
