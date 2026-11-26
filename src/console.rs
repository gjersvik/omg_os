use core::fmt::Write;

use crate::uart::Uart;

pub struct Console {
    uart: Uart,
}

impl Console {
    pub fn new(uart: Uart) -> Console {
        Console { uart }
    }

    pub fn print(&mut self, text: &str) {
        for b in text.bytes() {
            self.uart.put(b);
        }
    }

    pub fn println(&mut self, text: &str) {
        self.print(text);
        self.print("\r\n");
    }
}

impl Write for Console {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.print(s);
        Ok(())
    }
}
