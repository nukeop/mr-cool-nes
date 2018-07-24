use cpu;
use core::memory;
use core::ppu;

pub struct NES {
    cpu: cpu::CPU,
    ppu: ppu::PPU
}

pub struct NESBuilder {
    cpu: Option<cpu::CPU>,
    ppu: Option<ppu::PPU>
}

impl NESBuilder {
    pub fn new() -> NESBuilder {
        let ppu = None;
        let cpu = None;
        
        NESBuilder {
            cpu: cpu,
            ppu: ppu
        }
    }

    pub fn cpu(mut self, cpu: cpu::CPU) -> NESBuilder {
        self.cpu = Some(cpu);
        self
    }

    pub fn ppu(mut self, ppu: ppu::PPU) -> NESBuilder {
        self.ppu = Some(ppu);
        self
    }

    pub fn finalize(self) -> NES {
        info!("Creating a NES...");
        NES {
            cpu: self.cpu.unwrap(),
            ppu: self.ppu.unwrap()
        }
    }
}

