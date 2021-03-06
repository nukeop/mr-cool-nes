use sdl2::rect::Rect;

const GRID_UNIT: u32 = 16;

pub fn get_letter(letter: &char) -> Rect {
    match letter {
        &' ' => Rect::new(0, 0, GRID_UNIT, GRID_UNIT),
        &'!' => Rect::new(GRID_UNIT as i32, 0, GRID_UNIT, GRID_UNIT),
        &'"' => Rect::new((GRID_UNIT*2) as i32, 0, GRID_UNIT, GRID_UNIT),
        &'#' => Rect::new((GRID_UNIT*3) as i32, 0, GRID_UNIT, GRID_UNIT),
        &'$' => Rect::new((GRID_UNIT*4) as i32, 0, GRID_UNIT, GRID_UNIT),
        &'%' => Rect::new((GRID_UNIT*5) as i32, 0, GRID_UNIT, GRID_UNIT),
        &'&' => Rect::new((GRID_UNIT*6) as i32, 0, GRID_UNIT, GRID_UNIT),
        &'\'' => Rect::new((GRID_UNIT*7) as i32, 0, GRID_UNIT, GRID_UNIT),
        &'(' => Rect::new((GRID_UNIT*8) as i32, 0, GRID_UNIT, GRID_UNIT),
        &')' => Rect::new((GRID_UNIT*9) as i32, 0, GRID_UNIT, GRID_UNIT),
        &'*' => Rect::new((GRID_UNIT*10) as i32, 0, GRID_UNIT, GRID_UNIT),
        &'+' => Rect::new((GRID_UNIT*11) as i32, 0, GRID_UNIT, GRID_UNIT),
        &',' => Rect::new((GRID_UNIT*12) as i32, 0, GRID_UNIT, GRID_UNIT),
        &'-' => Rect::new((GRID_UNIT*13) as i32, 0, GRID_UNIT, GRID_UNIT),
        &'.' => Rect::new((GRID_UNIT*14) as i32, 0, GRID_UNIT, GRID_UNIT),
        &'/' => Rect::new((GRID_UNIT*15) as i32, 0, GRID_UNIT, GRID_UNIT),
        &'0' => Rect::new(0, GRID_UNIT as i32, GRID_UNIT, GRID_UNIT),
        &'1' => Rect::new(GRID_UNIT as i32, GRID_UNIT as i32, GRID_UNIT, GRID_UNIT),
        &'2' => Rect::new((GRID_UNIT*2) as i32, GRID_UNIT as i32, GRID_UNIT, GRID_UNIT),
        &'3' => Rect::new((GRID_UNIT*3) as i32, GRID_UNIT as i32, GRID_UNIT, GRID_UNIT),
        &'4' => Rect::new((GRID_UNIT*4) as i32, GRID_UNIT as i32, GRID_UNIT, GRID_UNIT),
        &'5' => Rect::new((GRID_UNIT*5) as i32, GRID_UNIT as i32, GRID_UNIT, GRID_UNIT),
        &'6' => Rect::new((GRID_UNIT*6) as i32, GRID_UNIT as i32, GRID_UNIT, GRID_UNIT),
        &'7' => Rect::new((GRID_UNIT*7) as i32, GRID_UNIT as i32, GRID_UNIT, GRID_UNIT),
        &'8' => Rect::new((GRID_UNIT*8) as i32, GRID_UNIT as i32, GRID_UNIT, GRID_UNIT),
        &'9' => Rect::new((GRID_UNIT*9) as i32, GRID_UNIT as i32, GRID_UNIT, GRID_UNIT),
        &':' => Rect::new((GRID_UNIT*10) as i32, GRID_UNIT as i32, GRID_UNIT, GRID_UNIT),
        &';' => Rect::new((GRID_UNIT*11) as i32, GRID_UNIT as i32, GRID_UNIT, GRID_UNIT),
        &'<' => Rect::new((GRID_UNIT*12) as i32, GRID_UNIT as i32, GRID_UNIT, GRID_UNIT),
        &'=' => Rect::new((GRID_UNIT*13) as i32, GRID_UNIT as i32, GRID_UNIT, GRID_UNIT),
        &'>' => Rect::new((GRID_UNIT*14) as i32, GRID_UNIT as i32, GRID_UNIT, GRID_UNIT),
        &'?' => Rect::new((GRID_UNIT*15) as i32, GRID_UNIT as i32, GRID_UNIT, GRID_UNIT),
        &'@' => Rect::new(0, (GRID_UNIT*2) as i32, GRID_UNIT, GRID_UNIT),
        &'A' => Rect::new(GRID_UNIT as i32, (GRID_UNIT*2) as i32, GRID_UNIT, GRID_UNIT),
        &'B' => Rect::new((GRID_UNIT*2) as i32, (GRID_UNIT*2) as i32, GRID_UNIT, GRID_UNIT),
        &'C' => Rect::new((GRID_UNIT*3) as i32, (GRID_UNIT*2) as i32, GRID_UNIT, GRID_UNIT),
        &'D' => Rect::new((GRID_UNIT*4) as i32, (GRID_UNIT*2) as i32, GRID_UNIT, GRID_UNIT),
        &'E' => Rect::new((GRID_UNIT*5) as i32, (GRID_UNIT*2) as i32, GRID_UNIT, GRID_UNIT),
        &'F' => Rect::new((GRID_UNIT*6) as i32, (GRID_UNIT*2) as i32, GRID_UNIT, GRID_UNIT),
        &'G' => Rect::new((GRID_UNIT*7) as i32, (GRID_UNIT*2) as i32, GRID_UNIT, GRID_UNIT),
        &'H' => Rect::new((GRID_UNIT*8) as i32, (GRID_UNIT*2) as i32, GRID_UNIT, GRID_UNIT),
        &'I' => Rect::new((GRID_UNIT*9) as i32, (GRID_UNIT*2) as i32, GRID_UNIT, GRID_UNIT),
        &'J' => Rect::new((GRID_UNIT*10) as i32, (GRID_UNIT*2) as i32, GRID_UNIT, GRID_UNIT),
        &'K' => Rect::new((GRID_UNIT*11) as i32, (GRID_UNIT*2) as i32, GRID_UNIT, GRID_UNIT),
        &'L' => Rect::new((GRID_UNIT*12) as i32, (GRID_UNIT*2) as i32, GRID_UNIT, GRID_UNIT),
        &'M' => Rect::new((GRID_UNIT*13) as i32, (GRID_UNIT*2) as i32, GRID_UNIT, GRID_UNIT),
        &'N' => Rect::new((GRID_UNIT*14) as i32, (GRID_UNIT*2) as i32, GRID_UNIT, GRID_UNIT),
        &'O' => Rect::new((GRID_UNIT*15) as i32, (GRID_UNIT*2) as i32, GRID_UNIT, GRID_UNIT),
        &'P' => Rect::new(0, (GRID_UNIT*3) as i32, GRID_UNIT, GRID_UNIT),
        &'Q' => Rect::new(GRID_UNIT as i32, (GRID_UNIT*3) as i32, GRID_UNIT, GRID_UNIT),
        &'R' => Rect::new((GRID_UNIT*2) as i32, (GRID_UNIT*3) as i32, GRID_UNIT, GRID_UNIT),
        &'S' => Rect::new((GRID_UNIT*3) as i32, (GRID_UNIT*3) as i32, GRID_UNIT, GRID_UNIT),
        &'T' => Rect::new((GRID_UNIT*4) as i32, (GRID_UNIT*3) as i32, GRID_UNIT, GRID_UNIT),
        &'U' => Rect::new((GRID_UNIT*5) as i32, (GRID_UNIT*3) as i32, GRID_UNIT, GRID_UNIT),
        &'V' => Rect::new((GRID_UNIT*6) as i32, (GRID_UNIT*3) as i32, GRID_UNIT, GRID_UNIT),
        &'W' => Rect::new((GRID_UNIT*7) as i32, (GRID_UNIT*3) as i32, GRID_UNIT, GRID_UNIT),
        &'X' => Rect::new((GRID_UNIT*8) as i32, (GRID_UNIT*3) as i32, GRID_UNIT, GRID_UNIT),
        &'Y' => Rect::new((GRID_UNIT*9) as i32, (GRID_UNIT*3) as i32, GRID_UNIT, GRID_UNIT),
        &'Z' => Rect::new((GRID_UNIT*10) as i32, (GRID_UNIT*3) as i32, GRID_UNIT, GRID_UNIT),
        &'[' => Rect::new((GRID_UNIT*11) as i32, (GRID_UNIT*3) as i32, GRID_UNIT, GRID_UNIT),
        &'\\' => Rect::new((GRID_UNIT*12) as i32, (GRID_UNIT*3) as i32, GRID_UNIT, GRID_UNIT),
        &']' => Rect::new((GRID_UNIT*13) as i32, (GRID_UNIT*3) as i32, GRID_UNIT, GRID_UNIT),
        &'^' => Rect::new((GRID_UNIT*14) as i32, (GRID_UNIT*3) as i32, GRID_UNIT, GRID_UNIT),
        &'_' => Rect::new((GRID_UNIT*15) as i32, (GRID_UNIT*3) as i32, GRID_UNIT, GRID_UNIT),
        &'a' => Rect::new((GRID_UNIT) as i32, (GRID_UNIT*4) as i32, GRID_UNIT, GRID_UNIT),
        &'b' => Rect::new((GRID_UNIT*2) as i32, (GRID_UNIT*4) as i32, GRID_UNIT, GRID_UNIT),
        &'c' => Rect::new((GRID_UNIT*3) as i32, (GRID_UNIT*4) as i32, GRID_UNIT, GRID_UNIT),
        &'d' => Rect::new((GRID_UNIT*4) as i32, (GRID_UNIT*4) as i32, GRID_UNIT, GRID_UNIT),
        &'e' => Rect::new((GRID_UNIT*5) as i32, (GRID_UNIT*4) as i32, GRID_UNIT, GRID_UNIT),
        &'f' => Rect::new((GRID_UNIT*6) as i32, (GRID_UNIT*4) as i32, GRID_UNIT, GRID_UNIT),
        &'g' => Rect::new((GRID_UNIT*7) as i32, (GRID_UNIT*4) as i32, GRID_UNIT, GRID_UNIT),
        &'h' => Rect::new((GRID_UNIT*8) as i32, (GRID_UNIT*4) as i32, GRID_UNIT, GRID_UNIT),
        &'i' => Rect::new((GRID_UNIT*9) as i32, (GRID_UNIT*4) as i32, GRID_UNIT, GRID_UNIT),
        &'j' => Rect::new((GRID_UNIT*10) as i32, (GRID_UNIT*4) as i32, GRID_UNIT, GRID_UNIT),
        &'k' => Rect::new((GRID_UNIT*11) as i32, (GRID_UNIT*4) as i32, GRID_UNIT, GRID_UNIT),
        &'l' => Rect::new((GRID_UNIT*12) as i32, (GRID_UNIT*4) as i32, GRID_UNIT, GRID_UNIT),
        &'m' => Rect::new((GRID_UNIT*13) as i32, (GRID_UNIT*4) as i32, GRID_UNIT, GRID_UNIT),
        &'n' => Rect::new((GRID_UNIT*14) as i32, (GRID_UNIT*4) as i32, GRID_UNIT, GRID_UNIT),
        &'o' => Rect::new((GRID_UNIT*15) as i32, (GRID_UNIT*4) as i32, GRID_UNIT, GRID_UNIT),
        &'p' => Rect::new(0, (GRID_UNIT*5) as i32, GRID_UNIT, GRID_UNIT),
        &'q' => Rect::new(GRID_UNIT as i32, (GRID_UNIT*5) as i32, GRID_UNIT, GRID_UNIT),
        &'r' => Rect::new((GRID_UNIT*2) as i32, (GRID_UNIT*5) as i32, GRID_UNIT, GRID_UNIT),
        &'s' => Rect::new((GRID_UNIT*3) as i32, (GRID_UNIT*5) as i32, GRID_UNIT, GRID_UNIT),
        &'t' => Rect::new((GRID_UNIT*4) as i32, (GRID_UNIT*5) as i32, GRID_UNIT, GRID_UNIT),
        &'u' => Rect::new((GRID_UNIT*5) as i32, (GRID_UNIT*5) as i32, GRID_UNIT, GRID_UNIT),
        &'v' => Rect::new((GRID_UNIT*6) as i32, (GRID_UNIT*5) as i32, GRID_UNIT, GRID_UNIT),
        &'w' => Rect::new((GRID_UNIT*7) as i32, (GRID_UNIT*5) as i32, GRID_UNIT, GRID_UNIT),
        &'x' => Rect::new((GRID_UNIT*8) as i32, (GRID_UNIT*5) as i32, GRID_UNIT, GRID_UNIT),
        &'y' => Rect::new((GRID_UNIT*9) as i32, (GRID_UNIT*5) as i32, GRID_UNIT, GRID_UNIT),
        &'z' => Rect::new((GRID_UNIT*10) as i32, (GRID_UNIT*5) as i32, GRID_UNIT, GRID_UNIT),
        &'{' => Rect::new((GRID_UNIT*11) as i32, (GRID_UNIT*5) as i32, GRID_UNIT, GRID_UNIT),
        &'|' => Rect::new((GRID_UNIT*12) as i32, (GRID_UNIT*5) as i32, GRID_UNIT, GRID_UNIT),
        &'}' => Rect::new((GRID_UNIT*13) as i32, (GRID_UNIT*5) as i32, GRID_UNIT, GRID_UNIT),
        &'~' => Rect::new((GRID_UNIT*14) as i32, (GRID_UNIT*5) as i32, GRID_UNIT, GRID_UNIT),
        &_ => Rect::new((GRID_UNIT*15) as i32, GRID_UNIT as i32, GRID_UNIT, GRID_UNIT)
    }
}
