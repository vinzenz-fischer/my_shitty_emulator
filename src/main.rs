use my_shitty_emulator::{Emulator, Instruction};

fn main() {
    let mut emulator = Emulator::new(u8::MAX, 8);

    println!("Writing program to memory...");
    // Use register 0 as stack pointer
    emulator.memory[0] = Instruction::SETV.to_byte();
    emulator.memory[1] = 0; // register a
    emulator.memory[2] = 120; // value b

    // Register 1 = string length
    emulator.memory[3] = Instruction::SETV.to_byte();
    emulator.memory[4] = 1; // register a
    emulator.memory[5] = 11; // value b

    // write to memory
    emulator.memory[6] = Instruction::SMEM.to_byte();
    emulator.memory[7] = 0; // stack pointer
    emulator.memory[8] = 1; // register b

    // increment stack pointer
    emulator.memory[9] = Instruction::INC.to_byte();
    emulator.memory[10] = 0;

    // Set register 1 to 72 (ascii 'h')
    emulator.memory[11] = Instruction::SETV.to_byte();
    emulator.memory[12] = 1; // register a
    emulator.memory[13] = 72; // value b

    // write to memory
    emulator.memory[14] = Instruction::SMEM.to_byte();
    emulator.memory[15] = 0; // stack pointer
    emulator.memory[16] = 1; // value b

    // increment stack pointer
    emulator.memory[17] = Instruction::INC.to_byte();
    emulator.memory[18] = 0; // register a

    // set register 1 to 101 (ascii 'e')
    emulator.memory[19] = Instruction::SETV.to_byte();
    emulator.memory[20] = 1; // register a
    emulator.memory[21] = 101; // value b

    // write to memory
    emulator.memory[22] = Instruction::SMEM.to_byte();
    emulator.memory[23] = 0; // stack pointer
    emulator.memory[24] = 1; // value b

    // increment stack pointer
    emulator.memory[25] = Instruction::INC.to_byte();
    emulator.memory[26] = 0;

    // set register 1 to 108 (ascii 'l')
    emulator.memory[27] = Instruction::SETV.to_byte();
    emulator.memory[28] = 1; // register a
    emulator.memory[29] = 108; // value b

    // write to memory
    emulator.memory[30] = Instruction::SMEM.to_byte();
    emulator.memory[31] = 0; // stack pointer
    emulator.memory[32] = 1; // value b

    // increment stack pointer
    emulator.memory[33] = Instruction::INC.to_byte();
    emulator.memory[34] = 0;

    //write to memory
    emulator.memory[35] = Instruction::SMEM.to_byte();
    emulator.memory[36] = 0; // stack pointer
    emulator.memory[37] = 1; // value b

    // increment stack pointer
    emulator.memory[38] = Instruction::INC.to_byte();
    emulator.memory[39] = 0;

    // set register 1 to 111 (ascii 'o')
    emulator.memory[40] = Instruction::SETV.to_byte();
    emulator.memory[41] = 1; // register a
    emulator.memory[42] = 111; // value b

    // write to memory
    emulator.memory[43] = Instruction::SMEM.to_byte();
    emulator.memory[44] = 0; // stack pointer
    emulator.memory[45] = 1; // value b

    // increment stack pointer
    emulator.memory[46] = Instruction::INC.to_byte();
    emulator.memory[47] = 0;

    // set register 1 to 32 (ascii ' ')
    emulator.memory[48] = Instruction::SETV.to_byte();
    emulator.memory[49] = 1; // register a
    emulator.memory[50] = 32; // value b

    // write to memory
    emulator.memory[51] = Instruction::SMEM.to_byte();
    emulator.memory[52] = 0; // stack pointer
    emulator.memory[53] = 1; // value b

    // increment stack pointer
    emulator.memory[54] = Instruction::INC.to_byte();
    emulator.memory[55] = 0;

    // set register 1 to 119 (ascii 'w')
    emulator.memory[56] = Instruction::SETV.to_byte();
    emulator.memory[57] = 1; // register a
    emulator.memory[58] = 119; // value b

    // write to memory
    emulator.memory[59] = Instruction::SMEM.to_byte();
    emulator.memory[60] = 0; // stack pointer
    emulator.memory[61] = 1; // value b

    // increment stack pointer
    emulator.memory[62] = Instruction::INC.to_byte();
    emulator.memory[63] = 0;

    // set register 1 to 111 (ascii 'o')
    emulator.memory[64] = Instruction::SETV.to_byte();
    emulator.memory[65] = 1; // register a
    emulator.memory[66] = 111; // value b

    // write to memory
    emulator.memory[67] = Instruction::SMEM.to_byte();
    emulator.memory[68] = 0; // stack pointer
    emulator.memory[69] = 1; // value b

    // increment stack pointer
    emulator.memory[70] = Instruction::INC.to_byte();
    emulator.memory[71] = 0;

    // set register 1 to 114 (ascii 'r')
    emulator.memory[72] = Instruction::SETV.to_byte();
    emulator.memory[73] = 1; // register a
    emulator.memory[74] = 114; // value b

    // write to memory
    emulator.memory[75] = Instruction::SMEM.to_byte();
    emulator.memory[76] = 0; // stack pointer
    emulator.memory[77] = 1; // value b

    // increment stack pointer
    emulator.memory[78] = Instruction::INC.to_byte();
    emulator.memory[79] = 0;

    // set register 1 to 108 (ascii 'l')
    emulator.memory[80] = Instruction::SETV.to_byte();
    emulator.memory[81] = 1; // register a
    emulator.memory[82] = 108; // value b

    // write to memory
    emulator.memory[83] = Instruction::SMEM.to_byte();
    emulator.memory[84] = 0; // stack pointer
    emulator.memory[85] = 1; // value b

    // increment stack pointer
    emulator.memory[86] = Instruction::INC.to_byte();
    emulator.memory[87] = 0;

    // set register 1 to 100 (ascii 'd')
    emulator.memory[88] = Instruction::SETV.to_byte();
    emulator.memory[89] = 1; // register a
    emulator.memory[90] = 100; // value b

    // write to memory
    emulator.memory[91] = Instruction::SMEM.to_byte();
    emulator.memory[92] = 0; // stack pointer
    emulator.memory[93] = 1; // value b

    // increment stack pointer
    emulator.memory[94] = Instruction::INC.to_byte();
    emulator.memory[95] = 0;

    // set register 1 to 120
    emulator.memory[96] = Instruction::SETV.to_byte();
    emulator.memory[97] = 1; // register a
    emulator.memory[98] = 120; // value b

    // print string
    emulator.memory[99] = Instruction::SYSCALL.to_byte();
    emulator.memory[100] = 0; // register a
    emulator.memory[101] = 1; // register b

    emulator.memory[102] = Instruction::HALT.to_byte();

    println!("Go!");
    while !emulator.is_halted {
        emulator.step();
    }
}
