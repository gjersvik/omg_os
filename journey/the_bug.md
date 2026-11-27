# Console and the mother of all bugs

* Created a simple QEMU module to trigger an exist.
* The UART module was also mostly strait forward.
  * Did want to use computed constants for UART registers. But had problem with providence.
  * Function based solution worked.
* I did want there to be only one UART. So created a static AtomicBool to track it. 
  * I had some trouble with it it seems to not work all the time. I thought i had fixed it. (Foreshadowing. )
* Created a Console module to write strings to the terminal. 
  * For some reason it enter an endless lop it should not have.
  * The strange ting it stops in the middle of the text. Not beginning.
  * And while messing with the there can only be one code also stopped working.
* What the hell is going on?
  * Did atomic to work? Nope yes they do. 
  * Was my code wrong? Yes. I had wrong if statement.
  * Did that fix it? No. Random results flipped was still random.
* Debugger to the rescue.
  * After learning some basic GDB-foo. I found ut that my Atomic bool had the value of 0x18 right after boot.
  * Bools can only have the value 1 or 0. There is no code I could write that puts hex 18 into a bool value. 
  * And this was the first line of rust code. 
* I remembered that the linker script i used felt a bit off on the stack.
  * RISC-V stack grow down. Towards the zero address.
  * My linker script created a _stack_top right after globals. And then moved the pointer up from that by 4KB. 
  * Wait up? What is down from _stack_top. Global variables and code. 
  * Well that explains it.
* I fixed the stack.
  * Moved the _stack_top 32KB after globals. 
  * There is still a risk there is no traps. A stack overflow will still overwrite the code. 
  * But i hope it will give me a very big buffer before i have some proper memory management.
