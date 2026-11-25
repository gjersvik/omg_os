# OmgOS
As RISK5 Unikernel for the learning and art of it.

OmgOS is toy Unikernel mostly ment for lear

## Setup developed environment.
### Install QEMU 

I plan to use RISC V 64 bit. So need qemu-system-misc. I have Mint/Ubuntu linux.

1. Install Qemu `sudo apt install qemu-system-misc`
2. Install some test images: `sudo apt install opensbi`
3. Run test image: 
  `qemu-system-riscv64 -machine virt -m 256M -nographic \
    -bios /usr/lib/riscv64-linux-gnu/opensbi/generic/fw_jump.bin`
4. To quit pres ctrl+a then x in rapid succession.

### Setup debugging.
1. Install gdb-multiarch: `sudo apt install gdb-multiarch`
2. Run image with debugger paused: 
  `qemu-system-riscv64 -machine virt -s -S -m 256M -nographic  \
    -bios /usr/lib/riscv64-linux-gnu/opensbi/generic/fw_jump.bin`
3. Use the **QEMU RISC-V: Debug** config
4. You can terminate from debugger.

### Setup rust.
I assume you have rust installed and running. 
1. Add target: `rustup target add riscv64gc-unknown-none-elf`
2. Install cargo-binutil for objdump: `cargo install cargo-binutils`
3. Install the underlying llvm tools: `rustup component add llvm-tools`

## Run project.
1. Build using `cargo build`
2. Run using `qemu-system-riscv64 -machine virt -s -S -m 256M -nographic  \
    -bios none --kernel ./target/riscv64gc-unknown-none-elf/debug/omg_os`
3. Use the **QEMU RISC-V: Debug** config
4. You can terminate from debugger.
