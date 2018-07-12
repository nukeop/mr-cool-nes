use std::io;
use std::io::prelude::*;
use std::fs::File;

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


pub struct Rom {
    header: INesHeader,
    prg_rom: Vec<u8>,
    chr_rom: Vec<u8>
}

impl Rom {
    fn load(path: String) -> Rom {
        let mut f = File::open(path).expect("Rom file not found");
        let mut header_buffer = [0;16];
        f.read_exact(&mut header_buffer);
        let mut header = INesHeader {
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

        Rom {
            header: header,
            prg_rom: vec![0u8],
            chr_rom: vec![0u8]
        }

    }
}

