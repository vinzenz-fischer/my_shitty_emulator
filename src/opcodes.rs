// TODO: The opcodes will need some changing before we can properly emulate a CPU
pub const NOOP         : u8 = 0b0000_0000; pub const HALT          : u8 = 0b0000_0001;
pub const MOVE_LITERAL : u8 = 0b0000_0011; pub const MOVE_REGISTER : u8 = 0b0000_0010;
pub const STORE_LITERAL: u8 = 0b0000_0100; pub const STORE_REGISTER: u8 = 0b0000_0101;
pub const LOAD_MEMORY  : u8 = 0b0000_0110;
pub const DEBUG        : u8 = 0b0000_0111;
pub const PUSH_LITERAL : u8 = 0b0000_1000; pub const PUSH_REGISTER : u8 = 0b0000_1001;
pub const POP          : u8 = 0b0000_1010;
pub const CALL         : u8 = 0b0000_1011; pub const RETURN        : u8 = 0b0000_1100;
pub const JUMP_LITERAL : u8 = 0b0000_1101; pub const JUMP_REGISTER : u8 = 0b0000_1110;
pub const MOD_LITERAL  : u8 = 0b0000_1111; pub const MOD_REGISTER  : u8 = 0b0001_0000;
pub const ADD_LITERAL  : u8 = 0b0001_0001; pub const ADD_REGISTER  : u8 = 0b0001_0010;
pub const SUB_LITERAL  : u8 = 0b0001_0011; pub const SUB_REGISTER  : u8 = 0b0001_0100;
pub const MUL_LITERAL  : u8 = 0b0001_0101; pub const MUL_REGISTER  : u8 = 0b0001_0110;
pub const DIV_LITERAL  : u8 = 0b0001_0111; pub const DIV_REGISTER  : u8 = 0b0001_1000;
pub const AND_LITERAL  : u8 = 0b0001_1001; pub const AND_REGISTER  : u8 = 0b0001_1010;
pub const NAND_LITERAL : u8 = 0b0001_1011; pub const NAND_REGISTER : u8 = 0b0001_1100;
pub const OR_LITERAL   : u8 = 0b0001_1101; pub const OR_REGISTER   : u8 = 0b0001_1110;
pub const NOR_LITERAL  : u8 = 0b0001_1111; pub const NOR_REGISTER  : u8 = 0b0010_0000;
pub const XOR_LITERAL  : u8 = 0b0010_0001; pub const XOR_REGISTER  : u8 = 0b0010_0010;
pub const XNOR_LITERAL : u8 = 0b0010_0011; pub const XNOR_REGISTER : u8 = 0b0010_0100;
pub const BSL_LITERAL  : u8 = 0b0010_0101; pub const BSL_REGISTER  : u8 = 0b0010_0110;
pub const BSR_LITERAL  : u8 = 0b0010_0111; pub const BSR_REGISTER  : u8 = 0b0010_1000;
pub const INC          : u8 = 0b0010_1001; pub const DEC           : u8 = 0b0010_1010;
pub const NOT          : u8 = 0b0010_1011;