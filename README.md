# OmgOS
As toy rust Unikernel for RISC-V 64.

Warning this project will have some heavy works on my computer vibe. I dont know what i am doing. Learning is the point. 

## Project goals
1. Have fun. 
2. Learn about the boot process. What happen between power on and main.
3. Unikernel that can run the tokio ecosystem.

## Setup developed environment.
### Setup rust
1. Have rust installed and working.
2. Add target: `rustup target add riscv64gc-unknown-none-elf`
3. (Optional) Install cargo-binutil for objdump: `cargo install cargo-binutils`
   * Llvm tools was already installed if not: `rustup component add llvm-tools`
4. `cargo build` should just work. `cargo run` require Qemu.

### Install QEMU 

I am running Mint Cinnamon linux.

1. Install Qemu `sudo apt install qemu-system-misc`
2. `cargo run` should work now. 
3. To quit pres ctrl+a then x in rapid succession.

### Setup debugging.
I am using the zed editor. See <.zed/debug.json> for details.

1. Install gdb-multiarch: `sudo apt install gdb-multiarch`
2. You need to start the OS. If you use `cargo run -- -S` it will pause at start. 
3. Use the **QEMU RISC-V: Debug** config
4. You can terminate from debugger.

## The Journey 

For now its only notes. There if I want to create a blog or talk in the future.

1. [Hello world is not that easy](./journey/hello_world.md)
2. [Console and the mother of all bugs](./journey/the_bug.md)
