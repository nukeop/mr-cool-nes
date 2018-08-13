use std::collections::HashMap;
use clap::{App, Arg, ArgMatches};
use core;
use core::mapper;
use emu_config::EmuConfig;
use renderer::Renderer;

pub fn read_cl_args<'a>() -> ArgMatches<'a> {
    let matches = App::new("mr-cool-nes")
        .version("0.1.0")
        .about("nes emulator")
        .author("nukeop <nukeop@gumblert.tech>")
        .arg(Arg::with_name("rom")
             .short("r")
             .long("rom")
             .value_name("FILE")
             .help("Rom to load and run")
             .required(true))
        .arg(Arg::with_name("config")
             .short("c")
             .long("config")
             .value_name("FILE")
             .help("Optional custom config file"))
        .arg(Arg::with_name("headless")
             .short("h")
             .long("headless")
             .help("Run without graphics"))
        .get_matches();

    let rom = matches.value_of("rom").unwrap_or("rom.nes").to_owned();
    let config = matches.value_of("config").unwrap_or(".mrcoolnes").to_owned();
    matches
}

pub fn start<R: Renderer>(rom: core::rom::Rom, config: EmuConfig, rom_path: &String, mut renderer: Box<R>) {
    info!("Initializing the emulator");    
    let mapper = mapper::select_mapper(rom);
    let ppu = core::ppu::PPU::new();
    let ram = core::memory::RAM::new();
    let cpu = core::cpu::CPU::new(ppu, ram, mapper);

    let mut nes = core::nes::NESBuilder::new()
        .ppu(ppu)
        .cpu(cpu)
        .finalize();
    
    nes.cpu.reset();
    
    renderer.start_loop(|| nes.cpu.step());
}
