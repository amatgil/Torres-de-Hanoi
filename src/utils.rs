
pub fn idx_to_coords(idx: usize, w: usize) -> (usize, usize) {
    (
        idx % w,
        idx / w,
    )
}

pub fn coords_to_idx(x: usize, y: usize, w: usize) -> usize {
    x + w*y
}

pub fn rgb_to_str(r: u8, g: u8, b: u8) -> String {
    format!("{:>3} {:>3} {:>3}", r, g, b)
}
