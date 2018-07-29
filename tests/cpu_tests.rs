extern crate mr_cool_nes;

#[cfg(test)]
mod ppu_tests {
    use mr_cool_nes::core::cpu::CPU;
    use mr_cool_nes::core::ppu::PPU;
    use mr_cool_nes::core::memory::{Memory, RAM};

    fn setup_cpu() -> CPU {
        let ram = RAM::new();
        let ppu = PPU::new();
        CPU::new(ppu, ram)
    }

    #[test]
    fn store_byte_ram() {
        let mut cpu = setup_cpu();
        cpu.store_byte(0x0, 0xDD);
        assert_eq!(cpu.mem_map.ram.mem[0x0], 0xDD);
    }

    #[test]
    fn store_byte_vram() {
        let mut cpu = setup_cpu();
        cpu.store_byte(0x2000, 0xDD);
        assert_eq!(cpu.mem_map.ppu.regs.ppu_ctrl, 0xDD);
    }

    #[test]
    fn store_byte_dma() {
        let mut cpu = setup_cpu();
        let final_addr = ((0x0 as u16) << 8) + 256;
        cpu.store_byte(final_addr, 0xDD);
        cpu.store_byte(0x4014, 0x0);
        assert_eq!(cpu.mem_map.ram.mem[final_addr as usize], 0xDD);
    }
}
