/* I got an idea
struct Instruction {
    name: String,
    args: Vec<Arg>,
    opcode: u8,
    micro_instructions: Vec<…>, // this should replace `opcode` later
}

How do you declare new Instructions?
> `const MOVE_LITERAL: Instruction = Instruction::new("MOVE", [Arg::Any, Arg::Register], …);`
> Maybe even load the instructions from markdown file table??

Now how would you do the `match instruction {…}` in the emulator?
> Microcode might be able to fix this
> Check opcode `instructions::MOVE_LITERAL.opcode => {…}`

How do I ensure unique opcodes?
> Could you check this at compile time using a macro?
*/

use crate::opcodes;

#[derive(Debug, Copy, Clone)]
pub enum Arg {
    Register,
    Any,
}

#[derive(Debug, Copy, Clone)]
pub enum Instruction {
    DEBUG, // print out stuff for example
    NOOP, HALT,
    MOVE,
    SMEM, LMEM,
    PUSH, POP ,
    CALL, RET,
    JUMP,
    MOD , NOT ,
    ADD , SUB ,
    MUL , DIV ,
    AND , NAND,
    OR  , NOR ,
    XOR , XNOR,
    BSL , BSR ,
    INC , DEC ,
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:}",
            match self {
                Instruction::DEBUG => "DEBUG",
                Instruction::NOOP => "NOOP",
                Instruction::HALT => "HALT",
                Instruction::MOVE => "MOVE",
                Instruction::SMEM => "SMEM",
                Instruction::LMEM => "LMEM",
                Instruction::PUSH => "PUSH",
                Instruction::POP  => "POP",
                Instruction::CALL => "CALL",
                Instruction::RET  => "RET",
                Instruction::JUMP => "JUMP",
                Instruction::MOD  => "MOD",
                Instruction::NOT  => "NOT",
                Instruction::ADD  => "ADD",
                Instruction::SUB  => "SUB",
                Instruction::MUL  => "MUL",
                Instruction::DIV  => "DIV",
                Instruction::AND  => "AND",
                Instruction::NAND => "NAND",
                Instruction::OR   => "OR",
                Instruction::NOR  => "NOR",
                Instruction::XOR  => "XOR",
                Instruction::XNOR => "XNOR",
                Instruction::BSL  => "BSL",
                Instruction::BSR  => "BSR",
                Instruction::INC  => "INC",
                Instruction::DEC  => "DEC",
            }
        )
    }
}

impl Instruction {
    pub fn from_slice(slice: &str) -> Option<Instruction> {
        match slice.to_uppercase().as_str() {
            "DEBUG" => Some(Instruction::DEBUG),
            "NOOP" => Some(Instruction::NOOP),
            "HALT" => Some(Instruction::HALT),
            "MOVE" => Some(Instruction::MOVE),
            "SMEM" => Some(Instruction::SMEM),
            "LMEM" => Some(Instruction::LMEM),
            "PUSH" => Some(Instruction::PUSH),
            "POP"  => Some(Instruction::POP),
            "CALL" => Some(Instruction::CALL),
            "RET"  => Some(Instruction::RET),
            "JUMP" => Some(Instruction::JUMP),
            "MOD"  => Some(Instruction::MOD),
            "NOT"  => Some(Instruction::NOT),
            "ADD"  => Some(Instruction::ADD),
            "SUB"  => Some(Instruction::SUB),
            "MUL"  => Some(Instruction::MUL),
            "DIV"  => Some(Instruction::DIV),
            "AND"  => Some(Instruction::AND),
            "NAND" => Some(Instruction::NAND),
            "OR"   => Some(Instruction::OR),
            "NOR"  => Some(Instruction::NOR),
            "XOR"  => Some(Instruction::XOR),
            "XNOR" => Some(Instruction::XNOR),
            "BSL"  => Some(Instruction::BSL),
            "BSR"  => Some(Instruction::BSR),
            "INC"  => Some(Instruction::INC),
            "DEC"  => Some(Instruction::DEC),
            _ => None
        }
    }

    pub fn get_arguments(instruction: Instruction) -> Vec<Arg> {
        match instruction {
            Instruction::DEBUG => vec![Arg::Register, Arg::Register],
            Instruction::NOOP => vec![],
            Instruction::HALT => vec![],
            Instruction::MOVE => vec![Arg::Any, Arg::Register ],
            Instruction::SMEM => vec![Arg::Any, Arg::Register ],
            Instruction::LMEM => vec![Arg::Any, Arg::Register],
            Instruction::PUSH => vec![Arg::Any],
            Instruction::POP  => vec![Arg::Register],
            Instruction::CALL => vec![Arg::Register],
            Instruction::RET  => vec![],
            Instruction::JUMP => vec![Arg::Any],
            Instruction::MOD  => vec![Arg::Any, Arg::Register ],
            Instruction::NOT  => vec![Arg::Register],
            Instruction::ADD  => vec![Arg::Any, Arg::Register ],
            Instruction::SUB  => vec![Arg::Any, Arg::Register],
            Instruction::MUL  => vec![Arg::Any, Arg::Register ],
            Instruction::DIV  => vec![Arg::Any, Arg::Register],
            Instruction::AND  => vec![Arg::Any, Arg::Register ],
            Instruction::NAND => vec![Arg::Any, Arg::Register],
            Instruction::OR   => vec![Arg::Any, Arg::Register],
            Instruction::NOR  => vec![Arg::Any, Arg::Register],
            Instruction::XOR  => vec![Arg::Any, Arg::Register],
            Instruction::XNOR => vec![Arg::Any, Arg::Register],
            Instruction::BSL  => vec![Arg::Any, Arg::Register],
            Instruction::BSR  => vec![Arg::Any, Arg::Register],
            Instruction::INC  => vec![Arg::Register],
            Instruction::DEC  => vec![Arg::Register],
        }
    }

    pub fn to_opcode(&self) -> Option<u8> {
        match self {
            Instruction::DEBUG => Some(opcodes::DEBUG),
            Instruction::NOOP => Some(opcodes::NOOP),
            Instruction::HALT => Some(opcodes::HALT),
            Instruction::MOVE => Some(opcodes::MOVE),
            Instruction::SMEM => Some(opcodes::SMEM),
            Instruction::LMEM => Some(opcodes::LMEM),
            Instruction::PUSH => Some(opcodes::PUSH),
            Instruction::POP  => Some(opcodes::POP),
            Instruction::CALL => Some(opcodes::CALL),
            Instruction::RET  => Some(opcodes::RET),
            Instruction::JUMP => Some(opcodes::JUMP),
            Instruction::MOD  => Some(opcodes::MOD),
            Instruction::NOT  => Some(opcodes::NOT),
            Instruction::ADD  => Some(opcodes::ADD),
            Instruction::SUB  => Some(opcodes::SUB),
            Instruction::MUL  => Some(opcodes::MUL),
            Instruction::DIV  => Some(opcodes::DIV),
            Instruction::AND  => Some(opcodes::AND),
            Instruction::NAND => Some(opcodes::NAND),
            Instruction::OR   => Some(opcodes::OR),
            Instruction::NOR  => Some(opcodes::NOR),
            Instruction::XOR  => Some(opcodes::XOR),
            Instruction::XNOR => Some(opcodes::XNOR),
            Instruction::BSL  => Some(opcodes::BSL),
            Instruction::BSR  => Some(opcodes::BSR),
            Instruction::INC  => Some(opcodes::INC),
            Instruction::DEC  => Some(opcodes::DEC),
        }
    }
}
