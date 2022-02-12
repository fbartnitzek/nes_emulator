use crate::cpu::{CPU, CpuFlags, Mem};

#[test]
fn test_0xa9_lda_immediate_load_data() {
  let mut cpu = CPU::new();
  cpu.load_and_run(vec![0xA9, 0x05, 0x00]);

  assert_eq!(cpu.register_a, 0x05);
  assert_eq!(cpu.status & CpuFlags::ZERO, CpuFlags::empty());
  assert_eq!(cpu.status & CpuFlags::NEGATIV, CpuFlags::empty());
}

#[test]
fn test_0xa9_lda_zero_flag() {
  let mut cpu = CPU::new();
  cpu.load_and_run(vec![0xA9, 0x00, 0x00]);

  assert_eq!(cpu.status & CpuFlags::ZERO, CpuFlags::ZERO);
}

#[test]
fn test_0xaa_tax_transfer_a_to_x() {
  let mut cpu = CPU::new();
  cpu.load_and_run(vec![0xA9, 0x0A, 0xAA, 0x00]);

  assert_eq!(cpu.register_x, 10)
}

#[test]
fn test_0xe8_inx_increment_x() {
  let mut cpu = CPU::new();
  cpu.load_and_run(vec![0xA9, 0x0A, 0xAA, 0xE8, 0x00]);

  assert_eq!(cpu.register_x, 11)
}

#[test]
fn test_5_ops_working_together() {
  let mut cpu = CPU::new();
  cpu.load_and_run(vec![0xA9, 0xC0, 0xAA, 0xE8, 0x00]);

  assert_eq!(cpu.register_x, 0xC1);
}

#[test]
fn test_inx_overflow() {
  let mut cpu = CPU::new();
  cpu.load_and_run(vec![0xA9, 0xFF, 0xAA, 0xE8, 0xE8, 0x00]);

  assert_eq!(cpu.register_x, 1)
}

#[test]
fn test_lda_from_memory() {
  let mut cpu = CPU::new();
  cpu.mem_write(0x10, 0x55);

  cpu.load_and_run(vec![0xA5, 0x10, 0x00]);

  assert_eq!(cpu.register_a, 0x55);
}

#[test]
fn test_lda_immediate_sta_absolute_mode() {
  let mut cpu = CPU::new();

  cpu.load_and_run(vec![0xA9, 0x42, 0x8D, 0x00, 0x02, 0x00]);

  let mem = cpu.mem_read_u16(0x0200);
  assert_eq!(cpu.register_a, 0x42);
  assert_eq!(mem, 0x42);
}

#[test]
fn test_ldx_immediate_stx_zero_page() {
  let mut cpu = CPU::new();

  cpu.load_and_run(vec![0xA2, 0x42, 0x86, 0x02, 0x00]);

  let mem = cpu.mem_read(0x02);
  assert_eq!(cpu.register_x, 0x42);
  assert_eq!(mem, 0x42);
}

#[test]
fn test_ldy_immediate_ldx_immediate_sty_zero_page_x() {
  let mut cpu = CPU::new();

  cpu.load_and_run(vec![0xA0, 0x42, 0xA2, 0x42, 0x94, 0x02, 0x00]);

  let mem = cpu.mem_read(0x44);
  assert_eq!(cpu.register_y, 0x42);
  assert_eq!(mem, 0x42);
}

#[test]
fn test_reset() {
  let mut cpu = CPU::new();
  cpu.reset();

  assert_eq!(cpu.status & CpuFlags::INTERRUPT_DISABLE, CpuFlags::INTERRUPT_DISABLE);
  assert_eq!(cpu.status & CpuFlags::BREAK2, CpuFlags::BREAK2);
}

#[test]
fn test_0xa8_tay_transfer_a_to_y() {
  let mut cpu = CPU::new();
  cpu.load_and_run(vec![0xA9, 0x0A, 0xA8, 0x00]);

  assert_eq!(cpu.register_y, 10)
}

#[test]
fn test_0x9a_txs_transfer_x_to_stack_pointer() {
  let mut cpu = CPU::new();
  cpu.load_and_run(vec![0xA9, 0x0A, 0xAA, 0x9A, 0x00]);

  assert_eq!(cpu.register_x, 10);
  assert_eq!(cpu.stack_pointer, 10);
}

#[test]
fn test_0xba_tsx_transfer_stack_pointer_to_x() {
  let mut cpu = CPU::new();
  cpu.load_and_run(vec![0xA9, 0x0A, 0xAA, 0x9A, // set stack_pointer = x (10)
                        0xA9, 0x0B, 0xAA, // reset x to 11
                        0xBA, 0x00,       // set x = stack_pointer
  ]);

  assert_eq!(cpu.register_x, 10);
  assert_eq!(cpu.stack_pointer, 10);
}

#[test]
fn test_0x8a_txa_transfer_x_to_acc() {
  let mut cpu = CPU::new();
  cpu.load_and_run(vec![0xE8, 0x8A, 0x00]); // inc x (1), acc = x (1)

  assert_eq!(cpu.register_a, 1);
  assert_eq!(cpu.register_x, 1);
}

#[test]
fn test_0xc8_0x98_iny_tya_transfer_y_to_acc() {
  let mut cpu = CPU::new();
  cpu.load_and_run(vec![0xC8, 0x98, 0x00]); // inc y (1), acc = y (1)

  assert_eq!(cpu.register_y, 1);
  assert_eq!(cpu.register_a, 1);
}