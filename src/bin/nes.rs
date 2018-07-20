#[macro_use]
extern crate log;
extern crate pretty_env_logger;

extern crate mr_cool_nes;

use mr_cool_nes::init::read_cl_args;
use mr_cool_nes::emu_config::EmuConfig;
use mr_cool_nes::core::nes;
use mr_cool_nes::core::rom;
use mr_cool_nes::renderer;

fn main() {
    pretty_env_logger::init();
    info!("Mr. Cool NES starting up...");
    
    let args = read_cl_args();
    let config_path = args.get("config").unwrap();
    let rom_path = args.get("rom").unwrap();

    info!("Loading a config file from: {}", config_path);
    let config = EmuConfig::from_path(config_path);
    
    info!("Loading a ROM from: {}", rom_path);
    let rom = rom::Rom::load(rom_path).unwrap();

    let nes = nes::NES::new();

    let mut renderer = renderer::Renderer::new(config, rom_path);
    renderer.start_loop();
}
