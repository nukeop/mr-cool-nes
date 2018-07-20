use cpu;
use core::memory;

pub struct NES {
    cpu: cpu::CPU,
    ram: memory::RAM
}

impl NES {
    pub fn new() -> NES {
        NES {
            cpu: cpu::CPU::new(),
            ram: memory::RAM { mem: [ 0; 0x800 ] }
        }
    }
}
