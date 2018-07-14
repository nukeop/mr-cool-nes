extern crate clap;
extern crate config;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;

pub mod init;
pub mod emu_config;
pub mod core;

use core::cpu;
use core::rom;
