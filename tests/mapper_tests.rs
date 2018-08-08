extern crate mr_cool_nes;

#[cfg(test)]
mod mapper_tests {
    use mr_cool_nes::core::mapper::{select_mapper};
    use mr_cool_nes::core::rom::{INesHeader, Rom};
    
    fn setup_rom() -> Rom {
        Rom {
            header: INesHeader {
                magic: ['N' as u8, 'E' as u8, 'S' as u8, '\x1a' as u8],
                prg_rom_size: 1,
                chr_rom_size: 1,
                flags_6: 0,
                flags_7: 0,
                prg_ram_size: 1,
                flags_9: 0,
                flags_10: 0,
                zero: [0; 5]
            },
            prg_rom: vec![0; 16384],
            chr_rom: vec![0; 16384]
        }
    }

    #[test]
    fn select_mapper_nrom() {
        let rom = setup_rom();
        let mapper = select_mapper(rom);
        assert_eq!(mapper.type_of(), "NROM");
    }

    #[test]
    fn select_mapper_sxrom() {
        let mut rom = setup_rom();
        rom.header.flags_6 = 0x10;
        let mapper = select_mapper(rom);
        assert_eq!(mapper.type_of(), "SxROM");
    }

    #[test]
    fn nrom_load_prg_byte() {
        let mut rom = setup_rom();
        rom.prg_rom[0xDE] = 0xAD;
        let mapper = select_mapper(rom);
        let byte = mapper.load_prg_byte(0x80DE);
        assert_eq!(byte, 0xAD);
    }
    
    #[test]
    fn nrom_load_chr_byte() {
        let mut rom = setup_rom();
        rom.chr_rom[0xDE] = 0xAD;
        let mapper = select_mapper(rom);
        let byte = mapper.load_chr_byte(0xDE);
        assert_eq!(byte, 0xAD);
    }
}
