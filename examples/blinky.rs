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
use gpio::Port;

app! {
    device: lpc1347,

    resources: {
        static ON: bool = false;
    },

    tasks: {
        CT16B0: {
            path: clock0_tick,
            resources: [GPIO_PORT, ON],
        },
        CT16B1: {
            path: clock1_tick,
            resources: [GPIO_PORT, ON],
        }
    }
}

fn init(p: init::Peripherals, r: init::Resources) {
    {
        let mut stdout = hio::hstdout().unwrap();
        let _ = writeln!(stdout, "Initializing...");
    }

    // Initialize GPIO and set pio0_7 to output
    gpio::init(&p);
    gpio::set_dir(&p, Port::Port0, 7, true);

    // Clock 0 setup
    timers::reset(&p, Timer16::Timer0);
    timers::init(&p, Timer16::Timer0);
    timers::set_interrupt(&p, Timer16::Timer0, MatchReg::Reg0, true);
    timers::set_enabled(&p, Timer16::Timer0, true);
    unsafe { timers::set_match(&p, Timer16::Timer0, MatchReg::Reg0, 2u16); }

    // Clock 1 setup
    timers::reset(&p, Timer16::Timer1);
    timers::init(&p, Timer16::Timer1);
    timers::set_interrupt(&p, Timer16::Timer1, MatchReg::Reg0, true);
    timers::set_enabled(&p, Timer16::Timer1, true);
    unsafe { timers::set_match(&p, Timer16::Timer1, MatchReg::Reg0, 2u16); }

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

fn clock0_tick(_t: &mut Threshold, r: CT16B0::Resources) {
    let mut stdout = hio::hstdout().unwrap();
    let _ = writeln!(stdout, "Clock 0!");
    r.GPIO_PORT.not0.write(|w| w.notp7().bit(true));
}

fn clock1_tick(_t: &mut Threshold, r: CT16B1::Resources) {
    let mut stdout = hio::hstdout().unwrap();
    let _ = writeln!(stdout, "Clock 1!");
}
