#![no_std]
#![no_main]

mod qemu;
mod uart;

use core::arch::global_asm;

use crate::uart::Uart;

// Sets up the globals and stack pointer.
// This is the minimum for rust core.
// Calls the start function.
global_asm!(
    r#"
    .section .text.entry
    .global _start
    _start:
        .option push
        .option norelax
        la gp, __global_pointer$
        .option pop

        .option push
        .option norelax
        la sp, _stack_top
        .option pop

        call start
    spin:
        j spin
    "#
);

#[unsafe(no_mangle)]
pub extern "C" fn start() -> ! {
    let mut uart = Uart::claim().unwrap();
    let message = "Hello, world!\r\n";
    for b in message.bytes() {
        uart.put(b);
    }

    qemu::exit_normally()
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    qemu::exit_failure()
}
