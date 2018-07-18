use sdl2::rect::Rect;

const GRID_UNIT: u32 = 16;

pub fn get_letter(letter: &char) -> Rect {
    match letter {
        &' ' => Rect::new(0, 0, GRID_UNIT, GRID_UNIT),
        &',' => Rect::new((GRID_UNIT*12) as i32, 0, GRID_UNIT, GRID_UNIT),
        &'-' => Rect::new((GRID_UNIT*13) as i32, 0, GRID_UNIT, GRID_UNIT),
        &'.' => Rect::new((GRID_UNIT*14) as i32, 0, GRID_UNIT, GRID_UNIT),
        &':' => Rect::new((GRID_UNIT*10) as i32, GRID_UNIT as i32, GRID_UNIT, GRID_UNIT),
        &';' => Rect::new((GRID_UNIT*11) as i32, GRID_UNIT as i32, GRID_UNIT, GRID_UNIT),
        &'_' => Rect::new((GRID_UNIT*15) as i32, (GRID_UNIT*3) as i32, GRID_UNIT, GRID_UNIT),
        &'A' => Rect::new(GRID_UNIT as i32, (GRID_UNIT*2) as i32, GRID_UNIT, GRID_UNIT),
        &'B' => Rect::new((GRID_UNIT*2) as i32, (GRID_UNIT*2) as i32, GRID_UNIT, GRID_UNIT),
        &'C' => Rect::new((GRID_UNIT*3) as i32, (GRID_UNIT*2) as i32, GRID_UNIT, GRID_UNIT),
        &'D' => Rect::new((GRID_UNIT*4) as i32, (GRID_UNIT*2) as i32, GRID_UNIT, GRID_UNIT),
        &'E' => Rect::new((GRID_UNIT*5) as i32, (GRID_UNIT*2) as i32, GRID_UNIT, GRID_UNIT),
        &'M' => Rect::new((GRID_UNIT*13) as i32, (GRID_UNIT*2) as i32, GRID_UNIT, GRID_UNIT),
        &'N' => Rect::new((GRID_UNIT*14) as i32, (GRID_UNIT*2) as i32, GRID_UNIT, GRID_UNIT),
        &'O' => Rect::new((GRID_UNIT*15) as i32, (GRID_UNIT*2) as i32, GRID_UNIT, GRID_UNIT),
        &'P' => Rect::new(0, (GRID_UNIT*3) as i32, GRID_UNIT, GRID_UNIT),
        &'Q' => Rect::new(GRID_UNIT as i32, (GRID_UNIT*3) as i32, GRID_UNIT, GRID_UNIT),
        &'R' => Rect::new((GRID_UNIT*2) as i32, (GRID_UNIT*3) as i32, GRID_UNIT, GRID_UNIT),
        &'S' => Rect::new((GRID_UNIT*3) as i32, (GRID_UNIT*3) as i32, GRID_UNIT, GRID_UNIT),
        &'a' => Rect::new((GRID_UNIT) as i32, (GRID_UNIT*4) as i32, GRID_UNIT, GRID_UNIT),
        &'c' => Rect::new((GRID_UNIT*3) as i32, (GRID_UNIT*4) as i32, GRID_UNIT, GRID_UNIT),
        &'d' => Rect::new((GRID_UNIT*4) as i32, (GRID_UNIT*4) as i32, GRID_UNIT, GRID_UNIT),
        &'e' => Rect::new((GRID_UNIT*5) as i32, (GRID_UNIT*4) as i32, GRID_UNIT, GRID_UNIT),
        &'l' => Rect::new((GRID_UNIT*12) as i32, (GRID_UNIT*4) as i32, GRID_UNIT, GRID_UNIT),
        &'m' => Rect::new((GRID_UNIT*13) as i32, (GRID_UNIT*4) as i32, GRID_UNIT, GRID_UNIT),
        &'n' => Rect::new((GRID_UNIT*14) as i32, (GRID_UNIT*4) as i32, GRID_UNIT, GRID_UNIT),
        &'o' => Rect::new((GRID_UNIT*15) as i32, (GRID_UNIT*4) as i32, GRID_UNIT, GRID_UNIT),
        &'p' => Rect::new(0, (GRID_UNIT*5) as i32, GRID_UNIT, GRID_UNIT),
        &'r' => Rect::new((GRID_UNIT*2) as i32, (GRID_UNIT*5) as i32, GRID_UNIT, GRID_UNIT),
        &'s' => Rect::new((GRID_UNIT*3) as i32, (GRID_UNIT*5) as i32, GRID_UNIT, GRID_UNIT),
        &'t' => Rect::new((GRID_UNIT*4) as i32, (GRID_UNIT*5) as i32, GRID_UNIT, GRID_UNIT),
        &'u' => Rect::new((GRID_UNIT*5) as i32, (GRID_UNIT*5) as i32, GRID_UNIT, GRID_UNIT),
        &'y' => Rect::new((GRID_UNIT*9) as i32, (GRID_UNIT*5) as i32, GRID_UNIT, GRID_UNIT),
        &_ => Rect::new(GRID_UNIT as i32, (GRID_UNIT*15) as i32, GRID_UNIT, GRID_UNIT)
    }
}
