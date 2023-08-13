use rust_NES::{
    cpu::{AddressingMode, CARRY, CPU, DECIMAL_MODE, INTERRUPT_DISABLE, NEGATIVE, OVERFLOW, ZERO},
    opcode::OPCODES_MAP,
};

#[test]
fn test_5_ops_working_together() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0xc0, 0xaa, 0xe8, 00]);

    assert_eq!(cpu.register_x, 0xc1);
}

//ADC
#[test]
fn test_adc_from_memory() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x10, 0x55);

    cpu.load_and_run(vec![0xa5, 0x10, 0x69, 0xff, 0x00]);

    assert_eq!(cpu.accumulator, 0x54);
    assert!(cpu.status & CARRY != 0);
}

#[test]
fn test_adc_flags() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0xc0, 0x69, 0xc4, 0x00]);

    assert_eq!(cpu.accumulator, 0x84);
    assert!(cpu.status & CARRY == 1);
    assert!(cpu.status & NEGATIVE == 1);
}

#[test]
fn test_adc_address_modes() {
    // 0x69 | 0x65 | 0x75 | 0x6d | 0x7d | 0x79 | 0x61 | 0x71

    let adc_immediate_opcode = *OPCODES_MAP
        .get(&0x69)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x69));
    let adc_zero_page_opcode = *OPCODES_MAP
        .get(&0x65)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x65));
    let adc_zero_page_x_opcode = *OPCODES_MAP
        .get(&0x75)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x75));
    let adc_absolute_opcode = *OPCODES_MAP
        .get(&0x6d)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x6d));
    let adc_absolute_x_opcode = *OPCODES_MAP
        .get(&0x7d)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x7d));
    let adc_absolute_y_opcode = *OPCODES_MAP
        .get(&0x79)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x79));
    let adc_indirect_x_opcode = *OPCODES_MAP
        .get(&0x61)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x61));
    let adc_indirect_y_opcode = *OPCODES_MAP
        .get(&0x71)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x71));

    assert_eq!(adc_immediate_opcode.mode, AddressingMode::Immediate);
    assert_eq!(adc_zero_page_opcode.mode, AddressingMode::ZeroPage);
    assert_eq!(adc_zero_page_x_opcode.mode, AddressingMode::ZeroPage_X);
    assert_eq!(adc_absolute_opcode.mode, AddressingMode::Absolute);
    assert_eq!(adc_absolute_x_opcode.mode, AddressingMode::Absolute_X);
    assert_eq!(adc_absolute_y_opcode.mode, AddressingMode::Absolute_Y);
    assert_eq!(adc_indirect_x_opcode.mode, AddressingMode::Indirect_X);
    assert_eq!(adc_indirect_y_opcode.mode, AddressingMode::Indirect_Y);
}

//AND
#[test]
fn test_and_from_memory() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x10, 0x55);

    cpu.load_and_run(vec![0xa5, 0x10, 0x29, 0xaa, 0x00]);

    assert_eq!(cpu.accumulator, 0x00);
}

#[test]
fn test_and_addressing_modes() {
    // 0x29 | 0x25 | 0x35 | 0x2D | 0x3D | 0x39 | 0x21 | 0x31

    let and_immediate_opcode = *OPCODES_MAP
        .get(&0x29)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x29));
    let and_zero_page_opcode = *OPCODES_MAP
        .get(&0x25)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x25));
    let and_zero_page_x_opcode = *OPCODES_MAP
        .get(&0x35)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x35));
    let and_absolute_opcode = *OPCODES_MAP
        .get(&0x2D)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x2D));
    let and_absolute_x_opcode = *OPCODES_MAP
        .get(&0x3D)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x3D));
    let and_absolute_y_opcode = *OPCODES_MAP
        .get(&0x39)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x39));
    let and_indirect_x_opcode = *OPCODES_MAP
        .get(&0x21)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x21));
    let and_indirect_y_opcode = *OPCODES_MAP
        .get(&0x31)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x31));

    assert_eq!(and_immediate_opcode.mode, AddressingMode::Immediate);
    assert_eq!(and_zero_page_opcode.mode, AddressingMode::ZeroPage);
    assert_eq!(and_zero_page_x_opcode.mode, AddressingMode::ZeroPage_X);
    assert_eq!(and_absolute_opcode.mode, AddressingMode::Absolute);
    assert_eq!(and_absolute_x_opcode.mode, AddressingMode::Absolute_X);
    assert_eq!(and_absolute_y_opcode.mode, AddressingMode::Absolute_Y);
    assert_eq!(and_indirect_x_opcode.mode, AddressingMode::Indirect_X);
    assert_eq!(and_indirect_y_opcode.mode, AddressingMode::Indirect_Y);
}

//ASL
#[test]
fn test_asl_from_memory() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x10, 0x55);

    cpu.load_and_run(vec![0xa5, 0x10, 0x0a, 0x00]);

    assert_eq!(cpu.accumulator, 0xAA);

    cpu.mem_write(0x10, 0xA8);

    cpu.load_and_run(vec![0x06, 0x10, 0xa5, 0x10, 0x00]);
    assert_eq!(cpu.accumulator, 0x50);
    assert!(cpu.status & CARRY != 0);
}

#[test]
fn test_asl_addressing_modes() {
    // 0x0a => self.asl_accumulator(),
    // 0x06 | 0x16 | 0x0e | 0x1e
    let asl_accumulator_opcode = *OPCODES_MAP
        .get(&0x0a)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x0a));
    let asl_zero_page_opcode = *OPCODES_MAP
        .get(&0x06)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x06));
    let asl_zero_page_x_opcode = *OPCODES_MAP
        .get(&0x16)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x16));
    let asl_absolute_opcode = *OPCODES_MAP
        .get(&0x0e)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x0e));
    let asl_absolute_x_opcode = *OPCODES_MAP
        .get(&0x1e)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x1e));

    assert_eq!(asl_accumulator_opcode.mode, AddressingMode::NoneAddressing);
    assert_eq!(asl_zero_page_opcode.mode, AddressingMode::ZeroPage);
    assert_eq!(asl_zero_page_x_opcode.mode, AddressingMode::ZeroPage_X);
    assert_eq!(asl_absolute_opcode.mode, AddressingMode::Absolute);
    assert_eq!(asl_absolute_x_opcode.mode, AddressingMode::Absolute_X);
}

//BCC
#[test]
fn test_bcc_addressing_modes() {
    // 0x90 => self.branch(self.status & CARRY == 0),

    let bcc_opcode = *OPCODES_MAP
        .get(&0x90)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x90));
    assert_eq!(bcc_opcode.mode, AddressingMode::NoneAddressing);
}
//BCS
#[test]
fn test_bcs_addressing_modes() {
    // 0xb0 => self.branch(self.status & CARRY != 0),

    let bcs_opcode = *OPCODES_MAP
        .get(&0xb0)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0xb0));
    assert_eq!(bcs_opcode.mode, AddressingMode::NoneAddressing);
}
//BEQ
#[test]
fn test_beq_addressing_modes() {
    // 0xf0 => self.branch(self.status & ZERO != 0),

    let beq_opcode = *OPCODES_MAP
        .get(&0xb0)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0xb0));
    assert_eq!(beq_opcode.mode, AddressingMode::NoneAddressing);
}
//BIT
#[test]
fn test_bit() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x10, 0xff);

    cpu.load_and_run(vec![0xa9, 0x00, 0x24, 0x10, 0x00]);
    assert!(cpu.status & NEGATIVE != 0);
    assert!(cpu.status & OVERFLOW != 0);
    assert!(cpu.status & ZERO != 0);
}

#[test]
fn test_bit_addressing_modes() {
    // 0x24 | 0x2c => self.bit(&opcode.mode),

    let bit_zero_page_opcode = *OPCODES_MAP
        .get(&0x24)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x24));
    let bit_absolute_opcode = *OPCODES_MAP
        .get(&0x2c)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x2c));
    assert_eq!(bit_zero_page_opcode.mode, AddressingMode::ZeroPage);
    assert_eq!(bit_absolute_opcode.mode, AddressingMode::Absolute);
}
//BMI
#[test]
fn test_bmi_addressing_modes() {
    // 0x30 => self.branch(self.status & NEGATIVE != 0),

    let bmi_opcode = *OPCODES_MAP
        .get(&0x30)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x30));
    assert_eq!(bmi_opcode.mode, AddressingMode::NoneAddressing);
}
//BNE
#[test]
fn test_bne_addressing_modes() {
    // 0xd0 => self.branch(self.status & ZERO == 0),

    let bne_opcode = *OPCODES_MAP
        .get(&0xd0)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0xd0));
    assert_eq!(bne_opcode.mode, AddressingMode::NoneAddressing);
}
//BPL
#[test]
fn test_bpl_addressing_modes() {
    // 0x10 => self.branch(self.status & NEGATIVE == 0),

    let bpl_opcode = *OPCODES_MAP
        .get(&0x10)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x10));
    assert_eq!(bpl_opcode.mode, AddressingMode::NoneAddressing);
}
//BRK

//BVC
#[test]
fn test_bvc_addressing_modes() {
    // 0x50 => self.branch(self.status & OVERFLOW == 0),

    let bvc_opcode = *OPCODES_MAP
        .get(&0x50)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x50));
    assert_eq!(bvc_opcode.mode, AddressingMode::NoneAddressing);
}
//BVS
#[test]
fn test_bvs_addressing_modes() {
    // 0x70 => self.branch(self.status & OVERFLOW != 0),

    let bvs_opcode = *OPCODES_MAP
        .get(&0x70)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x70));
    assert_eq!(bvs_opcode.mode, AddressingMode::NoneAddressing);
}
//CLC
#[test]
fn test_clc() {
    let mut cpu = CPU::new();
    cpu.status = 0xff;

    cpu.load_and_run(vec![0x18, 0x00]);
    assert!(cpu.status & CARRY == 0);
}
#[test]
fn test_clc_addressing_modes() {
    // 0x18 => self.reset_status_flag(CARRY),

    let clc_opcode = *OPCODES_MAP
        .get(&0x18)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x18));
    assert_eq!(clc_opcode.mode, AddressingMode::NoneAddressing);
}
//CLD
#[test]
fn test_cld() {
    let mut cpu = CPU::new();
    cpu.status = 0xff;

    cpu.load_and_run(vec![0xd8, 0x00]);
    assert!(cpu.status & DECIMAL_MODE == 0);
}

#[test]
fn test_cld_addressing_modes() {
    // 0xd8 => self.reset_status_flag(DECIMAL_MODE),

    let cld_opcode = *OPCODES_MAP
        .get(&0xd8)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0xd8));
    assert_eq!(cld_opcode.mode, AddressingMode::NoneAddressing);
}
//CLI
#[test]
fn test_cli() {
    let mut cpu = CPU::new();
    cpu.status = 0xff;

    cpu.load_and_run(vec![0x58, 0x00]);
    assert!(cpu.status & INTERRUPT_DISABLE == 0);
}

#[test]
fn test_cli_addressing_modes() {
    // 0x58 => self.reset_status_flag(INTERRUPT_DISABLE),

    let cli_opcode = *OPCODES_MAP
        .get(&0x58)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x58));
    assert_eq!(cli_opcode.mode, AddressingMode::NoneAddressing);
}
//CLV
#[test]
fn test_clv() {
    let mut cpu = CPU::new();
    cpu.status = 0xff;

    cpu.load_and_run(vec![0xb8, 0x00]);
    assert!(cpu.status & OVERFLOW == 0);
}

#[test]
fn test_clv_addressing_modes() {
    // 0xb8 => self.reset_status_flag(OVERFLOW),

    let clv_opcode = *OPCODES_MAP
        .get(&0xb8)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0xb8));
    assert_eq!(clv_opcode.mode, AddressingMode::NoneAddressing);
}
//CMP
#[test]
fn test_cmp() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0xff, 0xc9, 0xff]);

    assert!(cpu.status & CARRY != 0);
    assert!(cpu.status & NEGATIVE == 0);
}

#[test]
fn test_cmp_addressing_modes() {
    // 0xc9 | 0xc5 | 0xd5 | 0xcd | 0xdd | 0xd9 | 0xc1 | 0xd1

    let beq_immediate_opcode = *OPCODES_MAP
        .get(&0xc9)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0xc9));
    let beq_zero_page_opcode = *OPCODES_MAP
        .get(&0xc5)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0xc5));
    let beq_zero_page_x_opcode = *OPCODES_MAP
        .get(&0xd5)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0xd5));
    let beq_absolute_opcode = *OPCODES_MAP
        .get(&0xcd)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0xcd));
    let beq_absolute_x_opcode = *OPCODES_MAP
        .get(&0xdd)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0xdd));
    let beq_absolute_y_opcode = *OPCODES_MAP
        .get(&0xd9)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0xd9));
    let beq_indirect_x_opcode = *OPCODES_MAP
        .get(&0xc1)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0xc1));
    let beq_indirect_y_opcode = *OPCODES_MAP
        .get(&0xd1)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0xd1));

    assert_eq!(beq_immediate_opcode.mode, AddressingMode::Immediate);
    assert_eq!(beq_zero_page_opcode.mode, AddressingMode::ZeroPage);
    assert_eq!(beq_zero_page_x_opcode.mode, AddressingMode::ZeroPage_X);
    assert_eq!(beq_absolute_opcode.mode, AddressingMode::Absolute);
    assert_eq!(beq_absolute_x_opcode.mode, AddressingMode::Absolute_X);
    assert_eq!(beq_absolute_y_opcode.mode, AddressingMode::Absolute_Y);
    assert_eq!(beq_indirect_x_opcode.mode, AddressingMode::Indirect_X);
    assert_eq!(beq_indirect_y_opcode.mode, AddressingMode::Indirect_Y);
}
//CPX
#[test]
fn test_cpx_addressing_modes() {
    // 0xe0 | 0xe4 | 0xec

    let cpx_immediate_opcode = *OPCODES_MAP
        .get(&0xe0)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0xe0));
    let cpx_zero_page_opcode = *OPCODES_MAP
        .get(&0xe4)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0xe4));
    let cpx_absolute_opcode = *OPCODES_MAP
        .get(&0xec)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0xec));

    assert_eq!(cpx_immediate_opcode.mode, AddressingMode::Immediate);
    assert_eq!(cpx_zero_page_opcode.mode, AddressingMode::ZeroPage);
    assert_eq!(cpx_absolute_opcode.mode, AddressingMode::Absolute);
}
//CPY
#[test]
fn test_cpy_addressing_modes() {
    // 0xc0 | 0xc4 | 0xcc

    let cpy_immediate_opcode = *OPCODES_MAP
        .get(&0xc0)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0xc0));
    let cpy_zero_page_opcode = *OPCODES_MAP
        .get(&0xc4)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0xc4));
    let cpy_absolute_opcode = *OPCODES_MAP
        .get(&0xcc)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0xcc));

    assert_eq!(cpy_immediate_opcode.mode, AddressingMode::Immediate);
    assert_eq!(cpy_zero_page_opcode.mode, AddressingMode::ZeroPage);
    assert_eq!(cpy_absolute_opcode.mode, AddressingMode::Absolute);
}
//DEC
#[test]
fn test_dec() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x10, 0x00);
    cpu.load_and_run(vec![0xc6, 0x10, 0x00]);

    assert!(cpu.status & NEGATIVE != 0);
}

#[test]
fn test_dec_addressing_modes() {
    // 0xc6 | 0xd6 | 0xce | 0xde

    let dec_zero_page_opcode = *OPCODES_MAP
        .get(&0xc6)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0xc6));
    let dec_zero_page_x_opcode = *OPCODES_MAP
        .get(&0xd6)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0xd6));
    let dec_absolute_opcode = *OPCODES_MAP
        .get(&0xce)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0xce));
    let dec_absolute_x_opcode = *OPCODES_MAP
        .get(&0xde)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0xde));

    assert_eq!(dec_zero_page_opcode.mode, AddressingMode::ZeroPage);
    assert_eq!(dec_zero_page_x_opcode.mode, AddressingMode::ZeroPage_X);
    assert_eq!(dec_absolute_opcode.mode, AddressingMode::Absolute);
    assert_eq!(dec_absolute_x_opcode.mode, AddressingMode::Absolute_X);
}
//DEX
#[test]
fn test_dex() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa2, 0x00, 0xca, 0x00]);

    assert_eq!(cpu.register_x, 0xff);
    assert!(cpu.status & NEGATIVE != 0);
}

#[test]
fn test_dex_addressing_modes() {
    // 0xca => self.dex(),

    let dex_opcode = *OPCODES_MAP
        .get(&0xca)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0xca));
    assert_eq!(dex_opcode.mode, AddressingMode::NoneAddressing);
}
//DEY
#[test]
fn test_dey() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa0, 0x00, 0x88, 0x00]);

    assert_eq!(cpu.register_y, 0xff);
    assert!(cpu.status & NEGATIVE != 0);
}

#[test]
fn test_dey_addressing_modes() {
    // 0x88 => self.dey(),

    let dey_opcode = *OPCODES_MAP
        .get(&0x88)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x88));
    assert_eq!(dey_opcode.mode, AddressingMode::NoneAddressing);
}
//EOR
#[test]
fn test_eor() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0xaa, 0x49, 0x55, 0x00]);

    assert_eq!(cpu.accumulator, 0xff);
    assert!(cpu.status & NEGATIVE != 0);
}

#[test]
fn test_eor_addressing_modes() {
    // 0x49 | 0x45 | 0x55 | 0x4d | 0x5d | 0x59 | 0x41 | 0x51

    let eor_immediate_opcode = *OPCODES_MAP
        .get(&0x49)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x49));
    let eor_zero_page_opcode = *OPCODES_MAP
        .get(&0x45)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x45));
    let eor_zero_page_x_opcode = *OPCODES_MAP
        .get(&0x55)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x55));
    let eor_absolute_opcode = *OPCODES_MAP
        .get(&0x4d)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x4d));
    let eor_absolute_x_opcode = *OPCODES_MAP
        .get(&0x5d)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x5d));
    let eor_absolute_y_opcode = *OPCODES_MAP
        .get(&0x59)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x59));
    let eor_indirect_x_opcode = *OPCODES_MAP
        .get(&0x41)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x41));
    let eor_indirect_y_opcode = *OPCODES_MAP
        .get(&0x51)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x51));
    assert_eq!(eor_immediate_opcode.mode, AddressingMode::Immediate);
    assert_eq!(eor_zero_page_opcode.mode, AddressingMode::ZeroPage);
    assert_eq!(eor_zero_page_x_opcode.mode, AddressingMode::ZeroPage_X);
    assert_eq!(eor_absolute_opcode.mode, AddressingMode::Absolute);
    assert_eq!(eor_absolute_x_opcode.mode, AddressingMode::Absolute_X);
    assert_eq!(eor_absolute_y_opcode.mode, AddressingMode::Absolute_Y);
    assert_eq!(eor_indirect_x_opcode.mode, AddressingMode::Indirect_X);
    assert_eq!(eor_indirect_y_opcode.mode, AddressingMode::Indirect_Y);
}
//INC
#[test]
fn test_inc() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x10, 0xff);

    cpu.load_and_run(vec![0xe6, 0x10, 0x00]);
    assert!(cpu.status & ZERO != 0);
}

#[test]
fn test_inc_addressing_modes() {
    // 0xe6 | 0xf6 | 0xee | 0xfe

    let inc_zero_page_opcode = *OPCODES_MAP
        .get(&0xe6)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0xe6));
    let inc_zero_page_x_opcode = *OPCODES_MAP
        .get(&0xf6)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0xf6));
    let incn_absolute_opcode = *OPCODES_MAP
        .get(&0xee)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0xee));
    let inc_absolute_x_opcode = *OPCODES_MAP
        .get(&0xfe)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0xfe));

    assert_eq!(inc_zero_page_opcode.mode, AddressingMode::ZeroPage);
    assert_eq!(inc_zero_page_x_opcode.mode, AddressingMode::ZeroPage_X);
    assert_eq!(incn_absolute_opcode.mode, AddressingMode::Absolute);
    assert_eq!(inc_absolute_x_opcode.mode, AddressingMode::Absolute_X);
}
//INX
#[test]
fn test_inx_addressing_modes() {
    // 0xe8 => self.inx(),

    let inx_opcode = *OPCODES_MAP
        .get(&0xe8)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0xe8));
    assert_eq!(inx_opcode.mode, AddressingMode::NoneAddressing);
}

#[test]
fn test_inx_overflow() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0xff, 0xaa, 0xe8, 0xe8, 0x00]);

    assert_eq!(cpu.register_x, 1);
}
//INY
#[test]
fn test_iny_addressing_modes() {
    // 0xc8 => self.iny(),

    let iny_opcode = *OPCODES_MAP
        .get(&0xc8)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0xc8));
    assert_eq!(iny_opcode.mode, AddressingMode::NoneAddressing);
}
//JMP
#[test]
fn test_jmp_addressing_modes() {
    // 0x4c abs
    // 0x6c

    let jmp_absolute_opcode = *OPCODES_MAP
        .get(&0x4c)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x4c));
    let jmp_opcode = *OPCODES_MAP
        .get(&0x6c)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x6c));

    assert_eq!(jmp_absolute_opcode.mode, AddressingMode::Absolute);
    assert_eq!(jmp_opcode.mode, AddressingMode::NoneAddressing);
}
//JSR
#[test]
fn test_jsr_addressing_modes() {
    // 0x20 => {

    let jsr_opcode = *OPCODES_MAP
        .get(&0x20)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x20));
    assert_eq!(jsr_opcode.mode, AddressingMode::Absolute);
}

#[test]
fn test_lda_from_memory() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x10, 0x55);

    cpu.load_and_run(vec![0xa5, 0x10, 0x00]);

    assert_eq!(cpu.accumulator, 0x55);
}

#[test]
fn test_lda_zero_flag() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0x00, 0x00]);
    assert!(cpu.status & 0b0000_0010 == 0b10);
}

#[test]
fn test_lda_immediate_load_order() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0x05, 0x00]);
    assert_eq!(cpu.accumulator, 0x05);
    assert!(cpu.status & 0b0000_0010 == 0b00);
    assert!(cpu.status & 0b1000_0000 == 0);
}

//LDA
#[test]
fn test_lda_addressing_modes() {
    // 0xa9 | 0xa5 | 0xb5 | 0xad | 0xbd | 0xb9 | 0xa1 | 0xb1

    let lda_immediate_opcode = *OPCODES_MAP
        .get(&0xa9)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0xa9));
    let lda_zero_page_opcode = *OPCODES_MAP
        .get(&0xa5)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0xa5));
    let lda_zero_page_x_opcode = *OPCODES_MAP
        .get(&0xb5)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0xb5));
    let lda_absolute_opcode = *OPCODES_MAP
        .get(&0xad)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0xad));
    let lda_absolute_x_opcode = *OPCODES_MAP
        .get(&0xbd)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0xbd));
    let lda_absolute_y_opcode = *OPCODES_MAP
        .get(&0xb9)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0xb9));
    let lda_indirect_x_opcode = *OPCODES_MAP
        .get(&0xa1)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0xa1));
    let lda_indirect_y_opcode = *OPCODES_MAP
        .get(&0xb1)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0xb1));

    assert_eq!(lda_immediate_opcode.mode, AddressingMode::Immediate);
    assert_eq!(lda_zero_page_opcode.mode, AddressingMode::ZeroPage);
    assert_eq!(lda_zero_page_x_opcode.mode, AddressingMode::ZeroPage_X);
    assert_eq!(lda_absolute_opcode.mode, AddressingMode::Absolute);
    assert_eq!(lda_absolute_x_opcode.mode, AddressingMode::Absolute_X);
    assert_eq!(lda_absolute_y_opcode.mode, AddressingMode::Absolute_Y);
    assert_eq!(lda_indirect_x_opcode.mode, AddressingMode::Indirect_X);
    assert_eq!(lda_indirect_y_opcode.mode, AddressingMode::Indirect_Y);
}

//LDX
#[test]
fn test_ldx_addressing_modes() {
    // 0xa2 | 0xa6 | 0xb6 | 0xae | 0xbe

    let ldx_immediate_opcode = *OPCODES_MAP
        .get(&0xa2)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0xa2));
    let ldx_zero_page_opcode = *OPCODES_MAP
        .get(&0xa6)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0xa6));
    let ldx_zero_page_y_opcode = *OPCODES_MAP
        .get(&0xb6)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0xb6));
    let ldx_absolute_opcode = *OPCODES_MAP
        .get(&0xae)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0xae));
    let ldx_absolute_y_opcode = *OPCODES_MAP
        .get(&0xbe)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0xbe));

    assert_eq!(ldx_immediate_opcode.mode, AddressingMode::Immediate);
    assert_eq!(ldx_zero_page_opcode.mode, AddressingMode::ZeroPage);
    assert_eq!(ldx_zero_page_y_opcode.mode, AddressingMode::ZeroPage_Y);
    assert_eq!(ldx_absolute_opcode.mode, AddressingMode::Absolute);
    assert_eq!(ldx_absolute_y_opcode.mode, AddressingMode::Absolute_Y);
}
//LDY
#[test]
fn test_ldy_addressing_modes() {
    // 0xa0 | 0xa4 | 0xb4 | 0xac | 0xbc => self.ldy(&opcode.mode),

    let ldy_immediate_opcode = *OPCODES_MAP
        .get(&0xa0)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0xa0));
    let ldy_zero_page_opcode = *OPCODES_MAP
        .get(&0xa4)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0xa4));
    let ldy_zero_page_x_opcode = *OPCODES_MAP
        .get(&0xb4)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0xb4));
    let ldy_absolute_opcode = *OPCODES_MAP
        .get(&0xac)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0xac));
    let ldy_absolute_x_opcode = *OPCODES_MAP
        .get(&0xbc)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0xbc));

    assert_eq!(ldy_immediate_opcode.mode, AddressingMode::Immediate);
    assert_eq!(ldy_zero_page_opcode.mode, AddressingMode::ZeroPage);
    assert_eq!(ldy_zero_page_x_opcode.mode, AddressingMode::ZeroPage_X);
    assert_eq!(ldy_absolute_opcode.mode, AddressingMode::Absolute);
    assert_eq!(ldy_absolute_x_opcode.mode, AddressingMode::Absolute_X);
}
//LSR
#[test]
fn test_lsr() {
    let mut cpu = CPU::new();

    cpu.load_and_run(vec![0xa9, 0x5, 0x4a, 0x00]);
    assert_eq!(cpu.accumulator, 0x02);
    assert!(cpu.status & CARRY != 0);
}

#[test]
fn test_lsr_addressing_modes() {
    // 0x4a => self.lsr_accumulator(),
    // //LSR
    // 0x46 | 0x56 | 0x4e | 0x5e => self.lsr(&opcode.mode),

    let lsr_accumulator_opcode = *OPCODES_MAP
        .get(&0x4a)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x4a));
    let lsr_zero_page_opcode = *OPCODES_MAP
        .get(&0x46)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x46));
    let lsr_zero_page_x_opcode = *OPCODES_MAP
        .get(&0x56)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x56));
    let lsr_absolute_opcode = *OPCODES_MAP
        .get(&0x4e)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x4e));
    let lsr_absolute_x_opcode = *OPCODES_MAP
        .get(&0x5e)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x5e));

    assert_eq!(lsr_accumulator_opcode.mode, AddressingMode::NoneAddressing);
    assert_eq!(lsr_zero_page_opcode.mode, AddressingMode::ZeroPage);
    assert_eq!(lsr_zero_page_x_opcode.mode, AddressingMode::ZeroPage_X);
    assert_eq!(lsr_absolute_opcode.mode, AddressingMode::Absolute);
    assert_eq!(lsr_absolute_x_opcode.mode, AddressingMode::Absolute_X);
}
//NOP
#[test]
fn test_nop_addressing_modes() {
    // 0xea => self.nop(),

    let nop_opcode = *OPCODES_MAP
        .get(&0xea)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0xea));
    assert_eq!(nop_opcode.mode, AddressingMode::NoneAddressing);
}
//ORA
#[test]
fn test_ora() {
    let mut cpu = CPU::new();

    cpu.load_and_run(vec![0xa9, 0xaa, 0x09, 0x55, 0x00]);
    assert_eq!(cpu.accumulator, 0xff);
    assert!(cpu.status & NEGATIVE != 0);
}

#[test]
fn test_ora_addressing_modes() {
    // 0x09 | 0x05 | 0x15 | 0x0d | 0x1d | 0x19 | 0x01 | 0x11

    let ora_immediate_opcode = *OPCODES_MAP
        .get(&0x09)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x09));
    let ora_zero_page_opcode = *OPCODES_MAP
        .get(&0x05)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x05));
    let ora_zero_page_x_opcode = *OPCODES_MAP
        .get(&0x15)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x15));
    let ora_absolute_opcode = *OPCODES_MAP
        .get(&0x0d)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x0d));
    let ora_absolute_x_opcode = *OPCODES_MAP
        .get(&0x1d)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x1d));
    let ora_absolute_y_opcode = *OPCODES_MAP
        .get(&0x19)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x19));
    let ora_indirect_x_opcode = *OPCODES_MAP
        .get(&0x01)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x01));
    let ora_indirect_y_opcode = *OPCODES_MAP
        .get(&0x11)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x11));

    assert_eq!(ora_immediate_opcode.mode, AddressingMode::Immediate);
    assert_eq!(ora_zero_page_opcode.mode, AddressingMode::ZeroPage);
    assert_eq!(ora_zero_page_x_opcode.mode, AddressingMode::ZeroPage_X);
    assert_eq!(ora_absolute_opcode.mode, AddressingMode::Absolute);
    assert_eq!(ora_absolute_x_opcode.mode, AddressingMode::Absolute_X);
    assert_eq!(ora_absolute_y_opcode.mode, AddressingMode::Absolute_Y);
    assert_eq!(ora_indirect_x_opcode.mode, AddressingMode::Indirect_X);
    assert_eq!(ora_indirect_y_opcode.mode, AddressingMode::Indirect_Y);
}
//PHA
#[test]
fn test_pha_addressing_modes() {
    // 0x48 => self.pha(),

    let pha_opcode = *OPCODES_MAP
        .get(&0x48)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x48));
    assert_eq!(pha_opcode.mode, AddressingMode::NoneAddressing);
}
//PHP
#[test]
fn test_php_addressing_modes() {
    // 0x08 => self.php(),

    let php_opcode = *OPCODES_MAP
        .get(&0x08)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x08));
    assert_eq!(php_opcode.mode, AddressingMode::NoneAddressing);
}
//PLA
#[test]
fn test_pla_addressing_modes() {
    // 0x68 => self.pla(),

    let pla_opcode = *OPCODES_MAP
        .get(&0x68)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x68));
    assert_eq!(pla_opcode.mode, AddressingMode::NoneAddressing);
}
//PLP
#[test]
fn test_plp_addressing_modes() {
    // 0x28 => self.plp(),

    let plp_opcode = *OPCODES_MAP
        .get(&0x28)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x28));
    assert_eq!(plp_opcode.mode, AddressingMode::NoneAddressing);
}
//ROL
#[test]
fn test_rol() {
    let mut cpu = CPU::new();

    cpu.load_and_run(vec![0xa9, 0xff, 0x2a, 0x00]);
    assert_eq!(cpu.accumulator, 0xfe);
    assert!(cpu.status & NEGATIVE != 0);
    assert!(cpu.status & CARRY != 0);
}

#[test]
fn test_rol_addressing_modes() {
    // 0x2a => self.rol_accumulator(),
    // //ROL
    // 0x26 | 0x36 | 0x2e | 0x3e => self.rol(&opcode.mode),

    let rol_accumulator_opcode = *OPCODES_MAP
        .get(&0x2a)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x2a));
    let rol_zero_page_opcode = *OPCODES_MAP
        .get(&0x26)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x26));
    let rol_zero_page_x_opcode = *OPCODES_MAP
        .get(&0x36)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x36));
    let rol_absolute_opcode = *OPCODES_MAP
        .get(&0x2e)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x2e));
    let rol_absolute_x_opcode = *OPCODES_MAP
        .get(&0x3e)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x3e));

    assert_eq!(rol_accumulator_opcode.mode, AddressingMode::NoneAddressing);
    assert_eq!(rol_zero_page_opcode.mode, AddressingMode::ZeroPage);
    assert_eq!(rol_zero_page_x_opcode.mode, AddressingMode::ZeroPage_X);
    assert_eq!(rol_absolute_opcode.mode, AddressingMode::Absolute);
    assert_eq!(rol_absolute_x_opcode.mode, AddressingMode::Absolute_X);
}
//ROR
//ROL
#[test]
fn test_ror() {
    let mut cpu = CPU::new();

    cpu.load_and_run(vec![0xa9, 0xff, 0x38, 0x6a, 0x00]);
    assert_eq!(cpu.accumulator, 0xff);
    assert!(cpu.status & NEGATIVE != 0);
    assert!(cpu.status & CARRY != 0);
}

#[test]
fn test_ror_addressing_modes() {
    // 0x6a => self.ror_accumulator(),
    // //ROR
    // 0x66 | 0x76 | 0x6e | 0x7e => self.ror(&opcode.mode),

    let ror_accumulator_opcode = *OPCODES_MAP
        .get(&0x6a)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x6a));
    let ror_zero_page_opcode = *OPCODES_MAP
        .get(&0x66)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x66));
    let ror_zero_page_x_opcode = *OPCODES_MAP
        .get(&0x76)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x76));
    let ror_absolute_opcode = *OPCODES_MAP
        .get(&0x6e)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x6e));
    let ror_absolute_x_opcode = *OPCODES_MAP
        .get(&0x7e)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x7e));

    assert_eq!(ror_accumulator_opcode.mode, AddressingMode::NoneAddressing);
    assert_eq!(ror_zero_page_opcode.mode, AddressingMode::ZeroPage);
    assert_eq!(ror_zero_page_x_opcode.mode, AddressingMode::ZeroPage_X);
    assert_eq!(ror_absolute_opcode.mode, AddressingMode::Absolute);
    assert_eq!(ror_absolute_x_opcode.mode, AddressingMode::Absolute_X);
}
//RTI
#[test]
fn test_rti_addressing_modes() {
    // 0x40 => self.rti(),

    let rti_opcode = *OPCODES_MAP
        .get(&0x40)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x40));
    assert_eq!(rti_opcode.mode, AddressingMode::NoneAddressing);
}
//RTS
#[test]
fn test_rts_addressing_modes() {
    // 0x60 => self.rts(),

    let rts_opcode = *OPCODES_MAP
        .get(&0x60)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x60));
    assert_eq!(rts_opcode.mode, AddressingMode::NoneAddressing);
}
//SBC
#[test]
fn test_sbc() {
    let mut cpu = CPU::new();

    cpu.load_and_run(vec![0xa9, 0xc0, 0xe9, 0xc4, 0x00]);
    assert_eq!(cpu.accumulator, 0xfb);
    assert!(cpu.status & NEGATIVE != 0);
}

#[test]
fn test_sbc_addressing_modes() {
    // 0xe9 | 0xe5 | 0xf5 | 0xed | 0xfd | 0xf9 | 0xe1 | 0xf1

    let sbc_immediate_opcode = *OPCODES_MAP
        .get(&0xe9)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0xe9));
    let sbc_zero_page_opcode = *OPCODES_MAP
        .get(&0xe5)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0xe5));
    let sbc_zero_page_x_opcode = *OPCODES_MAP
        .get(&0xf5)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0xf5));
    let sbc_absolute_opcode = *OPCODES_MAP
        .get(&0xed)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0xed));
    let sbc_absolute_x_opcode = *OPCODES_MAP
        .get(&0xfd)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0xfd));
    let sbc_absolute_y_opcode = *OPCODES_MAP
        .get(&0xf9)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0xf9));
    let sbc_indirect_x_opcode = *OPCODES_MAP
        .get(&0xe1)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0xe1));
    let sbc_indirect_y_opcode = *OPCODES_MAP
        .get(&0xf1)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0xf1));

    assert_eq!(sbc_immediate_opcode.mode, AddressingMode::Immediate);
    assert_eq!(sbc_zero_page_opcode.mode, AddressingMode::ZeroPage);
    assert_eq!(sbc_zero_page_x_opcode.mode, AddressingMode::ZeroPage_X);
    assert_eq!(sbc_absolute_opcode.mode, AddressingMode::Absolute);
    assert_eq!(sbc_absolute_x_opcode.mode, AddressingMode::Absolute_X);
    assert_eq!(sbc_absolute_y_opcode.mode, AddressingMode::Absolute_Y);
    assert_eq!(sbc_indirect_x_opcode.mode, AddressingMode::Indirect_X);
    assert_eq!(sbc_indirect_y_opcode.mode, AddressingMode::Indirect_Y);
}
//SEC
#[test]
fn test_sec_addressing_modes() {
    // 0x38 => self.set_status_flag(CARRY),

    let sec_opcode = *OPCODES_MAP
        .get(&0x38)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x38));
    assert_eq!(sec_opcode.mode, AddressingMode::NoneAddressing);
}
//SED
#[test]
fn test_sed_addressing_modes() {
    // 0xf8 => self.set_status_flag(DECIMAL_MODE),

    let sed_opcode = *OPCODES_MAP
        .get(&0xf8)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0xf8));
    assert_eq!(sed_opcode.mode, AddressingMode::NoneAddressing);
}
//SEI
#[test]
fn test_sei_addressing_modes() {
    // 0x78 => self.set_status_flag(INTERRUPT_DISABLE),

    let sei_opcode = *OPCODES_MAP
        .get(&0x78)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x78));
    assert_eq!(sei_opcode.mode, AddressingMode::NoneAddressing);
}
//STA
#[test]
fn test_sta_addressing_modes() {
    // 0x85 | 0x95 | 0x8d | 0x9d | 0x99 | 0x81 | 0x91 => self.sta(&opcode.mode),

    let sta_zero_page_opcode = *OPCODES_MAP
        .get(&0x85)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x85));
    let sta_zero_page_x_opcode = *OPCODES_MAP
        .get(&0x95)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x95));
    let sta_absolute_opcode = *OPCODES_MAP
        .get(&0x8d)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x8d));
    let sta_absolute_x_opcode = *OPCODES_MAP
        .get(&0x9d)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x9d));
    let sta_absolute_y_opcode = *OPCODES_MAP
        .get(&0x99)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x99));
    let sta_indirect_x_opcode = *OPCODES_MAP
        .get(&0x81)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x81));
    let sta_indirect_y_opcode = *OPCODES_MAP
        .get(&0x91)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x91));

    assert_eq!(sta_zero_page_opcode.mode, AddressingMode::ZeroPage);
    assert_eq!(sta_zero_page_x_opcode.mode, AddressingMode::ZeroPage_X);
    assert_eq!(sta_absolute_opcode.mode, AddressingMode::Absolute);
    assert_eq!(sta_absolute_x_opcode.mode, AddressingMode::Absolute_X);
    assert_eq!(sta_absolute_y_opcode.mode, AddressingMode::Absolute_Y);
    assert_eq!(sta_indirect_x_opcode.mode, AddressingMode::Indirect_X);
    assert_eq!(sta_indirect_y_opcode.mode, AddressingMode::Indirect_Y);
}
//STX
#[test]
fn test_stx_addressing_modes() {
    // 0x86 | 0x96 | 0x8e => self.stx(&opcode.mode),

    let stx_zero_page_opcode = *OPCODES_MAP
        .get(&0x86)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x86));
    let stx_zero_page_y_opcode = *OPCODES_MAP
        .get(&0x96)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x96));
    let stx_absolute_opcode = *OPCODES_MAP
        .get(&0x8e)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x8e));

    assert_eq!(stx_zero_page_opcode.mode, AddressingMode::ZeroPage);
    assert_eq!(stx_zero_page_y_opcode.mode, AddressingMode::ZeroPage_Y);
    assert_eq!(stx_absolute_opcode.mode, AddressingMode::Absolute);
}
//STY
#[test]
fn test_sty_addressing_modes() {
    // 0x84 | 0x94 | 0x8c => self.sty(&opcode.mode),

    let sty_zero_page_opcode = *OPCODES_MAP
        .get(&0x84)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x84));
    let sty_zero_page_x_opcode = *OPCODES_MAP
        .get(&0x94)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x94));
    let sty_absolute_opcode = *OPCODES_MAP
        .get(&0x8c)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x8c));

    assert_eq!(sty_zero_page_opcode.mode, AddressingMode::ZeroPage);
    assert_eq!(sty_zero_page_x_opcode.mode, AddressingMode::ZeroPage_X);
    assert_eq!(sty_absolute_opcode.mode, AddressingMode::Absolute);
}
//TAX
#[test]
fn test_tax_addressing_modes() {
    // 0xaa => self.tax(),

    let tax_opcode = *OPCODES_MAP
        .get(&0xaa)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0xaa));
    assert_eq!(tax_opcode.mode, AddressingMode::NoneAddressing);
}

#[test]
fn test_tax() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0xa0, 0xaa, 0x00]);

    assert_eq!(cpu.register_x, 0xa0);
}
//TAY
#[test]
fn test_tay_addressing_modes() {
    // 0xa8 => self.tay(),

    let tay_opcode = *OPCODES_MAP
        .get(&0xa8)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0xa8));
    assert_eq!(tay_opcode.mode, AddressingMode::NoneAddressing);
}
//TSX
#[test]
fn test_tsx_addressing_modes() {
    // 0xba => self.tsx(),

    let tsx_opcode = *OPCODES_MAP
        .get(&0xba)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0xba));
    assert_eq!(tsx_opcode.mode, AddressingMode::NoneAddressing);
}
//TXA
#[test]
fn test_txa_addressing_modes() {
    // 0x8a => self.txa(),

    let txa_opcode = *OPCODES_MAP
        .get(&0x8a)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x8a));
    assert_eq!(txa_opcode.mode, AddressingMode::NoneAddressing);
}
//TXS
#[test]
fn test_txs_addressing_modes() {
    // 0x9a => self.txs(),

    let txs_opcode = *OPCODES_MAP
        .get(&0x9a)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x9a));
    assert_eq!(txs_opcode.mode, AddressingMode::NoneAddressing);
}
//TYA
#[test]
fn test_tya_addressing_modes() {
    // 0x98 => self.tya(),

    let tya_opcode = *OPCODES_MAP
        .get(&0x98)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", 0x98));
    assert_eq!(tya_opcode.mode, AddressingMode::NoneAddressing);
}
