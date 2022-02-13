use std::collections::HashMap;
use crate::cpu::AddressingMode;

pub struct OpCode {
  pub code: u8,
  pub mnemonic: &'static str,
  pub len: u8,
  pub cycles: u8,
  pub mode: AddressingMode,
}

impl OpCode {
  fn new(code: u8, mnemonic: &'static str, len: u8, cycles: u8, mode: AddressingMode) -> Self {
    OpCode {
      code,
      mnemonic,
      len,
      cycles,
      mode,
    }
  }
}

// see https://web.archive.org/web/20170224121759/http://www.obelisk.me.uk/6502/reference.html#TAX
lazy_static! {
  pub static ref CPU_OPS_CODES: Vec<OpCode> = vec![
    OpCode::new(0x69, "ADC", 2, 2, AddressingMode::Immediate), // todo
    OpCode::new(0x29, "AND", 2, 2, AddressingMode::Immediate), // todo
    OpCode::new(0x0A, "ASL", 2, 2, AddressingMode::Immediate), // todo
    OpCode::new(0x90, "BCC", 2, 2, AddressingMode::Immediate), // todo
    OpCode::new(0xB0, "BCS", 2, 2, AddressingMode::Immediate), // todo
    OpCode::new(0xF0, "BEQ", 2, 2, AddressingMode::Immediate), // todo
    OpCode::new(0x24, "BIT", 2, 2, AddressingMode::Immediate), // todo
    OpCode::new(0x30, "BMI", 2, 2, AddressingMode::Immediate), // todo
    OpCode::new(0xD0, "BNE", 2, 2, AddressingMode::Immediate), // todo
    OpCode::new(0x10, "BPL", 2, 2, AddressingMode::Immediate), // todo
    OpCode::new(0x00, "BRK", 1, 7, AddressingMode::NoneAddressing),
    OpCode::new(0x50, "BVC", 2, 2, AddressingMode::Immediate), // todo
    OpCode::new(0x70, "BVS", 2, 2, AddressingMode::Immediate), // todo
    OpCode::new(0x18, "CLC", 2, 2, AddressingMode::Immediate), // todo
    OpCode::new(0xD8, "CLD", 2, 2, AddressingMode::Immediate), // todo
    OpCode::new(0x58, "CLI", 2, 2, AddressingMode::Immediate), // todo
    OpCode::new(0xB8, "CLV", 2, 2, AddressingMode::Immediate), // todo
    OpCode::new(0xC9, "CMP", 2, 2, AddressingMode::Immediate), // todo
    OpCode::new(0xE0, "CPX", 2, 2, AddressingMode::Immediate), // todo
    OpCode::new(0xC0, "CPY", 2, 2, AddressingMode::Immediate), // todo
    OpCode::new(0xC6, "DEC", 2, 5, AddressingMode::ZeroPage),
    OpCode::new(0xD6, "DEC", 2, 6, AddressingMode::ZeroPage_X),
    OpCode::new(0xCE, "DEC", 3, 6, AddressingMode::Absolute),
    OpCode::new(0xDE, "DEC", 3, 7, AddressingMode::Absolute_X),
    OpCode::new(0xCA, "DEX", 1, 2, AddressingMode::Immediate),
    OpCode::new(0x88, "DEY", 1, 2, AddressingMode::Immediate),
    OpCode::new(0x49, "EOR", 2, 2, AddressingMode::Immediate), // todo
    OpCode::new(0xE6, "INC", 2, 2, AddressingMode::Immediate), // todo
    OpCode::new(0xE8, "INX", 1, 2, AddressingMode::NoneAddressing),
    OpCode::new(0xC8, "INY", 1, 2, AddressingMode::NoneAddressing),
    OpCode::new(0x4C, "JMP", 1, 2, AddressingMode::NoneAddressing), //todo
    OpCode::new(0x20, "JSR", 1, 2, AddressingMode::NoneAddressing), //todo

    OpCode::new(0xA9, "LDA", 2, 2, AddressingMode::Immediate),
    OpCode::new(0xA5, "LDA", 2, 3, AddressingMode::ZeroPage),
    OpCode::new(0xB5, "LDA", 2, 4, AddressingMode::ZeroPage_X),
    OpCode::new(0xAD, "LDA", 3, 4, AddressingMode::Absolute),
    OpCode::new(0xBD, "LDA", 3, 4 /* +1 if page crossed */, AddressingMode::Absolute_X),
    OpCode::new(0xB9, "LDA", 3, 4 /* +1 if page crossed */, AddressingMode::Absolute_Y),
    OpCode::new(0xA1, "LDA", 2, 6, AddressingMode::Indirect_X),
    OpCode::new(0xB1, "LDA", 2, 5 /* +1 if page crossed */, AddressingMode::Indirect_Y),

    OpCode::new(0xA2, "LDX", 2, 2, AddressingMode::Immediate),
    OpCode::new(0xA6, "LDX", 2, 3, AddressingMode::ZeroPage),
    OpCode::new(0xB6, "LDX", 2, 4, AddressingMode::ZeroPage_Y),
    OpCode::new(0xAE, "LDX", 3, 4, AddressingMode::Absolute),
    OpCode::new(0xBE, "LDX", 3, 4 /* +1 if page crossed */, AddressingMode::Absolute_Y),

    OpCode::new(0xA0, "LDY", 2, 2, AddressingMode::Immediate),
    OpCode::new(0xA4, "LDY", 2, 3, AddressingMode::ZeroPage),
    OpCode::new(0xB4, "LDY", 2, 4, AddressingMode::ZeroPage_X),
    OpCode::new(0xAC, "LDY", 3, 4, AddressingMode::Absolute),
    OpCode::new(0xBC, "LDY", 3, 4 /* +1 if page crossed */, AddressingMode::Absolute_X),

    OpCode::new(0x4A, "LSR", 1, 2, AddressingMode::NoneAddressing), //todo
    OpCode::new(0xEA, "NOP", 1, 2, AddressingMode::NoneAddressing), //todo
    OpCode::new(0x09, "ORA", 1, 2, AddressingMode::NoneAddressing), //todo
    OpCode::new(0x48, "PHA", 1, 2, AddressingMode::NoneAddressing), //todo
    OpCode::new(0x08, "PHP", 1, 2, AddressingMode::NoneAddressing), //todo
    OpCode::new(0x68, "PLA", 1, 2, AddressingMode::NoneAddressing), //todo
    OpCode::new(0x28, "PLP", 1, 2, AddressingMode::NoneAddressing), //todo
    OpCode::new(0x2A, "ROL", 1, 2, AddressingMode::NoneAddressing), //todo
    OpCode::new(0x6A, "ROR", 1, 2, AddressingMode::NoneAddressing), //todo
    OpCode::new(0x40, "RTI", 1, 2, AddressingMode::NoneAddressing), //todo
    OpCode::new(0x60, "RTS", 1, 2, AddressingMode::NoneAddressing), //todo
    OpCode::new(0xE9, "SBC", 1, 2, AddressingMode::NoneAddressing), //todo
    OpCode::new(0x38, "SEC", 1, 2, AddressingMode::NoneAddressing), //todo
    OpCode::new(0xF8, "SED", 1, 2, AddressingMode::NoneAddressing), //todo
    OpCode::new(0x78, "SEI", 1, 2, AddressingMode::NoneAddressing), //todo

    OpCode::new(0x85, "STA", 2, 3, AddressingMode::ZeroPage),
    OpCode::new(0x95, "STA", 2, 4, AddressingMode::ZeroPage_X),
    OpCode::new(0x8D, "STA", 3, 4, AddressingMode::Absolute),
    OpCode::new(0x9D, "STA", 3, 5, AddressingMode::Absolute_X),
    OpCode::new(0x99, "STA", 3, 5, AddressingMode::Absolute_Y),
    OpCode::new(0x81, "STA", 2, 6, AddressingMode::Indirect_X),
    OpCode::new(0x91, "STA", 2, 6, AddressingMode::Indirect_Y),

    OpCode::new(0x86, "STX", 2, 3, AddressingMode::ZeroPage),
    OpCode::new(0x96, "STX", 2, 4, AddressingMode::ZeroPage_Y),
    OpCode::new(0x8E, "STX", 3, 4, AddressingMode::Absolute),

    OpCode::new(0x84, "STY", 2, 3, AddressingMode::ZeroPage),
    OpCode::new(0x94, "STY", 2, 4, AddressingMode::ZeroPage_X),
    OpCode::new(0x8C, "STY", 3, 4, AddressingMode::Absolute),

    OpCode::new(0xAA, "TAX", 1, 2, AddressingMode::NoneAddressing),
    OpCode::new(0xA8, "TAY", 1, 2, AddressingMode::NoneAddressing),
    OpCode::new(0xBA, "TSX", 1, 2, AddressingMode::NoneAddressing),
    OpCode::new(0x8A, "TXA", 1, 2, AddressingMode::NoneAddressing),
    OpCode::new(0x9A, "TXS", 1, 2, AddressingMode::NoneAddressing),
    OpCode::new(0x98, "TYA", 1, 2, AddressingMode::NoneAddressing),
  ];

  pub static ref OPCODES_MAP: HashMap<u8, &'static OpCode> = {
    let mut map = HashMap::new();
    for cpu_op in &*CPU_OPS_CODES {
      map.insert(cpu_op.code, cpu_op);
    }
    map
  };
}

