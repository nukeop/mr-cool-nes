extern crate mr_cool_nes;
use mr_cool_nes::core::rom;

fn main() {
    let _rom = rom::Rom::load(&"tests/roms/cpu_dummy_reads/cpu_dummy_reads.nes".to_owned()).unwrap();
}
