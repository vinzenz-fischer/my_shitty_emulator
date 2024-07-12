#[allow(unused)] // TODO: remove
pub struct Emulator {
    pub memory: Vec<u8>,
    pub registers: Vec<u8>,
    pub program_counter: u8,
    pub is_halted: bool,
}

impl Emulator {
    /// Creates a new emulator with memory of `ram_size` bytes and `gp_register_count` general purpose registers.
    pub fn new(ram_size: u8, gp_register_count: u8) -> Self {
        assert!(ram_size > 0, "RAM size must be greater than 0");
        Self {
            memory: vec![0; ram_size as usize],
            registers: vec![0; gp_register_count as usize],
            program_counter: 0,
            is_halted: false,
        }
    }

    /// Execute the next instruction
    pub fn step(&mut self) {
        let instruction = self.memory[self.program_counter as usize];
        let a = self.memory[(self.program_counter + 1) as usize];
        let b = self.memory[(self.program_counter + 2) as usize];

        self.program_counter += match Instruction::from_byte(instruction).unwrap() {
            Instruction::NOOP => 1,
            Instruction::HALT => {
                self.is_halted = true;
                0
            },
            Instruction::SETV => { self.registers[a as usize] = b; 3 }
            Instruction::SETR => { self.registers[a as usize] = self.registers[b as usize]; 3 }
            Instruction::SETA => { self.registers[a as usize] = self.memory[b as usize] as u8; 3 }
            Instruction::SYSCALL => {
                println!("[SYSCALL] a = {}, b = {}", a, b);
                match a {
                    0 => { // Print-Syscall: `b` points to an array of chars (0th byte is length)
                        let start = self.registers[b as usize];
                        let length = self.memory[start as usize];
                        for i in 1..=length {
                            print!("{}", self.memory[(start + i) as usize] as char);
                        }
                        println!("");
                    }
                    _ => {
                        println!("Unknown System Call: {}", a);
                    }
                };
                3
            }
            Instruction::SMEM => {
                println!("[SMEM] memory[{}] = {}", self.registers[a as usize], self.registers[b as usize]);
                self.memory[self.registers[a as usize] as usize] = self.registers[b as usize];
                3
            }
            Instruction::MOD  => { self.registers[a as usize] %= self.registers[b as usize]; 3 }
            Instruction::ADD  => { self.registers[a as usize] += self.registers[b as usize]; 3 }
            Instruction::MUL  => { self.registers[a as usize] *= self.registers[b as usize]; 3 }
            Instruction::AND  => { self.registers[a as usize] &= self.registers[b as usize]; 3 }
            Instruction::OR   => { self.registers[a as usize] |= self.registers[b as usize]; 3 }
            Instruction::XOR  => { self.registers[a as usize] ^= self.registers[b as usize]; 3 }
            Instruction::INC  => { self.registers[a as usize] += 1; 1 }
            Instruction::BSL  => { self.registers[a as usize] <<= self.registers[b as usize]; 3 }
            Instruction::NOT  => { self.registers[a as usize] = !self.registers[a as usize]; 1 }
            Instruction::SUB  => { self.registers[a as usize] -= self.registers[b as usize]; 3 }
            Instruction::DIV  => { self.registers[a as usize] /= self.registers[b as usize]; 3 }
            Instruction::NAND => { self.registers[a as usize] = !(self.registers[a as usize] & self.registers[b as usize]); 3 }
            Instruction::NOR  => { self.registers[a as usize] = !(self.registers[a as usize] | self.registers[b as usize]); 3 }
            Instruction::XNOR => { self.registers[a as usize] = !(self.registers[a as usize] ^ self.registers[b as usize]); 3 }
            Instruction::DEC  => { self.registers[a as usize] -= 1; 1 }
            Instruction::BSR  => { self.registers[a as usize] >>= self.registers[b as usize]; 3 }
        };
    }
}

pub enum Instruction {
    NOOP    = 0b00000000,
    HALT    = 0b00000001,
    SETV    = 0b00000010,
    SETR    = 0b00000011,
    SETA    = 0b00000100,
    SYSCALL = 0b00000101,
    SMEM    = 0b00000111,
    MOD     = 0b00001000,
    ADD     = 0b00010000,
    MUL     = 0b00010010,
    AND     = 0b00010011,
    OR      = 0b00010100,
    XOR     = 0b00010101,
    INC     = 0b00010110,
    BSL     = 0b00010111,
    NOT     = 0b00011000,
    SUB     = 0b00011001,
    DIV     = 0b00011010,
    NAND    = 0b00011011,
    NOR     = 0b00011100,
    XNOR    = 0b00011101,
    DEC     = 0b00011110,
    BSR     = 0b00011111,
}

impl Instruction {
    pub fn from_byte(byte: u8) -> Result<Self, ()> {
        Ok(match byte {
            0b00000000 => Self::NOOP,
            0b00000001 => Self::HALT,
            0b00000010 => Self::SETV,
            0b00000011 => Self::SETR,
            0b00000100 => Self::SETA,
            0b00000101 => Self::SYSCALL,
            0b00000111 => Self::SMEM,
            0b00001000 => Self::MOD,
            0b00010000 => Self::ADD,
            0b00010010 => Self::MUL,
            0b00010011 => Self::AND,
            0b00010100 => Self::OR,
            0b00010101 => Self::XOR,
            0b00010110 => Self::INC,
            0b00010111 => Self::BSL,
            0b00011000 => Self::NOT,
            0b00011001 => Self::SUB,
            0b00011010 => Self::DIV,
            0b00011011 => Self::NAND,
            0b00011100 => Self::NOR,
            0b00011101 => Self::XNOR,
            0b00011110 => Self::DEC,
            0b00011111 => Self::BSR,
            _ => return Err(()),
        })
    }

    pub fn to_byte(&self) -> u8 {
        match self {
            Self::NOOP    => 0b00000000,
            Self::HALT    => 0b00000001,
            Self::SETV    => 0b00000010,
            Self::SETR    => 0b00000011,
            Self::SETA    => 0b00000100,
            Self::SYSCALL => 0b00000101,
            Self::SMEM    => 0b00000111,
            Self::MOD     => 0b00010000,
            Self::ADD     => 0b00010001,
            Self::MUL     => 0b00010010,
            Self::AND     => 0b00010011,
            Self::OR      => 0b00010100,
            Self::XOR     => 0b00010101,
            Self::INC     => 0b00010110,
            Self::BSL     => 0b00010111,
            Self::NOT     => 0b00011000,
            Self::SUB     => 0b00011001,
            Self::DIV     => 0b00011010,
            Self::NAND    => 0b00011011,
            Self::NOR     => 0b00011100,
            Self::XNOR    => 0b00011101,
            Self::DEC     => 0b00011110,
            Self::BSR     => 0b00011111,
        }
    }
}
