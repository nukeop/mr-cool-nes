use std::collections::HashMap;
use clap::{App, Arg, ArgMatches};
use core;
use core::mapper;
use core::ppu::Pixel;
use emu_config::EmuConfig;
use renderer::{Renderer, RenderingState};
use sdl_renderer::SDLRenderer;
use headless_renderer::HeadlessRenderer;

pub fn read_cl_args<'a>() -> ArgMatches<'a> {
    App::new("mr-cool-nes")
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
        .get_matches()
}

pub fn start<R: Renderer<SDLRenderer>>(rom: core::rom::Rom, config: EmuConfig, rom_path: &String, mut renderer: Box<R>) {
    info!("Initializing the emulator");    
    let mapper = mapper::select_mapper(rom);
    let mut ppu = core::ppu::PPU::new();
    
    let ram = core::memory::RAM::new();
    let cpu = core::cpu::CPU::new(&mut ppu, ram, mapper);

    let mut nes = core::nes::NESBuilder::new()
        .cpu(cpu)
        .finalize();
    
    nes.cpu.reset();

    renderer.start_loop(|r: &mut SDLRenderer| {
        nes.cpu.step();
        nes.cpu.mem_map.ppu.put_pixel(10, 10, Pixel{r: 0xFF, g: 0x00, b: 0x00});
        r.render_screen(&mut nes.cpu.mem_map.ppu);
        
    }, &RenderingState{state: "run"});
}

pub fn start_headless<R: Renderer<HeadlessRenderer>>(rom: core::rom::Rom, config: EmuConfig, rom_path: &String, mut renderer: Box<R>) {

}
