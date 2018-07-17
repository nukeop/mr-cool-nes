use std::io;
use std::io::prelude::*;
use std::fs::File;

#[derive(Debug)]
pub struct INesHeader {
    magic: [u8; 4],
    prg_rom_size: u8,
    chr_rom_size: u8,
    flags_6: u8,
    flags_7: u8,
    prg_ram_size: u8,
    flags_9: u8,
    flags_10: u8,
    zero: [u8; 5]
}

#[derive(Debug)]
pub struct Rom {
    header: INesHeader,
    prg_rom: Vec<u8>,
    chr_rom: Vec<u8>
}

impl Rom {
    pub fn load(path: &String) -> Result<Rom, io::Error> {
        let mut f = File::open(path).expect("Rom file not found");
        let mut header_buffer = [0;16];
        f.read_exact(&mut header_buffer).unwrap();
        let header = INesHeader {
            magic: [
                header_buffer[0],
                header_buffer[1],
                header_buffer[2],
                header_buffer[3]
            ],
            prg_rom_size: header_buffer[4],
            chr_rom_size: header_buffer[5],
            flags_6: header_buffer[6],
            flags_7: header_buffer[7],
            prg_ram_size: header_buffer[8],
            flags_9: header_buffer[9],
            flags_10: header_buffer[10],
            zero: [0;5]
        };

        if header.magic != *b"NES\x1a" {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Loaded file is not a NES rom."));
        }

        let prg_size = header.prg_rom_size as usize * 16384;
        let mut prg_rom = vec![0; prg_size];
        f.read_exact(&mut prg_rom).unwrap();
        
        
        let chr_size = header.chr_rom_size as usize * 8192;
        let mut chr_rom = vec![0; chr_size];
        f.read_exact(&mut chr_rom)?;
        
        Ok(Rom {
            header: header,
            prg_rom: prg_rom,
            chr_rom: chr_rom
        })

    }
}

