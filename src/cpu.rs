use crate::opcode::OPCODES_MAP;

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPage_X,
    ZeroPage_Y,
    Absolute,
    Absolute_X,
    Absolute_Y,
    Indirect_X,
    Indirect_Y,
    NoneAddressing,
}

const STACK: u16 = 0x0100;
const STACK_RESET: u8 = 0xfd;

/// # Status Register (P) http://wiki.nesdev.com/w/index.php/Status_flags
///
///  7 6 5 4 3 2 1 0
///  N V _ B D I Z C
///  | |   | | | | +--- Carry Flag
///  | |   | | | +----- Zero Flag
///  | |   | | +------- Interrupt Disable
///  | |   | +--------- Decimal Mode (not used on NES)
///  | |   +----------- Break Command
///  | +--------------- Overflow Flag
///  +----------------- Negative Flag
///
const CARRY: u8 = 0b00000001;
const ZERO: u8 = 0b00000010;
const INTERRUPT_DISABLE: u8 = 0b00000100;
const DECIMAL_MODE: u8 = 0b00001000;
const BREAK: u8 = 0b00010000;
const BREAK2: u8 = 0b00100000;
const OVERFLOW: u8 = 0b01000000;
const NEGATIVE: u8 = 0b10000000;

pub struct CPU {
    pub accumulator: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub status: u8,
    pub stack_pointer: u8,
    pub program_counter: u16,
    memory: [u8; 0xFFFF],
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            accumulator: 0,
            register_x: 0,
            register_y: 0,
            status: 0,
            stack_pointer: STACK_RESET,
            program_counter: 0,
            memory: [0; 0xFFFF],
        }
    }

    fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }

    fn mem_read_u16(&self, pos: u16) -> u16 {
        let low = self.mem_read(pos);
        let high = self.mem_read(pos + 1);
        (high as u16) << 8 | low as u16
    }

    fn mem_write_u16(&mut self, pos: u16, data: u16) {
        self.mem_write(pos, ((data & 0xFF) as u8));
        self.mem_write(pos + 1, ((data >> 8) as u8));
    }

    pub fn reset(&mut self) {
        self.accumulator = 0;
        self.register_x = 0;
        self.status = INTERRUPT_DISABLE;
        //        self.status = CpuFlags::from_bits_truncate(0b100100); this is what the tutorial has. Interrupt disable makes sense but not Negative
        self.stack_pointer = STACK_RESET;
        self.program_counter = self.mem_read_u16(0xFFFC)
    }

    pub fn load(&mut self, program: Vec<u8>) {
        self.memory[0x8000..(0x8000 + program.len())].copy_from_slice(&program[..]);
        self.mem_write_u16(0xFFFC, 0x8000);
    }

    fn set_accumulator(&mut self, value: u8) {
        self.accumulator = value;
        self.update_zero_and_negative_flags(self.accumulator);
    }

    pub fn run(&mut self) {
        loop {
            let code = self.mem_read(self.program_counter);
            self.program_counter += 1;

            let program_counter_state = self.program_counter;

            let opcode = *OPCODES_MAP
                .get(&code)
                .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", code));

            match code {
                //BRK
                0x00 => return,
                //TAX
                0xaa => {
                    self.tax();
                }
                //INX
                0xe8 => {
                    self.inx();
                }
                //ADC
                0x69 | 0x65 | 0x75 | 0x6d | 0x7d | 0x79 | 0x61 | 0x71 => self.adc(&opcode.mode),
                //AND
                0x29 | 0x25 | 0x35 | 0x2D | 0x3D | 0x39 | 0x21 | 0x31 => self.and(&opcode.mode),
                //ASL Accumulator
                0x0a => self.asl_accumulator(),
                //ASL
                0x06 | 0x16 | 0x0e | 0x1e => self.asl(&opcode.mode),
                //BCC
                0x90 => self.branch(self.status & CARRY == 0),
                //BCS
                0xb0 => self.branch(self.status & CARRY != 0),
                //BEQ
                0xf0 => self.branch(self.status & ZERO != 0),
                //BIT
                0x24 | 0x2c => self.bit(&opcode.mode),
                //LDA
                0xa9 | 0xa5 | 0xb5 | 0xad | 0xbd | 0xb9 | 0xa1 | 0xb1 => self.lda(&opcode.mode),
                //STA
                0x85 | 0x95 | 0x8d | 0x9d | 0x99 | 0x81 | 0x91 => self.sta(&opcode.mode),
                //Not implemented case
                _ => todo!("This is gna break if we make it here"),
            }

            if self.program_counter == program_counter_state {
                self.program_counter += (opcode.len - 1) as u16;
            }
        }
    }

    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset();
        self.run();
    }

    fn get_operand_address(&self, mode: &AddressingMode) -> u16 {
        match mode {
            AddressingMode::Immediate => self.program_counter,
            AddressingMode::ZeroPage => self.mem_read(self.program_counter) as u16,
            AddressingMode::Absolute => self.mem_read_u16(self.program_counter),

            AddressingMode::ZeroPage_X => self
                .mem_read(self.program_counter)
                .wrapping_add(self.register_x) as u16,

            AddressingMode::ZeroPage_Y => self
                .mem_read(self.program_counter)
                .wrapping_add(self.register_y) as u16,

            AddressingMode::Absolute_X => self
                .mem_read_u16(self.program_counter)
                .wrapping_add(self.register_x as u16),

            AddressingMode::Absolute_Y => self
                .mem_read_u16(self.program_counter)
                .wrapping_add(self.register_y as u16),

            AddressingMode::Indirect_X => {
                let base = self.mem_read(self.program_counter);

                let ptr = base.wrapping_add(self.register_x);
                let low = self.mem_read(ptr as u16) as u16;
                let high = self.mem_read(ptr.wrapping_add(1) as u16) as u16;
                high << 8 | low
            }

            AddressingMode::Indirect_Y => {
                let base = self.mem_read(self.program_counter);

                let low = self.mem_read(base as u16) as u16;
                let high = self.mem_read(base.wrapping_add(1) as u16) as u16;

                self.mem_read_u16((high << 8 | low).wrapping_add(self.register_y as u16))
            }

            AddressingMode::NoneAddressing => panic!("mode {:?} is not supported", mode),
        }
    }

    fn adc(&mut self, mode: &AddressingMode) {
        let accum = self.accumulator as u16;
        let mem_val = self.mem_read(self.get_operand_address(mode)) as u16;
        let sum = accum + mem_val + (self.status & CARRY) as u16;

        if sum > 0xFF {
            self.status |= 0b0000_0001;
        }

        let result = sum as u8;

        if (mem_val as u8 ^ result) & (self.accumulator ^ result) & 0x80 != 0 {
            self.status |= OVERFLOW;
        } else {
            self.status &= !OVERFLOW;
        }

        self.set_accumulator(result);
    }

    fn and(&mut self, mode: &AddressingMode) {
        self.set_accumulator(self.accumulator & self.mem_read(self.get_operand_address(mode)));
    }

    fn asl_accumulator(&mut self) {
        if self.accumulator & 0x80 != 0 {
            self.status |= CARRY
        } else {
            self.status &= !CARRY
        }

        self.set_accumulator(self.accumulator << 1);
    }

    fn asl(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let mut mem_val = self.mem_read(addr);

        if mem_val & 0x80 != 0 {
            self.status |= CARRY
        } else {
            self.status &= !CARRY
        }

        mem_val <<= 1;
        self.mem_write(addr, mem_val);
        self.update_zero_and_negative_flags(mem_val);
    }

    fn branch(&mut self, condition: bool) {
        if condition {
            let jump_addr = self.mem_read(self.program_counter) as u16;
            self.program_counter = self.program_counter.wrapping_add(jump_addr + 1);
        }
    }

    fn bit(&mut self, mode: &AddressingMode) {}

    fn lda(&mut self, mode: &AddressingMode) {
        self.set_accumulator(self.mem_read(self.get_operand_address(mode)));
    }

    fn tax(&mut self) {
        self.register_x = self.accumulator;
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn inx(&mut self) {
        self.register_x = self.register_x.wrapping_add(1);
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn sta(&mut self, mode: &AddressingMode) {
        self.mem_write(self.get_operand_address(mode), self.accumulator);
    }

    fn update_zero_and_negative_flags(&mut self, result: u8) {
        if result == 0 {
            self.status |= NEGATIVE; //the negative flag is the 2nd bit which is set if the param is 0
        } else {
            self.status &= !NEGATIVE;
        }

        if result & 0x80 != 0 {
            self.status |= CARRY;
        } else {
            self.status &= !CARRY;
        }
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::{CARRY, CPU, OVERFLOW};

    #[test]
    fn test_0xa9_immediate_load_order() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x05, 0x00]);
        assert_eq!(cpu.accumulator, 0x05);
        assert!(cpu.status & 0b0000_0010 == 0b00);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x00, 0x00]);
        assert!(cpu.status & 0b0000_0010 == 0b10);
    }

    #[test]
    fn test_0xaa_tax_move_a_to_x() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xa0, 0xaa, 0x00]);

        assert_eq!(cpu.register_x, 0xa0);
    }

    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xc0, 0xaa, 0xe8, 00]);

        assert_eq!(cpu.register_x, 0xc1);
    }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xff, 0xaa, 0xe8, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 1);
    }

    #[test]
    fn test_lda_from_memory() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0x55);

        cpu.load_and_run(vec![0xa5, 0x10, 0x00]);

        assert_eq!(cpu.accumulator, 0x55);
    }

    #[test]
    fn test_adc_from_memory() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0x55);

        cpu.load_and_run(vec![0xa5, 0x10, 0x69, 0xff, 0x00]);

        assert_eq!(cpu.accumulator, 0x54);
        assert!(cpu.status & CARRY != 0);
    }

    #[test]
    fn test_and_from_memory() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0x55);

        cpu.load_and_run(vec![0xa5, 0x10, 0x29, 0xaa, 0x00]);

        assert_eq!(cpu.accumulator, 0x00);
    }

    #[test]
    fn test_asl() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0x55);

        cpu.load_and_run(vec![0xa5, 0x10, 0x0a, 0x00]);

        assert_eq!(cpu.accumulator, 0xAA);

        cpu.mem_write(0x10, 0xA8);

        cpu.load_and_run(vec![0x06, 0x10, 0xa5, 0x10, 0x00]);
        assert_eq!(cpu.accumulator, 0x50);
        assert!(cpu.status & CARRY != 0);
    }
}
