mod emulator;
mod instruction;
mod opcodes;
mod program;

pub use emulator::Emulator;
pub use instruction::{Instruction, Arg};
pub use program::Program;
