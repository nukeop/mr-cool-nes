pub trait Memory {
    fn load_byte(&mut self, addr: u16) -> u8;
    fn store_byte(&mut self, addr: u16, val: u8);
}

pub struct RAM {
    pub mem: [u8; 0x800]
}

// $07ff is the last address in RAM
// The address argument is &'d with $07ff to ensure it's within range 
impl Memory for RAM {
    fn load_byte(&mut self, addr: u16) -> u8 {
        self.mem[addr as usize & 0x07ff]
    }
    
    fn store_byte(&mut self, addr: u16, val: u8) {
        self.mem[addr as usize & 0x07ff] = val;
    }
}
