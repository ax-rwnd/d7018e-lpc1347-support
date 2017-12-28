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

    resources: {
        static ON: bool = false;
    },

    tasks: {
        SYS_TICK: {
            path: sys_tick,
            resources: [ON],
        },
        CT16B0: {
            path: clock0_tick,
            resources: [GPIO_PORT, ON],
        }
    }
}

fn init(p: init::Peripherals, r: init::Resources) {
    {
        let mut stdout = hio::hstdout().unwrap();
        let _ = writeln!(stdout, "Initializing...");
    }

    gpio::init(&p);
    gpio::set_dir(&p, gpio::Port::Port0, 8, true);

    // Clock 0 setup
    timers::reset(&p, Timer16::Timer0);
    timers::init(&p, Timer16::Timer0);
    timers::set_interrupt(&p, Timer16::Timer0, MatchReg::Reg0, true);
    timers::set_enabled(&p, Timer16::Timer0, true);
    unsafe { timers::set_match(&p, Timer16::Timer0, MatchReg::Reg0, 2u16); }

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

fn sys_tick(_t: &mut Threshold, r: SYS_TICK::Resources) {
}

fn clock0_tick(_t: &mut Threshold, r: CT16B0::Resources) {
    /*
    let mut stdout = hio::hstdout().unwrap();
    let _ = writeln!(stdout, "Clock 0!");
    */
    r.GPIO_PORT.not0.write(|w| w.notp8().bit(true));
}
