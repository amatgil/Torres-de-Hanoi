use std::{error::Error, fs::File, io::Write, fmt::Display};

use crate::{utils::{rgb_to_str, draw_box}, COLORS_SEQ, BLOCKS_WIDTH, BLOCKS_HEIGHT, BACKGROUND_COL, FROZEN_FRAMES_START, FROZEN_FRAMES_END, FROZEN_FRAMES_NUMBER};

#[derive(Debug)]
pub struct World {
    width: usize,
    height: usize,
    n: usize,
    stack1: Stack,
    stack2: Stack,
    stack3: Stack,
}

pub struct Generation<'a> (&'a mut usize);

impl Generation<'_> {
    pub fn inc(&mut self) { *self.0 += 1; }
}

impl Display for Generation<'_> {
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
            stack1: Stack::new(blocks),
            stack2: Stack::default(),
            stack3: Stack::default(),
        };
        s
    }

    pub fn save_to_file(&self, gen: &mut Generation) -> Result<(), Box<dyn Error>> {
        let file_name = format!("output/frame_{}.ppm", gen);
        gen.inc();

        print!("\rSaving '{}' (of {})", file_name, 2_i32.pow(self.n as u32) - 1 + FROZEN_FRAMES_NUMBER as i32);
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
            7 * eighth
        ];


        let location_pairs = [
            (&self.stack1.0, horizontal_positions[2]),
            (&self.stack2.0, horizontal_positions[1]),
            (&self.stack3.0, horizontal_positions[0]),
        ];
        let y_base = self.height / 3;

        for (stack, x_base) in location_pairs {
            for (i, bloc) in stack.iter().enumerate() {
                let width = bloc.val * BLOCKS_WIDTH;
                let delta_altura = i * BLOCKS_HEIGHT;

                let top_left = (x_base - width / 2, y_base + delta_altura + BLOCKS_HEIGHT / 2 );
                let bottom_right = (x_base + width / 2, y_base + delta_altura - BLOCKS_HEIGHT / 2 );

                draw_box(&mut buffer, top_left, bottom_right, bloc.color, self.width);
            }
            
        }

        // ppm spec backwards pending rotation 
        buffer.push("255\n".to_string());                                          // Max color val
        buffer.push(format!("{} {}\n", self.width, self.height));                  // Dimensions
        buffer.push("P3\n".to_string());                                           // Color
                                                                                  
        let bytes: Vec<u8> = buffer.iter() 
            .rev()
            .map(|s| s.as_bytes()) 
            .flatten() 
            .map(|n| *n)
            .collect(); 

        file.write(&bytes)?;
        Ok(())
    }

    pub fn solve(&mut self, origin: StackSelect, destination: StackSelect) {
        let mut g = 0;
        let mut gen = Generation(&mut g);
        for _ in 0..FROZEN_FRAMES_START { self.save_to_file(&mut gen).ok(); }
        self.move_stack(self.n, origin, destination, &mut gen);
        for _ in 0..FROZEN_FRAMES_END { self.save_to_file(&mut gen).ok(); }
        println!("Done!");
    }

    pub fn move_stack(&mut self, n: usize, origin: StackSelect, destination: StackSelect, t: &mut Generation ) -> Option<()> {
        use StackSelect as Sel;

        if n == 1 {
            self.move_block(origin, destination, t).unwrap();
        } else {
            let temp_stack = match (origin, destination) {
                (Sel::Stack1, Sel::Stack2) => Sel::Stack3,
                (Sel::Stack1, Sel::Stack3) => Sel::Stack2,
                (Sel::Stack2, Sel::Stack1) => Sel::Stack3,
                (Sel::Stack2, Sel::Stack3) => Sel::Stack1,
                (Sel::Stack3, Sel::Stack1) => Sel::Stack2,
                (Sel::Stack3, Sel::Stack2) => Sel::Stack1,
                _ => unreachable!(),
            };

            self.move_stack(n - 1, origin, temp_stack, t)?;
            self.move_block(origin, destination, t)?;
            self.move_stack(n - 1, temp_stack, destination, t)?;
        }

        Some(())
    }
    pub fn move_block(&mut self, origin: StackSelect, destination: StackSelect, gen: &mut Generation) -> Option<()>{
        if origin == destination { 
            eprintln!("Attempted to move a block to its own pile ({origin}-{destination})");
            return None; 
        }

        self.save_to_file(gen).ok()?;

        let block_origin: Option<&Block> = match origin {
            StackSelect::Stack1 => self.stack1.0.last(),
            StackSelect::Stack2 => self.stack2.0.last(),
            StackSelect::Stack3 => self.stack3.0.last(),
        };

        let block_destin: Option<&Block> = match destination {
            StackSelect::Stack1 => self.stack1.0.last(),
            StackSelect::Stack2 => self.stack2.0.last(),
            StackSelect::Stack3 => self.stack3.0.last(),
        };

        if let (Some(orig_block), Some(destin_block)) = (block_origin, block_destin) {
                if orig_block.val > destin_block.val {
                    eprintln!("Attempted to move a block breaking the Only Rule ({orig_block}>{destin_block}, which is illegal)");
                    return None; 
                }
        } 

        let elem = match origin {
            StackSelect::Stack1 => self.stack1.0.pop(),
            StackSelect::Stack2 => self.stack2.0.pop(),
            StackSelect::Stack3 => self.stack3.0.pop(),
        }.expect(&format!("Error agafant element de la pila: {origin} - {destination}"));

        match destination {
            StackSelect::Stack1 => self.stack1.0.push(elem),
            StackSelect::Stack2 => self.stack2.0.push(elem),
            StackSelect::Stack3 => self.stack3.0.push(elem),
        }
        Some(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum StackSelect {
    Stack1,
    Stack2,
    Stack3
}

#[derive(Debug, Clone)]
pub struct Stack (Vec<Block>); // End is top

impl Default for Stack {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}
impl Stack {
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
impl Display for StackSelect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let o = match self {
            StackSelect::Stack1 => "Pila A",
            StackSelect::Stack2 => "Pila B",
            StackSelect::Stack3 => "Pila C",
        };
        write!(f, "{}", o)
    }
}
impl Display for Stack {
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
            s.push_str(&format!("[A: {}]", self.stack1));
            s.push_str(&format!("[B: {}]", self.stack2));
            s.push_str(&format!("[C: {}]", self.stack3));

            s
        };

        write!(f, "{}", out)
    }
}
