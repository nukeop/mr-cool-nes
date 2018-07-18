extern crate clap;
extern crate config;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;
extern crate sdl2;


pub mod core;
pub mod emu_config;
pub mod font_map;
pub mod init;
pub mod renderer;

use core::cpu;
use core::rom;
