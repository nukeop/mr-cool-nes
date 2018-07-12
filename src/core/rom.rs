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

