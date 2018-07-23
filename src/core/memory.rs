use core::ppu::PPU;

pub trait Memory {
    fn load_byte(&mut self, addr: u16) -> u8;
    fn store_byte(&mut self, addr: u16, val: u8);

    fn load_word(&mut self, addr: u16) -> u16 {
        self.load_byte(addr) as u16 | self.load_byte(addr + 1) as u16
    }
    
    fn store_word(&mut self, addr: u16, val: u16) {
        self.store_byte(addr, (val & 0xff) as u8);
        self.store_byte(addr + 1, ((val >> 8) & 0xff) as u8);
    }
}

pub struct RAM {
    pub mem: [u8; 0x800]
}

impl RAM {
    pub fn new() -> RAM {
        RAM {
            mem: [0; 0x800]
        }
    }
}

// $07ff is the last address in RAM
// The address argument is &'d with $07ff to ensure it's within range
// and mirrored
impl Memory for RAM {
    fn load_byte(&mut self, addr: u16) -> u8 {
        self.mem[addr as usize & 0x07ff]
    }
    
    fn store_byte(&mut self, addr: u16, val: u8) {
        self.mem[addr as usize & 0x07ff] = val;
    }
}

pub struct CPUMemoryMap {
    ram: RAM,
    ppu: PPU
}

impl CPUMemoryMap {
    pub fn new(ppu: PPU) -> CPUMemoryMap {
        CPUMemoryMap {
            ram: RAM::new(),
            ppu: ppu
        }
    }
}

impl Memory for CPUMemoryMap {
    fn load_byte(&mut self, addr: u16) -> u8 {
        if addr < 0x2000 {
            self.ram.load_byte(addr)
        }
        else if addr < 0x4000 {
            self.ppu.load_byte(addr)
        }
        else {
            0x00
        }
    }

    fn store_byte(&mut self, addr: u16, val: u8) {
        if addr < 0x2000 {
            self.ram.store_byte(addr, val);
        } else if addr < 0x4000 {
            self.ppu.store_byte(addr, val);
        } else {
            
        }
    }
}
