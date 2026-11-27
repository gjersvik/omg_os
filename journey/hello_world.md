# Hello world is not that easy

* I knew I wanted to create a virtual only OS.
* So the first big decision was what architecture.
  * X64 have 40 years of legacy you have too boot trough.
  * RISC-V have almost one. Chose it.
* Needed to install Qemu. 
* Got some test images from opensbi.
  * Running the elf file did not work. There was memory region overlap that qemu did not like. The bin worked fine. 
  * Can still use the elf for debugging target.
* Debugger was a bit tricky. Zed editors documentation was basic. And my use case was not. 
  * Was not able to get `"tcp_connection": { "port": 1234 }` to work. 
  * But `"-ex", "target remote :1234"` seems to work fine. 
* Compiling rust was very easy just worked. 
  * Debugging the resulting binary was tricky.
  * The breakpoints did not fire.
  * That was before created the linker file.
* I asked an AI to both show me a linker script. And ask it how to read it. 
  * The linker script worked. And debugger started to work. 
* The next was stack pointer. I needed to use inline assembly.
  * Having the AI explain it was less useful.
* Printing to UART was surprisingly easy. No Assembly required. Just poke some addresses.
* Hello world was working. :D
