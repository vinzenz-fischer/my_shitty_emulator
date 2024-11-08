# My shitty emulator
My own emulator! :D

## Registers
|     Name      | Purpose         | Size |
| :-----------: | :-------------- | ---: |
|     `RPC`     | Program Counter |    8 |
|     `RSP`     | Stack Pointer   |    8 |
| `R00` - `R15` | General Purpose |    8 |

## My own Assembly Language
* Syntax: `<instruction> <src> <dst>`

|  Name   | Description                                   | `src` | `dst` |
| :-----: | :-------------------------------------------- | :---- | :---- |
| `DEBUG` | Debug call `dst` `src`                        |       |       |
| `NOOP`  | Does nothing                                  |       |       |
| `HALT`  | Stops the program                             |       |       |
| `MOVE`  | reg. `dst` <- val./reg. `src`                 | R L   | R     |
| `LMEM`  | reg. `dst` <- memory `src`                    | M     |       |
| `SMEM`  | memory `dst` <- val./reg. `src`               | R L   | M     |
| `PUSH`  | `INC RSP`, memory `RSP` <- val./reg. `src`    | R L   |       |
|  `POP`  | reg. `dst` <- memory `RSP`, `DEC RSP`         |       |       |
|         |                                               |       |       |
|  `MOD`  | reg. `dst` <- reg. `dst` %    val./reg. `src` | R L   | R     |
|  `ADD`  | reg. `dst` <- reg. `dst` +    val./reg. `src` | R L   | R     |
|  `MUL`  | reg. `dst` <- reg. `dst` *    val./reg. `src` | R L   | R     |
|  `AND`  | reg. `dst` <- reg. `dst` AND  val./reg. `src` | R L   | R     |
|  `OR`   | reg. `dst` <- reg. `dst` OR   val./reg. `src` | R L   | R     |
|  `XOR`  | reg. `dst` <- reg. `dst` XOR  val./reg. `src` | R L   | R     |
|  `BSL`  | reg. `dst` <- reg. `dst` <<   val./reg. `src` | R L   | R     |
|  `INC`  | reg. `src` <- reg. `src` + 1                  | R     |       |
|         |                                               |       |       |
|  `NOT`  | reg. `src` <- Bitwise NOT reg. `src`          | R     |       |
|  `SUB`  | reg. `dst` <- reg. `dst` -    val./reg. `src` | R L   | R     |
|  `DIV`  | reg. `dst` <- reg. `dst` /    val./reg. `src` | R L   | R     |
| `NAND`  | reg. `dst` <- reg. `dst` NAND val./reg. `src` | R L   | R     |
|  `NOR`  | reg. `dst` <- reg. `dst` NOR  val./reg. `src` | R L   | R     |
| `XNOR`  | reg. `dst` <- reg. `dst` XNOR val./reg. `src` | R L   | R     |
|  `BSR`  | reg. `dst` <- reg. `dst` >> val./reg. `src`   | R L   | R     |
|  `DEC`  | reg. `src` <- reg. `src` - 1                  | R     |       |
