#![deny(overflowing_literals)]
#![feature(proc_macro)]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rtfm as rtfm;
extern crate cortex_m_semihosting;
extern crate lpc1347;

use rtfm::{app, Threshold, wfi};
use cortex_m_semihosting::hio;
use core::fmt::Write;

mod gpio;

mod timers;
use timers::Timer16;
use timers::MatchReg;

app! {
    device: lpc1347,
}

fn init(p: init::Peripherals) { //, r: init::Resources) {
    {
        let mut stdout = hio::hstdout().unwrap();
        let _ = writeln!(stdout, "Initializing...");
    }

    // Set pio0_9 as PWM output
    p.SYSCON.sysahbclkctrl.modify(|_, w| w.iocon().bit(true));
    unsafe {
        p.IOCON.pio0_9.modify(|_, w| w.func().bits(0x2));
    }

    // Set PWM match registers
    timers::set_pwm(&p, Timer16::Timer0, 4000, 3991, 1000, 1000);
    unsafe {
        p.CT16B0.pr.modify(|_, w| w.pcval().bits(30));
    }

    // Configure match properties
    // Here, mr0 determines DC and mr1 when the output goes high
    p.CT16B0.mcr.modify(|_, w| w.mr0r().bit(true));
    p.CT16B0.mcr.modify(|_, w| w.mr1r().bit(false));
    p.CT16B0.mcr.modify(|_, w| w.mr1s().bit(false));

    // Enable the PWM
    timers::set_enabled(&p, Timer16::Timer0, true);

    {
        let mut stdout = hio::hstdout().unwrap();
        let _ = writeln!(stdout, "Done");
    }
}

fn idle() -> ! {
    loop {
        wfi();
    }
}
