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

|  Name   | Description                                   | Notes          |
| :-----: | :-------------------------------------------- | :------------- |
| `DEBUG` | Debug call `dst` `src`                        |                |
| `NOOP`  | Does nothing                                  |                |
| `HALT`  | Stops the program                             |                |
| `MOVE`  | reg. `dst` <- val./reg. `src`                 |                |
| `LMEM`  | reg. `dst` <- memory `src`                    |                |
| `SMEM`  | memory `dst` <- val./reg. `src`               |                |
| `PUSH`  | `INC RSP`, memory `RSP` <- reg. `dst`         |                |
| `PUSH`  | `INC RSP`, memory `RSP` <- val. `dst`         |                |
|  `POP`  | reg. `dst` <- memory `RSP`, `DEC RSP`         |                |
|         |                                               |                |
|  `MOD`  | reg. `dst` <- reg. `dst` %    val./reg. `src` |                |
|  `ADD`  | reg. `dst` <- reg. `dst` +    val./reg. `src` |                |
|  `MUL`  | reg. `dst` <- reg. `dst` *    val./reg. `src` |                |
|  `AND`  | reg. `dst` <- reg. `dst` AND  val./reg. `src` |                |
|  `OR`   | reg. `dst` <- reg. `dst` OR   val./reg. `src` |                |
|  `XOR`  | reg. `dst` <- reg. `dst` XOR  val./reg. `src` |                |
|  `BSL`  | reg. `dst` <- reg. `dst` <<   val./reg. `src` |                |
|  `INC`  | reg. `dst` <- reg. `dst` + 1                  |                |
|         |                                               |                |
|  `NOT`  | reg. `dst` <- Bitwise NOT reg. `dst`          |                |
|  `SUB`  | reg. `dst` <- reg. `dst` -    val./reg. `src` |                |
|  `DIV`  | reg. `dst` <- reg. `dst` /    val./reg. `src` | floor division |
| `NAND`  | reg. `dst` <- reg. `dst` NAND val./reg. `src` |                |
|  `NOR`  | reg. `dst` <- reg. `dst` NOR  val./reg. `src` |                |
| `XNOR`  | reg. `dst` <- reg. `dst` XNOR val./reg. `src` |                |
|  `BSR`  | reg. `dst` <- reg. `dst` >> val./reg. `src`   |                |
|  `DEC`  | reg. `dst` <- reg. `dst` - 1                  |                |
