use core::memory::Memory;

pub struct Registers {
    a: u8,
    x: u8,
    y: u8,
    pc: u16,
    s: u8,
    p: u8
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            a: 0,
            x: 0,
            y: 0,
            pc: 0,
            s: 0xFD,
            p: 0x34
        }
    }
}

pub struct CPU {
    regs: Registers
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            regs: Registers::new(),
        }
    }
}
