use my_shitty_emulator::*;

fn main() {
    let assembly = std::fs::read_to_string("myasm_examples/helloworld.myasm").expect("Failed to read file.");

    match Program::from_assembly(&assembly.as_str())
    {
        Ok(program) => {
            let mut emulator = Emulator::new(u8::MAX);
            emulator.load_program(program);
            
            println!("Go!");
            while !emulator.is_halted {
                emulator.step();
            }
        },
        Err(problems) => for problem in problems {
            println!("{}\n", problem)
        },
    }
}
