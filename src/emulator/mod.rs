use super::opcodes;

pub struct Emulator {
    pub memory: Vec<u8>,
    registers: Vec<u8>,
    program_counter: u8,
    stack_pointer: u8,
    pub is_halted: bool,
}

impl Emulator {
    /// Creates a new emulator with memory of `ram_size` bytes and `gp_register_count` general purpose registers.
    pub fn new(ram_size: u8) -> Self {
        assert!(ram_size > 0, "RAM size must be greater than 0");
        Self {
            memory: vec![0; ram_size as usize],
            registers: vec![0; 16],
            program_counter: 0, // TODO: Make part in `registers`
            stack_pointer: 0,   // TODO: Make part in `registers`
            is_halted: false,
        }
    }

    pub fn load_program(&mut self, program: Program) {
        for byte in program.get_bytecode() {
            self.memory[self.stack_pointer as usize] = *byte;
            self.stack_pointer += 1;
        }
    }
    
    /// Execute the next instruction
    pub fn step(&mut self) {
        let instruction = self.memory[self.program_counter as usize];
        let src = self.memory[(self.program_counter + 1) as usize];
        let dst = self.memory[(self.program_counter + 2) as usize];
        
        self.program_counter += match instruction {
            opcodes::NOOP => 1,
            opcodes::HALT => { self.is_halted = true; 0 },
            opcodes::MOVE_LITERAL  => { self.registers[src as usize] = dst; 3 }
            opcodes::MOVE_REGISTER => { self.registers[src as usize] = self.registers[dst as usize]; 3 }
            opcodes::LOAD_MEMORY   => { self.registers[src as usize] = self.memory[dst as usize] as u8; 3 }
            opcodes::DEBUG => {
                print!("[DEBUG] ");
                match src {
                    0 => { // Print: `dst` points to an array of chars (0th byte is length)
                        print!("[PRINT] Address: {} \"", dst);
                        let length = self.memory[dst as usize];
                        for i in 1..=length {
                            print!("{}", self.memory[(dst + i) as usize] as char);
                        }
                        print!("\"");
                    }
                    _ => {
                        print!("Unknown Debug: {}", src);
                    }
                };
                println!();
                3
            }
            opcodes::STORE_LITERAL => {
                self.memory[self.registers[src as usize] as usize] = dst;
                3
            }
            opcodes::STORE_REGISTER => {
                self.memory[self.registers[src as usize] as usize] = self.registers[dst as usize];
                3
            }
            opcodes::PUSH_LITERAL => {
                self.stack_pointer += 1;
                self.memory[self.stack_pointer as usize] = src;
                2
            }
            opcodes::PUSH_REGISTER => {
                self.stack_pointer += 1;
                self.memory[self.stack_pointer as usize] = self.registers[src as usize];
                2
            }
            opcodes::POP  => {
                self.registers[src as usize] = self.memory[self.stack_pointer as usize];
                self.stack_pointer -= 1;
                2
            }
            opcodes::CALL => {
                self.stack_pointer += 1;
                self.memory[self.stack_pointer as usize] = self.program_counter + 2;
                self.program_counter = src;
                2
            }
            opcodes::RETURN => {
                self.program_counter = self.memory[self.stack_pointer as usize];
                self.stack_pointer -= 1;
                1
            }
            opcodes::JUMP_LITERAL => { self.program_counter = src; 2 }
            opcodes::JUMP_REGISTER => { self.program_counter = self.registers[src as usize]; 2 }

            opcodes::MOD_LITERAL  => { self.registers[src as usize] %= dst; 3 }
            opcodes::ADD_LITERAL  => { self.registers[src as usize] += dst; 3 }
            opcodes::SUB_LITERAL  => { self.registers[src as usize] -= dst; 3 }
            opcodes::MUL_LITERAL  => { self.registers[src as usize] *= dst; 3 }
            opcodes::DIV_LITERAL  => { self.registers[src as usize] /= dst; 3 }
            opcodes::AND_LITERAL  => { self.registers[src as usize] &= dst; 3 }
            opcodes::NAND_LITERAL => { self.registers[src as usize] = !(self.registers[src as usize] & dst); 3 }
            opcodes::OR_LITERAL   => { self.registers[src as usize] |= dst; 3 }
            opcodes::NOR_LITERAL  => { self.registers[src as usize] = !(self.registers[src as usize] | dst); 3 }
            opcodes::XOR_LITERAL  => { self.registers[src as usize] ^= dst; 3 }
            opcodes::XNOR_LITERAL => { self.registers[src as usize] = !(self.registers[src as usize] ^ dst); 3 }
            opcodes::BSL_LITERAL  => { self.registers[src as usize] <<= dst; 3 }
            opcodes::BSR_LITERAL  => { self.registers[src as usize] >>= dst; 3 }

            opcodes::MOD_REGISTER  => { self.registers[src as usize] %= self.registers[dst as usize]; 3 }
            opcodes::ADD_REGISTER  => { self.registers[src as usize] += self.registers[dst as usize]; 3 }
            opcodes::SUB_REGISTER  => { self.registers[src as usize] -= self.registers[dst as usize]; 3 }
            opcodes::MUL_REGISTER  => { self.registers[src as usize] *= self.registers[dst as usize]; 3 }
            opcodes::DIV_REGISTER  => { self.registers[src as usize] /= self.registers[dst as usize]; 3 }
            opcodes::AND_REGISTER  => { self.registers[src as usize] &= self.registers[dst as usize]; 3 }
            opcodes::NAND_REGISTER => { self.registers[src as usize] = !(self.registers[src as usize] & self.registers[dst as usize]); 3 }
            opcodes::OR_REGISTER   => { self.registers[src as usize] |= self.registers[dst as usize]; 3 }
            opcodes::NOR_REGISTER  => { self.registers[src as usize] = !(self.registers[src as usize] | self.registers[dst as usize]); 3 }
            opcodes::XOR_REGISTER  => { self.registers[src as usize] ^= self.registers[dst as usize]; 3 }
            opcodes::XNOR_REGISTER => { self.registers[src as usize] = !(self.registers[src as usize] ^ self.registers[dst as usize]); 3 }
            opcodes::BSL_REGISTER  => { self.registers[src as usize] <<= self.registers[dst as usize]; 3 }
            opcodes::BSR_REGISTER  => { self.registers[src as usize] >>= self.registers[dst as usize]; 3 }
            opcodes::INC => { self.registers[src as usize] += 1; 1 }
            opcodes::DEC => { self.registers[src as usize] -= 1; 1 }
            opcodes::NOT => { self.registers[src as usize] = !self.registers[src as usize]; 1 }
            _ => { panic!("Unknown instruction: {}", instruction) }
        };
    }
    
    /// Very crude debugging.
    pub fn print_memory(&self) {
        let mut skipped = false;
        for i in 0..self.memory.len() {
            match self.memory[i] {
                0 => {
                    if !skipped {
                        println!();
                    }
                    skipped = true;
                },
                _ => println!("memory[{:3}] {}", i, self.memory[i]),
            }
        }
    }
}

pub struct Program {
    bytecode: Vec<u8>,
}

impl Program {
    pub fn new() -> Self {
        Self { bytecode: vec![] }
    }

    pub fn from_instructions(instructions: Vec<Instruction>) -> Self {
        let mut program = Program::new();
        for instruction in instructions {
            program.add_instruction(instruction);
        }
        program
    }
    
    pub fn add_instruction(&mut self, instruction: Instruction) {
        for byte in instruction.assemble() {
            self.bytecode.push(byte);
        }
    }

    pub fn get_bytecode(&self) -> &Vec<u8> {
        &self.bytecode
    }
}

pub enum Arg {
    Register(u8),
    Value(u8),
}

pub enum Instruction {
    DEBUG(u8, u8), // print out stuff for example
    NOOP,           HALT,
    MOVE(Arg, u8 ),
    SMEM(Arg, u8 ), LMEM(Arg, u8),
    PUSH(Arg     ), POP (u8  ),
    CALL(u8      ), RET,
    JUMP(Arg     ),
    MOD (Arg, u8 ), NOT (u8     ),
    ADD (Arg, u8 ), SUB (Arg, u8),
    MUL (Arg, u8 ), DIV (Arg, u8),
    AND (Arg, u8 ), NAND(Arg, u8),
    OR  (Arg, u8 ), NOR (Arg, u8),
    XOR (Arg, u8 ), XNOR(Arg, u8),
    BSL (Arg, u8 ), BSR (Arg, u8),
    INC (u8      ), DEC (u8     ),
}

impl Instruction {
    pub fn assemble(&self) -> Vec<u8> {
        match self {
            Instruction::DEBUG(a, b) => vec![opcodes::DEBUG, *a, *b],
            Instruction::NOOP => vec![opcodes::NOOP],
            Instruction::HALT => vec![opcodes::HALT],
            Instruction::MOVE(src, dst) => match src {
                Arg::Register(reg) => vec![opcodes::MOVE_REGISTER, *reg, *dst],
                Arg::Value(val) => vec![opcodes::MOVE_LITERAL, *val, *dst],
            }
            Instruction::SMEM(src, dst) => match src {
                Arg::Register(reg) => vec![opcodes::STORE_REGISTER, *reg, *dst],
                Arg::Value(val) => vec![opcodes::STORE_LITERAL, *val, *dst],
            }
            Instruction::LMEM(src, dst) => match src {
                Arg::Register(reg) => vec![opcodes::LOAD_MEMORY, *reg, *dst],
                Arg::Value(val) => vec![opcodes::LOAD_MEMORY, *val, *dst],
            }
            Instruction::PUSH(src) => match src {
                Arg::Register(reg) => vec![opcodes::PUSH_REGISTER, *reg],
                Arg::Value(val) => vec![opcodes::PUSH_LITERAL, *val],
            }
            Instruction::POP(src) => vec![opcodes::POP, *src],
            Instruction::CALL(src) => vec![opcodes::CALL, *src],
            Instruction::RET => vec![opcodes::RETURN],
            Instruction::JUMP(src) => match src {
                Arg::Register(reg) => vec![opcodes::JUMP_REGISTER, *reg],
                Arg::Value(val) => vec![opcodes::JUMP_LITERAL, *val],
            }
            Instruction::MOD(src, dst) => match src {
                Arg::Register(reg) => vec![opcodes::MOD_REGISTER, *reg, *dst],
                Arg::Value(val) => vec![opcodes::MOD_LITERAL, *val, *dst],
            }
            Instruction::NOT(src) => vec![opcodes::NOT, *src],
            Instruction::ADD(src, dst) => match src {
                Arg::Register(reg) => vec![opcodes::ADD_REGISTER, *reg, *dst],
                Arg::Value(val) => vec![opcodes::ADD_LITERAL, *val, *dst],
            }
            Instruction::SUB(src, dst) => match src {
                Arg::Register(reg) => vec![opcodes::SUB_REGISTER, *reg, *dst],
                Arg::Value(val) => vec![opcodes::SUB_LITERAL, *val, *dst],
            }
            Instruction::MUL(src, dst) => match src {
                Arg::Register(reg) => vec![opcodes::MUL_REGISTER, *reg, *dst],
                Arg::Value(val) => vec![opcodes::MUL_LITERAL, *val, *dst],
            }
            Instruction::DIV(src, dst) => match src {
                Arg::Register(reg) => vec![opcodes::DIV_REGISTER, *reg, *dst],
                Arg::Value(val) => vec![opcodes::DIV_LITERAL, *val, *dst],
            }
            Instruction::AND(src, dst) => match src {
                Arg::Register(reg) => vec![opcodes::AND_REGISTER, *reg, *dst],
                Arg::Value(val) => vec![opcodes::AND_LITERAL, *val, *dst],
            }
            Instruction::NAND(src, dst) => match src {
                Arg::Register(reg) => vec![opcodes::NAND_REGISTER, *reg, *dst],
                Arg::Value(val) => vec![opcodes::NAND_LITERAL, *val, *dst],
            }
            Instruction::OR(src, dst) => match src {
                Arg::Register(reg) => vec![opcodes::OR_REGISTER, *reg, *dst],
                Arg::Value(val) => vec![opcodes::OR_LITERAL, *val, *dst],
            }
            Instruction::NOR(src, dst) => match src {
                Arg::Register(reg) => vec![opcodes::NOR_REGISTER, *reg, *dst],
                Arg::Value(val) => vec![opcodes::NOR_LITERAL, *val, *dst],
            }
            Instruction::XOR(src, dst) => match src {
                Arg::Register(reg) => vec![opcodes::XOR_REGISTER, *reg, *dst],
                Arg::Value(val) => vec![opcodes::XOR_LITERAL, *val, *dst],
            }
            Instruction::XNOR(src, dst) => match src {
                Arg::Register(reg) => vec![opcodes::XNOR_REGISTER, *reg, *dst],
                Arg::Value(val) => vec![opcodes::XNOR_LITERAL, *val, *dst],
            }
            Instruction::BSL(src, dst) => match src {
                Arg::Register(reg) => vec![opcodes::BSL_REGISTER, *reg, *dst],
                Arg::Value(val) => vec![opcodes::BSL_LITERAL, *val, *dst],
            }
            Instruction::BSR(src, dst) => match src {
                Arg::Register(reg) => vec![opcodes::BSR_REGISTER, *reg, *dst],
                Arg::Value(val) => vec![opcodes::BSR_LITERAL, *val, *dst],
            }
            Instruction::INC(src) => vec![opcodes::INC, *src],
            Instruction::DEC(src) => vec![opcodes::DEC, *src],
        }
    }
}
