use core::memory::{CPUMemoryMap, Memory, RAM};
use core::ppu::PPU;

trait AddressingMode {
    fn load(&self, cpu: &mut CPU) -> u8;
    fn store(&self, cpu: &mut CPU, val: u8);
}

struct AccumulatorAddressingMode;
impl AddressingMode for AccumulatorAddressingMode {
    fn load(&self, cpu: &mut CPU) -> u8 { cpu.regs.a }
    fn store(&self, cpu: &mut CPU, val: u8) { cpu.regs.a = val; }
}

struct ImmediateAddressingMode;
impl AddressingMode for ImmediateAddressingMode {
    fn load(&self, cpu: &mut CPU) -> u8 { 0x00 }
    fn store(&self, cpu: &mut CPU, val: u8) {}
}

pub struct Registers {
    a: u8,
    x: u8,
    y: u8,
    pc: u16,
    s: u8,
    p: u8
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            a: 0,
            x: 0,
            y: 0,
            pc: 0,
            s: 0xFD,
            p: 0x34
        }
    }
}

pub struct CPU {
    pub regs: Registers,
    pub mem_map: CPUMemoryMap
}

impl CPU {
    pub fn new(ppu: PPU, ram: RAM) -> CPU {
        info!("Creating a CPU...");
        CPU {
            regs: Registers::new(),
            mem_map: CPUMemoryMap::new(ppu, ram)
        }
    }

    fn dma(&mut self, high_byte: u8) {
        let start = (high_byte as u16) << 8;

        for addr in start..start + 256 {
            let val = self.load_byte(addr);
            self.store_byte(0x2004, val);
        }
    }
}

impl Memory for CPU {
    fn load_byte(&mut self, addr: u16) -> u8 {
        self.mem_map.load_byte(addr)
    }

    fn store_byte(&mut self, addr: u16, val: u8) {
        if addr == 0x4014 {
            self.dma(val);
        } else {
            self.mem_map.store_byte(addr, val);
        }
    }
}
