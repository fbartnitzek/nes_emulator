use crate::cpu::{CPU, CpuFlags, Mem};

#[test]
fn test_5_ops_working_together() {
  let mut cpu = CPU::new();
  cpu.load_reset_and_run(vec![0xA9, 0xC0, 0xAA, 0xE8, 0x00]);

  assert_eq!(0xC1, cpu.register_x);
}

#[test]
fn test_adc_add_with_no_carry() {
  let mut cpu = CPU::new();
  cpu.register_a = 0x21;

  cpu.load_and_run(vec![0x69, 0x42, 0x00]);

  assert_eq!(0x63, cpu.register_a);
  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::CARRY);
  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::OVERFLOW);
}

#[test]
fn test_adc_add_with_resulting_carry() {
  let mut cpu = CPU::new();
  cpu.register_a = 0xF0;
  cpu.mem_write(0x42, 0x21);

  cpu.load_and_run(vec![0x65, 0x42, 0x00]);

  assert_eq!(0x11, cpu.register_a);
  assert_eq!(CpuFlags::CARRY, cpu.status & CpuFlags::CARRY);
  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::OVERFLOW);
}

#[test]
fn test_adc_add_with_carry_in() {
  let mut cpu = CPU::new();
  cpu.register_a = 0x42;
  cpu.status.insert(CpuFlags::CARRY);
  cpu.mem_write(0x42, 0x21);

  cpu.load_and_run(vec![0x65, 0x42, 0x00]);

  assert_eq!(0x64, cpu.register_a);
  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::CARRY);
  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::OVERFLOW);
}

// overflow adc & sbc examples, see: http://www.6502.org/tutorials/vflag.html

#[test]
fn test_adc_add_with_positive_overflow() {
  let mut cpu = CPU::new();

  // 127 + 1 = 128
  cpu.register_a = 0x7F;
  cpu.load_and_run(vec![0x69, 0x01, 0x00]);

  assert_eq!(0x80, cpu.register_a);
  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::CARRY);
  assert_eq!(CpuFlags::OVERFLOW, cpu.status & CpuFlags::OVERFLOW);
}

#[test]
fn test_adc_add_no_overflow() {
  let mut cpu = CPU::new();

  // 1 + -1 = 128
  cpu.register_a = 0x01;
  cpu.load_and_run(vec![0x69, 0xFF, 0x00]);

  assert_eq!(0x00, cpu.register_a);
  assert_eq!(CpuFlags::CARRY, cpu.status & CpuFlags::CARRY);
  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::OVERFLOW);
}

#[test]
fn test_adc_add_with_overflow_negative() {
  let mut cpu = CPU::new();

  // -128 + -1 = -129
  cpu.register_a = 0x80;
  cpu.load_and_run(vec![0x69, 0xFF, 0x00]);

  assert_eq!(0x7F, cpu.register_a);
  assert_eq!(CpuFlags::CARRY, cpu.status & CpuFlags::CARRY);
  assert_eq!(CpuFlags::OVERFLOW, cpu.status & CpuFlags::OVERFLOW);
}

#[test]
fn test_sdc_subtract_with_default_carry() {
  let mut cpu = CPU::new();
  cpu.register_a = 0x42;
  cpu.mem_write(0x3412, 0x21);

  cpu.load_and_run(vec![0xED, 0x12, 0x34, 0x00]);

  assert_eq!(0x21, cpu.register_a);
  assert_eq!(CpuFlags::CARRY, cpu.status & CpuFlags::CARRY);
  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::OVERFLOW);
}

#[test]
fn test_sdc_subtract_with_carry_in() {
  let mut cpu = CPU::new();
  cpu.register_a = 0x42;
  cpu.status.insert(CpuFlags::CARRY);
  cpu.mem_write(0x3412, 0x21);

  cpu.load_and_run(vec![0xED, 0x12, 0x34, 0x00]);

  assert_eq!(0x22, cpu.register_a);
  assert_eq!(CpuFlags::CARRY, cpu.status & CpuFlags::CARRY);
  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::OVERFLOW);
}

#[test]
fn test_sbc_subtract_without_flags() {
  let mut cpu = CPU::new();

  // 0 - 1 = -1
  cpu.register_a = 0x00;
  cpu.load_and_run(vec![0xE9, 0x01, 0x00]);

  assert_eq!(0xFF, cpu.register_a);
  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::CARRY);
  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::OVERFLOW);
}

#[test]
fn test_sbc_subtract_with_overflow() {
  let mut cpu = CPU::new();

  // 127 - -1 = 128
  cpu.register_a = 0x7F;
  cpu.load_and_run(vec![0xE9, 0xFF, 0x00]);

  assert_eq!(0x80, cpu.register_a);
  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::CARRY);
  assert_eq!(CpuFlags::OVERFLOW, cpu.status & CpuFlags::OVERFLOW);
}

#[test]
fn test_and_acc_and_immediate_memory() {
  let mut cpu = CPU::new();

  cpu.register_a = 0b1001_1001;
  cpu.load_and_run(vec![0x29, 0b0111_0111, 0x00]);

  assert_eq!(0b0001_0001, cpu.register_a);
  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::ZERO);
  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::NEGATIVE);
}

#[test]
fn test_and_acc_and_absolute_memory() {
  let mut cpu = CPU::new();

  cpu.register_a = 0b1001_1001;
  cpu.mem_write(0x1234, 0b1000_0011);
  cpu.load_and_run(vec![0x2D, 0x34, 0x12, 0x00]);

  assert_eq!(0b1000_0001, cpu.register_a);
  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::ZERO);
  assert_eq!(CpuFlags::NEGATIVE, cpu.status & CpuFlags::NEGATIVE);
}

#[test]
fn test_asl_arithmetic_shift_left_acc() {
  let mut cpu = CPU::new();

  cpu.register_a = 0b0101_1001;
  cpu.load_and_run(vec![0x0A, 0x00]);

  assert_eq!(0b1011_0010, cpu.register_a);
  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::CARRY);
  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::ZERO);
  assert_eq!(CpuFlags::NEGATIVE, cpu.status & CpuFlags::NEGATIVE);
}

#[test]
fn test_asl_arithmetic_shift_left_zero() {
  let mut cpu = CPU::new();

  cpu.mem_write(0x0042, 0b1000_0000);
  cpu.load_and_run(vec![0x06, 0x42, 0x00]);

  assert_eq!(0b0000_0000, cpu.mem_read(0x0042));
  assert_eq!(CpuFlags::CARRY, cpu.status & CpuFlags::CARRY);
  assert_eq!(CpuFlags::ZERO, cpu.status & CpuFlags::ZERO);
  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::NEGATIVE);
}

#[test]
fn test_bcc_branch_if_carry_clear_with_carry() {
  let mut cpu = CPU::new();

  cpu.status.insert(CpuFlags::CARRY);
  cpu.load_and_run(vec![0x90, 0x42, 0x00]);

  assert_eq!(0x8003, cpu.program_counter);
}

#[test]
fn test_bcc_branch_if_carry_clear_no_carry() {
  let mut cpu = CPU::new();

  cpu.load_and_run(vec![0x90, 0x42, 0x00]);

  assert_eq!(0x8045, cpu.program_counter);
}

#[test]
fn test_bcs_branch_if_carry_set_with_carry() {
  let mut cpu = CPU::new();

  cpu.status.insert(CpuFlags::CARRY);
  cpu.load_and_run(vec![0xB0, 0x42, 0x00]);

  assert_eq!(0x8045, cpu.program_counter);
}

#[test]
fn test_clc_clear_carry_flag() {
  let mut cpu = CPU::new();

  cpu.status = CpuFlags::CARRY;
  cpu.load_and_run(vec![0x18, 0x00]);

  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::CARRY);
}

#[test]
fn test_cld_clear_decimal_mode() {
  let mut cpu = CPU::new();

  cpu.status = CpuFlags::DECIMAL_MODE;
  cpu.load_and_run(vec![0xD8, 0x00]);

  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::DECIMAL_MODE);
}

#[test]
fn test_cli_clear_interrupt_disable() {
  let mut cpu = CPU::new();

  cpu.status = CpuFlags::INTERRUPT_DISABLE;
  cpu.load_and_run(vec![0x58, 0x00]);

  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::INTERRUPT_DISABLE);
}

#[test]
fn test_clv_clear_overflow_flag() {
  let mut cpu = CPU::new();

  cpu.status = CpuFlags::OVERFLOW;
  cpu.load_and_run(vec![0xB8, 0x00]);

  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::OVERFLOW);
}

#[test]
fn test_cmp_compare_acc_with_memory_absolute_equal() {
  let mut cpu = CPU::new();

  cpu.mem_write(0x2142, 0x42);
  cpu.register_a = 0x42;
  cpu.load_and_run(vec![0xCD, 0x42, 0x21, 0x00]);

  assert_eq!(CpuFlags::ZERO, cpu.status & CpuFlags::ZERO);
  assert_eq!(CpuFlags::CARRY, cpu.status & CpuFlags::CARRY);
  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::NEGATIVE);
}

#[test]
fn test_cmp_compare_acc_with_memory_absolute_lower() {
  let mut cpu = CPU::new();

  cpu.mem_write(0x2142, 0x42);
  cpu.register_a = 0x41;
  cpu.load_and_run(vec![0xCD, 0x42, 0x21, 0x00]);

  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::ZERO);
  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::CARRY);
  assert_eq!(CpuFlags::NEGATIVE, cpu.status & CpuFlags::NEGATIVE);
}

#[test]
fn test_cmp_compare_acc_with_memory_absolute_greater() {
  let mut cpu = CPU::new();

  cpu.mem_write(0x2142, 0x42);
  cpu.register_a = 0x43;
  cpu.load_and_run(vec![0xCD, 0x42, 0x21, 0x00]);

  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::ZERO);
  assert_eq!(CpuFlags::CARRY, cpu.status & CpuFlags::CARRY);
  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::NEGATIVE);
}

#[test]
fn test_cpx_compare_x_with_memory_absolute_equal() {
  let mut cpu = CPU::new();

  cpu.mem_write(0x2142, 0x42);
  cpu.register_x = 0x42;
  cpu.load_and_run(vec![0xEC, 0x42, 0x21, 0x00]);

  assert_eq!(CpuFlags::ZERO, cpu.status & CpuFlags::ZERO);
  assert_eq!(CpuFlags::CARRY, cpu.status & CpuFlags::CARRY);
  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::NEGATIVE);
}

#[test]
fn test_cpy_compare_y_with_memory_absolute_equal() {
  let mut cpu = CPU::new();

  cpu.mem_write(0x2142, 0x42);
  cpu.register_y = 0x42;
  cpu.load_and_run(vec![0xCC, 0x42, 0x21, 0x00]);

  assert_eq!(CpuFlags::ZERO, cpu.status & CpuFlags::ZERO);
  assert_eq!(CpuFlags::CARRY, cpu.status & CpuFlags::CARRY);
  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::NEGATIVE);
}

#[test]
fn test_dec_decrement_memory_zero_page() {
  let mut cpu = CPU::new();

  cpu.mem_write(0x0021, 0x0043);
  cpu.load_and_run(vec![0xC6, 0x21, 0x00]);

  assert_eq!(0x0042, cpu.mem_read(0x0021))
}

#[test]
fn test_dec_decrement_memory_absolute() {
  let mut cpu = CPU::new();

  cpu.mem_write(0x2142, 0x43);
  cpu.load_and_run(vec![0xCE, 0x42, 0x21, 0x00]);

  assert_eq!(0x42, cpu.mem_read(0x2142))
}

#[test]
fn test_dex_decrement_x() {
  let mut cpu = CPU::new();

  cpu.load_and_run(vec![0xCA, 0x00]);

  assert_eq!(0xFF, cpu.register_x)
}

#[test]
fn test_dey_decrement_y() {
  let mut cpu = CPU::new();

  cpu.load_and_run(vec![0x88, 0x00]);

  assert_eq!(0xFF, cpu.register_y)
}

#[test]
fn test_inc_increment_memory_zero_page_x() {
  let mut cpu = CPU::new();

  cpu.mem_write(0x0021, 0x0041);
  cpu.register_x = 0x01;
  cpu.load_and_run(vec![0xF6, 0x20, 0x00]);

  assert_eq!(0x0042, cpu.mem_read(0x0021))
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
fn test_lda_immediate_load_data() {
  let mut cpu = CPU::new();

  cpu.load_and_run(vec![0xA9, 0x05, 0x00]);

  assert_eq!(0x05, cpu.register_a);
  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::ZERO);
  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::NEGATIVE);
}

#[test]
fn test_lda_zero_flag() {
  let mut cpu = CPU::new();

  cpu.load_and_run(vec![0xA9, 0x00, 0x00]);

  assert_eq!(CpuFlags::ZERO, cpu.status & CpuFlags::ZERO);
}

#[test]
fn test_lda_from_memory() {
  let mut cpu = CPU::new();
  cpu.mem_write(0x10, 0x55);

  cpu.load_and_run(vec![0xA5, 0x10, 0x00]);

  assert_eq!(0x55, cpu.register_a);
}

#[test]
fn test_ldx_immediate() {
  let mut cpu = CPU::new();

  cpu.load_and_run(vec![0xA2, 0x42, 0x00]);

  assert_eq!(0x42, cpu.register_x);
}

#[test]
fn test_ldy_immediate() {
  let mut cpu = CPU::new();

  cpu.load_and_run(vec![0xA0, 0x42, 0x00]);

  assert_eq!(0x42, cpu.register_y);
}

#[test]
fn test_pha_push_accumulator_to_stack() {
  let mut cpu = CPU::new();

  cpu.register_a = 0x42;
  cpu.load_and_run(vec![0x48, 0x00]);

  assert_eq!(0xFE, cpu.stack_pointer);
  assert_eq!(0x42, cpu.mem_read(0x01FF));
}

#[test]
fn test_pha_3_stack_pushes() {
  let mut cpu = CPU::new();

  cpu.load_reset_and_run(vec![0xA9, 0x20, 0x48, 0xA9, 0x21, 0x48, 0xA9, 0x22, 0x48, 0x00]);

  cpu.dump_non_empty_memory();
  assert_eq!(0xFC, cpu.stack_pointer);
  assert_eq!(0x20, cpu.mem_read(0x01FF));
  assert_eq!(0x21, cpu.mem_read(0x01FE));
  assert_eq!(0x22, cpu.mem_read(0x01FD));
}

#[test]
fn test_pha_stack_overflow() {
  let mut cpu = CPU::new();

  cpu.register_a = 0x42;
  cpu.stack_pointer = 0x00;
  cpu.load_and_run(vec![0x48, 0x00]);

  assert_eq!(0xFF, cpu.stack_pointer);
  assert_eq!(0x42, cpu.mem_read(0x0100));
}

#[test]
fn test_php_push_processor_status() {
  let mut cpu = CPU::new();
  cpu.status = CpuFlags::NEGATIVE | CpuFlags::CARRY | CpuFlags::INTERRUPT_DISABLE;

  cpu.load_and_run(vec![0x08, 0x00]);

  assert_eq!(0xFE, cpu.stack_pointer);
  assert_eq!(0xB5, cpu.mem_read(0x01FF)); // 0x04 + 0x01 + 0x80 + 0x10 + 0x20 = 0xB5 (BREAK + BREAK2)
}

#[test]
fn test_plp_pull_processor_status() {
  let mut cpu = CPU::new();
  cpu.mem_write(0x01FF, 0x85);  // 0x04 + 0x01 + 0x80 = 0x85
  cpu.stack_pointer = 0xFE;

  cpu.load_and_run(vec![0x28, 0x00]);

  assert_eq!(0xFF, cpu.stack_pointer);
  // additional break as program stopped
  assert_eq!(CpuFlags::NEGATIVE | CpuFlags::CARRY | CpuFlags::INTERRUPT_DISABLE | CpuFlags::BREAK2,
             cpu.status);
}

#[test]
fn test_rti_return_from_interrupt() {
  let mut cpu = CPU::new();
  cpu.mem_write(0x0181, 0x85);  // 0x04 + 0x01 + 0x80 = 0x85
  cpu.mem_write(0x0182, 0x34);
  cpu.mem_write(0x0183, 0x12);
  cpu.stack_pointer = 0x80;

  cpu.load_and_run(vec![0x40, 0x00]);

  assert_eq!(0x83, cpu.stack_pointer);
  assert_eq!(0x1234 + 1, cpu.program_counter);
  assert_eq!(CpuFlags::NEGATIVE | CpuFlags::CARRY | CpuFlags::INTERRUPT_DISABLE | CpuFlags::BREAK2,
             cpu.status);
}

#[test]
fn test_rol_rotate_left_accumulator() {
  let mut cpu = CPU::new();
  cpu.register_a = 0b1100_0011;

  cpu.load_and_run(vec![0x2A, 0x00]);

  assert_eq!(0b1000_0111, cpu.register_a);
  assert_eq!(CpuFlags::CARRY, cpu.status & CpuFlags::CARRY);
  assert_eq!(CpuFlags::NEGATIVE, cpu.status & CpuFlags::NEGATIVE);
  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::ZERO);
}

#[test]
fn test_rol_rotate_left_accumulator_zero() {
  let mut cpu = CPU::new();
  cpu.register_a = 0b0000_0000;

  cpu.load_and_run(vec![0x2A, 0x00]);

  assert_eq!(0b0000_0000, cpu.register_a);
  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::CARRY);
  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::NEGATIVE);
  assert_eq!(CpuFlags::ZERO, cpu.status & CpuFlags::ZERO);
}

#[test]
fn test_rol_rotate_left_absolute() {
  let mut cpu = CPU::new();
  cpu.mem_write(0x1234, 0b0010_1010);

  cpu.load_and_run(vec![0x2E, 0x34, 0x12, 0x00]);

  assert_eq!(0b0101_0100, cpu.mem_read_u16(0x1234));
  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::CARRY);
  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::NEGATIVE);
  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::ZERO);
}

#[test]
fn test_ror_rotate_right_accumulator() {
  let mut cpu = CPU::new();
  cpu.register_a = 0b1100_0011;

  cpu.load_and_run(vec![0x6A, 0x00]);

  assert_eq!(0b1110_0001, cpu.register_a);
  assert_eq!(CpuFlags::CARRY, cpu.status & CpuFlags::CARRY);
  assert_eq!(CpuFlags::NEGATIVE, cpu.status & CpuFlags::NEGATIVE);
  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::ZERO);
}

#[test]
fn test_ror_rotate_right_zero_page() {
  let mut cpu = CPU::new();
  cpu.mem_write(0x0034, 0b0010_1010);

  cpu.load_and_run(vec![0x66, 0x34, 0x00]);

  assert_eq!(0b0001_0101, cpu.mem_read_u16(0x0034));
  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::CARRY);
  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::NEGATIVE);
  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::ZERO);
}

#[test]
fn test_pla_pull_accumulator_from_stack() {
  let mut cpu = CPU::new();

  cpu.mem_write(0x01FF, 0x42);
  cpu.stack_pointer = 0xFE;
  cpu.load_and_run(vec![0x68, 0x00]);

  assert_eq!(0xFF, cpu.stack_pointer);
  assert_eq!(0x42, cpu.register_a);
}

#[test]
fn test_pla_3rd_stack_pop() {
  let mut cpu = CPU::new();

  cpu.mem_write(0x01FF, 0x42);
  cpu.mem_write(0x01FE, 0x41);
  cpu.mem_write(0x01FD, 0x40);
  cpu.stack_pointer = 0xFC;
  cpu.load_and_run(vec![0x68, 0x00]);

  assert_eq!(0xFD, cpu.stack_pointer);
  assert_eq!(0x40, cpu.register_a);
}

#[test]
fn test_pla_stack_overflow() {
  let mut cpu = CPU::new();

  cpu.mem_write(0x01FF, 0x41);
  cpu.mem_write(0x0100, 0x42);
  cpu.stack_pointer = 0xFF;
  cpu.load_and_run(vec![0x68, 0x00]);

  assert_eq!(0x00, cpu.stack_pointer);
  assert_eq!(0x42, cpu.register_a);
}

#[test]
fn test_reset() {
  let mut cpu = CPU::new();

  cpu.reset();

  assert_eq!(CpuFlags::INTERRUPT_DISABLE, cpu.status & CpuFlags::INTERRUPT_DISABLE);
  assert_eq!(CpuFlags::BREAK2, cpu.status & CpuFlags::BREAK2);
}

#[test]
fn test_sta_zero_page() {
  let mut cpu = CPU::new();
  cpu.register_a = 0x42;

  cpu.load_and_run(vec![0x85, 0x21, 0x00]);

  assert_eq!(0x42, cpu.mem_read_u16(0x0021));
}

#[test]
fn test_sta_zero_page_x() {
  let mut cpu = CPU::new();
  cpu.register_a = 0x42;
  cpu.register_x = 0x80;

  cpu.load_and_run(vec![0x95, 0x09, 0x00]);

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

  assert_eq!(0x42, cpu.mem_read_u16(0x1237));
}

#[test]
fn test_sta_absolute_y() {
  let mut cpu = CPU::new();
  cpu.register_a = 0x42;
  cpu.register_y = 0x14;

  cpu.load_and_run(vec![0x99, 0x23, 0x12, 0x00]);

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

  assert_eq!(0x42, cpu.mem_read_u16(0x0705));
}

#[test]
fn test_sta_indirect_y() {
  let mut cpu = CPU::new();
  cpu.register_a = 0x42;
  cpu.register_y = 0x10;
  cpu.mem_write(0x0011, 0x21);

  cpu.load_and_run(vec![0x91, 0x11, 0x00]);

  assert_eq!(0x42, cpu.mem_read_u16(0x0031));
}

#[test]
fn test_stx_zero_page() {
  let mut cpu = CPU::new();
  cpu.register_x = 0x42;
  cpu.load_and_run(vec![0x86, 0x02, 0x00]);

  assert_eq!(0x42, cpu.mem_read(0x02));
}

#[test]
fn test_sty_zero_page_x() {
  let mut cpu = CPU::new();
  cpu.register_x = 0x20;
  cpu.register_y = 0x42;

  cpu.load_and_run(vec![0x94, 0x01, 0x00]);

  assert_eq!(0x42, cpu.mem_read(0x21));
}

#[test]
fn test_tax_transfer_a_to_x() {
  let mut cpu = CPU::new();
  cpu.register_a = 10;

  cpu.load_and_run(vec![0xAA, 0x00]);

  assert_eq!(10, cpu.register_x)
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