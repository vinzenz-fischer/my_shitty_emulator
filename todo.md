# TO-DO
- [x] simplest possible thing that could be called an emulator
- [x] hello world program
- [x] dedicated stack_pointer register
  - [ ] PROBLEM: How to properly set RSP to some value?
- [x] dedicated PUSH(u8) instruction
- [ ] debugging tool(s)
- [ ] custom assembler
- [ ] interpreter mode (input instructions live in terminal) <!--maybe not>

## What do I want my Instructions Implementation to be able to do?
* ONE place where I define the bytecode
* each argument can be a `#`*literal value* or *register index*

## Assembly Syntax
```
PUSH #5
PUSH #'H'
PUSH #'e'
PUSH #'l'
PUSH #'l'
PUSH #'o'
DEBUG #0 #0 // 1st arg: Print a string. 2nd arg: The string to print (first byte = length).
```
