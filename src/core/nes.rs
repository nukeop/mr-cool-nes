use cpu;
use core::memory;
use core::ppu;

pub struct NES {
    cpu: cpu::CPU,
    ppu: ppu::PPU,
    ram: memory::RAM
}

pub struct NESBuilder {
    cpu: cpu::CPU,
    ppu: ppu::PPU,
    ram: memory::RAM
}

impl NESBuilder {
    pub fn new() -> NESBuilder {
        let ppu = ppu::PPU::new();
        let cpu = cpu::CPU::new(ppu);
        let ram = memory::RAM::new();
        
        NESBuilder {
            cpu: cpu,
            ppu: ppu,
            ram: ram
        }
    }

    pub fn cpu(&mut self, cpu: cpu::CPU) -> &mut NESBuilder {
        self.cpu = cpu;
        self
    }

    pub fn ppu(&mut self, ppu: ppu::PPU) -> &mut NESBuilder {
        self.ppu = ppu;
        self
    }

    pub fn ram(&mut self, ram: memory::RAM) -> &mut NESBuilder {
        self.ram = ram;
        self
    }

    pub fn finalize(self) -> NES {
        NES {
            cpu: self.cpu,
            ppu: self.ppu,
            ram: self.ram
        }
    }
}

