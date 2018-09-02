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

pub struct CPU<'a> {
    pub regs: Registers,
    pub mem_map: CPUMemoryMap<'a>
}

impl<'a> CPU<'a> {
    pub fn new(ppu: &mut PPU, ram: RAM, mapper: Box<Mapper>) -> CPU {
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

    pub fn absolute_addressing_mode(&mut self) -> MemoryAddressingMode {
        MemoryAddressingMode {val: self.load_word_increment_pc() as u16}
    }

    pub fn absolute_x_addressing_mode(&mut self) -> MemoryAddressingMode {
        MemoryAddressingMode {val: self.load_word_increment_pc() + self.regs.x as u16}
    }

    pub fn absolute_y_addressing_mode(&mut self) -> MemoryAddressingMode {
        MemoryAddressingMode {val: self.load_word_increment_pc() + self.regs.y as u16}
    }

    pub fn indexed_indirect_addressing_mode(&mut self) -> MemoryAddressingMode {
        let addr = self.load_byte_increment_pc();
        let x = self.regs.x;
        let val = self.load_word_zeropage_wraparound(addr + x);
        MemoryAddressingMode {val}
    }

    pub fn indirect_indexed_addressing_mode(&mut self) -> MemoryAddressingMode {
        let addr = self.load_byte_increment_pc();
        let y = self.regs.y;
        let val = self.load_word_zeropage_wraparound(addr) + y as u16;
        MemoryAddressingMode {val}
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
        self.load_byte(addr + 1)
    }

    pub fn pop_word(&mut self) -> u16 {
        let low = self.pop_byte();
        let hi = self.pop_byte();

        ((hi as u16) << 8) | (low as u16)
    }

    pub fn printable_stack(& mut self) -> String {
        let mut stack = "".to_owned();
        for i in self.regs.s..0xFF {
            stack += &format!("{:X} ", self.load_byte(0x100 + i as u16));
        }
        stack
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

            //println!("Opcode: {:X}", next);
            //println!("PC: {:X}\n", self.regs.pc);
            //`println!("Stack: {}\n", self.printable_stack());
        
        self.decode(next);
    }

    pub fn decode(&mut self, opcode: u8) {
        match opcode {
            0x00 => self.brk(),
            0x01 => {let mode = self.indexed_indirect_addressing_mode(); self.ora(mode);},
            0x02 => self.hlt(),
            0x05 => {let mode = self.zero_page_addressing_mode(); self.ora(mode);},
            0x06 => {let mode = self.zero_page_addressing_mode(); self.asl(mode);},
            0x08 => self.php(),
            0x09 => self.ora(ImmediateAddressingMode),
            0x0A => self.asl(AccumulatorAddressingMode),
            0x0B => self.noop(), // Illegal opcode - ANC ImmediateAddressingMode
            0x0D => {let mode = self.absolute_addressing_mode(); self.ora(mode);},
            0x10 => self.bpl(),
            0x11 => {let mode = self.indirect_indexed_addressing_mode(); self.ora(mode);},
            0x15 => {let mode = self.zero_page_x_addressing_mode(); self.ora(mode);},
            0x17 => self.noop(), // Illegal opcode - SLO
            0x18 => self.clc(),
            0x19 => {let mode = self.absolute_y_addressing_mode(); self.ora(mode);},
            0x1A => self.noop(),
            0x1C => self.noop(),
            0x1D => {let mode = self.absolute_x_addressing_mode(); self.ora(mode);},
            0x20 => self.jsr(),
            0x21 => {let mode = self.indexed_indirect_addressing_mode(); self.and(mode);},
            0x24 => {let mode = self.zero_page_addressing_mode(); self.bit(mode);},
            0x26 => {let mode = self.zero_page_addressing_mode(); self.rol(mode);},
            0x28 => self.plp(),
            0x29 => self.and(ImmediateAddressingMode),
            0x2A => self.rol(AccumulatorAddressingMode),
            0x2C => {let mode = self.absolute_addressing_mode(); self.bit(mode);},
            0x30 => self.bmi(),
            0x32 => self.hlt(),
            0x38 => self.sec(),
            0x3A => self.noop(),
            0x40 => self.rti(),
            0x41 => {let mode = self.indexed_indirect_addressing_mode(); self.eor(mode);},
            0x44 => self.noop(),
            0x45 => {let mode = self.zero_page_addressing_mode(); self.eor(mode);},
            0x46 => {let mode = self.zero_page_addressing_mode(); self.lsr(mode)},
            0x48 => self.pha(),
            0x49 => self.eor(ImmediateAddressingMode),
            0x4A => self.lsr(AccumulatorAddressingMode),
            0x4C => self.jmp(),
            0x4D => {let mode = self.absolute_addressing_mode(); self.eor(mode);},
            0x54 => self.noop(),
            0x58 => self.cli(),
            0x5A => self.noop(),
            0x5C => self.noop(),
            0x5D => {let mode = self.absolute_x_addressing_mode(); self.eor(mode);},
            0x60 => self.rts(),
            0x66 => {let mode = self.zero_page_addressing_mode(); self.ror(mode);},
            0x68 => self.pla(),
            0x69 => self.adc(ImmediateAddressingMode),
            0x6A => self.ror(AccumulatorAddressingMode),
            0x6B => self.noop(), // Illegal opcode - ARR ImmediateAddressingMode
            0x6C => self.jmp_indirect(),
            0x70 => self.bvs(),
            0x78 => self.sei(),
            0x7A => self.noop(),
            0x7C => self.noop(),
            0x80 => self.noop(),
            0x81 => {let mode = self.indexed_indirect_addressing_mode(); self.sta(mode);},
            0x84 => {let mode = self.zero_page_addressing_mode(); self.sty(mode);},
            0x85 => {let mode = self.zero_page_addressing_mode(); self.sta(mode);},
            0x86 => {let mode = self.zero_page_addressing_mode(); self.stx(mode);},
            0x88 => self.dey(),
            0x8A => self.txa(),
            0x8C => {let mode = self.absolute_addressing_mode(); self.sty(mode);},
            0x8D => {let mode = self.absolute_addressing_mode(); self.sta(mode);},
            0x8E => {let mode = self.absolute_addressing_mode(); self.stx(mode);},
            0x90 => self.bcc(),
            0x91 => {let mode = self.indirect_indexed_addressing_mode(); self.sta(mode);},
            0x95 => {let mode = self.zero_page_x_addressing_mode(); self.sta(mode);},
            0x98 => self.tya(),
            0x99 => {let mode = self.absolute_y_addressing_mode(); self.sta(mode);},
            0x9A => self.txs(),
            0x9D => {let mode = self.absolute_x_addressing_mode(); self.sta(mode)},
            0xA0 => self.ldy(ImmediateAddressingMode),
            0xA2 => self.ldx(ImmediateAddressingMode),
            0xA4 => {let mode = self.zero_page_addressing_mode(); self.ldy(mode);},
            0xA5 => {let mode = self.zero_page_addressing_mode(); self.lda(mode);},
            0xA6 => {let mode = self.zero_page_addressing_mode(); self.ldx(mode);},
            0xA8 => self.tay(),
            0xA9 => self.lda(ImmediateAddressingMode),
            0xAA => self.tax(),
            0xAC => {let mode = self.absolute_addressing_mode(); self.ldy(mode);},
            0xAD => {let mode = self.absolute_addressing_mode(); self.lda(mode);},
            0xAE => {let mode = self.absolute_addressing_mode(); self.ldx(mode);},
            0xB0 => self.bcs(),
            0xB1 => {let mode = self.indirect_indexed_addressing_mode(); self.lda(mode);},
            0xB5 => {let mode = self.zero_page_x_addressing_mode(); self.lda(mode);},
            0xB8 => self.clv(),
            0xB9 => {let mode = self.absolute_y_addressing_mode(); self.lda(mode);},
            0xBA => self.tsx(),
            0xBD => {let mode = self.absolute_x_addressing_mode(); self.lda(mode);},
            0xC0 => self.cpy(ImmediateAddressingMode),
            0xC4 => {let mode = self.zero_page_addressing_mode(); self.cpy(mode);},
            0xCA => self.dex(),
            0xC8 => self.iny(),
            0xC9 => self.cmp(ImmediateAddressingMode),
            0xD0 => self.bne(),
            0xD8 => self.cld(),
            0xD9 => {let mode = self.absolute_y_addressing_mode(); self.cmp(mode);},
            0xDA => self.noop(),
            0xDD => {let mode = self.absolute_x_addressing_mode(); self.cmp(mode);},
            0xDE => {let mode = self.absolute_x_addressing_mode(); self.dec(mode);},
            0xE0 => self.cpx(ImmediateAddressingMode),
            0xE3 => self.noop(), // Illegal opcode
            0xE6 => {let mode = self.zero_page_addressing_mode(); self.inc(mode);},
            0xE8 => self.inx(),
            0xE9 => self.sbc(ImmediateAddressingMode),
            0xEA => self.noop(),
            0xEB => self.sbc(ImmediateAddressingMode),
            0xED => {let mode = self.absolute_addressing_mode(); self.sbc(mode);},
            0xF0 => self.beq(),
            0xF1 => {let mode = self.indirect_indexed_addressing_mode(); self.sbc(mode);},
            0xF8 => self.sed(),
            0xF9 => {let mode = self.absolute_y_addressing_mode(); self.sbc(mode);},
            0xFA => self.noop(),
            0xFB => self.noop(), // Illegal opcode - RMW AbsoluteYAddressingMode
            0xFC => self.noop(), // Illegal opcode
            0xFE => {let mode = self.absolute_x_addressing_mode(); self.inc(mode);},
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

    fn sec(&mut self) {
        self.set_flag(F_CARRY, true);
    }

    fn sed(&mut self) {
        self.set_flag(F_DECIMAL, true);
    }

    fn cld(&mut self) {
        self.set_flag(F_DECIMAL, false);
    }

    fn cli(&mut self) {
        self.set_flag(F_INTERRUPT, false);
    }

    fn clc(&mut self) {
        self.set_flag(F_CARRY, false);
    }

    fn clv(&mut self) {
        self.set_flag(F_OVERFLOW, false);
    }

    fn eor<M: AddressingMode>(&mut self, mode: M) {
        let result = mode.load(self) ^ self.regs.a;
        self.regs.a = self.set_zn(result);
    }

    fn jmp(&mut self) {
        let operand = self.load_word_increment_pc();
        self.regs.pc = operand;
    }

    fn jmp_indirect(&mut self) {
        let addr = self.load_word_increment_pc();
        let destination = self.load_word(addr);
        self.regs.pc = destination;
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

    fn tya(&mut self) {
        let y = self.regs.y;
        self.regs.a = self.set_zn(y);
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

    fn tay(&mut self) {
        let a = self.regs.a;
        self.regs.y = self.set_zn(a);
    }
    
    fn tsx(&mut self) {
        let s = self.regs.s;
        self.regs.x = self.set_zn(s);
    }


    fn inx(&mut self) {
        let x = self.regs.x;
        self.regs.x = self.set_zn(x.wrapping_add(1));
    }

    fn iny(&mut self) {
        let y = self.regs.y;
        self.regs.y = self.set_zn(y.wrapping_add(1));
    }

    fn dex(&mut self) {
        let x = self.regs.x;
        self.regs.x = self.set_zn(x.wrapping_sub(1));
    }

    fn dey(&mut self) {
        let y = self.regs.y;
        self.regs.y = self.set_zn(y.wrapping_sub(1));
    }

    fn dec<M: AddressingMode>(&mut self, mode: M) {
        let mut val = mode.load(self);
        val = self.set_zn(val.wrapping_sub(1));
        mode.store(self, val);
    }
    
    fn bne(&mut self) {
        let flag = self.get_flag(F_ZERO);
        self.branch(!flag);
    }

    fn beq(&mut self) {
        let flag = self.get_flag(F_ZERO);
        self.branch(flag);
    }

    fn bmi(&mut self) {
        let flag = self.get_flag(F_NEGATIVE);
        self.branch(flag);
    }

    fn bcs(&mut self) {
        let flag = self.get_flag(F_CARRY);
        self.branch(flag);
    }

    fn bvs(&mut self) {
        let flag = self.get_flag(F_OVERFLOW);
        self.branch(flag);
    }
    
    fn bcc(&mut self) {
        let flag = self.get_flag(F_CARRY);
        self.branch(!flag);
    }

    fn bpl(&mut self) {
        let flag = self.get_flag(F_NEGATIVE);
        self.branch(!flag);
    }

    fn cmp<M: AddressingMode>(&mut self, mode: M) {
        let a = self.regs.a;
        self.compare(a, mode);
    }

    fn cpx<M:AddressingMode>(&mut self, mode: M) {
        let x = self.regs.x;
        self.compare(x, mode);
    }
    
    fn cpy<M:AddressingMode>(&mut self, mode: M) {
        let y = self.regs.y;
        self.compare(y, mode);
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
            val |= 1;
        }
        self.set_flag(F_CARRY, new_carry);
        val = self.set_zn(val);
        mode.store(self, val);
    }

    fn ror<M: AddressingMode>(&mut self, mode: M) {
        let mut val = mode.load(self);
        let carry = self.get_flag(F_CARRY);
        self.set_flag(F_CARRY, val & 0x01 != 0);
        val = val >> 1;
        if(carry) {
            val |= 0x80;
        }
        val = self.set_zn(val);
        mode.store(self, val);
    }

    fn lsr<M: AddressingMode>(&mut self, mode: M) {
        let mut val = mode.load(self);
        self.set_flag(F_CARRY, val & 0x01 != 0);
        val = val >> 1;
        val = self.set_zn(val);
        mode.store(self, val);
    }

    pub fn inc<M: AddressingMode>(&mut self, mode: M) {
        let mut val = mode.load(self);
        val = self.set_zn(val.wrapping_add(1));
        mode.store(self, val);
    }

    fn pla(&mut self) {
        let val = self.pop_byte();
        self.regs.a = self.set_zn(val);
    }

    fn plp(&mut self) {
        let val = self.pop_byte();
        self.regs.p = val;
    }

    fn php(&mut self) {
        let flags = self.regs.p;
        self.push_byte(flags);
    }
    
    fn asl<M: AddressingMode>(&mut self, mode: M) {
        let mut val = mode.load(self);
        self.set_flag(F_CARRY, 0x80 & val != 0);
        val = val << 1;
        val = self.set_zn(val);
        mode.store(self, val);
    }

    fn adc<M: AddressingMode>(&mut self, mode: M) {
        let val = mode.load(self);
        let mut result = val as u32;
        result += self.regs.a as u32;
        if self.get_flag(F_CARRY) {
            result += 1;
        }
        self.set_flag(F_CARRY, (result & 0x100) != 0);
        let result = result as u8;
        let a = self.regs.a;
        self.set_flag(F_OVERFLOW, (a^val) & 0x80 == 0 && (a^result) & 0x80 == 0);
        self.regs.a = self.set_zn(result);
    }
}

impl<'a> Memory for CPU<'a> {
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
