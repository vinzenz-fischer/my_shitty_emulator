use crate::{AssemblerError, Assembler};

pub struct Program {
    bytecode: Vec<u8>,
}

impl Program {
    pub fn new() -> Self {
        Self { bytecode: Vec::new() }
    }
            
    pub fn get_bytecode(&self) -> &Vec<u8> {
        &self.bytecode
    }

    pub fn from_assembly(assembly: &str) -> Result<Program, Vec<AssemblerError>> {
        match Assembler::assemble(assembly) {
            Ok(bytecode) => Ok(
                Self::from_raw_bytes(bytecode)
            ),
            Err(problems) => Err(problems)
        }
    }
    
    fn from_raw_bytes(bytecode: Vec<u8>) -> Self {
        Self { bytecode }
    }
}
