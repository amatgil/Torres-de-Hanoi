use std::{error::Error, fs::File, io::{BufWriter, Write}};

use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};


#[derive(Debug)]
pub struct World {
    pila1: Pila,
    pila2: Pila,
    pila3: Pila,
}

impl World {
    pub fn new(n: usize) -> Self {
        let blocks = {
            let mut v = Vec::new();
            for i in (1..n).rev() {
                v.push(Block(i));
            }
            v
        };
        let s = World {
            pila1: Pila::new(blocks),
            pila2: Pila::default(),
            pila3: Pila::default(),
        };

        s
    }
    pub fn save_to_file(file_name: &str) -> Result<(), Box<dyn Error>> {
        let file = File::create(file_name)?;
        let mut buffer = BufWriter::new(file);

        writeln!(buffer, "P3").unwrap();                                 // Color
        writeln!(buffer, "{} {}", WINDOW_WIDTH, WINDOW_HEIGHT).unwrap(); // Dimensions
        writeln!(buffer, "255").unwrap();                                // Max color val

        for _ in 0..WINDOW_HEIGHT * WINDOW_WIDTH {
            writeln!(buffer, "255").unwrap();                                // Max color val
        }



        Ok(())
    }
    pub fn moure_block(&mut self, origin: PilaSelect, destinacio: PilaSelect) -> Option<()>{
        if origin == destinacio { return None; }
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
            if orig_block.0 > destin_block.0 { return None; }

            let elem = match origin {
                PilaSelect::Pila1 => self.pila1.0.pop(),
                PilaSelect::Pila2 => self.pila2.0.pop(),
                PilaSelect::Pila3 => self.pila3.0.pop(),
            }.unwrap();

            println!("Movent: {:?}", elem);

            match destinacio {
                PilaSelect::Pila1 => self.pila1.0.push(elem),
                PilaSelect::Pila2 => self.pila2.0.push(elem),
                PilaSelect::Pila3 => self.pila3.0.push(elem),
            }

        } else if let Some(_orig_block) = block_origin {
            let elem = match origin {
                PilaSelect::Pila1 => self.pila1.0.pop(),
                PilaSelect::Pila2 => self.pila2.0.pop(),
                PilaSelect::Pila3 => self.pila3.0.pop(),
            }.unwrap();

            println!("Movent: {:?}", elem);

            match destinacio {
                PilaSelect::Pila1 => self.pila1.0.push(elem),
                PilaSelect::Pila2 => self.pila2.0.push(elem),
                PilaSelect::Pila3 => self.pila3.0.push(elem),
            }
        } else {
            return None;
        }

        Some(())
    }
}

#[derive(Debug, PartialEq, Eq)]
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
pub struct Block(usize);

