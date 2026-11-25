# OmgOS
As RISK5 Unikernel for the learning and art of it.

OmgOS is toy Unikernel mostly ment for lear

## Setup developed environment.
#### Install QEMU 

I plan to use RISC V 64 bit. So need qemu-system-misc. I have Mint/Ubuntu linux.

`sudo apt install qemu-system-misc`

#### Test that QEMU works. (optional)

Install some firmware to test with.

`sudo apt install opensbi`

Run the command to test:
`qemu-system-riscv64 -machine virt -m 256M -nographic \
    -bios /usr/lib/riscv64-linux-gnu/opensbi/generic/fw_jump.bin`

To quit pres ctrl+a then x in rapid succession.
