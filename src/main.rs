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
use lpc1347::Peripherals;

mod gpio;

mod timers;
use timers::Timer16;
use timers::MatchReg;

app! {
    device: lpc1347,
}

fn init_pwm(p: &Peripherals) {
    // Set pio0_9 as PWM output
    unsafe {
        p.IOCON.pio0_9.modify(|_, w| w.func().bits(0x2));
    }

    // Set PWM match registers
    const CYCLE: u16 = 12000;
    timers::set_pwm(&p, Timer16::Timer0, CYCLE, CYCLE-300, 1000, 1000);
    unsafe {
        p.CT16B0.pr.modify(|_, w| w.pcval().bits(9));
    }

    // Configure match properties
    // Here, mr0 determines DC and mr1 when the output goes high
    p.CT16B0.mcr.modify(|_, w| w.mr0r().bit(true));
    p.CT16B0.mcr.modify(|_, w| w.mr1r().bit(false));
    p.CT16B0.mcr.modify(|_, w| w.mr1s().bit(false));

    // Enable the PWM
    timers::set_enabled(&p, Timer16::Timer0, true);
}

fn init(p: init::Peripherals) { //, r: init::Resources) {
    {
        let mut stdout = hio::hstdout().unwrap();
        let _ = writeln!(stdout, "Initializing...");
    }

    // Configure IOCON clock
    p.SYSCON.sysahbclkctrl.modify(|_, w| w.iocon().bit(true));
    init_pwm(&p);

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
