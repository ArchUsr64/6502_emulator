# 6502 Emulator

Emulates a MOS 6502 chip with an interactive debugger.  
**[LIVE DEMO](https://archusr64.github.io/6502_emulator/)**



https://github.com/ArchUsr64/6502_emulator/assets/83179501/75f19672-a8c2-446f-b395-1c2cca4db83d



Source Code for [snake](examples/snake.asm)

## Build and Execution
1. Clone the repository:  
   `git clone https://github.com/ArchUsr64/6502_emulator`
2. Change to newly created directory:  
   `cd 6502_emulator`
3. Run the emulator:  
   `cargo run --release`
   
## Usage

Assemble one of the provided examples under `examples/` using the provided python build script:  
`python build_asm.py examples/snake.asm`  
This should build an `a.out` binary which the emulator can understand along with `symbols.dbg` for debugging.

Run the emulator with the newly generated files:

`cargo run -- a.out -a examples/snake.asm -d symbols.dbg`

### Debugging
Click the 'Pause Execution' button in the Debug Controls window to pause the execution at any time or start in paused state via the `-s` flag.
Once paused, use the `Step` button to execute the next instruction. Add breakpoints from the 'Breakpoints' window and press the 'X' button to
remove previously added entires. Watchpoints can be used to observe and change memory addresses at runtime.

The symbols required for debugging are included in a newline delimited entries of line numbers and program counter addresses, with the addresses in hexadecimal.
For an example take a look at the provided [`symbols.dbg`](./symbols.dbg).

### CLI Arguments

| Option | Description | Default |
| :--: | -- | -- |
| -e | **Executable:** Path for the 6502 binary | `a.out` |
| -v | **Verbosity:** Verbosity level for console logs | `0` (Errors only) |
| -s | **Start debug:** Stard the emulator in debug mode | `false` |
| -i | **Instructions per frame:** The number of CPU instructions to execute per rendered frame | `100` |
| -d | **Debug symbols:** Path for the file containing debug symbols | [`symbols.dbg`](./symbols.dbg) |
| -a | **Assembly source:** Path for the assembly source file | [`examples/snake.asm`](./examples/snake.asm) |
  
### Inputs
Use WASD or the arrow keys to provide input events.

## Memory Layout  
| Address | Description |
| -- | -- |
| `0xfb - 0xfe` | Keyboard Inputs stored here in: `left`, `down`, `up`, `right` order where 1 indicates `KeyDown` |
| `0xff` | Random Number Generator (Value is updated to a random byte on every instruction execution |
| `0x100 - 0x1ff` | Stack to store subroutine return addresses |
| `0xfb00 - 0xffff` | `0x400` (1024) byte space to store the RGB values for pixels on a 32x32 grid in standard raster scan order | 

### RGB color format:
Each color byte is divided into bit fields of size 3, 3 and 2. The bit field if size 2 is least significant and represents the blue color, with the most significant bit field representing red as shown below:
```f#
MSB      LSB
 ^        ^
 765 432 10
 |   |   |
 RRR GGG BB
```

## Screenshots
### Debugger UI
![image](https://github.com/ArchUsr64/6502_emulator/assets/83179501/7c077fbf-0ba2-4534-93a7-aaef24da32c0)
<br>
### Console Logs
![image](https://github.com/ArchUsr64/6502_emulator/assets/83179501/66a6e6f8-802d-4ed5-931f-bc01a5cdf2f6)

