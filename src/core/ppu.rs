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
