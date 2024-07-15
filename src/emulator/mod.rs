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
            NOOP => 1,
            HALT => { self.is_halted = true; 0 },
            MOVE_LITERAL  => { self.registers[src as usize] = dst; 3 }
            MOVE_REGISTER => { self.registers[src as usize] = self.registers[dst as usize]; 3 }
            LOAD_MEMORY   => { self.registers[src as usize] = self.memory[dst as usize] as u8; 3 }
            DEBUG => {
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
            STORE_LITERAL => {
                self.memory[self.registers[src as usize] as usize] = dst;
                3
            }
            STORE_REGISTER => {
                self.memory[self.registers[src as usize] as usize] = self.registers[dst as usize];
                3
            }
            PUSH_LITERAL => {
                self.stack_pointer += 1;
                self.memory[self.stack_pointer as usize] = src;
                2
            }
            PUSH_REGISTER => {
                self.stack_pointer += 1;
                self.memory[self.stack_pointer as usize] = self.registers[src as usize];
                2
            }
            POP  => {
                self.registers[src as usize] = self.memory[self.stack_pointer as usize];
                self.stack_pointer -= 1;
                2
            }
            CALL => {
                self.stack_pointer += 1;
                self.memory[self.stack_pointer as usize] = self.program_counter + 2;
                self.program_counter = src;
                2
            }
            RETURN => {
                self.program_counter = self.memory[self.stack_pointer as usize];
                self.stack_pointer -= 1;
                1
            }
            JUMP_LITERAL => { self.program_counter = src; 2 }
            JUMP_REGISTER => { self.program_counter = self.registers[src as usize]; 2 }

            MOD_LITERAL  => { self.registers[src as usize] %= dst; 3 }
            ADD_LITERAL  => { self.registers[src as usize] += dst; 3 }
            SUB_LITERAL  => { self.registers[src as usize] -= dst; 3 }
            MUL_LITERAL  => { self.registers[src as usize] *= dst; 3 }
            DIV_LITERAL  => { self.registers[src as usize] /= dst; 3 }
            AND_LITERAL  => { self.registers[src as usize] &= dst; 3 }
            NAND_LITERAL => { self.registers[src as usize] = !(self.registers[src as usize] & dst); 3 }
            OR_LITERAL   => { self.registers[src as usize] |= dst; 3 }
            NOR_LITERAL  => { self.registers[src as usize] = !(self.registers[src as usize] | dst); 3 }
            XOR_LITERAL  => { self.registers[src as usize] ^= dst; 3 }
            XNOR_LITERAL => { self.registers[src as usize] = !(self.registers[src as usize] ^ dst); 3 }
            BSL_LITERAL  => { self.registers[src as usize] <<= dst; 3 }
            BSR_LITERAL  => { self.registers[src as usize] >>= dst; 3 }

            MOD_REGISTER  => { self.registers[src as usize] %= self.registers[dst as usize]; 3 }
            ADD_REGISTER  => { self.registers[src as usize] += self.registers[dst as usize]; 3 }
            SUB_REGISTER  => { self.registers[src as usize] -= self.registers[dst as usize]; 3 }
            MUL_REGISTER  => { self.registers[src as usize] *= self.registers[dst as usize]; 3 }
            DIV_REGISTER  => { self.registers[src as usize] /= self.registers[dst as usize]; 3 }
            AND_REGISTER  => { self.registers[src as usize] &= self.registers[dst as usize]; 3 }
            NAND_REGISTER => { self.registers[src as usize] = !(self.registers[src as usize] & self.registers[dst as usize]); 3 }
            OR_REGISTER   => { self.registers[src as usize] |= self.registers[dst as usize]; 3 }
            NOR_REGISTER  => { self.registers[src as usize] = !(self.registers[src as usize] | self.registers[dst as usize]); 3 }
            XOR_REGISTER  => { self.registers[src as usize] ^= self.registers[dst as usize]; 3 }
            XNOR_REGISTER => { self.registers[src as usize] = !(self.registers[src as usize] ^ self.registers[dst as usize]); 3 }
            BSL_REGISTER  => { self.registers[src as usize] <<= self.registers[dst as usize]; 3 }
            BSR_REGISTER  => { self.registers[src as usize] >>= self.registers[dst as usize]; 3 }
            INC => { self.registers[src as usize] += 1; 1 }
            DEC => { self.registers[src as usize] -= 1; 1 }
            NOT => { self.registers[src as usize] = !self.registers[src as usize]; 1 }
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

// Opcodes
// TODO: The opcodes will need some changing before we can properly emulate a CPU
// TODO: Look for a way to group these constants together
const NOOP         : u8 = 0b0000_0000; const HALT          : u8 = 0b0000_0001;
const MOVE_LITERAL : u8 = 0b0000_0011; const MOVE_REGISTER : u8 = 0b0000_0010;
const STORE_LITERAL: u8 = 0b0000_0100; const STORE_REGISTER: u8 = 0b0000_0101;
const LOAD_MEMORY  : u8 = 0b0000_0110;
const DEBUG        : u8 = 0b0000_0111;
const PUSH_LITERAL : u8 = 0b0000_1000; const PUSH_REGISTER : u8 = 0b0000_1001;
const POP          : u8 = 0b0000_1010;
const CALL         : u8 = 0b0000_1011; const RETURN        : u8 = 0b0000_1100;
const JUMP_LITERAL : u8 = 0b0000_1101; const JUMP_REGISTER : u8 = 0b0000_1110;
const MOD_LITERAL  : u8 = 0b0000_1111; const MOD_REGISTER  : u8 = 0b0001_0000;
const ADD_LITERAL  : u8 = 0b0001_0001; const ADD_REGISTER  : u8 = 0b0001_0010;
const SUB_LITERAL  : u8 = 0b0001_0011; const SUB_REGISTER  : u8 = 0b0001_0100;
const MUL_LITERAL  : u8 = 0b0001_0101; const MUL_REGISTER  : u8 = 0b0001_0110;
const DIV_LITERAL  : u8 = 0b0001_0111; const DIV_REGISTER  : u8 = 0b0001_1000;
const AND_LITERAL  : u8 = 0b0001_1001; const AND_REGISTER  : u8 = 0b0001_1010;
const NAND_LITERAL : u8 = 0b0001_1011; const NAND_REGISTER : u8 = 0b0001_1100;
const OR_LITERAL   : u8 = 0b0001_1101; const OR_REGISTER   : u8 = 0b0001_1110;
const NOR_LITERAL  : u8 = 0b0001_1111; const NOR_REGISTER  : u8 = 0b0010_0000;
const XOR_LITERAL  : u8 = 0b0010_0001; const XOR_REGISTER  : u8 = 0b0010_0010;
const XNOR_LITERAL : u8 = 0b0010_0011; const XNOR_REGISTER : u8 = 0b0010_0100;
const BSL_LITERAL  : u8 = 0b0010_0101; const BSL_REGISTER  : u8 = 0b0010_0110;
const BSR_LITERAL  : u8 = 0b0010_0111; const BSR_REGISTER  : u8 = 0b0010_1000;
const INC          : u8 = 0b0010_1001; const DEC           : u8 = 0b0010_1010;
const NOT          : u8 = 0b0010_1011;

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
            Instruction::DEBUG(a, b) => vec![DEBUG, *a, *b],
            Instruction::NOOP => vec![NOOP],
            Instruction::HALT => vec![HALT],
            Instruction::MOVE(src, dst) => match src {
                Arg::Register(reg) => vec![MOVE_REGISTER, *reg, *dst],
                Arg::Value(val) => vec![MOVE_LITERAL, *val, *dst],
            }
            Instruction::SMEM(src, dst) => match src {
                Arg::Register(reg) => vec![STORE_REGISTER, *reg, *dst],
                Arg::Value(val) => vec![STORE_LITERAL, *val, *dst],
            }
            Instruction::LMEM(src, dst) => match src {
                Arg::Register(reg) => vec![LOAD_MEMORY, *reg, *dst],
                Arg::Value(val) => vec![LOAD_MEMORY, *val, *dst],
            }
            Instruction::PUSH(src) => match src {
                Arg::Register(reg) => vec![PUSH_REGISTER, *reg],
                Arg::Value(val) => vec![PUSH_LITERAL, *val],
            }
            Instruction::POP(src) => vec![POP, *src],
            Instruction::CALL(src) => vec![CALL, *src],
            Instruction::RET => vec![RETURN],
            Instruction::JUMP(src) => match src {
                Arg::Register(reg) => vec![JUMP_REGISTER, *reg],
                Arg::Value(val) => vec![JUMP_LITERAL, *val],
            }
            Instruction::MOD(src, dst) => match src {
                Arg::Register(reg) => vec![MOD_REGISTER, *reg, *dst],
                Arg::Value(val) => vec![MOD_LITERAL, *val, *dst],
            }
            Instruction::NOT(src) => vec![NOT, *src],
            Instruction::ADD(src, dst) => match src {
                Arg::Register(reg) => vec![ADD_REGISTER, *reg, *dst],
                Arg::Value(val) => vec![ADD_LITERAL, *val, *dst],
            }
            Instruction::SUB(src, dst) => match src {
                Arg::Register(reg) => vec![SUB_REGISTER, *reg, *dst],
                Arg::Value(val) => vec![SUB_LITERAL, *val, *dst],
            }
            Instruction::MUL(src, dst) => match src {
                Arg::Register(reg) => vec![MUL_REGISTER, *reg, *dst],
                Arg::Value(val) => vec![MUL_LITERAL, *val, *dst],
            }
            Instruction::DIV(src, dst) => match src {
                Arg::Register(reg) => vec![DIV_REGISTER, *reg, *dst],
                Arg::Value(val) => vec![DIV_LITERAL, *val, *dst],
            }
            Instruction::AND(src, dst) => match src {
                Arg::Register(reg) => vec![AND_REGISTER, *reg, *dst],
                Arg::Value(val) => vec![AND_LITERAL, *val, *dst],
            }
            Instruction::NAND(src, dst) => match src {
                Arg::Register(reg) => vec![NAND_REGISTER, *reg, *dst],
                Arg::Value(val) => vec![NAND_LITERAL, *val, *dst],
            }
            Instruction::OR(src, dst) => match src {
                Arg::Register(reg) => vec![OR_REGISTER, *reg, *dst],
                Arg::Value(val) => vec![OR_LITERAL, *val, *dst],
            }
            Instruction::NOR(src, dst) => match src {
                Arg::Register(reg) => vec![NOR_REGISTER, *reg, *dst],
                Arg::Value(val) => vec![NOR_LITERAL, *val, *dst],
            }
            Instruction::XOR(src, dst) => match src {
                Arg::Register(reg) => vec![XOR_REGISTER, *reg, *dst],
                Arg::Value(val) => vec![XOR_LITERAL, *val, *dst],
            }
            Instruction::XNOR(src, dst) => match src {
                Arg::Register(reg) => vec![XNOR_REGISTER, *reg, *dst],
                Arg::Value(val) => vec![XNOR_LITERAL, *val, *dst],
            }
            Instruction::BSL(src, dst) => match src {
                Arg::Register(reg) => vec![BSL_REGISTER, *reg, *dst],
                Arg::Value(val) => vec![BSL_LITERAL, *val, *dst],
            }
            Instruction::BSR(src, dst) => match src {
                Arg::Register(reg) => vec![BSR_REGISTER, *reg, *dst],
                Arg::Value(val) => vec![BSR_LITERAL, *val, *dst],
            }
            Instruction::INC(src) => vec![INC, *src],
            Instruction::DEC(src) => vec![DEC, *src],
        }
    }
}
