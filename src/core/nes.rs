use core::cpu;
use core::memory;
use core::ppu;

pub struct NES<'a> {
    pub cpu: cpu::CPU<'a>
}

pub struct NESBuilder<'a> {
    cpu: Option<cpu::CPU<'a>>
}

impl<'a> NESBuilder<'a> {
    pub fn new() -> NESBuilder<'a> {
        let cpu = None;
        
        NESBuilder {
            cpu: cpu
        }
    }

    pub fn cpu(mut self, cpu: cpu::CPU<'a>) -> NESBuilder<'a> {
        self.cpu = Some(cpu);
        self
    }

    pub fn finalize(self) -> NES<'a> {
        info!("Creating a NES...");
        NES {
            cpu: self.cpu.unwrap()
        }
    }
}

