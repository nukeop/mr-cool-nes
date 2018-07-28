extern crate mr_cool_nes;

use mr_cool_nes::core::ppu::PPU;
use mr_cool_nes::core::cpu::CPU;
use mr_cool_nes::core::memory::RAM;
use mr_cool_nes::core::nes::NESBuilder;

fn main() {
    let ppu = PPU::new();
    let ram = RAM::new();
    let cpu = CPU::new(ppu, ram);
    let _nes = NESBuilder::new()
        .ppu(ppu)
        .cpu(cpu)
        .finalize();
}
