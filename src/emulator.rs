use super::Program;
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
