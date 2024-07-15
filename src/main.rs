use my_shitty_emulator::*;

fn main() {
    let mut emulator = Emulator::new(u8::MAX);

    let hello_world = Program::from_instructions(vec![
        Instruction::PUSH(Arg::Value(13)),
        Instruction::PUSH(Arg::Value('H' as u8)),
        Instruction::PUSH(Arg::Value('e' as u8)),
        Instruction::PUSH(Arg::Value('l' as u8)),
        Instruction::PUSH(Arg::Value('l' as u8)),
        Instruction::PUSH(Arg::Value('o' as u8)),
        Instruction::PUSH(Arg::Value(',' as u8)),
        Instruction::PUSH(Arg::Value(' ' as u8)),
        Instruction::PUSH(Arg::Value('W' as u8)),
        Instruction::PUSH(Arg::Value('o' as u8)),
        Instruction::PUSH(Arg::Value('r' as u8)),
        Instruction::PUSH(Arg::Value('l' as u8)),
        Instruction::PUSH(Arg::Value('d' as u8)),
        Instruction::PUSH(Arg::Value('!' as u8)),
        Instruction::DEBUG(0, 33),
        Instruction::HALT
    ]);

    emulator.load_program(hello_world);
    
    println!("Go!");
    while !emulator.is_halted {
        emulator.step();
    }

    // emulator.print_memory();
}
