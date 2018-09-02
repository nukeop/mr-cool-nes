#[macro_use]
extern crate log;
extern crate pretty_env_logger;

extern crate mr_cool_nes;

use mr_cool_nes::init::{read_cl_args, start, start_headless};
use mr_cool_nes::emu_config::EmuConfig;
use mr_cool_nes::renderer::Renderer;
use mr_cool_nes::sdl_renderer::SDLRenderer;
use mr_cool_nes::headless_renderer::HeadlessRenderer;
use mr_cool_nes::core::rom;

fn main() {
    pretty_env_logger::init();
    info!("Mr. Cool NES starting up...");
    
    let args = read_cl_args();
    let config_path = args.value_of("config").unwrap_or(".mrcoolnes").to_owned();
    let rom_path = args.value_of("rom").unwrap_or("rom.nes").to_owned();
    let headless = args.is_present("headless");

    info!("Loading a config file from: {}", config_path);
    let config = EmuConfig::from_path(&config_path);
    
    info!("Loading a ROM from: {}", rom_path);
    let rom = rom::Rom::load(&rom_path).unwrap();

    if(headless) {
        let headless_renderer = Box::new(HeadlessRenderer::new(&rom_path));
        start_headless(rom, config, &rom_path, headless_renderer)
    } else {
        let sdl_renderer = Box::new(SDLRenderer::new(&config, &rom_path));
        start(rom, config, &rom_path, sdl_renderer);       
    }
}
