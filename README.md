# 6502 Emulator

Emulates a MOS 6502 chip with DMA graphics and input capibalities

https://github.com/ArchUsr64/6502_emulator/assets/83179501/17add7d3-14d8-47e4-bffd-77c96e9250e6  

Source Code for [snake](examples/snake.asm)

## Build and Execution
1. Clone the repository  
   `git clone https://github.com/ArchUsr64/6502_emulator`
2. Change to newly created directory  
   `cd 6502_emulator`
4. Assemble one of the provided examples under `examples/` using [vasm](http://www.compilers.de/vasm.html) or just use the provided `a.out`  
   `vasm6502_oldstyle -Fbin -dotdir examples/snake.asm`  
   This should build an `a.out` file which the emulator can understand.
5. Run the emulator  
   `cargo run -- a.out`
6. For an explaination of all possible arguments  
   `cargo run -- -help`

## Usage
### Debugging
Left click on the window to pause the execution at any time or start in paused state via the `-s` flag. Once paused, use the `Space` key to execute instructions step by step. Left click again to resume execution.  
Additionally the `-e` option can be used to specify the number of instructions to execute on the emulated CPU per frame.
### Logging
Use the `-v` flag to specify the level of verbosity for log output:
| `-v` | Log Level |
| -- | -- |
| 0 | Error |
| 1 | Info |
| 2 | Debug |
### Inputs
Currently only four inputs are supported, `LEFT`, `DOWN`, `UP` and `RIGHT`. Both the arrow keys and the WASD cluster can be used to activate their respective inputs.

## Memory Layout  
| Address | Description |
| -- | -- |
| `0xfb - 0xfe` | Keyboard Inputs stored here in: `left`, `down`, `up`, `right` order where 1 indicates `KeyDown` |
| `0xff` | Random Number Generator (Value is updated to a random byte on every instruction execution |
| `0x100 - 0x1ff` | Stack to store subroutine return addresses |
| `0xfb00 - 0xffff` | `0x400` (1024) byte space to store the RGB values for pixels on a 32x32 grid in standard raster scan order | 

### RGB color format:
Each color byte is divided into bit fields of size 3, 3 and 2. The bit field if size 2 is least significant and represents the blue color, with the most significant bit field representing red as shown below:
```
MSB      LSB
 ^        ^
 765 432 10
 |   |   |
 RRR GGG BB
```

## Screenshots
#### [examples/rgb.asm](examples/rgb.asm)  
![image](https://github.com/ArchUsr64/6502_emulator/assets/83179501/9a6a5d93-d806-431a-af00-5bded1c93793)  
<br>
#### [examples/ferris.asm](examples/ferris.asm)  
![image](https://github.com/ArchUsr64/6502_emulator/assets/83179501/8fcb2804-92d0-43a3-abd1-ef00b96d773d)

## TODO
- [X] Graphics Support
- [ ] Deploy with Wasm
- [X] Input Events
- [ ] Interrupt Support
- [ ] Integrated Debugger
