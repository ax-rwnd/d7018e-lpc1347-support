#![allow(dead_code)]

extern crate lpc1347;

// ADC clock frequency
const ADC_CLK: u32 = 4000000;

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
pub fn init(syscon: &lpc1347::SYSCON, adc: &lpc1347::ADC, pinnum: u8, edge: Capture, system_core_clock: u32) {
    // Power up ADC module
    syscon.pdruncfg.modify(|_, w| w.adc_pd().bit(false));
    syscon.sysahbclkctrl.modify(|_, w| w.adc().bit(true));

    // Select channels
    unsafe {
        adc.cr.modify(|r, w| w.sel().bits(r.sel().bits() | 1 << pinnum));
    }

    // Set ADC clock divider
    unsafe {
        adc.cr.modify(|_, w| w.clkdiv().bits(((system_core_clock / ADC_CLK) - 1) as u8));
    }

    // Set software control
    adc.cr.modify(|_, w| w.burst().bit(false));

    // Stop the ADC
    unsafe {
        adc.cr.modify(|_, w| w.start().bits(0x0));
    }

    // Set rising/falling edge
    match edge {
        Capture::Rising => {
            adc.cr.modify(|_, w| w.edge().bit(false));
        },
        Capture::Falling => {
            adc.cr.modify(|_, w| w.edge().bit(true));
        }
    }
}

pub fn set_adc_pins(p: &lpc1347::Peripherals, pin: PinPos) {
}

/// Read from the ADC at some channel
pub fn read(adc: &lpc1347::ADC, channel: u8) -> u32 {
    // TODO: yet to implement 10-bit mode and low-power mode
    if channel > 7 {
        panic!("invalid channel selected")
    }

    unsafe {
        adc.cr.modify(|_, w| w.start().bits(0x1));
        adc.cr.modify(|_, w| w.sel().bits(1<<channel));
    }

    // Read data
    let mut register_value: u32 = 0;
    loop {
        match channel {
            0 => register_value = adc.dr0.read().bits(),
            1 => register_value = adc.dr1.read().bits(),
            2 => register_value = adc.dr2.read().bits(),
            3 => register_value = adc.dr3.read().bits(),
            4 => register_value = adc.dr4.read().bits(),
            5 => register_value = adc.dr5.read().bits(),
            6 => register_value = adc.dr6.read().bits(),
            7 => register_value = adc.dr7.read().bits(),
            _ => panic!("invalid channel selected!")
        }
        if register_value & (1<<31) != 0 {
            break;
        }
    }

    // Stop conversion
    adc.cr.modify(|_, w| w.start().bits(0x0));
    return (register_value >> 4) & 0xFFF;
}
