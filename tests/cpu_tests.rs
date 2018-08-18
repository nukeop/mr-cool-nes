extern crate mr_cool_nes;

#[cfg(test)]
mod cpu_tests {
    use mr_cool_nes::core::cpu::*;
    use mr_cool_nes::core::ppu::PPU;
    use mr_cool_nes::core::memory::{Memory, RAM};
    use mr_cool_nes::core::rom::{INesHeader, Rom};
    use mr_cool_nes::core::mapper::NROM;

    fn setup_rom() -> Rom {
        Rom {
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
        }
    }
    
    fn setup_cpu() -> CPU {
        let rom = setup_rom();        
        let ram = RAM::new();
        let ppu = PPU::new();
        let mapper = NROM::new(rom);
        CPU::new(ppu, ram, Box::new(mapper))
    }

    #[test]
    fn load_byte_ram() {
        let mut cpu = setup_cpu();
        cpu.mem_map.ram.mem[0xFF] = 0xDD;
        let byte = cpu.load_byte(0xFF);
        assert_eq!(byte, 0xDD);
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

    #[test]
    fn push_byte_on_stack() {
        let mut cpu = setup_cpu();
        cpu.push_byte(0xDD);
        assert_eq!(cpu.regs.s, 0xFC);
        assert_eq!(cpu.mem_map.ram.mem[(0x100 + (cpu.regs.s + 1) as u16) as usize], 0xDD);
    }

    #[test]
    fn push_word_on_stack() {
        let mut cpu = setup_cpu();
        cpu.push_word(0xCCDD);
        assert_eq!(cpu.regs.s, 0xFB);
        assert_eq!(cpu.mem_map.ram.mem[(0x100 + (cpu.regs.s + 1) as u16) as usize], 0xDD);
        assert_eq!(cpu.mem_map.ram.mem[(0x100 + (cpu.regs.s + 2) as u16) as usize], 0xCC);
    }

    #[test]
    fn pop_byte_from_stack() {
        let mut cpu = setup_cpu();
        cpu.regs.s = 0xFC;
        cpu.mem_map.ram.mem[0x100 + 0xFC] = 0xDE;
        let val = cpu.pop_byte();

        assert_eq!(val, 0xDE);
        assert_eq!(cpu.regs.s, 0xFD);
    }

    #[test]
    fn pop_word_from_stack() {
        let mut cpu = setup_cpu();
        cpu.regs.s = 0xFC;
        cpu.mem_map.ram.mem[0x100 + 0xFD] = 0xDE;
        cpu.mem_map.ram.mem[0x100 + 0xFC] = 0xAD;
        let val = cpu.pop_word();

        assert_eq!(val, 0xDEAD);
        assert_eq!(cpu.regs.s, 0xFE);
    }

    #[test]
    fn reset() {
        let mut cpu = setup_cpu();
        cpu.reset();
        assert_eq!(cpu.regs.pc, 0x0000);
    }

    #[test]
    fn set_flag_carry() {
        let mut cpu = setup_cpu();
        cpu.set_flag(F_CARRY, true);
        assert_eq!(cpu.regs.p & 0x01, 1);
    }

    #[test]
    fn set_flag_interrupt() {
        let mut cpu = setup_cpu();
        cpu.set_flag(F_INTERRUPT, true);
        assert_eq!(cpu.regs.p & 0x04, 0x04);
    }

    #[test]
    fn set_flag_decimal() {
        let mut cpu = setup_cpu();
        cpu.set_flag(F_DECIMAL, true);
        assert_eq!(cpu.regs.p & 0x08, 0x08);
    }

    #[test]
    fn set_flag_break() {
        let mut cpu = setup_cpu();
        cpu.set_flag(F_BREAK, true);
        assert_eq!(cpu.regs.p & 0x10, 0x10);
    }

    #[test]
    fn set_flag_overflow() {
        let mut cpu = setup_cpu();
        cpu.set_flag(F_OVERFLOW, true);
        assert_eq!(cpu.regs.p & 0x40, 0x40);
    }

    #[test]
    fn set_flag_negative() {
        let mut cpu = setup_cpu();
        cpu.set_flag(F_NEGATIVE, true);
        assert_eq!(cpu.regs.p & 0x80, 0x80);
    }

    #[test]
    fn load_byte_increment_pc() {
        let mut cpu = setup_cpu();
        let pc = cpu.regs.pc;
        cpu.mem_map.ram.mem[pc as usize] = 0xDD;
        let byte = cpu.load_byte_increment_pc();
        assert_eq!(byte, 0xDD);
        assert_eq!(cpu.regs.pc, pc+1);
    }

    #[test]
    fn load_word_increment_pc() {
        let mut cpu = setup_cpu();
        let pc = cpu.regs.pc;
        cpu.mem_map.ram.mem[pc as usize] = 0xAD;
        cpu.mem_map.ram.mem[(pc+1) as usize] = 0xDE;
        let word = cpu.load_word_increment_pc();
        assert_eq!(word, 0xDEAD);
        assert_eq!(cpu.regs.pc, pc+2);
    }

    #[test]
    fn set_zn_zero() {
        let mut cpu = setup_cpu();
        cpu.set_zn(0);
        assert_eq!(cpu.regs.p & 0x02, 0x02);
        assert_eq!(cpu.regs.p & 0x80, 0);
    }

    #[test]
    fn set_zn_nonzero() {
        let mut cpu = setup_cpu();
        cpu.set_zn(0x40);
        assert_eq!(cpu.regs.p & 0x02, 0);
        assert_eq!(cpu.regs.p & 0x80, 0);
    }

    #[test]
    fn set_zn_negative() {
        let mut cpu = setup_cpu();
        cpu.set_zn(0xA0);
        assert_eq!(cpu.regs.p & 0x02, 0);
        assert_eq!(cpu.regs.p & 0x80, 0x80);
    }

    #[test]
    fn cpu_reset() {
        let mut rom = setup_rom();
        
        let ram = RAM::new();
        let ppu = PPU::new();
        
        rom.prg_rom[0xFFFC & 0x3FFF] = 0xAD;
        rom.prg_rom[(0xFFFC + 1 )& 0x3FFF] = 0xDE;

        let mapper = NROM::new(rom);
        let mut cpu = CPU::new(ppu, ram, Box::new(mapper));
        
        cpu.reset();
        assert_eq!(cpu.regs.pc, 0xDEAD);
    }

    #[test]
    fn compare_greater() {
        let mut rom = setup_rom();

        let mut ram = RAM::new();
        let ppu = PPU::new();
        
        rom.prg_rom[0xFFFC & 0x3FFF] = 0x00;
        ram.mem[0x00] = 0x10;

        let mapper = NROM::new(rom);
        let mut cpu = CPU::new(ppu, ram, Box::new(mapper));
        cpu.regs.a = 0x00;
        
        cpu.reset();
        let a  = cpu.regs.a;
        cpu.compare(a, ImmediateAddressingMode);
        assert_eq!(cpu.get_flag(F_CARRY), false);
        assert_eq!(cpu.get_flag(F_ZERO), false);
        assert_eq!(cpu.get_flag(F_NEGATIVE), true);
    }

    #[test]
    fn compare_less() {
        let mut rom = setup_rom();

        let mut ram = RAM::new();
        let ppu = PPU::new();
        
        rom.prg_rom[0xFFFC & 0x3FFF] = 0x00;
        ram.mem[0x00] = 0x10;

        let mapper = NROM::new(rom);
        let mut cpu = CPU::new(ppu, ram, Box::new(mapper));
        cpu.regs.a = 0x20;
        
        cpu.reset();
        let a  = cpu.regs.a;
        cpu.compare(a, ImmediateAddressingMode);
        assert_eq!(cpu.get_flag(F_CARRY), true);
        assert_eq!(cpu.get_flag(F_ZERO), false);
        assert_eq!(cpu.get_flag(F_NEGATIVE), false);
    }

    #[test]
    fn compare_zero() {
        let mut rom = setup_rom();

        let mut ram = RAM::new();
        let ppu = PPU::new();
        
        rom.prg_rom[0xFFFC & 0x3FFF] = 0x00;
        ram.mem[0x00] = 0x10;

        let mapper = NROM::new(rom);
        let mut cpu = CPU::new(ppu, ram, Box::new(mapper));
        cpu.regs.a = 0x10;
        
        cpu.reset();
        let a  = cpu.regs.a;
        cpu.compare(a, ImmediateAddressingMode);
        assert_eq!(cpu.get_flag(F_CARRY), true);
        assert_eq!(cpu.get_flag(F_ZERO), true);
        assert_eq!(cpu.get_flag(F_NEGATIVE), false);
    }

    #[test]
    fn branch_go() {
        let mut rom = setup_rom();
        let ram = RAM::new();
        let ppu = PPU::new();
        
        rom.prg_rom[0xFF00 & 0x3FFF] = 0x04;

        let mapper = NROM::new(rom);
        let mut cpu = CPU::new(ppu, ram, Box::new(mapper));
        cpu.reset();
        cpu.regs.pc = 0xFF00;

        cpu.branch(true);
        assert_eq!(cpu.regs.pc, 0xFF05);
    }

    #[test]
    fn branch_dont_go() {
        let mut rom = setup_rom();
        let ram = RAM::new();
        let ppu = PPU::new();
        
        rom.prg_rom[0xFF00 & 0x3FFF] = 0x04;

        let mapper = NROM::new(rom);
        let mut cpu = CPU::new(ppu, ram, Box::new(mapper));
        cpu.reset();
        cpu.regs.pc = 0xFF00;

        cpu.branch(false);
        assert_eq!(cpu.regs.pc, 0xFF01);
    }

    #[test]
    fn sta() {
        let mut cpu = setup_cpu();
        cpu.regs.pc = 0x100;
        cpu.mem_map.ram.mem[0x100] = 0xAA;
        cpu.mem_map.ram.mem[0x101] = 0x01;
        cpu.regs.a = 0xDE;
        cpu.sta(AbsoluteAddressingMode);
        assert_eq!(cpu.mem_map.ram.mem[0x01AA], 0xDE);
    }

    #[test]
    fn stx() {
        let mut cpu = setup_cpu();
        cpu.regs.pc = 0x100;
        cpu.mem_map.ram.mem[0x100] = 0xAA;
        cpu.mem_map.ram.mem[0x101] = 0x01;
        cpu.regs.x = 0xDE;
        cpu.stx(AbsoluteAddressingMode);
        assert_eq!(cpu.mem_map.ram.mem[0x01AA], 0xDE);
    }

    #[test]
    fn sty() {
        let mut cpu = setup_cpu();
        cpu.regs.pc = 0x100;
        cpu.mem_map.ram.mem[0x100] = 0xAA;
        cpu.mem_map.ram.mem[0x101] = 0x01;
        cpu.regs.y = 0xDE;
        cpu.sty(AbsoluteAddressingMode);
        assert_eq!(cpu.mem_map.ram.mem[0x01AA], 0xDE);
    }

    #[test]
    fn inc() {
        let mut cpu = setup_cpu();
        cpu.regs.pc = 0x100;
        cpu.mem_map.ram.mem[0xAA] = 0x09;
        cpu.mem_map.ram.mem[0x100] = 0xAA;
        cpu.inc(ZeroPageAddressingMode);
        assert_eq!(cpu.mem_map.ram.mem[0xAA], 0x0A);
    }
}
