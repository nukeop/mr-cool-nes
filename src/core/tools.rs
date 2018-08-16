use std::io;
use std::fs::File;
use std::io::prelude::*;
use core::rom::Rom;

pub fn split_rom(rom: &Rom, path: &String) -> io::Result<()> {
    let mut prg_file_path = path.to_string();
    prg_file_path += "rom.prg.bin";
    let mut prg_file = File::create(prg_file_path)?;
    prg_file.write_all(&rom.prg_rom)?;

    let mut chr_file_path = path.to_string();
    chr_file_path += "rom.chr.bin";
    let mut chr_file = File::create(chr_file_path)?;
    chr_file.write_all(&rom.chr_rom)?;

    Ok(())
}
