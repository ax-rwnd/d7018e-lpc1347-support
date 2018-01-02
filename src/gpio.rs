#![allow(dead_code)]
#![feature(proc_macro)]
#![no_std]

extern crate lpc1347;
use lpc1347::Peripherals;

use lpc1347::Interrupt::PIN_INT0;
use lpc1347::Interrupt::PIN_INT1;
use lpc1347::Interrupt::PIN_INT2;
use lpc1347::Interrupt::PIN_INT3;
use lpc1347::Interrupt::PIN_INT4;
use lpc1347::Interrupt::PIN_INT5;
use lpc1347::Interrupt::PIN_INT6;
use lpc1347::Interrupt::PIN_INT7;

/// Writes to a register using value and bitpos
macro_rules! write_reg {
    ($field:expr, $bitpos:expr, $value:expr) => {{
       unsafe {
           $field.write(|w| w.bits($value << $bitpos));
       }
    }};
}

/// Or into register
macro_rules! or_reg {
    ($field:expr, $value:expr) => {{
       unsafe {
           $field.modify(|r, w| w.bits(r.bits() | $value ));
       }
    }};

    ($field:expr, $bitpos:expr, $value:expr) => {{
       unsafe {
           $field.modify(|r, w| w.bits(r.bits() | ($value << $bitpos) ));
       }
    }};
}

/// And into register
macro_rules! neg_and_reg {
    ($field:expr, $value:expr) => {{
       unsafe {
           $field.modify(|r, w| w.bits(r.bits() & (! $value)  ));
       }
    }};

    ($field:expr, $bitpos:expr, $value:expr) => {{
       unsafe {
           $field.modify(|r, w| w.bits(r.bits() & (! ($value << $bitpos))  ));
       }
    }};
}

/// Check port 0/1
#[derive(Copy, Clone)]
pub enum Port {
    Port0,
    Port1
}

/// User edge or level detection
#[derive(Copy, Clone)]
pub enum Sense {
    Edge,
    Level
}

/// Falling/rising edge detected
#[derive(Copy, Clone)]
pub enum Event {
    Falling,
    Rising,
    High,
    Low
}

/// Initialize the GPIO ports
pub fn init(p: &lpc1347::Peripherals) {
    p.SYSCON.sysahbclkctrl.modify(|_, w| w.gpio().bit(true));
    p.SYSCON.sysahbclkctrl.modify(|_, w| w.pint().bit(true));
    p.SYSCON.sysahbclkctrl.modify(|_, w| w.group0int().bit(true));
    p.SYSCON.sysahbclkctrl.modify(|_, w| w.group1int().bit(true));
}

/// Set pin for an interrupt
pub fn set_pin_interrupt(p: &Peripherals, channel: u8, port: u8, bitpos: u32, sense: Sense, event: Event) {
    match channel {
        0 => {
            or_reg!(p.SYSCON.pintsel0, bitpos+24u32);
            p.NVIC.enable(PIN_INT0);
        }
        1 => {
            or_reg!(p.SYSCON.pintsel1, bitpos+24u32);
            p.NVIC.enable(PIN_INT1);
        }
        2 => {
            or_reg!(p.SYSCON.pintsel2, bitpos+24u32);
            p.NVIC.enable(PIN_INT2);
        }
        3 => {
            or_reg!(p.SYSCON.pintsel3, bitpos+24u32);
            p.NVIC.enable(PIN_INT3);
        }
        4 => {
            or_reg!(p.SYSCON.pintsel4, bitpos+24u32);
            p.NVIC.enable(PIN_INT4);
        }
        5 => {
            or_reg!(p.SYSCON.pintsel5, bitpos+24u32);
            p.NVIC.enable(PIN_INT5);
        }
        6 => {
            or_reg!(p.SYSCON.pintsel6, bitpos+24u32);
            p.NVIC.enable(PIN_INT6);
        }
        7 => {
            or_reg!(p.SYSCON.pintsel7, bitpos+24u32);
            p.NVIC.enable(PIN_INT7);
        }
        
        _ => {
            panic!("Invalid channel passed!");
        }
    }

    // Either use edge detection or level detection
    match sense {
        Sense::Edge => {
            neg_and_reg!(&p.GPIO_PIN_INT.isel, channel, 1);
            
            match event {
                Event::Falling => {
                    or_reg!(&p.GPIO_PIN_INT.ienf, channel, 1);
                }
                Event::Rising => {
                    or_reg!(&p.GPIO_PIN_INT.ienr, channel, 1);
                }

                _ => {
                    panic!("invalid combinaiton for sense and event, use Event::Falling or Event::Rising here")
                }
            }
        }

        Sense::Level => {
            or_reg!(&p.GPIO_PIN_INT.isel, channel, 1);
            or_reg!(&p.GPIO_PIN_INT.ienr, channel, 1);

            match event {
                Event::Low => {
                    neg_and_reg!(&p.GPIO_PIN_INT.ienf, channel, 1);
                }
                Event::High => {
                    or_reg!(&p.GPIO_PIN_INT.ienf, channel, 1);
                }
                _ => {
                    panic!("invalid combinaiton for sense and event, use Event::Low or Event::High here")
                }
            }
        }
    }
}

/// Determine if an interrupt is enabled or not
pub fn get_status(p: &Peripherals, channel: u8) -> bool {
    return (p.GPIO_PIN_INT.ist.read().bits() & (1<<channel)) == 1u32;
}

/// Clear the pin interrupt status
pub fn clear_status(p: &Peripherals, channel: u8) {
    if p.GPIO_PIN_INT.isel.read().bits() & (1<<channel) == 0 {
        write_reg!(&p.GPIO_PIN_INT.ist, channel, 1u32);
    }
}

pub fn set_grouped_interrupt() {
    panic!("not implemented");
}

/// Get current state of the pin
pub fn get_pin_value(p: &Peripherals, port: Port, bitpos: u32) -> u32 {
    match port {
        Port::Port0 => {
            return p.GPIO_PORT.set0.read().bits() & (1<<bitpos);
        }
        Port::Port1 => {
            return p.GPIO_PORT.set1.read().bits() & (1<<bitpos);
        }
    }
}

/// Set value for pin
pub fn set_pin_value(p: &Peripherals, port: Port, bitpos: u32, value: bool) {
    match port {
        Port::Port0 => {
            write_reg!(p.GPIO_PORT.set0, bitpos, if value {1} else {0});
        }
        Port::Port1 => {
            write_reg!(p.GPIO_PORT.set1, bitpos, if value {1} else {0});
        }
    }
}

/// Set pin direction
pub fn set_dir(p: &Peripherals, port: Port, bitpos: u32, output: bool) {
    match port {
        Port::Port0 => {
            if output {
                or_reg!(p.GPIO_PORT.dir0, bitpos, 1);
            } else {
                neg_and_reg!(p.GPIO_PORT.dir0, bitpos, 0);
            }
        }
        Port::Port1 => {
            if output {
                or_reg!(p.GPIO_PORT.dir1, bitpos, 1);
            } else {
                neg_and_reg!(p.GPIO_PORT.dir1, bitpos, 0);
            }
        }
    }
}
