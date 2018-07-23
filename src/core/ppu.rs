use core::memory::Memory;

pub struct Registers {
    ppu_ctrl: u8,      // $2000
    ppu_mask: u8,      // $2001
    ppu_status: u8,    // $2002
    oam_addr: u8,      // $2003
    oam_data: u8,      // $2004
    ppu_scroll: u8,    // $2005
    ppu_addr: u8,      // $2006
    ppu_data: u8,      // $2007
    oam_dma: u8        // $4014
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            ppu_ctrl: 0,
            ppu_mask: 0,
            ppu_status: 0,
            oam_addr: 0,
            oam_data: 0,
            ppu_scroll: 0,
            ppu_addr: 0,
            ppu_data: 0,
            oam_dma: 0
        }
    }
}

pub struct VRAM {
    nametables: [u8; 0x800],
    palettes: [u8; 0x20]
}

pub struct OAM {
    oam: [u8; 0x100]
}

pub struct PPU {
    regs: Registers,
    vram: VRAM,
    oam: OAM
}

impl Memory for PPU {
    fn load_byte(&mut self, addr: u16) -> u8 {
        match addr & 7 {
            0 => self.regs.ppu_ctrl,
            1 => self.regs.ppu_mask,
            2 => self.regs.ppu_status,
            3 => 0x00,
            4 => self.regs.oam_data,
            5 => 0x00,
            6 => 0x00,
            7 => self.regs.ppu_data,
            _ => panic!("Invalid memory address read from PPU")
        }
    }
    // Addresses returning 0x00 above are write-only

    fn store_byte(&mut self, addr: u16, val: u8) {
        match addr & 7 {
            0 => self.regs.ppu_ctrl = val,
            1 => self.regs.ppu_mask = val,
            2 => (),
            3 => self.regs.oam_addr = val,
            4 => self.regs.oam_data = val,
            5 => self.regs.ppu_scroll = val,
            6 => self.regs.ppu_addr = val,
            7 => self.regs.ppu_data = val,
            _ => panic!("Invalid memory address written to on PPU")
        }
    }
}
