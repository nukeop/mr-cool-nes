use core::rom::Rom;

pub trait Mapper {
    fn load_prg_byte(&self, addr: u16) -> u8;
    fn load_chr_byte(&self, addr: u16) -> u8;
}

pub struct NROM {
    rom: Rom
}

impl Mapper for NROM {
    fn load_prg_byte(&self, addr: u16) -> u8 {
        if addr < 0x8000 {
            0
        } else if self.rom.prg_rom.len() > 0x4000 {
            self.rom.prg_rom[addr as usize & 0x7fff]
        } else {
            self.rom.prg_rom[addr as usize & 0x3fff]
        }
    }

    fn load_chr_byte(&self, addr: u16) -> u8 {
        self.rom.chr_rom[addr as usize]
    }
}

pub struct SxROMRegisters {
    
}

pub struct SxROM {
    rom: Rom
}

impl Mapper for SxROM {
    fn load_prg_byte(&self, addr: u16) -> u8 {
        0x00
    }

    fn load_chr_byte(&self, addr: u16) -> u8 {
        0x00
    }
}
