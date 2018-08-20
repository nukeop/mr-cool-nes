use core::rom::Rom;

pub trait Mapper {
    fn type_of(&self) -> String;
    fn get_rom(&self) -> &Rom;
    fn load_prg_byte(&self, addr: u16) -> u8;
    fn load_chr_byte(&self, addr: u16) -> u8;
    fn store_prg_byte(&mut self, addr: u16, val: u8);
    fn store_chr_byte(&mut self, addr: u16, val: u8);
}

pub fn select_mapper(rom: Rom) -> Box<Mapper> {
    let mapper_number = rom.header.mapper_number();
    info!("Mapper number: {:X}", mapper_number);
    
    match mapper_number {
        0 => Box::new(NROM::new(rom)) as Box<Mapper>,
        1 => Box::new(SxROM::new(rom)) as Box<Mapper>,
        _ => panic!("Unimplemented mapper: {}", mapper_number)
    }
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
    fn type_of(&self) -> String {
        "NROM".to_string()
    }

    fn get_rom(&self) -> &Rom {
        return &self.rom;
    }
    
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

    fn store_prg_byte(&mut self, addr: u16, val: u8) {}
    fn store_chr_byte(&mut self, addr: u16, val: u8) {}
}

pub struct TestMapper {
    pub rom: Rom,
    mem: [u8; 0x2000]
}

impl TestMapper {
    pub fn new(rom: Rom) -> TestMapper {
        TestMapper {
            rom,
            mem: [0; 0x2000]
        }
    }
}

impl Mapper for TestMapper {
    fn type_of(&self) -> String {
        "Test".to_string()
    }

    fn get_rom(&self) -> &Rom {
        &self.rom
    }

    fn load_prg_byte(&self, addr: u16) -> u8 {
        if addr < 0x6000 {
            0
        } else if addr < 0x8000 {
            self.mem[addr as usize & 0x1FFF]
        } else if self.rom.prg_rom.len() > 0x4000 {
            self.rom.prg_rom[addr as usize & 0x7fff]
        } else {
            self.rom.prg_rom[addr as usize & 0x3fff]
        }
    }

    fn load_chr_byte(&self, addr: u16) -> u8 {
        0
    }

    fn store_prg_byte(&mut self, addr: u16, val: u8) {
        if addr < 0x6000 {
            
        } else if addr < 0x8000 {
            self.mem[addr as usize & 0x1FFF] = val;
        } else if self.rom.prg_rom.len() > 0x4000 {
            self.rom.prg_rom[addr as usize & 0x7fff] = val;
        } else {
            self.rom.prg_rom[addr as usize & 0x3fff] = val;
        }
    }
    
    fn store_chr_byte(&mut self, addr: u16, val: u8) { }
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
    fn type_of(&self) -> String {
        "SxROM".to_string()
    }

    fn get_rom(&self) -> &Rom {
        return &self.rom;
    }
    
    fn load_prg_byte(&self, addr: u16) -> u8 {
        0x00
    }

    fn load_chr_byte(&self, addr: u16) -> u8 {
        0x00
    }

    fn store_prg_byte(&mut self, addr: u16, val: u8) {}
    fn store_chr_byte(&mut self, addr: u16, val: u8) {}
}
