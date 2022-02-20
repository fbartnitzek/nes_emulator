use crate::opcodes;
use std::collections::HashMap;
use std::ops::{BitAnd, BitOr, BitXor};
use crate::cpu::{AddressingMode, CpuFlags};

const STACK: u16 = 0x0100;
const STACK_RESET: u8 = 0xFF;

pub struct CPU {
  pub register_a: u8,
  pub register_x: u8,
  pub register_y: u8,
  pub status: CpuFlags,
  pub program_counter: u16,
  pub stack_pointer: u8,
  memory: [u8; 0xFFFF],
}

pub trait Mem {
  fn mem_read(&self, addr: u16) -> u8;

  fn mem_write(&mut self, addr: u16, data: u8);

  fn mem_read_u16(&self, pos: u16) -> u16 {
    let lo = self.mem_read(pos) as u16;
    let hi = self.mem_read(pos + 1) as u16;
    hi << 8 | lo
  }

  fn mem_write_u16(&mut self, pos: u16, data: u16) {
    let hi = (data >> 8) as u8;
    let lo = (data & 0xff) as u8;
    self.mem_write(pos, lo);
    self.mem_write(pos + 1, hi);
  }
}

impl Mem for CPU {
  fn mem_read(&self, addr: u16) -> u8 {
    self.memory[addr as usize]
  }

  fn mem_write(&mut self, addr: u16, data: u8) {
    self.memory[addr as usize] = data;
  }
}

impl CPU {
  pub fn new() -> Self {
    CPU {
      register_a: 0,
      register_x: 0,
      register_y: 0,
      stack_pointer: STACK_RESET,
      program_counter: 0x8000,
      status: CpuFlags::INTERRUPT_DISABLE | CpuFlags::BREAK2,
      memory: [0; 0xFFFF],
    }
  }

  fn get_operand_address(&self, mode: &AddressingMode) -> u16 {
    match mode {
      AddressingMode::Immediate => self.program_counter,

      AddressingMode::ZeroPage => self.mem_read(self.program_counter) as u16,

      AddressingMode::Absolute => self.mem_read_u16(self.program_counter),

      AddressingMode::ZeroPage_X => {
        let pos = self.mem_read(self.program_counter);
        let addr = pos.wrapping_add(self.register_x) as u16;
        addr
      }
      AddressingMode::ZeroPage_Y => {
        let pos = self.mem_read(self.program_counter);
        let addr = pos.wrapping_add(self.register_y) as u16;
        addr
      }

      AddressingMode::Absolute_X => {
        let base = self.mem_read_u16(self.program_counter);
        let addr = base.wrapping_add(self.register_x as u16);
        addr
      }
      AddressingMode::Absolute_Y => {
        let base = self.mem_read_u16(self.program_counter);
        let addr = base.wrapping_add(self.register_y as u16);
        addr
      }

      AddressingMode::Indirect_X => {
        let base = self.mem_read(self.program_counter);

        let ptr: u8 = (base as u8).wrapping_add(self.register_x);
        let lo = self.mem_read(ptr as u16);
        let hi = self.mem_read(ptr.wrapping_add(1) as u16);
        (hi as u16) << 8 | (lo as u16)
      }
      AddressingMode::Indirect_Y => {
        let base = self.mem_read(self.program_counter);

        let lo = self.mem_read(base as u16);
        let hi = self.mem_read((base as u8).wrapping_add(1) as u16);
        let deref_base = (hi as u16) << 8 | (lo as u16);
        let deref = deref_base.wrapping_add(self.register_y as u16);
        deref
      }

      AddressingMode::NoneAddressing => {
        panic!("mode {:?} is not supported", mode);
      }
    }
  }

  // fn set_register_a(&mut self, value: u8) {
  //   self.register_a = value;
  //   self.update_zero_and_negative_flags(self.register_a);
  // }

  fn tax(&mut self) {
    self.register_x = self.register_a;
    self.update_zero_and_negative_flags(self.register_x);
  }

  fn update_zero_and_negative_flags(&mut self, result: u8) {
    if result == 0 {
      self.status.insert(CpuFlags::ZERO);
    } else {
      self.status.remove(CpuFlags::ZERO);
    }

    if result >> 7 == 1 {
      self.status.insert(CpuFlags::NEGATIVE);
    } else {
      self.status.remove(CpuFlags::NEGATIVE);
    }
  }

  pub fn load_and_run(&mut self, program: Vec<u8>) {
    self.load(program);
    self.reset();
    self.run()
  }

  pub fn load(&mut self, program: Vec<u8>) {
    self.memory[0x0600..(0x0600 + program.len())].copy_from_slice(&program[..]);
    self.mem_write_u16(0xFFFC, 0x0600);
  }

  pub fn load_with_address(&mut self, program: Vec<u8>, start_address: u16) {
    self.memory[start_address as usize .. (start_address + program.len() as u16) as usize].copy_from_slice(&program[..]);
    self.mem_write_u16(0xFFFC, start_address);
  }

  pub fn reset(&mut self) {
    self.register_a = 0;
    self.register_x = 0;
    self.register_y = 0;
    self.stack_pointer = STACK_RESET;
    self.status = CpuFlags::INTERRUPT_DISABLE | CpuFlags::BREAK2;

    self.program_counter = self.mem_read_u16(0xFFFC);
  }


  // /// note: ignoring decimal mode
  // /// http://www.righto.com/2012/12/the-6502-overflow-flag-explained.html
  // fn add_to_register_a(&mut self, data: u8) {
  //   let sum = self.register_a as u16
  //     + data as u16
  //     + (if self.status.contains(CpuFlags::CARRY) {
  //     1
  //   } else {
  //     0
  //   }) as u16;
  //
  //   let carry = sum > 0xff;
  //
  //   if carry {
  //     self.status.insert(CpuFlags::CARRY);
  //   } else {
  //     self.status.remove(CpuFlags::CARRY);
  //   }
  //
  //   let result = sum as u8;
  //
  //   if (data ^ result) & (result ^ self.register_a) & 0x80 != 0 {
  //     self.status.insert(CpuFlags::OVERFLOW);
  //   } else {
  //     self.status.remove(CpuFlags::OVERFLOW)
  //   }
  //
  //   self.set_register_a(result);
  // }

  // fn sbc(&mut self, mode: &AddressingMode) {
  //   let addr = self.get_operand_address(&mode);
  //   let data = self.mem_read(addr);
  //   self.add_to_register_a(((data as i8).wrapping_neg().wrapping_sub(1)) as u8);
  // }


  fn stack_pop(&mut self) -> u8 {
    self.stack_pointer = self.stack_pointer.wrapping_add(1);
    self.mem_read((STACK as u16) + self.stack_pointer as u16)
  }

  fn stack_push(&mut self, data: u8) {
    self.mem_write((STACK as u16) + self.stack_pointer as u16, data);
    self.stack_pointer = self.stack_pointer.wrapping_sub(1)
  }

  fn stack_push_u16(&mut self, data: u16) {
    let hi = (data >> 8) as u8;
    let lo = (data & 0xff) as u8;
    self.stack_push(hi);
    self.stack_push(lo);
  }

  fn stack_pop_u16(&mut self) -> u16 {
    let lo = self.stack_pop() as u16;
    let hi = self.stack_pop() as u16;

    hi << 8 | lo
  }


  fn set_flags(&mut self, flags: u8) {
    self.status.set(CpuFlags::CARRY,flags & 0x01 != 0);
    self.status.set(CpuFlags::ZERO,flags & 0x02 != 0);
    self.status.set(CpuFlags::INTERRUPT_DISABLE,flags & 0x04 != 0);
    self.status.set(CpuFlags::DECIMAL_MODE,flags & 0x08 != 0);
    self.status.set(CpuFlags::BREAK,flags & 0x10 != 0);
    self.status.set(CpuFlags::BREAK2,flags & 0x20 != 0);
    self.status.set(CpuFlags::OVERFLOW,flags & 0x40 != 0);
    self.status.set(CpuFlags::NEGATIVE,flags & 0x80 != 0);
  }


  fn branch(&mut self, condition: bool) {
    if condition {
      let jump: i8 = self.mem_read(self.program_counter) as i8;
      let jump_addr = self
        .program_counter
        .wrapping_add(1)
        .wrapping_add(jump as u16);

      self.program_counter = jump_addr;
    }
  }

  pub fn run(&mut self) {
    self.run_with_callback(|_| {});
  }

  pub fn run_with_callback<F>(&mut self, mut callback: F)
    where
      F: FnMut(&mut CPU),
  {
    let ref opcodes: HashMap<u8, &'static opcodes::OpCode> = *opcodes::OPCODES_MAP;

    loop {
      let code = self.mem_read(self.program_counter);
      self.program_counter += 1;
      let program_counter_state = self.program_counter;

      let opcode = opcodes.get(&code).unwrap();

      match code {
        0x00 => return,

        0x69 | 0x65 | 0x75 | 0x6d | 0x7d | 0x79 | 0x61 | 0x71 => self.adc(&opcode.mode),
        0x29 | 0x25 | 0x35 | 0x2d | 0x3d | 0x39 | 0x21 | 0x31 => self.and(&opcode.mode),
        0x0a | 0x06 | 0x16 | 0x0e | 0x1e => self.asl(&opcode.mode),

        0x90 => self.bcc(),
        0xb0 => self.bcs(),
        0xf0 => self.beq(),
        0x30 => self.bmi(),
        0xd0 => self.bne(),
        0x10 => self.bpl(),
        0x50 => self.bvc(),
        0x70 => self.bvs(),

        0x24 | 0x2C => self.bit(&opcode.mode),

        0x18 => self.clc(),
        0xD8 => self.cld(),
        0x58 => self.cli(),
        0xB8 => self.clv(),

        0xC9 | 0xC5 | 0xD5 | 0xCD | 0xDD | 0xD9 | 0xC1 | 0xD1 => self.cmp(&opcode.mode),
        0xE0 | 0xE4 | 0xEC => self.cpx(&opcode.mode),
        0xC0 | 0xC4 | 0xCC => self.cpy(&opcode.mode),

        0xC6 | 0xD6 | 0xCE | 0xDE => self.dec(&opcode.mode),
        0xCA => self.dex(),
        0x88 => self.dey(),

        0x49 | 0x45 | 0x55 | 0x4D | 0x5D | 0x59 | 0x41 | 0x51 => self.eor(&opcode.mode),

        0xE6 | 0xF6 | 0xEE | 0xFE => self.inc(&opcode.mode),
        0xE8 => self.inx(),
        0xC8 => self.iny(),

        0x4C | 0x6c => self.jmp(&opcode.mode),
        0x20 => self.jsr(),

        0xA9 | 0xA5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 | 0xB1 => self.lda(&opcode.mode),
        0xA2 | 0xA6 | 0xB6 | 0xAE | 0xBE => self.ldx(&opcode.mode),
        0xA0 | 0xA4 | 0xB4 | 0xAC | 0xBC => self.ldy(&opcode.mode),

        0x4A | 0x46 | 0x56 | 0x4E | 0x5E => self.lsr(&opcode.mode),

        0xEA => self.nop(),
        0x09 | 0x05 | 0x15 | 0x0D | 0x1D | 0x19 | 0x01 | 0x11 => self.ora(&opcode.mode),

        0x85 | 0x95 | 0x8D | 0x9D | 0x99 | 0x81 | 0x91 => self.sta(&opcode.mode),
        0x86 | 0x96 | 0x8E => self.stx(&opcode.mode),
        0x84 | 0x94 | 0x8C => self.sty(&opcode.mode),

        0x48 => self.pha(),
        0x08 => self.php(),
        0x68 => self.pla(),
        0x28 => self.plp(),

        0x2A | 0x26 | 0x36 | 0x2E | 0x3E => self.rol(&opcode.mode),
        0x6A | 0x66 | 0x76 | 0x6E | 0x7E => self.ror(&opcode.mode),
        0x60 => self.rts(),
        0x40 => self.rti(),

        0xE9 | 0xE5 | 0xF5 | 0xED | 0xFD | 0xF9 | 0xE1 | 0xF1 => self.sbc(&opcode.mode),

        0x38 => self.sec(),



        0xAA => self.tax(),



        /* SEI */ 0x78 => self.status.insert(CpuFlags::INTERRUPT_DISABLE),

        /* SED */ 0xf8 => self.status.insert(CpuFlags::DECIMAL_MODE),

        /* TAY */
        0xa8 => {
          self.register_y = self.register_a;
          self.update_zero_and_negative_flags(self.register_y);
        }

        /* TSX */
        0xba => {
          self.register_x = self.stack_pointer;
          self.update_zero_and_negative_flags(self.register_x);
        }

        /* TXA */
        0x8a => {
          self.register_a = self.register_x;
          self.update_zero_and_negative_flags(self.register_a);
        }

        /* TXS */
        0x9a => {
          self.stack_pointer = self.register_x;
        }

        /* TYA */
        0x98 => {
          self.register_a = self.register_y;
          self.update_zero_and_negative_flags(self.register_a);
        }

        _ => todo!(),
      }

      if program_counter_state == self.program_counter {
        self.program_counter += (opcode.len - 1) as u16;
      }

      callback(self);
    }
  }

  fn adc(&mut self, mode: &AddressingMode) {
    let addr = self.get_operand_address(mode);
    let data = self.mem_read(addr);
    self.add_to_acc(data);
  }

  fn sbc(&mut self, mode: &AddressingMode) {
    let addr = self.get_operand_address(mode);
    let value = self.mem_read(addr);

    // A - B = A + (-B). And -B = !B + 1
    // self.add_to_acc(value.wrapping_neg());
    let inverted_value = ((value as i8).wrapping_neg().wrapping_sub(1)) as u8;
    self.add_to_acc(inverted_value);
  }

  fn add_to_acc(&mut self, data: u8) {
    let carry = if self.status.contains(CpuFlags::CARRY) { 1 } else { 0 };
    let sum = self.register_a as u16 + data as u16 + carry;
    self.status.set(CpuFlags::CARRY, sum > 0xFF);

    let result = sum as u8;
    // some highest bit set...
    self.status.set(CpuFlags::OVERFLOW,
                    (data ^ result) & (result ^ self.register_a) & 0x80 != 0);

    self.register_a = result;
    self.update_zero_and_negative_flags(self.register_a);
  }

  fn and(&mut self, mode: &AddressingMode) {
    let addr = self.get_operand_address(mode);
    let value = self.mem_read(addr);

    self.register_a = value.bitand(self.register_a);
    self.update_zero_and_negative_flags(self.register_a);
  }

  fn asl(&mut self, mode: &AddressingMode) {
    if matches!(mode, AddressingMode::NoneAddressing) {
      self.status.set(CpuFlags::CARRY, self.register_a & 0b1000_0000 != 0);
      self.register_a <<= 1;
      self.update_zero_and_negative_flags(self.register_a);
    } else {
      let addr = self.get_operand_address(mode);
      let mut value = self.mem_read(addr);
      self.status.set(CpuFlags::CARRY, value & 0b1000_0000 != 0);

      value <<= 1;
      self.mem_write(addr, value);
      self.update_zero_and_negative_flags(value);
    }
  }

  fn bcc(&mut self) {
    self.branch(!self.status.contains(CpuFlags::CARRY))
  }

  fn bcs(&mut self) {
    self.branch(self.status.contains(CpuFlags::CARRY))
  }

  fn beq(&mut self) {
    self.branch(self.status.contains(CpuFlags::ZERO))
  }

  fn bmi(&mut self) {
    self.branch(self.status.contains(CpuFlags::NEGATIVE))
  }

  fn bne(&mut self) {
    self.branch(!self.status.contains(CpuFlags::ZERO))
  }

  fn bpl(&mut self) {
    self.branch(!self.status.contains(CpuFlags::NEGATIVE))
  }

  fn bvc(&mut self) {
    self.branch(!self.status.contains(CpuFlags::OVERFLOW))
  }

  fn bvs(&mut self) {
    self.branch(self.status.contains(CpuFlags::OVERFLOW))
  }

  fn bit(&mut self, mode: &AddressingMode) {
    let addr = self.get_operand_address(mode);
    let data = self.mem_read(addr);

    self.status.set(CpuFlags::ZERO, data.bitand(self.register_a) == 0);
    self.status.set(CpuFlags::OVERFLOW, data & 0b0100_0000 != 0);
    self.status.set(CpuFlags::NEGATIVE, data & 0b1000_0000 != 0);
  }

  fn clc(&mut self) {
    self.status.remove(CpuFlags::CARRY)
  }

  fn cld(&mut self) {
    self.status.remove(CpuFlags::DECIMAL_MODE)
  }

  fn cli(&mut self) {
    self.status.remove(CpuFlags::INTERRUPT_DISABLE)
  }

  fn clv(&mut self) {
    self.status.remove(CpuFlags::OVERFLOW)
  }

  fn cmp(&mut self, mode: &AddressingMode) {
    self.compare(mode, self.register_a);
  }

  fn cpx(&mut self, mode: &AddressingMode) {
    self.compare(mode, self.register_x);
  }

  fn cpy(&mut self, mode: &AddressingMode) {
    self.compare(mode, self.register_y);
  }

  fn compare(&mut self, mode: &AddressingMode, reference: u8) {
    let addr = self.get_operand_address(mode);
    let data = self.mem_read(addr);

    // Z,C,N = A-M
    self.status.set(CpuFlags::CARRY, reference >= data);
    self.update_zero_and_negative_flags(reference.wrapping_sub(data))
  }

  fn dec(&mut self, mode: &AddressingMode) {
    let addr = self.get_operand_address(mode);
    let value = self.mem_read(addr);
    let new_value = value.wrapping_sub(1);
    self.mem_write(addr, new_value);
    self.update_zero_and_negative_flags(new_value);
  }

  fn dex(&mut self) {
    self.register_x = self.register_x.wrapping_sub(1);
    self.update_zero_and_negative_flags(self.register_x);
  }

  fn dey(&mut self) {
    self.register_y = self.register_y.wrapping_sub(1);
    self.update_zero_and_negative_flags(self.register_y);
  }

  fn inc(&mut self, mode: &AddressingMode) {
    let addr = self.get_operand_address(mode);
    let value = self.mem_read(addr);
    let new_value = value.wrapping_add(1);
    self.mem_write(addr, new_value);
    self.update_zero_and_negative_flags(new_value);
  }

  fn inx(&mut self) {
    self.register_x = self.register_x.wrapping_add(1);
    self.update_zero_and_negative_flags(self.register_x);
  }

  fn iny(&mut self) {
    self.register_y = self.register_y.wrapping_add(1);
    self.update_zero_and_negative_flags(self.register_y);
  }

  fn eor(&mut self, mode: &AddressingMode) {
    let addr = self.get_operand_address(mode);
    let value = self.mem_read(addr);

    self.register_a = self.register_a.bitxor(value);
    self.update_zero_and_negative_flags(self.register_a);
  }

  fn jmp(&mut self, mode: &AddressingMode) {
    let addr = self.mem_read_u16(self.program_counter);

    if matches!(mode, AddressingMode::Absolute) {
      self.program_counter = addr;
    } else {
      let indirect_addr =
        if addr.bitand(0x00FF) == 0x00FF {
          let lo = self.mem_read(addr);
          let hi = self.mem_read(addr & 0xFF00);
          (hi as u16) << 8 | (lo as u16)
        } else {
          self.mem_read_u16(addr)
        };
      self.program_counter = indirect_addr;
    }
  }

  fn jsr(&mut self){
    self.stack_push_u16(self.program_counter + 2 - 1);
    self.program_counter = self.mem_read_u16(self.program_counter);
  }

  fn lda(&mut self, mode: &AddressingMode) {
    let addr = self.get_operand_address(mode);
    let value = self.mem_read(addr);

    self.register_a = value;
    self.update_zero_and_negative_flags(self.register_a);
  }

  fn ldx(&mut self, mode: &AddressingMode) {
    let addr = self.get_operand_address(mode);
    let value = self.mem_read(addr);

    self.register_x = value;
    self.update_zero_and_negative_flags(self.register_x);
  }

  fn ldy(&mut self, mode: &AddressingMode) {
    let addr = self.get_operand_address(mode);
    let value = self.mem_read(addr);

    self.register_y = value;
    self.update_zero_and_negative_flags(self.register_y);
  }

  fn lsr(&mut self, mode: &AddressingMode) {
    if matches!(mode, AddressingMode::Immediate){
      self.status.set(CpuFlags::CARRY, self.register_a & 0x01 == 1);
      self.register_a >>= 1;
      self.update_zero_and_negative_flags(self.register_a);
    } else {
      let addr = self.get_operand_address(mode);
      let mut value = self.mem_read(addr);
      self.status.set(CpuFlags::CARRY, value & 0x01 == 1);
      value >>= 1;
      self.mem_write(addr, value);
      self.update_zero_and_negative_flags(value);
    }
  }
  fn highest_bit_set(value: u8) -> bool {
    value & 0b1000_0000 != 0
  }

  fn lowest_bit_set(value: u8) -> bool {
    value & 0b0000_0001 != 0
  }

  fn nop(&mut self) {
    // nothing
  }

  fn ora(&mut self, mode: &AddressingMode) {
    let addr = self.get_operand_address(mode);
    let data = self.mem_read(addr);

    self.register_a = self.register_a.bitor(data);
    self.update_zero_and_negative_flags(self.register_a);
  }

  fn sta(&mut self, mode: &AddressingMode) {
    let addr = self.get_operand_address(mode);
    self.mem_write(addr, self.register_a);
  }

  fn stx(&mut self, mode: &AddressingMode) {
    let addr = self.get_operand_address(mode);
    self.mem_write(addr, self.register_x);
  }

  fn sty(&mut self, mode: &AddressingMode) {
    let addr = self.get_operand_address(mode);
    self.mem_write(addr, self.register_y);
  }

  fn pha(&mut self) {
    self.stack_push(self.register_a);
  }

  fn php(&mut self) {
    let mut flags = self.status.clone();
    // https://wiki.nesdev.org/w/index.php/Status_flags#The_B_flag
    flags.insert(CpuFlags::BREAK);
    flags.insert(CpuFlags::BREAK2);
    self.stack_push(flags.bits());
  }

  fn pla(&mut self) {
    self.register_a = self.stack_pop();
    self.update_zero_and_negative_flags(self.register_a);
  }

  fn plp(&mut self) {
    let flags = self.stack_pop();
    self.set_flags(flags);
    // https://wiki.nesdev.org/w/index.php/Status_flags#The_B_flag
    self.status.remove(CpuFlags::BREAK);
    self.status.insert(CpuFlags::BREAK2);
  }

  fn rol(&mut self, mode: &AddressingMode) {
    if matches!(mode, AddressingMode::NoneAddressing) {
      let result = self.register_a.rotate_left(1);
      self.status.set(CpuFlags::CARRY, Self::highest_bit_set(self.register_a));
      self.update_zero_and_negative_flags(result);
      self.register_a = result;
    } else {
      let addr = self.get_operand_address(mode);
      let value = self.mem_read(addr);
      let result = value.rotate_left(1);
      self.status.set(CpuFlags::CARRY, Self::highest_bit_set(value));
      self.status.set(CpuFlags::NEGATIVE, result >> 7 == 1);
      self.mem_write(addr, result);
    }
  }

  fn ror(&mut self, mode: &AddressingMode) {
    if matches!(mode, AddressingMode::NoneAddressing) {
      let result = self.register_a.rotate_right(1);
      self.status.set(CpuFlags::CARRY, Self::lowest_bit_set(self.register_a));
      self.update_zero_and_negative_flags(result);
      self.register_a = result;
    } else {
      let addr = self.get_operand_address(mode);
      let value = self.mem_read(addr);
      let result = value.rotate_right(1);
      self.status.set(CpuFlags::CARRY, Self::lowest_bit_set(value));
      self.status.set(CpuFlags::NEGATIVE, result >> 7 == 1);
      self.mem_write(addr, result);
    }
  }

  fn rts(&mut self){
    // -1 based on https://web.archive.org/web/20170224121759/http://www.obelisk.me.uk/6502/reference.html#RTS
    // +1 based on http://www.6502.org/tutorials/6502opcodes.html#RTS
    // take +1 for now, as jsr already subtracts 1 ...
    self.program_counter = self.stack_pop_u16() +1;
  }

  fn rti(&mut self) {
    let flags = self.stack_pop();
    self.set_flags(flags);
    // https://wiki.nesdev.org/w/index.php/Status_flags#The_B_flag
    self.status.remove(CpuFlags::BREAK);
    self.status.insert(CpuFlags::BREAK2);

    self.program_counter = self.stack_pop_u16();
  }

  fn sec(&mut self) {
    self.status.insert(CpuFlags::CARRY);
  }
}


#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_0xa9_lda_immidiate_load_data() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0x05, 0x00]);
    assert_eq!(cpu.register_a, 5);
    assert!(cpu.status.bits() & 0b0000_0010 == 0b00);
    assert!(cpu.status.bits() & 0b1000_0000 == 0);
  }

  #[test]
  fn test_0xaa_tax_move_a_to_x() {
    let mut cpu = CPU::new();
    cpu.register_a = 10;
    cpu.load_and_run(vec![0xaa, 0x00]);

    assert_eq!(cpu.register_x, 10)
  }

  #[test]
  fn test_5_ops_working_together() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

    assert_eq!(cpu.register_x, 0xc1)
  }

  #[test]
  fn test_inx_overflow() {
    let mut cpu = CPU::new();
    cpu.register_x = 0xff;
    cpu.load_and_run(vec![0xe8, 0xe8, 0x00]);

    assert_eq!(cpu.register_x, 1)
  }

  #[test]
  fn test_lda_from_memory() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x10, 0x55);

    cpu.load_and_run(vec![0xa5, 0x10, 0x00]);

    assert_eq!(cpu.register_a, 0x55);
  }
}
