extern crate mr_cool_nes;

#[cfg(test)]
mod ppu_tests {
    use mr_cool_nes::core::cpu::CPU;
    use mr_cool_nes::core::ppu::PPU;
    use mr_cool_nes::core::memory::{Memory, RAM};
    use mr_cool_nes::core::rom::{INesHeader, Rom};
    use mr_cool_nes::core::mapper::NROM;

    fn setup_cpu() -> CPU {
        let rom = Rom {
            header: INesHeader {
                magic: ['N' as u8, 'E' as u8, 'S' as u8, '\x1a' as u8],
                prg_rom_size: 1,
                chr_rom_size: 1,
                flags_6: 0,
                flags_7: 0,
                prg_ram_size: 1,
                flags_9: 0,
                flags_10: 0,
                zero: [0; 5]
            },
            prg_rom: vec![0; 16384],
            chr_rom: vec![0; 16384]
        };
        
        let ram = RAM::new();
        let ppu = PPU::new();
        let mapper = NROM::new(rom);
        CPU::new(ppu, ram, Box::new(mapper))
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
