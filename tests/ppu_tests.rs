extern crate mr_cool_nes;

#[cfg(test)]
mod ppu_tests {
    use mr_cool_nes::core::ppu::PPU;
    use mr_cool_nes::core::memory::Memory;

    fn setup_ppu() -> PPU{
        return PPU::new();
    }

    #[test]
    fn store_byte_ppu_ctrl() {
        let mut ppu = setup_ppu();
        ppu.store_byte(0x2000, 0xDD);
        assert_eq!(ppu.regs.ppu_ctrl, 0xDD);
    }
    
    #[test]
    fn store_byte_ppu_mask() {
        let mut ppu = setup_ppu();
        ppu.store_byte(0x2001, 0xDD);
        assert_eq!(ppu.regs.ppu_mask, 0xDD);
    }

    #[test]
    fn store_byte_ppu_status() {
        let mut ppu = setup_ppu();
        ppu.store_byte(0x2002, 0xDD);
        assert_eq!(ppu.regs.ppu_status, 0x0);
    }

    #[test]
    fn store_byte_ppu_oam_addr() {
        let mut ppu = setup_ppu();
        ppu.store_byte(0x2003, 0xDD);
        assert_eq!(ppu.regs.oam_addr, 0xDD);
    }

    #[test]
    fn store_byte_ppu_oam_data() {
        let mut ppu = setup_ppu();
        ppu.store_byte(0x2004, 0xDD);
        assert_eq!(ppu.regs.oam_data, 0xDD);
    }

    #[test]
    fn store_byte_ppu_ppu_scroll() {
        let mut ppu = setup_ppu();
        ppu.store_byte(0x2005, 0xDD);
        assert_eq!(ppu.regs.ppu_scroll, 0xDD);
    }

    #[test]
    fn store_byte_ppu_ppu_addr() {
        let mut ppu = setup_ppu();
        ppu.store_byte(0x2006, 0xDD);
        assert_eq!(ppu.regs.ppu_addr, 0xDD);
    }

    #[test]
    fn store_byte_ppu_ppu_data() {
        let mut ppu = setup_ppu();
        ppu.store_byte(0x2007, 0xDD);
        assert_eq!(ppu.regs.ppu_data, 0xDD);
    }
}
