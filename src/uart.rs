use core::sync::atomic::{AtomicBool, Ordering};

// The first address for the UART registers.
const BASE: *mut u8 = 0x1000_0000 as *mut u8;

// Flags
const LSR_THR_EMPTY: u8 = 1 << 5; // the 5 bit

pub struct Uart {
    _private: (),
}

static CLAIMED: AtomicBool = AtomicBool::new(false);

impl Uart {
    // --High level api.--

    pub fn claim() -> Option<Uart> {
        let claimed = CLAIMED.swap(true, Ordering::SeqCst);
        if claimed != false {
            Some(Uart { _private: () })
        } else {
            None
        }
    }

    pub fn put(&mut self, val: u8) {
        while !self.thr_empty() {}
        self.write_thr(val);
    }

    // --Mid level flag access--

    /// Is the transmit buffer ready to accept a new byte?
    fn thr_empty(&mut self) -> bool {
        self.read_lsr() & LSR_THR_EMPTY != 0
    }

    // --Low level registry access.--

    /// Transmit Holding Register
    fn write_thr(&mut self, val: u8) {
        unsafe { BASE.write_volatile(val) } // offset 0x00
    }

    // Registers not used:
    // The read side of:
    // 0x00	THR / RBR	Transmit Holding Register (write) / Receive Buffer (read)
    //
    // 0x01 IER Interrupt Enable Register
    // 0x02	FCR	FIFO Control Register
    // 0x03	LCR	Line Control Register (data bits, stop bits, DLAB bit)
    // 0x04	MCR	Modem Control Register

    ///Line Status Register
    fn read_lsr(&mut self) -> u8 {
        unsafe { BASE.add(0x05).read_volatile() } // offset 0x05
    }

    // Registers not used:
    // 0x06	MSR	Modem Status Register
}
