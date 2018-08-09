extern crate mr_cool_nes;

#[cfg(test)]
mod memory_tests {
    use mr_cool_nes::core::memory;
    use mr_cool_nes::core::memory::Memory;

    fn setup_memory() -> memory::RAM {
        return memory::RAM { mem: [ 0; 0x800 ] };
    }
    
    #[test]
    fn store_byte_ram() {
        let mut ram = setup_memory();
        ram.store_byte(0x0000, 0x22);
        assert_eq!(ram.mem[0], 0x22);
    }

    #[test]
    fn store_byte_ram_out_of_bounds() {
        let mut ram = setup_memory();
        ram.store_byte(0x0800, 0xde);
        assert_eq!(ram.mem[0x0800 & 0x7ff], 0xde);
    }

    #[test]
    fn load_byte_ram() {
        let mut ram = setup_memory();
        ram.mem[0x0220] = 0x33;
        let b = ram.load_byte(0x0220);
        assert_eq!(b, 0x33);
    }

    #[test]
    fn load_byte_ram_out_of_bounds() {
        let mut ram = setup_memory();
        ram.mem[0x24F] = 0x91;
        let b = ram.load_byte(0x0A4F);
        assert_eq!(b, 0x91);
    }

    #[test]
    fn load_word_ram() {
        let mut ram = setup_memory();
        ram.mem[0x0200] = 0xAD;
        ram.mem[0x0201] = 0xDE;
        let w = ram.load_word(0x0200);
        assert_eq!(w, 0xDEAD);
    }

    #[test]
    fn store_word_ram() {
        let mut ram = setup_memory();
        ram.store_word(0x0200, 0xDEAD);
        assert_eq!(ram.mem[0x0200], 0xAD);
        assert_eq!(ram.mem[0x0201], 0xDE);
    }

    #[test]
    fn load_word_zeropage_no_wraparound() {
        let mut ram = setup_memory();
        ram.mem[0x10] = 0xAD;
        ram.mem[0x11] = 0xDE;
        let w = ram.load_word_zeropage_wraparound(0x10);
        assert_eq!(w, 0xDEAD);
    }

    #[test]
    fn load_word_zeropage_wraparound() {
        let mut ram = setup_memory();
        ram.mem[0xFF] = 0xAD;
        ram.mem[0x00] = 0xDE;
        
        let w = ram.load_word_zeropage_wraparound(0xFF);
        assert_eq!(w, 0xDEAD);
    }
}
