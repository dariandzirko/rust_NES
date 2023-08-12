use crate::opcode::OPCODES_MAP;

#[derive(Debug, PartialEq, Eq)]
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
pub const CARRY: u8 = 0b00000001;
pub const ZERO: u8 = 0b00000010;
pub const INTERRUPT_DISABLE: u8 = 0b00000100;
pub const DECIMAL_MODE: u8 = 0b00001000;
pub const BREAK: u8 = 0b00010000;
pub const BREAK2: u8 = 0b00100000;
pub const OVERFLOW: u8 = 0b01000000;
pub const NEGATIVE: u8 = 0b10000000;

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

    pub fn print_state(&self) {
        println!("accumulator: {:x} | register_x: {:x} | register_y: {:x} | status: {:x} | stack_pointer: {:x} | program_counter: {:x}",
        self.accumulator, self.register_x, self.register_y, self.status, self.stack_pointer, self.program_counter);
    }

    pub fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    pub fn mem_write(&mut self, addr: u16, data: u8) {
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
        self.status = INTERRUPT_DISABLE | BREAK2;
        //        self.status = CpuFlags::from_bits_truncate(0b100100); this is what the tutorial has. Interrupt disable makes sense but not Negative
        self.stack_pointer = STACK_RESET;
        self.program_counter = self.mem_read_u16(0xFFFC)
    }

    pub fn load(&mut self, program: Vec<u8>) {
        self.memory[0x0600..(0x0600 + program.len())].copy_from_slice(&program[..]);
        self.mem_write_u16(0xFFFC, 0x0600);
    }

    fn set_accumulator(&mut self, value: u8) {
        self.accumulator = value;
        self.update_zero_and_negative_flags(self.accumulator);
    }

    fn set_register_x(&mut self, value: u8) {
        self.register_x = value;
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn set_register_y(&mut self, value: u8) {
        self.register_y = value;
        self.update_zero_and_negative_flags(self.register_y);
    }

    pub fn run(&mut self) {
        self.run_with_callback(|_| {});
    }

    pub fn run_with_callback<F>(&mut self, mut callback: F)
    where
        F: FnMut(&mut CPU),
    {
        let opcodes = &OPCODES_MAP;
        loop {
            callback(self);

            let code = self.mem_read(self.program_counter);
            self.program_counter += 1;

            let program_counter_state = self.program_counter;

            let opcode = *opcodes
                .get(&code)
                .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", code));

            self.print_state();

            println!(
                "opcode mnemonic: {} | opcode value: {:x}",
                opcode.mnemonic, opcode.code
            );

            match code {
                //BRK
                0x00 => return,
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
                //BMI
                0x30 => self.branch(self.status & NEGATIVE != 0),
                //BNE
                0xd0 => self.branch(self.status & ZERO == 0),
                //BPL
                0x10 => self.branch(self.status & NEGATIVE == 0),
                //BVC
                0x50 => self.branch(self.status & OVERFLOW == 0),
                //BVS
                0x70 => self.branch(self.status & OVERFLOW != 0),
                //CLC
                0x18 => self.reset_status_flag(CARRY),
                //CLD
                0xd8 => self.reset_status_flag(DECIMAL_MODE),
                //CLI
                0x58 => self.reset_status_flag(INTERRUPT_DISABLE),
                //CLV
                0xb8 => self.reset_status_flag(OVERFLOW),
                //CMP
                0xc9 | 0xc5 | 0xd5 | 0xcd | 0xdd | 0xd9 | 0xc1 | 0xd1 => {
                    self.compare(self.accumulator, &opcode.mode)
                }
                //CPX
                0xe0 | 0xe4 | 0xec => self.compare(self.register_x, &opcode.mode),
                //CPY
                0xc0 | 0xc4 | 0xcc => self.compare(self.register_y, &opcode.mode),
                //DEC
                0xc6 | 0xd6 | 0xce | 0xde => self.dec(&opcode.mode),
                //DEX
                0xca => self.dex(),
                //DEY
                0x88 => self.dey(),
                //EOR
                0x49 | 0x45 | 0x55 | 0x4d | 0x5d | 0x59 | 0x41 | 0x51 => self.eor(&opcode.mode),
                //INC
                0xe6 | 0xf6 | 0xee | 0xfe => self.inc(&opcode.mode),
                //INX
                0xe8 => self.inx(),
                //INY
                0xc8 => self.iny(),
                //JMP Absolute
                0x4c => {
                    self.program_counter = self.mem_read_u16(self.program_counter);
                }
                //JMP Indirect
                0x6c => {
                    let addr = self.mem_read_u16(self.program_counter);

                    let indirect = if addr & 0xff == 0xff {
                        let low = self.mem_read(addr) as u16;
                        let high = self.mem_read(addr & 0xFF00) as u16;
                        self.mem_read_u16(high << 8 | low)
                    } else {
                        self.mem_read_u16(addr)
                    };

                    self.program_counter = indirect;
                }
                //JSR
                0x20 => {
                    self.stack_push_u16(self.program_counter + 2 - 1); //return point should be the next instruction
                    self.program_counter = self.mem_read_u16(self.program_counter);
                }
                //LDA
                0xa9 | 0xa5 | 0xb5 | 0xad | 0xbd | 0xb9 | 0xa1 | 0xb1 => self.lda(&opcode.mode),
                //LDX
                0xa2 | 0xa6 | 0xb6 | 0xae | 0xbe => self.ldx(&opcode.mode),
                //LDY
                0xa0 | 0xa4 | 0xb4 | 0xac | 0xbc => self.ldy(&opcode.mode),
                //LSR Accumulator
                0x4a => self.lsr_accumulator(),
                //LSR
                0x46 | 0x56 | 0x4e | 0x5e => self.lsr(&opcode.mode),
                //NOP
                0xea => self.nop(),
                //ORA
                0x09 | 0x05 | 0x15 | 0x0d | 0x1d | 0x19 | 0x01 | 0x11 => self.ora(&opcode.mode),
                //PHA
                0x48 => self.pha(),
                //PHP
                0x08 => self.php(),
                //PLA
                0x68 => self.pla(),
                //PLP
                0x28 => self.plp(),
                //ROL Accumulator
                0x2a => self.rol_accumulator(),
                //ROL
                0x26 | 0x36 | 0x2e | 0x3e => self.rol(&opcode.mode),
                //ROR Accumulator
                0x6a => self.ror_accumulator(),
                //ROR
                0x66 | 0x76 | 0x6e | 0x7e => self.ror(&opcode.mode),
                //RTI
                0x40 => self.rti(),
                //RTS
                0x60 => self.rts(),
                //SBC
                0xe9 | 0xe5 | 0xf5 | 0xed | 0xfd | 0xf9 | 0xe1 | 0xf1 => self.sbc(&opcode.mode),
                //SEC
                0x38 => self.set_status_flag(CARRY),
                //SED
                0xf8 => self.set_status_flag(DECIMAL_MODE),
                //SEI
                0x78 => self.set_status_flag(INTERRUPT_DISABLE),
                //STA
                0x85 | 0x95 | 0x8d | 0x9d | 0x99 | 0x81 | 0x91 => self.sta(&opcode.mode),
                //STX
                0x86 | 0x96 | 0x8e => self.stx(&opcode.mode),
                //STY
                0x84 | 0x94 | 0x8c => self.sty(&opcode.mode),
                //TAX
                0xaa => self.tax(),
                //TAY
                0xa8 => self.tay(),
                //TSX
                0xba => self.tsx(),
                //TXA
                0x8a => self.txa(),
                //TXS
                0x9a => self.txs(),
                //TYA
                0x98 => self.tya(),
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

            // I think this is wrong
            AddressingMode::Indirect_Y => {
                let base = self.mem_read(self.program_counter);

                let low = self.mem_read(base as u16) as u16;
                let high = self.mem_read(base.wrapping_add(1) as u16) as u16;
                println!("Indirect_Y low: {:x} | high: {:x}", low, high);
                (high << 8 | low).wrapping_add(self.register_y as u16)
            }

            AddressingMode::NoneAddressing => panic!("mode {:?} is not supported", mode),
        }
    }

    fn adc(&mut self, mode: &AddressingMode) {
        let accum = self.accumulator as u16;
        let mem_val = self.mem_read(self.get_operand_address(mode)) as u16;
        let sum = accum + mem_val + (self.status & CARRY) as u16;

        if sum > 0xFF {
            self.set_status_flag(CARRY);
        }

        let result = sum as u8;

        if (mem_val as u8 ^ result) & (self.accumulator ^ result) & 0x80 != 0 {
            self.set_status_flag(OVERFLOW);
        } else {
            self.reset_status_flag(OVERFLOW);
        }

        self.set_accumulator(result);
    }

    fn and(&mut self, mode: &AddressingMode) {
        self.set_accumulator(self.accumulator & self.mem_read(self.get_operand_address(mode)));
    }

    fn asl_accumulator(&mut self) {
        if self.accumulator & 0x80 != 0 {
            self.set_status_flag(CARRY);
        } else {
            self.reset_status_flag(CARRY);
        }

        self.set_accumulator(self.accumulator << 1);
    }

    fn asl(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let mut mem_val = self.mem_read(addr);

        if mem_val & 0x80 != 0 {
            self.set_status_flag(CARRY);
        } else {
            self.reset_status_flag(CARRY);
        }

        mem_val <<= 1;
        self.mem_write(addr, mem_val);
        self.update_zero_and_negative_flags(mem_val);
    }

    fn branch(&mut self, condition: bool) {
        if condition {
            let jump = self.mem_read(self.program_counter) as i8;

            println!(
                "jump: {:x}| program counter: {:x} | program_counter add: {:x} | jump_addr: {:x}",
                jump,
                self.program_counter,
                self.program_counter.wrapping_add(jump as u16),
                self.program_counter.wrapping_add(jump as u16 + 1)
            );
            self.program_counter = self.program_counter.wrapping_add(jump as u16 + 1);
        }
    }

    fn bit(&mut self, mode: &AddressingMode) {
        let result = self.accumulator & self.mem_read(self.get_operand_address(mode));
        //need to rewrite these things with functions instead of rewriting the same function 1 bajillion times
        //should write some function that takes bool/conditional and a flag ie. set_status_flag_conditionally(result & 0x80 != 0, NEGATIVE)
        if result & 0x80 != 0 {
            self.set_status_flag(NEGATIVE);
        } else {
            self.reset_status_flag(NEGATIVE);
        }
        if result & 0x40 != 0 {
            self.set_status_flag(OVERFLOW);
        } else {
            self.reset_status_flag(OVERFLOW);
        }
        if result == 0 {
            self.set_status_flag(ZERO);
        } else {
            self.reset_status_flag(ZERO);
        }
    }

    //Value is the input, like accumulator
    fn compare(&mut self, value: u8, mode: &AddressingMode) {
        let value2 = self.mem_read(self.get_operand_address(mode));
        if value >= value2 {
            self.set_status_flag(CARRY);
        } else {
            self.reset_status_flag(CARRY);
        }
        self.update_zero_and_negative_flags(value.wrapping_sub(value2));
    }

    fn dec(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr).wrapping_sub(1);
        self.mem_write(addr, value);
        self.update_zero_and_negative_flags(value);
    }

    fn dex(&mut self) {
        self.register_x = self.register_x.wrapping_sub(1);
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn dey(&mut self) {
        self.register_y = self.register_y.wrapping_sub(1);
        self.update_zero_and_negative_flags(self.register_y);
    }

    fn eor(&mut self, mode: &AddressingMode) {
        self.accumulator ^= self.mem_read(self.get_operand_address(mode));
        self.update_zero_and_negative_flags(self.accumulator);
    }

    fn inc(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr).wrapping_add(1);
        self.mem_write(addr, value);
        self.update_zero_and_negative_flags(value);
    }

    fn inx(&mut self) {
        self.register_x = self.register_x.wrapping_add(1);
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn iny(&mut self) {
        self.register_y = self.register_y.wrapping_add(1);
        self.update_zero_and_negative_flags(self.register_y);
    }

    fn lda(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        println!("lda addr: {:x} | value: {:x}", addr, value);

        self.set_accumulator(value);
    }

    fn ldx(&mut self, mode: &AddressingMode) {
        self.set_register_x(self.mem_read(self.get_operand_address(mode)));
    }

    fn ldy(&mut self, mode: &AddressingMode) {
        self.set_register_y(self.mem_read(self.get_operand_address(mode)));
    }

    fn lsr_accumulator(&mut self) {
        if self.accumulator & 0x01 != 0 {
            self.set_status_flag(CARRY);
        } else {
            self.reset_status_flag(CARRY);
        }
        self.set_accumulator(self.accumulator >> 1);
    }

    fn lsr(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let mut mem_val = self.mem_read(addr);

        if mem_val & 0x01 != 0 {
            self.set_status_flag(CARRY);
        } else {
            self.reset_status_flag(CARRY);
        }

        mem_val >>= 1;
        self.mem_write(addr, mem_val);
        self.update_zero_and_negative_flags(mem_val);
    }

    fn nop(&mut self) {
        //Does nothing
    }

    fn ora(&mut self, mode: &AddressingMode) {
        self.set_accumulator(self.mem_read(self.get_operand_address(mode)) | self.accumulator);
    }

    fn pha(&mut self) {
        self.stack_push(self.accumulator);
    }

    fn php(&mut self) {
        self.stack_push(self.status);
    }

    fn pla(&mut self) {
        let value = self.stack_pop();
        self.set_accumulator(value);
    }

    fn plp(&mut self) {
        self.status = self.stack_pop();
    }

    fn rol_accumulator(&mut self) {
        let mut value = self.accumulator as u16;
        value <<= 1;
        value |= (self.status & CARRY) as u16;
        if value & 0x100 != 0 {
            self.set_status_flag(CARRY)
        } else {
            self.reset_status_flag(CARRY)
        }
        self.set_accumulator((value & 0xFF) as u8);
    }

    fn rol(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let mut value = self.mem_read(addr) as u16;
        value <<= 1;
        value |= (self.status & CARRY) as u16;
        if value & 0x100 != 0 {
            self.set_status_flag(CARRY)
        } else {
            self.reset_status_flag(CARRY)
        }
        self.mem_write(addr, (value & 0xFF) as u8);
    }

    fn ror_accumulator(&mut self) {
        let mut value = self.accumulator as u16;

        if self.status & CARRY != 0 {
            value |= 0x80 //set 7th bit high
        }
        if value & 0x01 != 0 {
            self.set_status_flag(CARRY)
        } else {
            self.reset_status_flag(CARRY)
        }
        value >>= 1;
        self.set_accumulator((value & 0xFF) as u8);
    }

    fn ror(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let mut value = self.mem_read(addr) as u16;

        if self.status & CARRY != 0 {
            value |= 0x80 //set 7th bit high
        }
        if value & 0x01 != 0 {
            self.set_status_flag(CARRY)
        } else {
            self.reset_status_flag(CARRY)
        }
        value >>= 1;
        self.mem_write(addr, (value & 0xFF) as u8);
    }

    fn rti(&mut self) {
        self.status = self.stack_pop();
        self.program_counter = self.stack_pop_u16();
    }

    fn rts(&mut self) {
        self.program_counter = self.stack_pop_u16().wrapping_add(1);
    }

    //turbo wrong
    fn sbc(&mut self, mode: &AddressingMode) {
        let accum = self.accumulator as u16;
        let mem_val = (!self.mem_read(self.get_operand_address(mode)) as u16); //.wrapping_add(1); //Invert value at addr
        let sum = accum + mem_val + (self.status & CARRY) as u16;

        if sum > 0xFF {
            self.set_status_flag(CARRY);
        }

        let result = sum as u8;

        if (mem_val as u8 ^ result) & (self.accumulator ^ result) & 0x80 != 0 {
            self.set_status_flag(OVERFLOW);
        } else {
            self.reset_status_flag(OVERFLOW);
        }

        self.set_accumulator(result);
    }

    fn sta(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        println!("sta addr: {:x} | value: {:x}", addr, self.accumulator);
        self.mem_write(addr, self.accumulator);
    }

    fn stx(&mut self, mode: &AddressingMode) {
        self.mem_write(self.get_operand_address(mode), self.register_x);
    }

    fn sty(&mut self, mode: &AddressingMode) {
        self.mem_write(self.get_operand_address(mode), self.register_y);
    }

    fn tax(&mut self) {
        self.register_x = self.accumulator;
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn tay(&mut self) {
        self.register_y = self.accumulator;
        self.update_zero_and_negative_flags(self.register_y);
    }

    fn tsx(&mut self) {
        self.register_x = self.stack_pointer;
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn txa(&mut self) {
        self.set_accumulator(self.register_x);
    }

    fn txs(&mut self) {
        self.stack_pointer = self.register_x;
        self.update_zero_and_negative_flags(self.stack_pointer);
    }

    fn tya(&mut self) {
        self.set_accumulator(self.register_y);
    }

    fn update_zero_and_negative_flags(&mut self, result: u8) {
        if result == 0 {
            self.status |= ZERO;
        } else {
            self.status &= !ZERO;
        }

        if result & 0x80 != 0 {
            self.set_status_flag(NEGATIVE)
        } else {
            self.reset_status_flag(NEGATIVE);
        }
    }

    fn set_status_flag(&mut self, flag: u8) {
        self.status |= flag;
    }
    fn reset_status_flag(&mut self, flag: u8) {
        self.status &= !flag
    }

    fn stack_push(&mut self, value: u8) {
        // println!(
        //     "stack_push | value: {:x} | memory_at_sp: {:x}",
        //     value,
        //     self.mem_read(STACK + self.stack_pointer as u16)
        // );

        self.mem_write(STACK + self.stack_pointer as u16, value);
        // println!(
        //     "after memory_at_sp: {:x}",
        //     self.mem_read(STACK + self.stack_pointer as u16)
        // );

        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
    }

    fn stack_push_u16(&mut self, value: u16) {
        let low = value & 0xff as u16;
        let high = value >> 8;
        self.stack_push(high as u8);
        self.stack_push(low as u8);
    }

    fn stack_pop(&mut self) -> u8 {
        // println!(
        //     "stack_pop memory_at_sp: {:x}",
        //     self.mem_read(STACK + self.stack_pointer as u16)
        // );
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
        // println!(
        //     "stack_pop memory_at_sp: {:x}",
        //     self.mem_read(STACK + self.stack_pointer as u16)
        // );
        self.mem_read(STACK + self.stack_pointer as u16)
    }

    fn stack_pop_u16(&mut self) -> u16 {
        let low = self.stack_pop() as u16;
        let high = self.stack_pop() as u16;
        // println!("low: {:x} | high: {:x}", low, high);
        (high << 8) | low
    }
}
