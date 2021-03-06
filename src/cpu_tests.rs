use crate::Bus;
use crate::cartridge_tests::create_test_rom;
use crate::cpu::{MyCPU, CpuFlags, MyMem};

const START_ADDR: u16 = 0x0600;

fn init_cpu() -> MyCPU {
  let mut cpu = MyCPU::new(Bus::new(create_test_rom()));
  cpu.program_counter = START_ADDR;
  cpu
}

#[test]
fn test_5_ops_working_together() {
  let mut cpu = init_cpu();
  cpu.load_and_run(vec![0xA9, 0xC0, 0xAA, 0xE8]);

  cpu.dump_non_empty_memory();
  assert_eq!(0xC1, cpu.register_x);
}

#[test]
fn test_adc_add_with_no_carry() {
  let mut cpu = init_cpu();
  cpu.register_a = 0x21;

  cpu.load_and_run(vec![0x69, 0x42]);

  assert_eq!(0x63, cpu.register_a);
  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::CARRY);
  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::OVERFLOW);
}

#[test]
fn test_adc_add_with_resulting_carry() {
  let mut cpu = init_cpu();
  cpu.register_a = 0xF0;
  cpu.mem_write(0x42, 0x21);

  cpu.load_and_run(vec![0x65, 0x42]);

  assert_eq!(0x11, cpu.register_a);
  assert_eq!(CpuFlags::CARRY, cpu.status & CpuFlags::CARRY);
  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::OVERFLOW);
}

#[test]
fn test_adc_add_with_carry_in() {
  let mut cpu = init_cpu();
  cpu.register_a = 0x42;
  cpu.status.insert(CpuFlags::CARRY);
  cpu.mem_write(0x42, 0x21);

  cpu.load_and_run(vec![0x65, 0x42]);

  assert_eq!(0x64, cpu.register_a);
  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::CARRY);
  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::OVERFLOW);
}

// overflow adc & sbc examples, see: http://www.6502.org/tutorials/vflag.html

#[test]
fn test_adc_add_with_positive_overflow() {
  let mut cpu = init_cpu();

  // 127 + 1 = 128
  cpu.register_a = 0x7F;
  cpu.load_and_run(vec![0x69, 0x01]);

  assert_eq!(0x80, cpu.register_a);
  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::CARRY);
  assert_eq!(CpuFlags::OVERFLOW, cpu.status & CpuFlags::OVERFLOW);
}

#[test]
fn test_adc_add_no_overflow() {
  let mut cpu = init_cpu();

  // 1 + -1 = 128
  cpu.register_a = 0x01;
  cpu.load_and_run(vec![0x69, 0xFF]);

  assert_eq!(0x00, cpu.register_a);
  assert_eq!(CpuFlags::CARRY, cpu.status & CpuFlags::CARRY);
  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::OVERFLOW);
}

#[test]
fn test_adc_add_with_overflow_negative() {
  let mut cpu = init_cpu();

  // -128 + -1 = -129
  cpu.register_a = 0x80;
  cpu.load_and_run(vec![0x69, 0xFF]);

  assert_eq!(0x7F, cpu.register_a);
  assert_eq!(CpuFlags::CARRY, cpu.status & CpuFlags::CARRY);
  assert_eq!(CpuFlags::OVERFLOW, cpu.status & CpuFlags::OVERFLOW);
}

#[test]
fn test_sdc_subtract_with_default_carry() {
  let mut cpu = init_cpu();
  cpu.register_a = 0x42;
  cpu.mem_write(0x1412, 0x21);

  cpu.load_and_run(vec![0xED, 0x12, 0x14]);

  assert_eq!(0x20, cpu.register_a);
  assert_eq!(CpuFlags::CARRY, cpu.status & CpuFlags::CARRY);
  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::OVERFLOW);
}

#[test]
fn test_sdc_subtract_with_carry_in() {
  let mut cpu = init_cpu();
  cpu.register_a = 0x42;
  cpu.status.insert(CpuFlags::CARRY);
  cpu.mem_write(0x1412, 0x21);

  cpu.load_and_run(vec![0xED, 0x12, 0x14]);

  // previously: assert_eq!(0x22, cpu.register_a); - wrong for snake game
  assert_eq!(0x21, cpu.register_a);
  assert_eq!(CpuFlags::CARRY, cpu.status & CpuFlags::CARRY);
  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::OVERFLOW);
}

#[test]
fn test_sbc_subtract_without_flags() {
  let mut cpu= init_cpu();

  // 0 - 1 = -1
  cpu.register_a = 0x00;
  cpu.load_and_run(vec![0xE9, 0x01]);

  // previously: assert_eq!(0xFF, cpu.register_a); - wrong for snake game
  assert_eq!(0xFE, cpu.register_a);
  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::CARRY);
  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::OVERFLOW);
}

#[test]
fn test_sbc_subtract_with_overflow() {
  let mut cpu= init_cpu();

  // 127 - -1 = 128
  cpu.register_a = 0x7F;
  // previously: cpu.load_and_run(vec![0xE9, 0xFF]); - wrong for snake game
  cpu.load_and_run(vec![0xE9, 0xFE]);

  assert_eq!(0x80, cpu.register_a);
  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::CARRY);
  assert_eq!(CpuFlags::OVERFLOW, cpu.status & CpuFlags::OVERFLOW);
}

#[test]
fn test_and_acc_and_immediate_memory() {
  let mut cpu= init_cpu();

  cpu.register_a = 0b1001_1001;
  cpu.load_and_run(vec![0x29, 0b0111_0111]);

  assert_eq!(0b0001_0001, cpu.register_a);
  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::ZERO);
  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::NEGATIVE);
}

#[test]
fn test_and_acc_and_absolute_memory() {
  let mut cpu= init_cpu();

  cpu.register_a = 0b1001_1001;
  cpu.mem_write(0x1234, 0b1000_0011);
  cpu.load_and_run(vec![0x2D, 0x34, 0x12]);

  assert_eq!(0b1000_0001, cpu.register_a);
  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::ZERO);
  assert_eq!(CpuFlags::NEGATIVE, cpu.status & CpuFlags::NEGATIVE);
}

#[test]
fn test_asl_arithmetic_shift_left_acc() {
  let mut cpu= init_cpu();

  cpu.register_a = 0b0101_1001;
  cpu.load_and_run(vec![0x0A]);

  assert_eq!(0b1011_0010, cpu.register_a);
  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::CARRY);
  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::ZERO);
  assert_eq!(CpuFlags::NEGATIVE, cpu.status & CpuFlags::NEGATIVE);
}

#[test]
fn test_asl_arithmetic_shift_left_zero() {
  let mut cpu= init_cpu();

  cpu.mem_write(0x0042, 0b1000_0000);
  cpu.load_and_run(vec![0x06, 0x42]);

  assert_eq!(0b0000_0000, cpu.mem_read(0x0042));
  assert_eq!(CpuFlags::CARRY, cpu.status & CpuFlags::CARRY);
  assert_eq!(CpuFlags::ZERO, cpu.status & CpuFlags::ZERO);
  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::NEGATIVE);
}

#[test]
fn test_bcc_branch_if_carry_clear_with_carry() {
  let mut cpu= init_cpu();

  cpu.status.insert(CpuFlags::CARRY);
  cpu.load_and_run(vec![0x90, 0x42]);

  assert_eq!(START_ADDR + 0x0003, cpu.program_counter);
}

#[test]
fn test_bcc_branch_if_carry_clear_no_carry() {
  let mut cpu= init_cpu();

  cpu.load_and_run(vec![0x90, 0x42]);

  assert_eq!(START_ADDR + 0x0045, cpu.program_counter);
}

#[test]
fn test_bcs_branch_if_carry_set_with_carry() {
  let mut cpu= init_cpu();

  cpu.status.insert(CpuFlags::CARRY);
  cpu.load_and_run(vec![0xB0, 0x42]);

  assert_eq!(START_ADDR + 0x0045, cpu.program_counter);
}

#[test]
fn test_beq_branch_if_equal_with_zero() {
  let mut cpu= init_cpu();

  cpu.status.insert(CpuFlags::ZERO);
  cpu.load_and_run(vec![0xF0, 0x42]);

  assert_eq!(START_ADDR + 0x0045, cpu.program_counter);
}

#[test]
fn test_bmi_branch_if_minus_with_negative() {
  let mut cpu= init_cpu();

  cpu.status.insert(CpuFlags::NEGATIVE);
  cpu.load_and_run(vec![0x30, 0x42]);

  assert_eq!(START_ADDR + 0x0045, cpu.program_counter);
}

#[test]
fn test_bne_branch_if_not_equal_without_zero() {
  let mut cpu= init_cpu();

  cpu.load_and_run(vec![0xD0, 0x42]);

  assert_eq!(START_ADDR + 0x0045, cpu.program_counter);
}

#[test]
fn test_bpl_branch_if_positive_without_negative() {
  let mut cpu= init_cpu();

  cpu.load_and_run(vec![0x10, 0x42]);

  assert_eq!(START_ADDR + 0x0045, cpu.program_counter);
}

#[test]
fn test_bvc_branch_if_overflow_clear_without_overflow() {
  let mut cpu= init_cpu();

  cpu.load_and_run(vec![0x50, 0x42]);

  assert_eq!(START_ADDR + 0x0045, cpu.program_counter);
}

#[test]
fn test_bpl_branch_if_overflow_set_with_overflow() {
  let mut cpu= init_cpu();

  cpu.status.insert(CpuFlags::OVERFLOW);
  cpu.load_and_run(vec![0x70, 0x42]);

  assert_eq!(START_ADDR + 0x0045, cpu.program_counter);
}

#[test]
fn test_bit_bit_test_result_negative_overflow() {
  let mut cpu= init_cpu();

  cpu.register_a = 0b0001_1001;
  cpu.mem_write(0x0042, 0b1111_0000);
  cpu.load_and_run(vec![0x24, 0x42]);

  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::ZERO);
  assert_eq!(CpuFlags::OVERFLOW, cpu.status & CpuFlags::OVERFLOW);
  assert_eq!(CpuFlags::NEGATIVE, cpu.status & CpuFlags::NEGATIVE);
  assert_eq!(0b0001_1001, cpu.register_a);
  assert_eq!(0b1111_0000, cpu.mem_read(0x0042));
}

#[test]
fn test_bit_bit_test_result_zero() {
  let mut cpu = init_cpu();

  cpu.register_a = 0b0000_1001;
  cpu.mem_write(0x0042, 0b0000_0000);
  cpu.load_and_run(vec![0x2C, 0x42, 0x00]);

  assert_eq!(CpuFlags::ZERO, cpu.status & CpuFlags::ZERO);
  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::OVERFLOW);
  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::NEGATIVE);
  assert_eq!(0b0000_1001, cpu.register_a);
  assert_eq!(0b0000_0000, cpu.mem_read(0x0042));
}

#[test]
fn test_clc_clear_carry_flag() {
  let mut cpu = init_cpu();

  cpu.status = CpuFlags::CARRY;
  cpu.load_and_run(vec![0x18]);

  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::CARRY);
}

#[test]
fn test_cld_clear_decimal_mode() {
  let mut cpu = init_cpu();

  cpu.status = CpuFlags::DECIMAL_MODE;
  cpu.load_and_run(vec![0xD8]);

  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::DECIMAL_MODE);
}

#[test]
fn test_cli_clear_interrupt_disable() {
  let mut cpu = init_cpu();

  cpu.status = CpuFlags::INTERRUPT_DISABLE;
  cpu.load_and_run(vec![0x58]);

  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::INTERRUPT_DISABLE);
}

#[test]
fn test_clv_clear_overflow_flag() {
  let mut cpu = init_cpu();

  cpu.status = CpuFlags::OVERFLOW;
  cpu.load_and_run(vec![0xB8]);

  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::OVERFLOW);
}

#[test]
fn test_cmp_compare_acc_with_memory_absolute_equal() {
  let mut cpu = init_cpu();

  cpu.mem_write(0x1142, 0x42);
  cpu.register_a = 0x42;
  cpu.load_and_run(vec![0xCD, 0x42, 0x11]);

  assert_eq!(CpuFlags::ZERO, cpu.status & CpuFlags::ZERO);
  assert_eq!(CpuFlags::CARRY, cpu.status & CpuFlags::CARRY);
  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::NEGATIVE);
}

#[test]
fn test_cmp_compare_acc_with_memory_absolute_lower() {
  let mut cpu = init_cpu();

  cpu.mem_write(0x1142, 0x42);
  cpu.register_a = 0x41;
  cpu.load_and_run(vec![0xCD, 0x42, 0x11]);

  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::ZERO);
  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::CARRY);
  assert_eq!(CpuFlags::NEGATIVE, cpu.status & CpuFlags::NEGATIVE);
}

#[test]
fn test_cmp_compare_acc_with_memory_absolute_greater() {
  let mut cpu = init_cpu();

  cpu.mem_write(0x1142, 0x42);
  cpu.register_a = 0x43;
  cpu.load_and_run(vec![0xCD, 0x42, 0x11]);

  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::ZERO);
  assert_eq!(CpuFlags::CARRY, cpu.status & CpuFlags::CARRY);
  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::NEGATIVE);
}

#[test]
fn test_cpx_compare_x_with_memory_absolute_equal() {
  let mut cpu = init_cpu();

  cpu.mem_write(0x1142, 0x42);
  cpu.register_x = 0x42;
  cpu.load_and_run(vec![0xEC, 0x42, 0x11]);

  assert_eq!(CpuFlags::ZERO, cpu.status & CpuFlags::ZERO);
  assert_eq!(CpuFlags::CARRY, cpu.status & CpuFlags::CARRY);
  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::NEGATIVE);
}

#[test]
fn test_cpy_compare_y_with_memory_absolute_equal() {
  let mut cpu = init_cpu();

  cpu.mem_write(0x1142, 0x42);
  cpu.register_y = 0x42;
  cpu.load_and_run(vec![0xCC, 0x42, 0x11]);

  assert_eq!(CpuFlags::ZERO, cpu.status & CpuFlags::ZERO);
  assert_eq!(CpuFlags::CARRY, cpu.status & CpuFlags::CARRY);
  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::NEGATIVE);
}

#[test]
fn test_dec_decrement_memory_zero_page() {
  let mut cpu = init_cpu();

  cpu.mem_write(0x0021, 0x0043);
  cpu.load_and_run(vec![0xC6, 0x21]);

  assert_eq!(0x0042, cpu.mem_read(0x0021))
}

#[test]
fn test_dec_decrement_memory_absolute() {
  let mut cpu = init_cpu();

  cpu.mem_write(0x1142, 0x43);
  cpu.load_and_run(vec![0xCE, 0x42, 0x11]);

  assert_eq!(0x42, cpu.mem_read(0x1142))
}

#[test]
fn test_dex_decrement_x() {
  let mut cpu = init_cpu();

  cpu.load_and_run(vec![0xCA]);

  assert_eq!(0xFF, cpu.register_x)
}

#[test]
fn test_dey_decrement_y() {
  let mut cpu = init_cpu();

  cpu.load_and_run(vec![0x88]);

  assert_eq!(0xFF, cpu.register_y)
}

#[test]
fn test_eor_exclusive_or_acc_with_immediate() {
  let mut cpu = init_cpu();

  cpu.register_a = 0b0000_1111;
  cpu.load_and_run(vec![0x49, 0b0101_0101]);

  assert_eq!(0b0101_1010, cpu.register_a);
  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::ZERO);
  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::NEGATIVE);
}

#[test]
fn test_eor_exclusive_or_acc_with_absolute() {
  let mut cpu = init_cpu();

  cpu.register_a = 0b0000_1111;
  cpu.mem_write(0x1234, 0b0000_1111);
  cpu.load_and_run(vec![0x4D, 0x34, 0x12]);

  assert_eq!(0x00, cpu.register_a);
  assert_eq!(CpuFlags::ZERO, cpu.status & CpuFlags::ZERO);
  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::NEGATIVE);
}

#[test]
fn test_inc_increment_memory_zero_page_x() {
  let mut cpu = init_cpu();

  cpu.mem_write(0x0021, 0x0041);
  cpu.register_x = 0x01;
  cpu.load_and_run(vec![0xF6, 0x20]);

  assert_eq!(0x0042, cpu.mem_read(0x0021))
}

#[test]
fn test_inx_increment_x() {
  let mut cpu = init_cpu();
  cpu.register_x = 10;

  cpu.load_and_run(vec![0xE8]);

  assert_eq!(11, cpu.register_x)
}

#[test]
fn test_inx_overflow() {
  let mut cpu = init_cpu();
  cpu.register_x = 0xFF;

  cpu.load_and_run(vec![0xE8, 0xE8]);

  assert_eq!(1, cpu.register_x)
}

#[test]
fn test_iny_increment_y() {
  let mut cpu = init_cpu();
  cpu.load_and_run(vec![0xC8]);

  assert_eq!(1, cpu.register_y);
}

#[test]
fn test_jmp_jump_absolute() {
  let mut cpu = init_cpu();

  cpu.load_and_run(vec![0x4C, 0x34, 0x12]);

  assert_eq!(0x1235, cpu.program_counter);
}

#[test]
fn test_jmp_jump_indirect_no_page_boundary() {
  let mut cpu = init_cpu();

  cpu.mem_write(0x1700, 0x12);
  cpu.mem_write(0x1701, 0x14);
  cpu.load_and_run(vec![0x6C, 0x00, 0x17]);

  assert_eq!(0x1413, cpu.program_counter);
}

#[test]
fn test_jmp_jump_indirect_buggy_page_boundary() {
  // For example if address $3000 contains $40, $30FF contains $80, and $3100 contains $50,
  // the result of JMP ($30FF) will be a transfer of control to $4080 rather than $5080 as you intended
  // i.e. the 6502 took the low byte of the address from $30FF and the high byte from $3000.
  let mut cpu = init_cpu();

  cpu.mem_write(0x1200, 0x17);
  cpu.mem_write(0x12FF, 0x80);
  cpu.mem_write(0x1300, 0x18);
  cpu.load_and_run(vec![0x6C, 0xFF, 0x12]);

  assert_ne!(0x1881, cpu.program_counter);
  assert_eq!(0x1781, cpu.program_counter);
}

#[test]
fn test_jsr_jump_to_subroutine() {
  let mut cpu = init_cpu();

  cpu.load_and_run(vec![0x20, 0x12, 0x14]);

  assert_eq!(0xFD, cpu.stack_pointer);
  let finish_addr = START_ADDR + 2;
  let low_start = (finish_addr & 0xFF) as u8;
  let high_start = (finish_addr >> 8) as u8;
  assert_eq!(high_start, cpu.mem_read(0x01FF));
  assert_eq!(low_start, cpu.mem_read(0x01FE));
  assert_eq!(0x1413, cpu.program_counter);
}

#[test]
fn test_rts_return_from_subroutine() {
  let mut cpu = init_cpu();

  cpu.stack_pointer = 0xFD;
  cpu.mem_write(0x01FF, 0x07);
  cpu.mem_write(0x01FE, 0x21);
  cpu.load_and_run(vec![0x60]);

  cpu.dump_non_empty_memory();
  assert_eq!(0x0721 + 1/* RTS */ + 1/*0x00*/, cpu.program_counter);
}

#[test]
fn test_lda_immediate_load_data() {
  let mut cpu = init_cpu();
  cpu.load_and_run(vec![0xA9, 0x05]);

  assert_eq!(0x05, cpu.register_a);
  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::ZERO);
  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::NEGATIVE);
}

#[test]
fn test_lda_zero_flag() {
  let mut cpu = init_cpu();

  cpu.load_and_run(vec![0xA9, 0x00]);

  assert_eq!(CpuFlags::ZERO, cpu.status & CpuFlags::ZERO);
}

#[test]
fn test_lda_from_memory() {
  let mut cpu = init_cpu();
  cpu.mem_write(0x10, 0x55);

  cpu.load_and_run(vec![0xA5, 0x10]);

  assert_eq!(0x55, cpu.register_a);
}

#[test]
fn test_ldx_immediate() {
  let mut cpu = init_cpu();

  cpu.load_and_run(vec![0xA2, 0x42]);

  assert_eq!(0x42, cpu.register_x);
}

#[test]
fn test_ldy_immediate() {
  let mut cpu = init_cpu();

  cpu.load_and_run(vec![0xA0, 0x42]);

  assert_eq!(0x42, cpu.register_y);
}

#[test]
fn test_lsr_logical_shift_right_acc() {
  let mut cpu = init_cpu();

  cpu.register_a = 0b0100_0011;
  cpu.load_and_run(vec![0x4A]);

  assert_eq!(0b0010_0001, cpu.register_a);
  assert_eq!(CpuFlags::CARRY, cpu.status & CpuFlags::CARRY);
  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::ZERO);
  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::NEGATIVE);
}

#[test]
fn test_lsr_logical_shift_right_absolute() {
  let mut cpu = init_cpu();

  cpu.mem_write_u16(0x1234, 0b0000_0001);
  cpu.load_and_run(vec![0x4E, 0x34, 0x12]);

  assert_eq!(0b0000_0000, cpu.mem_read(0x1234));
  assert_eq!(CpuFlags::CARRY, cpu.status & CpuFlags::CARRY);
  assert_eq!(CpuFlags::ZERO, cpu.status & CpuFlags::ZERO);
  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::NEGATIVE);
}

#[test]
fn test_nop_no_operation() {
  let mut cpu = init_cpu();

  cpu.load_and_run(vec![0xEA]);

  assert_eq!(START_ADDR + 0x0002, cpu.program_counter);
  assert_eq!(CpuFlags::BREAK2 | CpuFlags::INTERRUPT_DISABLE, cpu.status);
  assert_eq!(0xFF, cpu.stack_pointer);
}

#[test]
fn test_ora_logical_inclusive_or_acc_against_immediate() {
  let mut cpu = init_cpu();

  cpu.register_a = 0b1000_0000;
  cpu.load_and_run(vec![0x09, 0b0000_1111]);

  assert_eq!(0b1000_1111, cpu.register_a);
  assert_eq!(CpuFlags::NEGATIVE, cpu.status & CpuFlags::NEGATIVE);
}

#[test]
fn test_ora_logical_inclusive_or_acc_against_zeropage() {
  let mut cpu = init_cpu();

  cpu.register_a = 0b1000_0000;
  cpu.mem_write(0x1234, 0b0000_1111);
  cpu.load_and_run(vec![0x0D, 0x34, 0x12]);

  assert_eq!(0b1000_1111, cpu.register_a);
}

#[test]
fn test_pha_push_accumulator_to_stack() {
  let mut cpu = init_cpu();

  cpu.register_a = 0x42;
  cpu.load_and_run(vec![0x48]);

  assert_eq!(0xFE, cpu.stack_pointer);
  assert_eq!(0x42, cpu.mem_read(0x01FF));
}

#[test]
fn test_pha_3_stack_pushes() {
  let mut cpu = init_cpu();

  cpu.load_and_run(vec![0xA9, 0x20, 0x48, 0xA9, 0x21, 0x48, 0xA9, 0x22, 0x48]);

  // cpu.dump_non_empty_memory();
  assert_eq!(0xFC, cpu.stack_pointer);
  assert_eq!(0x20, cpu.mem_read(0x01FF));
  assert_eq!(0x21, cpu.mem_read(0x01FE));
  assert_eq!(0x22, cpu.mem_read(0x01FD));
}

#[test]
fn test_pha_stack_overflow() {
  let mut cpu = init_cpu();

  cpu.register_a = 0x42;
  cpu.stack_pointer = 0x00;
  cpu.load_and_run(vec![0x48]);

  assert_eq!(0xFF, cpu.stack_pointer);
  assert_eq!(0x42, cpu.mem_read(0x0100));
}

#[test]
fn test_php_push_processor_status() {
  let mut cpu = init_cpu();
  cpu.status = CpuFlags::NEGATIVE | CpuFlags::CARRY | CpuFlags::INTERRUPT_DISABLE;

  cpu.load_and_run(vec![0x08]);

  assert_eq!(0xFE, cpu.stack_pointer);
  assert_eq!(0xB5, cpu.mem_read(0x01FF)); // 0x04 + 0x01 + 0x80 + 0x10 + 0x20 = 0xB5 (BREAK + BREAK2)
}

#[test]
fn test_plp_pull_processor_status() {
  let mut cpu = init_cpu();
  cpu.mem_write(0x01FF, 0x85);  // 0x04 + 0x01 + 0x80 = 0x85
  cpu.stack_pointer = 0xFE;

  cpu.load_and_run(vec![0x28]);

  assert_eq!(0xFF, cpu.stack_pointer);
  // additional break as program stopped
  assert_eq!(CpuFlags::NEGATIVE | CpuFlags::CARRY | CpuFlags::INTERRUPT_DISABLE | CpuFlags::BREAK2,
             cpu.status);
}

#[test]
fn test_rti_return_from_interrupt() {
  let mut cpu = init_cpu();
  cpu.mem_write(0x0181, 0x85);  // 0x04 + 0x01 + 0x80 = 0x85
  cpu.mem_write(0x0182, 0x34);
  cpu.mem_write(0x0183, 0x12);
  cpu.stack_pointer = 0x80;

  cpu.load_and_run(vec![0x40]);

  assert_eq!(0x83, cpu.stack_pointer);
  assert_eq!(0x1234 + 1, cpu.program_counter);
  assert_eq!(CpuFlags::NEGATIVE | CpuFlags::CARRY | CpuFlags::INTERRUPT_DISABLE | CpuFlags::BREAK2,
             cpu.status);
}

#[test]
fn test_rol_rotate_left_accumulator() {
  let mut cpu = init_cpu();
  cpu.register_a = 0b1100_0011;

  cpu.load_and_run(vec![0x2A]);

  assert_eq!(0b1000_0111, cpu.register_a);
  assert_eq!(CpuFlags::CARRY, cpu.status & CpuFlags::CARRY);
  assert_eq!(CpuFlags::NEGATIVE, cpu.status & CpuFlags::NEGATIVE);
  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::ZERO);
}

#[test]
fn test_rol_rotate_left_accumulator_zero() {
  let mut cpu = init_cpu();
  cpu.register_a = 0b0000_0000;

  cpu.load_and_run(vec![0x2A]);

  assert_eq!(0b0000_0000, cpu.register_a);
  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::CARRY);
  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::NEGATIVE);
  assert_eq!(CpuFlags::ZERO, cpu.status & CpuFlags::ZERO);
}

#[test]
fn test_rol_rotate_left_absolute() {
  let mut cpu = init_cpu();
  cpu.mem_write(0x1234, 0b0010_1010);

  cpu.load_and_run(vec![0x2E, 0x34, 0x12]);

  assert_eq!(0b0101_0100, cpu.mem_read_u16(0x1234));
  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::CARRY);
  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::NEGATIVE);
  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::ZERO);
}

#[test]
fn test_ror_rotate_right_accumulator() {
  let mut cpu = init_cpu();
  cpu.register_a = 0b1100_0011;

  cpu.load_and_run(vec![0x6A]);

  assert_eq!(0b1110_0001, cpu.register_a);
  assert_eq!(CpuFlags::CARRY, cpu.status & CpuFlags::CARRY);
  assert_eq!(CpuFlags::NEGATIVE, cpu.status & CpuFlags::NEGATIVE);
  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::ZERO);
}

#[test]
fn test_ror_rotate_right_zero_page() {
  let mut cpu = init_cpu();
  cpu.mem_write(0x0034, 0b0010_1010);

  cpu.load_and_run(vec![0x66, 0x34]);

  assert_eq!(0b0001_0101, cpu.mem_read_u16(0x0034));
  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::CARRY);
  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::NEGATIVE);
  assert_eq!(CpuFlags::empty(), cpu.status & CpuFlags::ZERO);
}

#[test]
fn test_pla_pull_accumulator_from_stack() {
  let mut cpu = init_cpu();

  cpu.mem_write(0x01FF, 0x42);
  cpu.stack_pointer = 0xFE;
  cpu.load_and_run(vec![0x68]);

  assert_eq!(0xFF, cpu.stack_pointer);
  assert_eq!(0x42, cpu.register_a);
}

#[test]
fn test_pla_3rd_stack_pop() {
  let mut cpu = init_cpu();

  cpu.mem_write(0x01FF, 0x42);
  cpu.mem_write(0x01FE, 0x41);
  cpu.mem_write(0x01FD, 0x40);
  cpu.stack_pointer = 0xFC;
  cpu.load_and_run(vec![0x68]);

  assert_eq!(0xFD, cpu.stack_pointer);
  assert_eq!(0x40, cpu.register_a);
}

#[test]
fn test_pla_stack_overflow() {
  let mut cpu = init_cpu();

  cpu.mem_write(0x01FF, 0x41);
  cpu.mem_write(0x0100, 0x42);
  cpu.stack_pointer = 0xFF;
  cpu.load_and_run(vec![0x68]);

  assert_eq!(0x00, cpu.stack_pointer);
  assert_eq!(0x42, cpu.register_a);
}

#[test]
fn test_reset() {
  let mut cpu = init_cpu();

  cpu.reset();

  assert_eq!(CpuFlags::INTERRUPT_DISABLE, cpu.status & CpuFlags::INTERRUPT_DISABLE);
  assert_eq!(CpuFlags::BREAK2, cpu.status & CpuFlags::BREAK2);
}

#[test]
fn test_sec_set_carry_flag() {
  let mut cpu = init_cpu();

  cpu.load_and_run(vec![0x38]);

  assert_eq!(CpuFlags::CARRY, cpu.status & CpuFlags::CARRY);
}

#[test]
fn test_sed_set_decimal_flag() {
  let mut cpu = init_cpu();

  cpu.load_and_run(vec![0xF8]);

  assert_eq!(CpuFlags::DECIMAL_MODE, cpu.status & CpuFlags::DECIMAL_MODE);
}

#[test]
fn test_sei_set_interrupt_disable() {
  let mut cpu = init_cpu();

  cpu.load_and_run(vec![0x78]);

  assert_eq!(CpuFlags::INTERRUPT_DISABLE, cpu.status & CpuFlags::INTERRUPT_DISABLE);
}

#[test]
fn test_sta_zero_page() {
  let mut cpu = init_cpu();
  cpu.register_a = 0x42;

  cpu.load_and_run(vec![0x85, 0x21]);

  assert_eq!(0x42, cpu.mem_read_u16(0x0021));
}

#[test]
fn test_sta_zero_page_x() {
  let mut cpu = init_cpu();
  cpu.register_a = 0x42;
  cpu.register_x = 0x80;

  cpu.load_and_run(vec![0x95, 0x09]);

  assert_eq!(0x42, cpu.mem_read_u16(0x0089));
}

#[test]
fn test_sta_absolute() {
  let mut cpu = init_cpu();
  cpu.register_a = 0x42;

  cpu.load_and_run(vec![0x8D, 0x00, 0x02]);

  assert_eq!(0x42, cpu.mem_read_u16(0x0200));
}

#[test]
fn test_sta_absolute_x() {
  let mut cpu = init_cpu();
  cpu.register_a = 0x42;
  cpu.register_x = 0x14;

  cpu.load_and_run(vec![0x9D, 0x23, 0x12]);

  assert_eq!(0x42, cpu.mem_read_u16(0x1237));
}

#[test]
fn test_sta_absolute_y() {
  let mut cpu = init_cpu();
  cpu.register_a = 0x42;
  cpu.register_y = 0x14;

  cpu.load_and_run(vec![0x99, 0x23, 0x12]);

  assert_eq!(0x42, cpu.mem_read_u16(0x1237));
}

#[test]
fn test_sta_indirect_x() {
  let mut cpu = init_cpu();
  cpu.register_a = 0x42;
  cpu.register_x = 0x10;
  cpu.mem_write(0x0021, 0x05);
  cpu.mem_write(0x0022, 0x07);

  cpu.load_and_run(vec![0x81, 0x11]);

  assert_eq!(0x42, cpu.mem_read_u16(0x0705));
}

#[test]
fn test_sta_indirect_y() {
  let mut cpu = init_cpu();
  cpu.register_a = 0x42;
  cpu.register_y = 0x10;
  cpu.mem_write(0x0011, 0x21);

  cpu.load_and_run(vec![0x91, 0x11]);

  assert_eq!(0x42, cpu.mem_read_u16(0x0031));
}

#[test]
fn test_stx_zero_page() {
  let mut cpu = init_cpu();
  cpu.register_x = 0x42;
  cpu.load_and_run(vec![0x86, 0x02]);

  assert_eq!(0x42, cpu.mem_read(0x02));
}

#[test]
fn test_sty_zero_page_x() {
  let mut cpu = init_cpu();
  cpu.register_x = 0x20;
  cpu.register_y = 0x42;

  cpu.load_and_run(vec![0x94, 0x01]);

  assert_eq!(0x42, cpu.mem_read(0x21));
}

#[test]
fn test_tax_transfer_a_to_x() {
  let mut cpu = init_cpu();
  cpu.register_a = 10;

  cpu.load_and_run(vec![0xAA]);

  assert_eq!(10, cpu.register_x)
}

#[test]
fn test_tay_transfer_a_to_y() {
  let mut cpu = init_cpu();
  cpu.register_a = 0x42;

  cpu.load_and_run(vec![0xA8]);

  assert_eq!(0x42, cpu.register_y)
}

#[test]
fn test_txs_transfer_x_to_stack_pointer() {
  let mut cpu = init_cpu();
  cpu.register_x = 0x42;

  cpu.load_and_run(vec![0x9A]);

  assert_eq!(0x42, cpu.stack_pointer);
}

#[test]
fn test_tsx_transfer_stack_pointer_to_x() {
  let mut cpu = init_cpu();
  cpu.stack_pointer = 0x42;

  cpu.load_and_run(vec![0xBA]);

  assert_eq!(0x42, cpu.register_x);
}

#[test]
fn test_txa_transfer_x_to_acc() {
  let mut cpu = init_cpu();
  cpu.register_x = 0x42;

  cpu.load_and_run(vec![0x8A]);

  assert_eq!(0x42, cpu.register_a);
}

#[test]
fn test_tya_transfer_y_to_acc() {
  let mut cpu = init_cpu();
  cpu.register_y = 0x42;
  cpu.load_and_run(vec![0x98]);

  assert_eq!(0x42, cpu.register_a);
}