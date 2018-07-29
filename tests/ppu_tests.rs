extern crate mr_cool_nes;

#[cfg(test)]
mod ppu_tests {
    use mr_cool_nes::core::ppu::PPU;
    use mr_cool_nes::core::memory::Memory;

    fn setup_ppu() -> PPU{
        PPU::new()
    }

    #[test]
    fn ppu_new() {
        let ppu = setup_ppu();
        assert_eq!(ppu.regs.ppu_ctrl, 0x0);
        assert_eq!(ppu.regs.ppu_mask, 0x0);
        assert_eq!(ppu.regs.ppu_status, 0x0);
        assert_eq!(ppu.regs.oam_addr, 0x0);
        assert_eq!(ppu.regs.oam_data, 0x0);
        assert_eq!(ppu.regs.ppu_scroll, 0x0);
        assert_eq!(ppu.regs.ppu_addr, 0x0);
        assert_eq!(ppu.regs.ppu_data, 0x0);
        assert_eq!(ppu.regs.oam_dma, 0x0);
        assert!(ppu.vram.nametables.iter().zip([0; 0x800].iter()).all(|(a,b)| a == b), "Arrays are not equal");
        assert!(ppu.vram.palettes.iter().zip([0; 0x20].iter()).all(|(a,b)| a == b), "Arrays are not equal");
        assert!(ppu.oam.oam.iter().zip([0; 0x100].iter()).all(|(a,b)| a == b), "Arrays are not equal");
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
