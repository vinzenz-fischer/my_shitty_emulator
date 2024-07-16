use super::Instruction;

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
