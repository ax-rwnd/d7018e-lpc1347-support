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
/*
    resources: {
        static ON: bool = false;
    },

    tasks: {
        CT16B0: {
            path: clock0_tick,
            resources: [CT16B0, GPIO_PORT, ON],
        },
        CT16B1: {
            path: clock1_tick,
            resources: [CT16B1, GPIO_PORT, ON],
        }
    }
    */
}

fn init(p: init::Peripherals) { //, r: init::Resources) {
    {
        let mut stdout = hio::hstdout().unwrap();
        let _ = writeln!(stdout, "Initializing...");
    }

    p.SYSCON.sysahbclkctrl.modify(|_, w| w.iocon().bit(true));
    unsafe {
        p.IOCON.pio0_9.modify(|_, w| w.func().bits(0x2));
    }

    gpio::init(&p);
    gpio::set_dir(&p, gpio::Port::Port0, 8, true);
    gpio::set_dir(&p, gpio::Port::Port0, 9, true);

    // Clock 0 setup
    /*timers::init(&p, Timer16::Timer0);
    timers::set_interrupt(&p, Timer16::Timer0, MatchReg::Reg0, true);
    timers::set_interrupt(&p, Timer16::Timer0, MatchReg::Reg1, true);
    timers::set_enabled(&p, Timer16::Timer0, true);*/
    timers::init(&p, Timer16::Timer0);
    timers::set_pwm(&p, Timer16::Timer0, 4000, 3991, 1000, 1000);
    p.CT16B0.mcr.modify(|_, w| w.mr0r().bit(true));
    p.CT16B0.mcr.modify(|_, w| w.mr1r().bit(false));
    p.CT16B0.mcr.modify(|_, w| w.mr1s().bit(false));
    timers::set_enabled(&p, Timer16::Timer0, true);
    p.CT16B0.tcr.modify(|_, w| w.cen().bit(true));
    unsafe {
        p.CT16B0.pr.modify(|_, w| w.pcval().bits(30));
    }

    // Clock 1 setup
   // timers::reset(&p, Timer16::Timer1);
   // timers::init(&p, Timer16::Timer1);
    //timers::set_interrupt(&p, Timer16::Timer1, MatchReg::Reg0, true);
   // timers::set_enabled(&p, Timer16::Timer1, true);

    /*unsafe {
        p.CT16B0.pc.write(|w| w.pcval().bits(2u16));
        //timers::set_match(&p, Timer16::Timer0, MatchReg::Reg0, 62000u16);
        timers::set_match(&p, Timer16::Timer0, MatchReg::Reg0, 65535u16);
        /*
        timers::set_match(&p, Timer16::Timer0, MatchReg::Reg2, 32768);
        timers::set_match(&p, Timer16::Timer0, MatchReg::Reg3, 32768);

        p.CT16B0.emr.write(|w| w.emc0().bits(0x01));
        p.CT16B0.emr.write(|w| w.emc1().bits(0x02));
        */
    }
    timers::reset(&p, Timer16::Timer0);
    //unsafe { timers::set_match(&p, Timer16::Timer0, MatchReg::Reg1, 16384u16); }
    //unsafe { timers::set_match(&p, Timer16::Timer1, MatchReg::Reg1, 32768u16); }
    */

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

/*fn clock0_tick(_t: &mut Threshold, r: CT16B0::Resources) {
    let mut stdout = hio::hstdout().unwrap();
    let _ = writeln!(stdout, "Done {:?}", r.CT16B0.pc.read().bits());
    r.GPIO_PORT.not0.write(|w| w.notp8().bit(true));
}

fn clock1_tick(_t: &mut Threshold, r: CT16B1::Resources) {
    r.GPIO_PORT.not0.write(|w| w.notp9().bit(true));
}*/
