use std::{error::Error, fs::File, io::{BufWriter, Write}, fmt::Display};

use crate::{WINDOW_HEIGHT, WINDOW_WIDTH, utils::rgb_to_str};


#[derive(Debug)]
pub struct World {
    n: usize,
    pila1: Pila,
    pila2: Pila,
    pila3: Pila,
}

impl World {
    pub fn new(n: usize) -> Self {
        let blocks = {
            let mut v = Vec::new();
            for i in (1..=n).rev() {
                v.push(Block(i));
            }
            v
        };
        let s = World {
            n,
            pila1: Pila::new(blocks),
            pila2: Pila::default(),
            pila3: Pila::default(),
        };

        s
    }
    pub fn save_to_file(&self, file_name: &str) -> Result<(), Box<dyn Error>> {
        let file = File::create(file_name)?;
        let mut buffer = BufWriter::new(file);

        writeln!(buffer, "P3").unwrap();                                 // Color
        writeln!(buffer, "{} {}", WINDOW_WIDTH, WINDOW_HEIGHT).unwrap(); // Dimensions
        writeln!(buffer, "255").unwrap();                                // Max color val

        for _ in 0..WINDOW_HEIGHT * WINDOW_WIDTH {
            writeln!(buffer, "{}", rgb_to_str(128, 128, 128)).unwrap();
        }


        buffer.flush()?;
        Ok(())
    }
    pub fn resoldre(&mut self, origin: PilaSelect, destinacio: PilaSelect) {
        // Moure pila de n - 1 a la segona pila
        // Mou la base
        // Torna lo altre a la fila 1
        // Repeat recursiu
        self.moure_pila(self.n , origin, destinacio);
    }

    pub fn moure_pila(&mut self, n: usize, origin: PilaSelect, destinacio: PilaSelect) -> Option<()> {
        use PilaSelect as Sel;

        //println!("Estic movent block, n: {n}");
        if n == 1 {
            self.moure_block(origin, destinacio).unwrap();
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

            self.moure_pila(n - 1, origin, temp_stack)?;

            self.moure_block(origin, destinacio)?;

            self.moure_pila(n - 1, temp_stack, destinacio)?;

        }

        Some(())
    }
    pub fn moure_block(&mut self, origin: PilaSelect, destinacio: PilaSelect) -> Option<()>{
        if origin == destinacio { 
            println!("S'ha intentat moure a la mateixa pila ({origin}-{destinacio})");
            return None; 
        }
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
                if orig_block.0 > destin_block.0 {
                    println!("S'ha intentat moure trencant la Una Norma ({orig_block}>{destin_block})");
                    return None; 
                }
        } 

        println!("{}", &self);
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
pub struct Block(usize);


impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
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
