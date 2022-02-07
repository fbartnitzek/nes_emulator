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
    OpCode::new(0x69, "ADC", 2, 2, AddressingMode::Immediate),
    // todo
    OpCode::new(0x29, "AND", 2, 2, AddressingMode::Immediate),
    // todo
    OpCode::new(0x00, "BRK", 1, 7, AddressingMode::NoneAddressing),
    OpCode::new(0xE8, "INX", 1, 2, AddressingMode::NoneAddressing),
    OpCode::new(0xAA, "TAX", 1, 2, AddressingMode::NoneAddressing),

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
  ];

  pub static ref OPCODES_MAP: HashMap<u8, &'static OpCode> = {
    let mut map = HashMap::new();
    for cpu_op in &*CPU_OPS_CODES {
      map.insert(cpu_op.code, cpu_op);
    }
    map
  };
}

