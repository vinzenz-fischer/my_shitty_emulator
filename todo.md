# TO-DO

- [x] simplest possible thing that could be called an emulator
- [x] hello world program
- [ ] dedicated stack_pointer register
- [ ] dedicated PUSH(u8) instruction
- [ ] debugging tool(s)
- [ ] let every argument be either a *direct value* or a *register index*
  - for that we can use two op-code bits as flags
  - the only way we can/should read from memory is by `SETA` (which btw should be renamed to something like "LOAD")
  - we can use rust's rich type system :3
- [ ] assembler (for my custom assembly language)
- [ ] interpreter mode (input instructions live in terminal) <!--This isn't really an emulator thing but it'll be nice to play around with (maybe)>
