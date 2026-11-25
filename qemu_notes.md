# QEMU Terminal guide. (From AI)
Every `qemu-system-riscv64` invocation follows the same skeleton:

```
qemu-system-riscv64 [machine] [cpu] [memory] [firmware/kernel] [devices] [I/O] [display]
```

Options are parsed left-to-right with **no mandatory ordering**. Each option either takes a bare value (`-m 512`) or a comma-separated `key=value` list (`-drive file=disk.qcow2,format=qcow2`).

## The mental model

Think of QEMU as building a virtual motherboard. Each option category wires up one piece of that board:

| Category | Flag(s) | What it configures |
|---|---|---|
| **Machine** | `-machine virt` | The virtual board (chipset, memory map, default devices). For RISC-V, `virt` is the standard paravirtualized board with VirtIO, CLINT, PLIC. |
| **CPU** | `-cpu rv64` | Which RISC-V core to emulate. `-cpu help` lists all. You can add extensions: `-cpu rv64,v=true` |
| **Accelerator** | `-accel tcg` | How the CPU is executed. Default is **TCG** (software emulation, slow but universal). KVM needs a host with the H extension. |
| **Memory** | `-m 512M` | Guest RAM size. |
| **SMP** | `-smp 4` | Number of harts (cores). |
| **Firmware / Kernel** | `-bios fw_jump.bin` / `-kernel my.elf` | What gets loaded into memory and executed. For **bare-metal**, you typically use `-bios none` and `-kernel my.elf` so QEMU loads your ELF directly at the reset vector (0x1000 on `virt`). |
| **Boot args** | `-append "..."` | Kernel command-line string (only meaningful for Linux; irrelevant for bare-metal). |
| **Devices** | `-device virtio-blk-device,drive=hd` | Extra hardware to attach (disk, NIC, RNG, …). |
| **Backends** | `-drive file=disk.img,if=none,id=hd` | The host-side resource (file, socket) that a device reads/writes. |
| **Display / I/O** | `-nographic` | Send serial output to the terminal, no GUI window. **This is what you want for bare-metal.** |
| **Monitor** | `-monitor stdio` / `-serial mon:stdio` | How you talk to the VM at runtime. |

## A minimal bare-metal RISC-V command

```bash
qemu-system-riscv64 \
    -machine virt \
    -cpu rv64 \
    -m 128M \
    -bios none \
    -kernel hello.elf \
    -nographic \
    -serial mon:stdio
```

Why each flag is there:

- **`-machine virt`** — gives you the standard RISC-V virtual platform. Your code will see UART at `0x10000000`, CLINT at `0x02000000`, PLIC at `0x0c000000`, etc.
- **`-cpu rv64`** — base RV64IMAC. Add extensions as you need them (e.g. `-cpu rv64,v=true` for vector).
- **`-m 128M`** — 128 MiB of RAM starting at `0x80000000`.
- **`-bios none`** — skip OpenSBI. Your ELF's entry point becomes the reset vector. Without this, QEMU loads OpenSBI first, which would jump to its own code instead of yours.
- **`-kernel hello.elf`** — loads your ELF (or raw binary) into guest memory and sets PC to its entry point.
- **`-nographic`** + **`-serial mon:stdio`** — no GUI; serial I/O goes straight to your terminal. Exit with **Ctrl-A, X**.

## How to stop guessing

1. **`-machine help`** — lists all RISC-V boards (`virt`, `sifive_u`, `spike`, …).
2. **`-cpu help`** — lists CPU models and extensions.
3. **`-device help`** — lists every emulated device you can attach.
4. **`-device virtio-blk-device,help`** — shows the specific properties one device accepts.
5. **`-M virt,help`** — shows machine-specific properties (e.g. `aia-guests`, `aia-guests-with-msi`).

The key insight: **the machine type defines the memory map and default devices; `-device` adds extras; `-drive`/`-netdev`/`-chardev` are the host-side backends that devices connect to.** Once you can read a command in terms of "which board, which CPU, how much RAM, what's in memory at reset, what devices are on the bus, where does serial go," you're no longer copy-pasting.

For a toy OS you'll mostly care about the UART (for `print`) and CLINT/PLIC (for timers and interrupts), both of which come for free with `-machine virt`.
