/* I got an idea
struct Instruction {
    name: String,
    args: Vec<Arg>,
    opcode: u8,
    micro_instructions: Vec<...>, // this should replace `opcode` later
}

How do you declare new Instructions?
> `const MOVE_LITERAL: Instruction = Instruction::new("MOVE", [Arg::Any, Arg::Register], ...);`
> Maybe even load the instructions from markdown file table??

Now how would you do the `match instruction {...}` in the emulator?
> Microcode might be able to fix this
> Check opcode `instructions::MOVE_LITERAL.opcode => {...}`

How do I ensure unique opcodes?
> Could you check this at compile time using a macro?
*/

pub enum Arg {
    Register,
    Any,
}

impl Clone for Arg {
    fn clone(&self) -> Self {
        match self {
            Self::Register => Self::Register,
            Self::Any => Self::Any,
        }
    }
}

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
        write!(f, "{:^16}",
            match self {
                Instruction::DEBUG => "Instruction::DEBUG",
                Instruction::NOOP => "Instruction::NOOP",
                Instruction::HALT => "Instruction::HALT",
                Instruction::MOVE => "Instruction::MOVE",
                Instruction::SMEM => "Instruction::SMEM",
                Instruction::LMEM => "Instruction::LMEM",
                Instruction::PUSH => "Instruction::PUSH",
                Instruction::POP  => "Instruction::POP",
                Instruction::CALL => "Instruction::CALL",
                Instruction::RET  => "Instruction::RET",
                Instruction::JUMP => "Instruction::JUMP",
                Instruction::MOD  => "Instruction::MOD",
                Instruction::NOT  => "Instruction::NOT",
                Instruction::ADD  => "Instruction::ADD",
                Instruction::SUB  => "Instruction::SUB",
                Instruction::MUL  => "Instruction::MUL",
                Instruction::DIV  => "Instruction::DIV",
                Instruction::AND  => "Instruction::AND",
                Instruction::NAND => "Instruction::NAND",
                Instruction::OR   => "Instruction::OR",
                Instruction::NOR  => "Instruction::NOR",
                Instruction::XOR  => "Instruction::XOR",
                Instruction::XNOR => "Instruction::XNOR",
                Instruction::BSL  => "Instruction::BSL",
                Instruction::BSR  => "Instruction::BSR",
                Instruction::INC  => "Instruction::INC",
                Instruction::DEC  => "Instruction::DEC",
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

    pub fn to_opcode(&self) -> Result<u8, ()> {
        match self {
            _ => todo!("{}::to_opcode()", self)
        }
    }
}
