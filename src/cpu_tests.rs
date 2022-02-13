use crate::cpu::{CPU, CpuFlags, Mem};

#[test]
fn test_lda_immediate_load_data() {
  let mut cpu = CPU::new();

  cpu.load_and_run(vec![0xA9, 0x05, 0x00]);

  assert_eq!(0x05, cpu.register_a);
  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::ZERO);
  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::NEGATIV);
}

#[test]
fn test_lda_zero_flag() {
  let mut cpu = CPU::new();

  cpu.load_and_run(vec![0xA9, 0x00, 0x00]);

  assert_eq!(CpuFlags::ZERO, cpu.status & CpuFlags::ZERO);
}

#[test]
fn test_tax_transfer_a_to_x() {
  let mut cpu = CPU::new();
  cpu.register_a = 10;

  cpu.load_and_run(vec![0xAA, 0x00]);

  assert_eq!(10, cpu.register_x)
}

#[test]
fn test_inx_increment_x() {
  let mut cpu = CPU::new();
  cpu.register_x = 10;

  cpu.load_and_run(vec![0xE8, 0x00]);

  assert_eq!(11, cpu.register_x)
}

#[test]
fn test_inx_overflow() {
  let mut cpu = CPU::new();
  cpu.register_x = 0xFF;

  cpu.load_and_run(vec![0xE8, 0xE8, 0x00]);

  assert_eq!(1, cpu.register_x)
}

#[test]
fn test_iny_increment_y() {
  let mut cpu = CPU::new();
  cpu.load_reset_and_run(vec![0xC8, 0x00]);

  assert_eq!(1, cpu.register_y);
}

#[test]
fn test_5_ops_working_together() {
  let mut cpu = CPU::new();

  cpu.load_reset_and_run(vec![0xA9, 0xC0, 0xAA, 0xE8, 0x00]);

  assert_eq!(0xC1, cpu.register_x);
}

#[test]
fn test_lda_from_memory() {
  let mut cpu = CPU::new();
  cpu.mem_write(0x10, 0x55);

  cpu.load_and_run(vec![0xA5, 0x10, 0x00]);

  assert_eq!(0x55, cpu.register_a);
}

#[test]
fn test_sta_zero_page() {
  let mut cpu = CPU::new();
  cpu.register_a = 0x42;

  cpu.load_and_run(vec![0x85, 0x21, 0x00]);

  cpu.dump_non_empty_memory();
  assert_eq!(0x42, cpu.mem_read_u16(0x0021));
}

#[test]
fn test_sta_zero_page_x() {
  let mut cpu = CPU::new();
  cpu.register_a = 0x42;
  cpu.register_x = 0x80;

  cpu.load_and_run(vec![0x95, 0x09, 0x00]);

  cpu.dump_non_empty_memory();
  assert_eq!(0x42, cpu.mem_read_u16(0x0089));
}

#[test]
fn test_sta_absolute() {
  let mut cpu = CPU::new();
  cpu.register_a = 0x42;

  cpu.load_and_run(vec![0x8D, 0x00, 0x02, 0x00]);

  assert_eq!(0x42, cpu.mem_read_u16(0x0200));
}

#[test]
fn test_sta_absolute_x() {
  let mut cpu = CPU::new();
  cpu.register_a = 0x42;
  cpu.register_x = 0x14;

  cpu.load_and_run(vec![0x9D, 0x23, 0x12, 0x00]);

  cpu.dump_non_empty_memory();
  assert_eq!(0x42, cpu.mem_read_u16(0x1237));
}

#[test]
fn test_sta_absolute_y() {
  let mut cpu = CPU::new();
  cpu.register_a = 0x42;
  cpu.register_y = 0x14;

  cpu.load_and_run(vec![0x99, 0x23, 0x12, 0x00]);

  cpu.dump_non_empty_memory();
  assert_eq!(0x42, cpu.mem_read_u16(0x1237));
}

#[test]
fn test_sta_indirect_x() {
  let mut cpu = CPU::new();
  cpu.register_a = 0x42;
  cpu.register_x = 0x10;
  cpu.mem_write(0x0021, 0x05);
  cpu.mem_write(0x0022, 0x07);

  cpu.load_and_run(vec![0x81, 0x11, 0x00]);

  cpu.dump_non_empty_memory();
  assert_eq!(0x42, cpu.mem_read_u16(0x0705));
}

#[test]
fn test_sta_indirect_y() {
  let mut cpu = CPU::new();
  cpu.register_a = 0x42;
  cpu.register_y = 0x10;
  cpu.mem_write(0x0011, 0x21);

  cpu.load_and_run(vec![0x91, 0x11, 0x00]);

  cpu.dump_non_empty_memory();
  assert_eq!(0x42, cpu.mem_read_u16(0x0031));
}

#[test]
fn test_ldx_immediate() {
  let mut cpu = CPU::new();

  cpu.load_and_run(vec![0xA2, 0x42, 0x00]);

  assert_eq!(0x42, cpu.register_x);
}

#[test]
fn test_stx_zero_page() {
  let mut cpu = CPU::new();
  cpu.register_x = 0x42;
  cpu.load_and_run(vec![0x86, 0x02, 0x00]);

  cpu.dump_non_empty_memory();
  assert_eq!(0x42, cpu.mem_read(0x02));
}

#[test]
fn test_ldy_immediate() {
  let mut cpu = CPU::new();

  cpu.load_and_run(vec![0xA0, 0x42, 0x00]);

  assert_eq!(cpu.register_y, 0x42);
}

#[test]
fn test_sty_zero_page_x() {
  let mut cpu = CPU::new();
  cpu.register_x = 0x20;
  cpu.register_y = 0x42;

  cpu.load_and_run(vec![0x94, 0x01, 0x00]);

  cpu.dump_non_empty_memory();
  assert_eq!(0x42, cpu.mem_read(0x21));
}

#[test]
fn test_reset() {
  let mut cpu = CPU::new();

  cpu.reset();

  assert_eq!(CpuFlags::INTERRUPT_DISABLE, cpu.status & CpuFlags::INTERRUPT_DISABLE);
  assert_eq!(CpuFlags::BREAK2, cpu.status & CpuFlags::BREAK2);
}

#[test]
fn test_tay_transfer_a_to_y() {
  let mut cpu = CPU::new();
  cpu.register_a = 0x42;

  cpu.load_and_run(vec![0xA8, 0x00]);

  assert_eq!(0x42, cpu.register_y)
}

#[test]
fn test_txs_transfer_x_to_stack_pointer() {
  let mut cpu = CPU::new();
  cpu.register_x = 0x42;

  cpu.load_and_run(vec![0x9A, 0x00]);

  assert_eq!(0x42, cpu.stack_pointer);
}

#[test]
fn test_tsx_transfer_stack_pointer_to_x() {
  let mut cpu = CPU::new();
  cpu.stack_pointer = 0x42;

  cpu.load_and_run(vec![0xBA, 0x00]);

  assert_eq!(0x42, cpu.register_x);
}

#[test]
fn test_txa_transfer_x_to_acc() {
  let mut cpu = CPU::new();
  cpu.register_x = 0x42;

  cpu.load_and_run(vec![0x8A, 0x00]);

  assert_eq!(0x42, cpu.register_a);
}

#[test]
fn test_tya_transfer_y_to_acc() {
  let mut cpu = CPU::new();
  cpu.register_y = 0x42;
  cpu.load_and_run(vec![0x98, 0x00]);

  assert_eq!(0x42, cpu.register_a);
}