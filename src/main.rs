#![no_std]
#![no_main]

mod console;
mod qemu;
mod uart;

use core::arch::global_asm;

use crate::{console::Console, uart::Uart};

// Sets up the globals and stack pointer.
// This is the minimum for rust core.
// Calls the start function.
global_asm!(
    r#"
    .section .text.entry
    .global _start
    _start:
        # create a norelax block
        # this stops some optimizations
        .option push
        .option norelax

        # Sets global pointer for faster global access.
        la gp, __global_pointer$

        # Give stack pointer an address so stack work.
        la sp, _stack_top

        # End the norelax block
        .option pop

        # Call the rust function below.
        call start

    # Defensive start will never return. But if do this loop catches it.
    spin:
        j spin
    "#
);

#[unsafe(no_mangle)]
pub extern "C" fn start() -> ! {
    let uart = Uart::claim().unwrap();
    let mut console = Console::new(uart);
    console.println("Hello, world!");

    qemu::exit_normally()
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    qemu::exit_failure()
}
