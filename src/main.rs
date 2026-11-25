#![no_std]
#![no_main]

use core::arch::global_asm;

//Sets up the globals and stack pointer.
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
    // Hopeful the debugger will see full stack frame now.
    inner_looper()
}

fn inner_looper() -> ! {
    loop {
        // Will hit a breakpoint here
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
