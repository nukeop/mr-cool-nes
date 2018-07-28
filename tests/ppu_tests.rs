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
    
}
