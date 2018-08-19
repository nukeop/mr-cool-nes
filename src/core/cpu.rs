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

pub trait AddressingMode {
    fn load(&self, cpu: &mut CPU) -> u8;
    fn store(&self, cpu: &mut CPU, val: u8);
}

pub struct AccumulatorAddressingMode;
impl AddressingMode for AccumulatorAddressingMode {
    fn load(&self, cpu: &mut CPU) -> u8 { cpu.regs.a }
    fn store(&self, cpu: &mut CPU, val: u8) { cpu.regs.a = val; }
}

pub struct ImmediateAddressingMode;
impl AddressingMode for ImmediateAddressingMode {
    fn load(&self, cpu: &mut CPU) -> u8 { cpu.load_byte_increment_pc() }
    fn store(&self, cpu: &mut CPU, val: u8) { panic!("Attempted write with immediate addressing mode") }
}

pub struct AbsoluteAddressingMode;
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

pub struct AbsoluteYAddressingMode;
impl AddressingMode for AbsoluteYAddressingMode {
    fn load(&self, cpu: &mut CPU) -> u8 {
        let addr = cpu.load_word_increment_pc() + cpu.regs.y as u16;
        cpu.load_byte(addr)
    }
    fn store(&self, cpu: &mut CPU, val: u8) {
        let addr = cpu.load_word_increment_pc() + cpu.regs.y as u16;
        cpu.store_byte(addr, val);
    }
}

pub struct AbsoluteXAddressingMode;
impl AddressingMode for AbsoluteXAddressingMode {
    fn load(&self, cpu: &mut CPU) -> u8 {
        let addr = cpu.load_word_increment_pc() + cpu.regs.x as u16;
        cpu.load_byte(addr)
    }
    fn store(&self, cpu: &mut CPU, val: u8) {
        let addr = cpu.load_word_increment_pc() + cpu.regs.x as u16;
        cpu.store_byte(addr, val);
    }
}

pub struct IndexedIndirectAddressingMode;
impl IndexedIndirectAddressingMode {
    fn addr(&self, cpu: &mut CPU) -> u16 {
        let addr = cpu.load_byte_increment_pc();
        let x = cpu.regs.x;
        cpu.load_word_zeropage_wraparound(addr + x)
    }
}

impl AddressingMode for IndexedIndirectAddressingMode {
    fn load(&self, cpu: &mut CPU) -> u8 {
        let addr = self.addr(cpu);
        cpu.load_byte(addr)
    }

    fn store(&self, cpu: &mut CPU, val: u8) {
        let addr = self.addr(cpu);
        cpu.store_byte(addr, val)
    }
}

pub struct IndirectIndexedAddressingMode;
impl IndirectIndexedAddressingMode {
    fn addr(&self, cpu: &mut CPU) -> u16 {
        let addr = cpu.load_byte_increment_pc();
        let y = cpu.regs.y;
        cpu.load_word_zeropage_wraparound(addr) + y as u16
    }
}

impl AddressingMode for IndirectIndexedAddressingMode {
    fn load(&self, cpu: &mut CPU) -> u8 {
        let addr = self.addr(cpu);
        cpu.load_byte(addr)
    }

    fn store(&self, cpu: &mut CPU, val: u8) {
        let addr = self.addr(cpu);
        cpu.store_byte(addr, val);
    }
}

pub struct MemoryAddressingMode{val: u16}
impl AddressingMode for MemoryAddressingMode {
    fn load(&self, cpu: &mut CPU) -> u8 {
        cpu.load_byte(self.val)
    }

    fn store(&self, cpu: &mut CPU, val: u8) {
        cpu.store_byte(self.val, val);
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
        self.regs.s -= 3;
        self.regs.p |= 0x04;
        info!("Regs after reset: {}", self.regs);
    }

    pub fn zero_page_addressing_mode(&mut self) -> MemoryAddressingMode {
        MemoryAddressingMode{val: self.load_byte_increment_pc() as u16}
    }

    pub fn zero_page_x_addressing_mode(&mut self) -> MemoryAddressingMode {
        MemoryAddressingMode{val: (self.load_byte_increment_pc() +
        self.regs.x) as u16}
    }

    pub fn stack_pointer(&self) -> u16 {
        self.regs.s as u16 + 0x100
    }

    pub fn push_byte(&mut self, val: u8) {
        let addr = self.stack_pointer();
        self.store_byte(addr, val);
        self.regs.s -= 1;
    }

    pub fn push_word(&mut self, val: u16) {
        let hi = (val >> 8) as u8;
        self.push_byte(hi);

        let low = val as u8;
        self.push_byte(low);
    }

    pub fn pop_byte(&mut self) -> u8 {
        let addr = self.stack_pointer();
        self.regs.s += 1;
        self.load_byte(addr+1)
    }

    pub fn pop_word(&mut self) -> u16 {
        let low = self.pop_byte();
        let hi = self.pop_byte();

        ((hi as u16) << 8) | (low as u16)
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
        let next = self.load_byte_increment_pc();
        self.decode(next);
    }

    pub fn decode(&mut self, opcode: u8) {
        match opcode {
            0x00 => self.brk(),
            0x01 => self.ora(IndexedIndirectAddressingMode),
            0x02 => self.hlt(),
            0x05 => {let mode = self.zero_page_addressing_mode(); self.ora(mode);},
            0x09 => self.ora(ImmediateAddressingMode),
            0x0B => self.noop(), // Illegal opcode - ANC ImmediateAddressingMode
            0x10 => self.bpl(),
            0x11 => self.ora(IndirectIndexedAddressingMode),
            0x15 => {let mode = self.zero_page_x_addressing_mode(); self.ora(mode);},
            0x17 => self.noop(), // Illegal opcode - SLO
            0x19 => self.ora(AbsoluteYAddressingMode),
            0x1C => self.noop(),
            0x1D => self.ora(AbsoluteXAddressingMode),
            0x20 => self.jsr(),
            0x21 => self.and(IndexedIndirectAddressingMode),
            0x24 => {let mode = self.zero_page_addressing_mode(); self.bit(mode);},
            0x26 => {let mode = self.zero_page_addressing_mode(); self.rol(mode);},
            0x2C => self.bit(AbsoluteAddressingMode),
            0x32 => self.hlt(),
            0x40 => self.rti(),
            0x41 => self.eor(IndexedIndirectAddressingMode),
            0x44 => self.noop(),
            0x45 => {let mode = self.zero_page_addressing_mode(); self.eor(mode);},
            0x48 => self.pha(),
            0x4C => self.jmp(),
            0x4D => self.eor(AbsoluteAddressingMode),
            0x5C => self.noop(),
            0x5D => self.eor(AbsoluteXAddressingMode),
            0x60 => self.rts(),
            0x68 => self.pla(),
            0x78 => self.sei(),
            0x85 => {let mode = self.zero_page_addressing_mode(); self.sta(mode);},
            0x86 => {let mode = self.zero_page_addressing_mode(); self.stx(mode);},
            0x8A => self.txa(),
            0x8C => self.sty(AbsoluteAddressingMode),
            0x8D => self.sta(AbsoluteAddressingMode),
            0x95 => {let mode = self.zero_page_x_addressing_mode(); self.sta(mode);},
            0x9A => self.txs(),
            0x9D => self.sta(AbsoluteXAddressingMode),
            0xA0 => self.ldy(ImmediateAddressingMode),
            0xA2 => self.ldx(ImmediateAddressingMode),
            0xA5 => {let mode = self.zero_page_addressing_mode(); self.lda(mode);},
            0xA6 => {let mode = self.zero_page_addressing_mode(); self.ldx(mode);},
            0xA9 => self.lda(ImmediateAddressingMode),
            0xAA => self.tax(),
            0xAD => self.lda(AbsoluteAddressingMode),
            0xBD => self.lda(AbsoluteXAddressingMode),
            0xC9 => self.cmp(ImmediateAddressingMode),
            0xD0 => self.bne(),
            0xD8 => self.cld(),
            0xDD => self.cmp(AbsoluteXAddressingMode),
            0xE3 => self.noop(), // Illegal opcode
            0xE6 => {let mode = self.zero_page_addressing_mode(); self.inc(mode);},
            0xE8 => self.inx(),
            0xED => self.sbc(AbsoluteAddressingMode),
            0xF0 => self.beq(),
            0xF9 => self.sbc(AbsoluteYAddressingMode),
            0xFB => self.noop(), // Illegal opcode - RMW AbsoluteYAddressingMode
            0xFC => self.noop(), // Illegal opcode
            0xFE => self.inc(AbsoluteXAddressingMode),
            0xFF => self.noop(), // Illegal opcode - ISC {adr} = INC {adr} + SBC {adr}
            _ => panic!("Unimplemented opcode: {:X}\nRegisters on crash: {}", opcode, self.regs)
        };
    }

    pub fn get_flag(&mut self, flag: u8) -> bool {
        (self.regs.p & flag) > 0
    }

    pub fn set_flag(&mut self, flag: u8, state: bool) {
        if state {
            self.regs.p |= flag;
        } else {
            self.regs.p &= !flag;
        }
    }

    pub fn set_flags(&mut self, flags: u8) {
        self.regs.p = (flags | 0x30) - 0x10;
    }

    pub fn set_zn(&mut self, val: u8) -> u8 {
        self.set_flag(F_ZERO, val == 0);
        self.set_flag(F_NEGATIVE, (val & 0x80) != 0);
        val
    }

    pub fn branch(&mut self, condition: bool) {
        let offset = self.load_byte_increment_pc() as i8;
        if(condition) {
            self.regs.pc = (self.regs.pc as i32 + offset as i32) as u16;
        }
    }

    pub fn compare<M: AddressingMode>(&mut self, register: u8, mode: M) {
        let val = mode.load(self);
        let result = (register as i32 - val as i32);
        self.set_flag(F_CARRY, register >= val);
        self.set_zn(result as u8);
    }
    
    fn noop(&self) {}

    fn brk(&mut self) {
        let pc = self.regs.pc;
        let p = self.regs.p;
        self.push_word(pc + 1);
        self.push_byte(p);
        self.set_flag(F_BREAK, true);
        self.regs.pc = self.load_word(BRK_VECTOR);
    }

    fn sei(&mut self) {
        self.set_flag(F_INTERRUPT, true);
    }

    fn cld(&mut self) {
        self.set_flag(F_DECIMAL, false);
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

    fn and<M: AddressingMode>(&mut self, mode: M) {
        let val = mode.load(self) & self.regs.a;
        self.regs.a = self.set_zn(val);
    }

    fn ora<M: AddressingMode>(&mut self, mode: M) {
        let val = mode.load(self) | self.regs.a;
        self.regs.a = self.set_zn(val);
    }

    fn hlt(&mut self) {
        info!("Halt instruction executed, reset required");
    }

    pub fn sta<M: AddressingMode>(&mut self, mode: M) {
        let a = self.regs.a;
        mode.store(self, a);
    }

    pub fn stx<M: AddressingMode>(&mut self, mode: M) {
        let x = self.regs.x;
        mode.store(self, x);
    }

    pub fn sty<M: AddressingMode>(&mut self, mode: M) {
        let y = self.regs.y;
        mode.store(self, y);
    }
    
    fn lda<M: AddressingMode>(&mut self, mode: M) {
        let val = mode.load(self);
        self.regs.a = self.set_zn(val);
    }

    fn ldx<M: AddressingMode>(&mut self, mode: M) {
        let val = mode.load(self);
        self.regs.x = self.set_zn(val);
    }

    fn ldy<M: AddressingMode>(&mut self, mode: M) {
        let val = mode.load(self);
        self.regs.y = self.set_zn(val);
    }

    fn pha(&mut self) {
        let a = self.regs.a;
        self.push_byte(a);
    }

    fn jsr(&mut self) {
        let addr = self.load_word_increment_pc();
        let pc = self.regs.pc;
        self.push_word(pc - 1);
        self.regs.pc = addr;
    }

    fn txs(&mut self) {
        let x = self.regs.x;
        self.regs.s = self.set_zn(x);
    }

    fn txa(&mut self) {
        let x = self.regs.x;
        self.regs.a = self.set_zn(x);
    }

    fn tax(&mut self) {
        let a = self.regs.a;
        self.regs.x = self.set_zn(a);
    }

    fn bpl(&mut self) {
        let flag = self.get_flag(F_NEGATIVE);
        self.branch(!flag);
    }

    fn inx(&mut self) {
        let x = self.regs.x;
        self.regs.x = self.set_zn(x + 1);
    }

    fn bne(&mut self) {
        let flag = self.get_flag(F_ZERO);
        self.branch(!flag);
    }

    fn beq(&mut self) {
        let flag = self.get_flag(F_ZERO);
        self.branch(flag);
    }

    fn cmp<M: AddressingMode>(&mut self, mode: M) {
        let a = self.regs.a;
        self.compare(a, mode);
    }

    fn sbc<M: AddressingMode>(&mut self, mode: M) {
        let val = mode.load(self);
        let a = self.regs.a;

        let temp = a as i16 - val as i16 - (1 - (self.get_flag(F_CARRY) as i16));
        self.set_flag(F_NEGATIVE, (temp >> 7) & 1 != 0);

        // Not worth it to put this long condition into set_flag
        if(((a ^ temp as u8) & 0x80) != 0 &&
           ((a ^ val) & 0x80) != 0) {
            self.set_flag(F_OVERFLOW, true);
        } else {
            self.set_flag(F_OVERFLOW, false);
        }

        self.set_flag(F_CARRY, temp < 0);
        self.regs.a = temp as u8 & 0xff;
        
    }

    fn rti(&mut self) {
        let flags = self.pop_byte();
        self.set_flags(flags);
        self.regs.pc = self.pop_word();
    }

    fn rts(&mut self) {
        let pc = self.pop_word();
        self.regs.pc = pc + 1;
    }

    fn bit<M: AddressingMode>(&mut self, mode: M) {
        let val = mode.load(self);
        let a = self.regs.a;
        self.set_flag(F_ZERO, (val & a) == 0);
        self.set_flag(F_NEGATIVE, (val & 0x80) != 0);
        self.set_flag(F_OVERFLOW, (val & 0x40) != 0);
    }

    fn rol<M: AddressingMode>(&mut self, mode: M) {
        let mut val = mode.load(self);
        let carry = self.get_flag(F_CARRY);
        let new_carry = (val & 0x80) > 0;
        val = val << 1;
        if(carry) {
            val = val | 1;
        }
        self.set_flag(F_CARRY, new_carry);
        val = self.set_zn(val);
        mode.store(self, val);
    }

    pub fn inc<M: AddressingMode>(&mut self, mode: M) {
        let mut val = mode.load(self);
        val = self.set_zn(val + 1);
        mode.store(self, val);
    }

    fn pla(&mut self) {
        let val = self.pop_byte();
        self.regs.a = self.set_zn(val);
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
