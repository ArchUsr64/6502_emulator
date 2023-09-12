# 6502 Emulator
Emulates a MOS 6502 chip with DMA graphics and input capibalities
![image](https://github.com/ArchUsr64/6502_emulator/assets/83179501/92f5f35a-5e2c-459c-ac8c-bbe036dd326e)

## Memory Layout  
0xffff -> Top of memory  
0xfb00 - 0xffff => Memory mapped 32x32 screen  
0x100 - 0x1ff -> Stack ends  
0xff -> RNG Address  
0xfb -> Input Location  

## Build and Execution
1. Assemble one of the provided examples under `examples/`  
   `vasm6502_oldstyle -Fbin -dotdir examples/ferris.asm`
2. Run the emulator
   `cargo run`
