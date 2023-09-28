fn main() {
    let mut world = World::new(4);

    dbg!(&world);
    world.moure_pila(PilaSelect::Pila1, PilaSelect::Pila2);
    dbg!(&world);
}

#[derive(Debug)]
struct World {
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
    pub fn moure_pila(&mut self, origin: PilaSelect, destinacio: PilaSelect) -> Option<()>{
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
enum PilaSelect {
    Pila1,
    Pila2,
    Pila3
}

#[derive(Debug, Clone)]
struct Pila (Vec<Block>); // End is top

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
struct Block(usize);

