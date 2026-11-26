//! This module contains QEMU specific hardware and capabilities.
//! That dont really make sense in real hardware

/// The memory address of the SiFive test hardware.
const SIFIVE_TEST: *mut u32 = 0x100000 as *mut u32;

const SUCCESS: u32 = 0x5555;
const FAILURE: u32 = 0x3333;

pub fn exit_normally() -> ! {
    exit(SUCCESS)
}

pub fn exit_failure() -> ! {
    exit(FAILURE)
}

fn exit(code: u32) -> ! {
    unsafe {
        SIFIVE_TEST.write_volatile(code);
    }
    // This code should be unreachable
    // But it's hold the app so you can attach a debugger.
    // If the impossible happens.
    loop {}
}
