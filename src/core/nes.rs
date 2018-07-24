use cpu;
use core::memory;
use core::ppu;

pub struct NES {
    cpu: cpu::CPU,
    ppu: ppu::PPU,
    ram: memory::RAM
}

pub struct NESBuilder {
    cpu: Option<cpu::CPU>,
    ppu: Option<ppu::PPU>,
    ram: Option<memory::RAM>
}

impl NESBuilder {
    pub fn new() -> NESBuilder {
        let ppu = None;
        let cpu = None;
        let ram = None;
        
        NESBuilder {
            cpu: cpu,
            ppu: ppu,
            ram: ram
        }
    }

    pub fn cpu(&mut self, cpu: cpu::CPU) -> &mut NESBuilder {
        self.cpu = Some(cpu);
        self
    }

    pub fn ppu(&mut self, ppu: ppu::PPU) -> &mut NESBuilder {
        self.ppu = Some(ppu);
        self
    }

    pub fn ram(&mut self, ram: memory::RAM) -> &mut NESBuilder {
        self.ram = Some(ram);
        self
    }

    pub fn finalize(self) -> NES {
        NES {
            cpu: self.cpu.unwrap(),
            ppu: self.ppu.unwrap(),
            ram: self.ram.unwrap()
        }
    }
}

