# TO-DO
- [x] simplest possible thing that could be called an emulator
- [x] hello world program
- [x] dedicated stack_pointer register
  - [ ] PROBLEM: How to properly set RSP to some value? I'd have to build a kernel that manages where in RAM the instructions and data go...
- [x] dedicated PUSH(u8) instruction
- [ ] comparison, conditional instructions
- [ ] bigger registers to allow for more RAM (than just 256 bytes)
- [ ] memory manager (run on the emulator)
- [ ] debugging tool(s)
- [ ] custom assembler
- [ ] interpreter mode (input instructions live in terminal) <!--maybe not>

| Question                                     | Identifies as an Answer                              |
| :------------------------------------------- | :--------------------------------------------------- |
| How to properly set RSP to some value?       | bruh i'll have to actually manage memory             |
| Literal values as constants in the bytecode? | Could increase performance, idk, not now later maybe |
