pub fn idx_to_coords(idx: usize, w: usize) -> (usize, usize) {
    (
        idx % w,
        idx / w,
    )
}

pub fn coords_to_idx(x: usize, y: usize, w: usize) -> usize {
    x + w*y + 3 // Tres per l'spec de ppm
}

pub fn rgb_to_str(r: u8, g: u8, b: u8) -> String {
    format!("{:>3} {:>3} {:>3}\n", r, g, b)
}

pub fn draw_box(buffer: &mut Vec<String>, top_left: (usize, usize),
            bottom_right: (usize, usize), col: (u8, u8, u8), w: usize)
{
    let left_x = top_left.0;
    let right_x = bottom_right.0;
    let top_y = top_left.1;
    let bottom_y = bottom_right.1;

    for y in bottom_y..=top_y {
        for x in left_x..=right_x {
            let idx = coords_to_idx(x, y, w);
            let color_str = rgb_to_str(col.0, col.1, col.2).to_string();
            buffer[idx] = color_str;
        }
    }

}
