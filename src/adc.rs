#![allow(dead_code)]

extern crate lpc1347;
use lpc1347::Peripherals;

// ADC clock frequency
const ADC_CLK: u32 = 40000000;

#[derive(Copy, Clone)]
pub enum PinPos {
    PIN0 = 11,
    PIN1 = 12,
    PIN2 = 13,
    PIN3 = 14,
    PIN4 = 15,
    PIN5 = 16,
    PIN6 = 22,
    PIN7 = 23
}

#[derive(Copy, Clone)]
pub enum Capture {
    Rising,
    Falling
}

/// Initialize the ADC
pub fn init(p: &lpc1347::Peripherals, pinnum: u8, edge: Capture, SystemCoreClock: u32) {
    // Power up ADC module
    p.SYSCON.pdruncfg.modify(|_, w| w.adc_pd().bit(false));
    p.SYSCON.sysahbclkctrl.modify(|_, w| w.adc().bit(true));

    // Select channels
    unsafe {
        p.ADC.cr.modify(|_, w| w.sel().bits(pinnum));
    }

    // Set ADC clock divider
    unsafe {
        p.ADC.cr.modify(|_, w| w.clkdiv().bits(((SystemCoreClock / ADC_CLK) - 1) as u8));
    }

    // Set software control
    p.ADC.cr.modify(|_, w| w.burst().bit(false));

    // Stop the ADC
    unsafe {
        p.ADC.cr.modify(|_, w| w.start().bits(0x0));
    }

    // Set rising/falling edge
    match edge {
        Capture::Rising => {
            p.ADC.cr.modify(|_, w| w.edge().bit(false));
        },
        Capture::Falling => {
            p.ADC.cr.modify(|_, w| w.edge().bit(true));
        }
    }
}

pub fn set_adc_pins(p: &lpc1347::Peripherals, pin: PinPos) {
}

/// Read from the ADC at some channel
pub fn read(p: &lpc1347::Peripherals, channel: u8) -> u32 {
    if channel > 7 {
        panic!("invalid channel selected")
    }

    unsafe {
        p.ADC.cr.modify(|_, w| w.start().bits(0x1));
        p.ADC.cr.modify(|_, w| w.sel().bits(1<<channel));
    }

    // Read data
    let mut register_value: u32 = 0;
    loop {
        match channel {
            0 => register_value = p.ADC.dr0.read().bits(),
            1 => register_value = p.ADC.dr1.read().bits(),
            2 => register_value = p.ADC.dr2.read().bits(),
            3 => register_value = p.ADC.dr3.read().bits(),
            4 => register_value = p.ADC.dr4.read().bits(),
            5 => register_value = p.ADC.dr5.read().bits(),
            6 => register_value = p.ADC.dr6.read().bits(),
            7 => register_value = p.ADC.dr7.read().bits(),
            _ => panic!("invalid channel selected!")
        }
        if register_value & 0x80000000 == 1 {
            break;
        }
    }

    // Stop conversion
    p.ADC.cr.modify(|_, w| w.start().bits(0x0));
    return register_value >> 4 & 0xFFF;
}
