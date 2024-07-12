# My shitty emulator
My own emulator! :D

## My own Assembly Language
### Syntax
`<instruction> <a> <b>`

### Registers
|     Name      | Purpose                  | Size |
| :-----------: | :----------------------- | ---: |
|     `RPC`     | Program Counter Register |    8 |
|     `RIN`     | Instruction Register     |    8 |
| `R00` - `R15` | General Purpose          |    8 |

### Instructions
|   Name    | Description                                                      |  Op Code   | Notes             |
| :-------: | :--------------------------------------------------------------- | :--------: | :---------------- |
|  `NOOP`   | Does nothing                                                     | `00000000` |                   |
|  `HALT`   | Stops the program                                                | `00000001` |                   |
|  `SETV`   | register `a` <- given value `b`                                  | `00000010` | `b` is only 8-bit |
|  `SETR`   | register `a` <- value in register `b`                            | `00000011` |                   |
|  `SETA`   | register `a` <- value at address `b`                             | `00000100` |                   |
| `SYSCALL` | Performs a system call (we'll figure that out later)             | `00000101` |                   |
|  `SMEM`   | value at address `a` <- value in register `b`                    | `00000111` | almost forgor     |
|           |                                                                  |            |                   |
|   `MOD`   | register `a` <- value in register `a` %    value in register `b` | `00010000` |                   |
|   `ADD`   | register `a` <- value in register `a` +    value in register `b` | `00010001` |                   |
|   `MUL`   | register `a` <- value in register `a` *    value in register `b` | `00010010` |                   |
|   `AND`   | register `a` <- value in register `a` AND  value in register `b` | `00010011` |                   |
|   `OR`    | register `a` <- value in register `a` OR   value in register `b` | `00010100` |                   |
|   `XOR`   | register `a` <- value in register `a` XOR  value in register `b` | `00010101` |                   |
|   `INC`   | register `a` <- value in register `a` + 1                        | `00010110` |                   |
|   `BSL`   | register `a` <- value in register `a` << value in register `b`   | `00010111` |                   |
|           |                                                                  |            |                   |
|   `NOT`   | register `a` <- Bitwise NOT value in register `a`                | `00011000` |                   |
|   `SUB`   | register `a` <- value in register `a` -    value in register `b` | `00011001` |                   |
|   `DIV`   | register `a` <- value in register `a` /    value in register `b` | `00011010` | floor division    |
|  `NAND`   | register `a` <- value in register `a` NAND value in register `b` | `00011011` |                   |
|   `NOR`   | register `a` <- value in register `a` NOR  value in register `b` | `00011100` |                   |
|  `XNOR`   | register `a` <- value in register `a` XNOR value in register `b` | `00011101` |                   |
|   `DEC`   | register `a` <- value in register `a` - 1                        | `00011110` |                   |
|   `BSR`   | register `a` <- value in register `a` >> value in register `b`   | `00011111` |                   |
