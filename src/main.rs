use my_shitty_emulator::*;

fn main() {
    let assembly = {
        let contents = std::fs::read("myasm_examples/helloworld.myasm").expect("Failed to read file.");
        String::from_utf8(contents).expect("File contains non-UTF-8 bytes.")
    };

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
