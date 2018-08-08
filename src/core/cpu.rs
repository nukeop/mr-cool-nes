use std::fmt;
use core::mapper::Mapper;
use core::memory::{CPUMemoryMap, Memory, RAM};
use core::ppu::PPU;

const NMI_VECTOR: u16 = 0xFFFA;
const RESET_VECTOR: u16 = 0xFFFC;
const BRK_VECTOR: u16 = 0xFFFE;

pub const F_CARRY: u8 = 0x01;
pub const F_ZERO: u8 = 0x02;
pub const F_INTERRUPT: u8 = 0x04;
pub const F_DECIMAL: u8 = 0x08;
pub const F_BREAK: u8 = 0x10;
pub const F_OVERFLOW: u8 = 0x40;
pub const F_NEGATIVE: u8 = 0x80;

trait AddressingMode {
    fn load(&self, cpu: &mut CPU) -> u8;
    fn store(&self, cpu: &mut CPU, val: u8);
}

struct AccumulatorAddressingMode;
impl AddressingMode for AccumulatorAddressingMode {
    fn load(&self, cpu: &mut CPU) -> u8 { cpu.regs.a }
    fn store(&self, cpu: &mut CPU, val: u8) { cpu.regs.a = val; }
}

struct ImmediateAddressingMode;
impl AddressingMode for ImmediateAddressingMode {
    fn load(&self, cpu: &mut CPU) -> u8 { cpu.load_byte_increment_pc() }
    fn store(&self, cpu: &mut CPU, val: u8) { panic!("Attempted write with immediate addressing mode") }
}

struct AbsoluteAddressingMode;
impl AddressingMode for AbsoluteAddressingMode {
    fn load(&self, cpu: &mut CPU) -> u8 {
        let addr = cpu.load_word_increment_pc();
        cpu.load_byte(addr)
    }
    fn store(&self, cpu: &mut CPU, val: u8) {
        let addr = cpu.load_word_increment_pc();
        cpu.store_byte(addr, val);
    }
}

pub struct Registers {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub pc: u16,
    pub s: u8,
    pub p: u8
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

impl fmt::Display for Registers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\na: {:X}\nx: {:X}\ny: {:X}\npc: {:X}\ns: {:X}\np: {:X}",
               self.a, self.x, self.y, self.pc, self.s, self.p)
    }
}

pub struct CPU {
    pub regs: Registers,
    pub mem_map: CPUMemoryMap
}

impl CPU {
    pub fn new(ppu: PPU, ram: RAM, mapper: Box<Mapper>) -> CPU {
        info!("Creating a CPU...");
        CPU {
            regs: Registers::new(),
            mem_map: CPUMemoryMap::new(ppu, ram, mapper)
        }
    }

    fn dma(&mut self, high_byte: u8) {
        let start = (high_byte as u16) << 8;

        for addr in start..start + 256 {
            let val = self.load_byte(addr);
            self.store_byte(0x2004, val);
        }
    }

    pub fn reset(&mut self) {
        self.regs.pc = self.load_word(RESET_VECTOR);
        info!("Regs after reset: {}", self.regs);
    }

    pub fn push_byte(&mut self, val: u8) {
        let s = self.regs.s;
        self.store_byte(0x100 + s as u16, val);
        self.regs.s -= 1;
    }

    pub fn push_word(&mut self, val: u16) {
        let s = self.regs.s;
        self.store_word(0x100 + (s - 1) as u16, val);
        self.regs.s -= 2;
    }

    pub fn load_byte_increment_pc(&mut self) -> u8 {
        let pc = self.regs.pc;
        let val = self.load_byte(pc);
        self.regs.pc += 1;
        val
    }

    pub fn load_word_increment_pc(&mut self) -> u16 {
        let pc = self.regs.pc;
        let val = self.load_word(pc);
        self.regs.pc += 2;
        val
    }

    pub fn step(&mut self) {
        let pc = self.regs.pc;
        let next = self.load_byte(pc);
        self.decode(next);
        
        self.regs.pc += 1;
    }

    pub fn decode(&mut self, opcode: u8) {
        match opcode {
            0x00 => self.brk(),
            0xE3 => self.noop(), // Unofficial opcode
            0x78 => self.sei(),
            0xD8 => self.cld(),
            0xA2 => self.ldx(ImmediateAddressingMode),
            0x4C => self.jmp(),
            0x4D => self.eor(AbsoluteAddressingMode),
            _ => panic!("Unimplemented opcode: {:X}\nRegisters on crash: {}", opcode, self.regs)
        };
    }

    pub fn set_flag(&mut self, flag: u8, state: bool) {
        if state {
            self.regs.p |= flag;
        } else {
            self.regs.p &= !flag;
        }
    }

    pub fn set_zn(&mut self, val: u8) -> u8 {
        self.set_flag(F_ZERO, val == 0);
        self.set_flag(F_NEGATIVE, (val & 0x80) != 0);
        val
    }

    fn noop(&self) {}

    fn brk(&mut self) {
        self.regs.pc += 2;
        let pc = self.regs.pc;
        let p = self.regs.p;
        self.push_word(pc);
        self.push_byte(p);
        self.set_flag(F_INTERRUPT, true);
        self.regs.pc = self.load_word(BRK_VECTOR);
    }

    fn sei(&mut self) {
        self.set_flag(F_INTERRUPT, true);
    }

    fn cld(&mut self) {
        self.set_flag(F_DECIMAL, false);
    }

    fn ldx<M: AddressingMode>(&mut self, mode: M) {
        let val = mode.load(self);
        self.regs.x = self.set_zn(val);
    }

    fn eor<M: AddressingMode>(&mut self, mode: M) {
        let result = mode.load(self) ^ self.regs.a;
        self.regs.a = self.set_zn(result);
    }

    fn jmp(&mut self) {
        let addr = self.load_word_increment_pc();
        let operand = self.load_word(addr);
        self.regs.pc = operand;
    }
}

impl Memory for CPU {
    fn load_byte(&mut self, addr: u16) -> u8 {
        self.mem_map.load_byte(addr)
    }

    fn store_byte(&mut self, addr: u16, val: u8) {
        if addr == 0x4014 {
            self.dma(val);
        } else {
            self.mem_map.store_byte(addr, val);
        }
    }
}
