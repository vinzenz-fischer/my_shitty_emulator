use super::opcodes;

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
