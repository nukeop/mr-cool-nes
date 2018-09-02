use core::ppu::PPU;
use core::mapper::Mapper;

pub trait Memory {
    fn load_byte(&mut self, addr: u16) -> u8;
    fn store_byte(&mut self, addr: u16, val: u8);

    fn load_word(&mut self, addr: u16) -> u16 {
        self.load_byte(addr) as u16 | (self.load_byte(addr + 1) as u16) << 8
    }
    
    fn store_word(&mut self, addr: u16, val: u16) {
        self.store_byte(addr, (val & 0xff) as u8);
        self.store_byte(addr + 1, ((val >> 8) & 0xff) as u8);
    }

    fn load_word_zeropage_wraparound(&mut self, addr: u8) -> u16 {
        let addr1 = addr as u16;
        let addr2 = addr.wrapping_add(1) as u16;
        
        self.load_byte(addr1) as u16 | (self.load_byte(addr2) as u16) << 8
    }
}

pub struct RAM {
    pub mem: [u8; 0x800]
}

impl RAM {
    pub fn new() -> RAM {
        info!("Creating RAM...");
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

pub struct CPUMemoryMap<'a> {
    pub ram: RAM,
    pub ppu: &'a mut PPU,
    pub mapper: Box<Mapper>
}

impl<'a> CPUMemoryMap<'a> {
    pub fn new(ppu: &mut PPU, ram: RAM, mapper: Box<Mapper>) -> CPUMemoryMap {
        CPUMemoryMap {
            ram,
            ppu,
            mapper
        }
    }
}

impl<'a> Memory for CPUMemoryMap<'a> {
    fn load_byte(&mut self, addr: u16) -> u8 {
        if addr < 0x2000 {
            self.ram.load_byte(addr)
        } else if addr < 0x4000 {
            self.ppu.load_byte(addr)
        } else if addr < 0x6000 {
            0
        } else {
            self.mapper.load_prg_byte(addr)
        }
    }

    fn store_byte(&mut self, addr: u16, val: u8) {
        if addr < 0x2000 {
            self.ram.store_byte(addr, val);
        } else if addr < 0x4000 {
            self.ppu.store_byte(addr, val);
        } else if addr < 0x6000 {

        } else {
            self.mapper.store_prg_byte(addr, val);
        }
    }
}
