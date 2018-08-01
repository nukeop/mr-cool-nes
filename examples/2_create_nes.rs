extern crate mr_cool_nes;

use mr_cool_nes::core::ppu::PPU;
use mr_cool_nes::core::cpu::CPU;
use mr_cool_nes::core::memory::RAM;
use mr_cool_nes::core::rom::Rom;
use mr_cool_nes::core::mapper::select_mapper;

use mr_cool_nes::core::nes::NESBuilder;

fn main() {
    let rom = Rom::load(&"tests/roms/cpu_dummy_reads/cpu_dummy_reads.nes".to_owned()).unwrap();
    let mapper = select_mapper(rom);
    let ppu = PPU::new();
    let ram = RAM::new();
    let cpu = CPU::new(ppu, ram, mapper);
    let _nes = NESBuilder::new()
        .ppu(ppu)
        .cpu(cpu)
        .finalize();
}
