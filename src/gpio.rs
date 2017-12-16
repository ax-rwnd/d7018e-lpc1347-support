#![feature(proc_macro)]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rtfm as rtfm;
extern crate cortex_m_semihosting;
extern crate lpc1347;

/// Initialize the GPIO ports
pub fn init(p: &lpc1347::Peripherals) {
    p.SYSCON.sysahbclkctrl.modify(|_, w| w.gpio().bit(true));
    p.GPIO_PORT.dir0.modify(|_, w| w.dirp7().bit(true));
}
