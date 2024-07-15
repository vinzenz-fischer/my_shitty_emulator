# TO-DO
- [x] simplest possible thing that could be called an emulator
- [x] hello world program
- [x] dedicated stack_pointer register
  - [ ] PROBLEM: How to properly set RSP to some value? I'd have to build an OS that manages where in RAM the instructions and data go...
- [x] dedicated PUSH(u8) instruction
- [ ] comparison, conditional instructions
- [ ] bigger registers to allow for more RAM (than just 256 bytes)
- [ ] memory manager (run on the emulator)
- [ ] debugging tool(s)
- [ ] custom assembler
- [ ] interpreter mode (input instructions live in terminal) <!--maybe not>

## What do I want my Instructions Implementation to be able to do?
* ONE place where I define the bytecode
* each argument can be a `#`*literal value* or *register index*
