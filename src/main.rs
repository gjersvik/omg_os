#![no_std]
#![no_main]

mod qemu;

use core::{arch::global_asm, ptr};

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
    uart_puts("Hello, world!\r\n");
    qemu::exit_normally()
}

const UART_BASE: *mut u8 = 0x1000_0000 as *mut u8;

fn uart_putc(c: u8) {
    // Poll LSR bit 5 (THR empty) — not strictly needed in QEMU,
    // but correct practice and required on real hardware.
    while unsafe { ptr::read_volatile(UART_BASE.add(5)) & 0x20 } == 0 {}
    unsafe { ptr::write_volatile(UART_BASE, c) }
}

fn uart_puts(s: &str) {
    for b in s.bytes() {
        uart_putc(b);
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    qemu::exit_failure()
}
