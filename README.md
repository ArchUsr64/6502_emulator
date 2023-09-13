# 6502 Emulator
Emulates a MOS 6502 chip with DMA graphics and input capibalities

https://github.com/ArchUsr64/6502_emulator/assets/83179501/17add7d3-14d8-47e4-bffd-77c96e9250e6  

Source Code for [snake](examples/snake.asm)

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

## Build and Execution
1. Assemble one of the provided examples under `examples/` using [vasm](http://www.compilers.de/vasm.html)  
   `vasm6502_oldstyle -Fbin -dotdir examples/snake.asm`  
   This should build an `a.out` file which the emulator can understand.
2. Run the emulator  
   `cargo run -- a.out`
3. For an explaination of all the possible arguments  
   `cargo run -- -help`
## Screenshots
#### [examples/rgb.asm](examples/rgb.asm)  
![image](https://github.com/ArchUsr64/6502_emulator/assets/83179501/9a6a5d93-d806-431a-af00-5bded1c93793)  
<br>
#### [examples/ferris.asm](examples/ferris.asm)  
![image](https://github.com/ArchUsr64/6502_emulator/assets/83179501/8fcb2804-92d0-43a3-abd1-ef00b96d773d)
