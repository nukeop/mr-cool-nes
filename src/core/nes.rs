use cpu;
use core::memory;
use core::ppu;

pub struct NES {
    cpu: cpu::CPU,
    ppu: ppu::PPU,
    ram: memory::RAM
}

pub struct NESBuilder {

}

impl NESBuilder {
    pub fn new() -> NESBuilder {
        NESBuilder {

        }
    }

    // pub fn finalize(&self) -> NES {
    //     NES {}
    // }
}

