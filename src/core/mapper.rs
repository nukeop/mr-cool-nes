use core::rom::Rom;

pub trait Mapper {
    fn load_prg_byte(&self, addr: u16) -> u8;
    fn load_chr_byte(&self, addr: u16) -> u8;
}

pub struct NROM {
    rom: Rom
}

impl NROM {
    pub fn new(rom: Rom) -> NROM {
        NROM {
            rom
        }
    }
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
    ctrl: u8,
    chr_bank0: u8,
    chr_bank1: u8,
    prg_bank: u8
}

impl SxROMRegisters {
    pub fn new() -> SxROMRegisters {
        SxROMRegisters {
            ctrl: 12,
            chr_bank0: 0,
            chr_bank1: 0,
            prg_bank: 0
        }
    }
}

pub struct SxROM {
    rom: Rom,
    regs: SxROMRegisters,
    chr_ram: [u8; 0x2000]
}

impl SxROM {
    pub fn new(rom: Rom) -> SxROM {
        SxROM {
            rom,
            regs: SxROMRegisters::new(),
            chr_ram: [0; 0x2000]
        }
    }
}

impl Mapper for SxROM {
    fn load_prg_byte(&self, addr: u16) -> u8 {
        0x00
    }

    fn load_chr_byte(&self, addr: u16) -> u8 {
        0x00
    }
}
