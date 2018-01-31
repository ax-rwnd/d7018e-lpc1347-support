#![allow(dead_code)]

extern crate lpc1347;
use lpc1347::Peripherals;
use lpc1347::Interrupt::USART;

const BUFFER_SIZE: usize = 1024;
static uart_pcb: Pcb = Pcb{initialized: false,
                            baud_rate: 0,
                            status: 0,
                            tx_data: 0,
                            rxfifo: UartBuffer {
                                ep_dir: 0,
                                length: 0,
                                rptr: 0,
                                wptr: 0,
                                buffer: [0; BUFFER_SIZE]
                                }
                            };

struct UartBuffer {
    ep_dir: u8,
    length: u8,
    rptr:   u8,
    wptr:   u8,
    buffer: [u8; BUFFER_SIZE]
}

struct Pcb {
    initialized: bool,
    baud_rate: u32,
    status: u32,
    tx_data: u32,
    rxfifo: UartBuffer
}

/// Initialize the USART controller
pub fn init(p: &lpc1347::Peripherals, baudrate: u32, flow_control: bool) {
    p.NVIC.disable(USART);

    // TODO: rxbufferinit

    unsafe {
        p.IOCON.pio0_18.modify(|_, w| w.func().bits(0x1));
        p.IOCON.pio0_19.modify(|_, w| w.func().bits(0x1));
    }

    // Setup flowcontrol (RTS/CTS)
    if flow_control {
        unsafe {
            p.IOCON.pio0_7.modify(|_, w| w.func().bits(0x1));
            p.IOCON.pio0_17.modify(|_, w| w.func().bits(0x1));
        }
    }

    // Start USART clock
    p.SYSCON.sysahbclkctrl.modify(|_, w| w.usart().bit(true));
    unsafe {
        p.SYSCON.uartclkdiv.modify(|_, w| w.div().bits(0x1));
    }

    // Setup Line Control Register
    unsafe {
        p.USART.lcr.modify(|_, w| w.wls().bits(0x3));
        p.USART.lcr.modify(|_, w| w.ps().bits(0x0));
    }
    p.USART.lcr.modify(|_, w| w.sbs().bit(false));
    p.USART.lcr.modify(|_, w| w.pe().bit(false));
    p.USART.lcr.modify(|_, w| w.bc().bit(false));
    p.USART.lcr.modify(|_, w| w.dlab().bit(true));

    // Setup baud rate
    {
        let register_value: u32 = p.SYSCON.uartclkdiv.read().div().bits() as u32;
        let fdiv: u32 = (((12000u32 / register_value) / 16u32) / baudrate as u32);
        unsafe {
            p.USART.dlm.modify(|_, w| w.dlmsb().bits((fdiv / 256u32) as u8));
            // TODO: what? DLL seems to be broken ... somehow?
            //p.USART.dll.modify(|_, w| w.dllsb().bits(fdiv as u8 % 256u8));
        }
    }

    // Reset divisor latch access bit
    p.USART.lcr.modify(|_, w| w.dlab().bit(false));

    // Enable and clear FIFO
    // TODO: these are broken in the SVD
    /*
    p.USART.fcr.modify(|_, w| w.fifoen().bit(true));
    p.USART.fcr.modify(|_, w| w.rxfifores().bit(true));
    p.USART.fcr.modify(|_, w| w.txfifores().bit(true));
    */

    // Enable auto RTS/CTS
    if flow_control {
        p.USART.mcr.modify(|_, w| w.rtsen().bit(true));
        p.USART.mcr.modify(|_, w| w.ctsen().bit(true));
    }

    // Ensure clean start
    while !p.USART.lsr.read().temt().bit() && !p.USART.lsr.read().thre().bit() {}
    while p.USART.lsr.read().rdr().bit() {
        // Dump data
        let _register_value = p.USART.rbr.read().bits();
    };

    // Turn on USART once config is complete
    p.NVIC.enable(USART);
    // TODO: IER is also broken...
    // p.USART.ier.modify(|_, w| w.rbrinten().bit(true));
    // p.USART.ier.modify(|_, w| w.rlsinten().bit(true));
}   

/// Send an arbitrary region of data over USART
pub fn send(p: &lpc1347::Peripherals, buffer: *mut u8, length: isize) {
    if buffer.is_null() {
        panic!("USART buffer was null");
    }

    let mut pos: isize = 0;
    while  pos < length {
        while !p.USART.lsr.read().thre().bit() {}
        // TODO: yet again, thr busted
        // p.USART.thr.modify(|_, w| w.thr.bits(*buffer.offset(pos)));
        pos += 1;
    }
}

/// Send a single byte over USART
pub fn send_byte(p: &lpc1347::Peripherals, byte: u8) {
    while !p.USART.lsr.read().thre().bit() {}
    // TODO: yet again, thr busted
    // p.USART.thr.modify(|_, w| w.thr.bits(byte));
}
