#[macro_use]
extern crate log;
extern crate pretty_env_logger;

extern crate mr_cool_nes;

use mr_cool_nes::init::{read_cl_args, start};
use mr_cool_nes::emu_config::EmuConfig;
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

    start(rom, config, &rom_path, headless);
}
