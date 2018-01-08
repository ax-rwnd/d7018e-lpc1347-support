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
use gpio::{Port, Event, Sense};

app! {
    device: lpc1347,

/*
    resources: {
        static IDLE_TIME: u16 = 0;
    },

    tasks: {
        CT16B1: {
            path: clock1_tick,
            resources: [GPIO_PORT, SYSCON, PMU, NVIC, SCB, CT16B1, IDLE_TIME],
        }
    }*/
}

/// Configure the idle timer to provide deep-sleep
fn init_timer1(p: &init::Peripherals) {
    timers::reset(&p, Timer16::Timer1);
    timers::init(&p, Timer16::Timer1);
    timers::set_interrupt(&p, Timer16::Timer1, MatchReg::Reg0, true);
    timers::set_enabled(&p, Timer16::Timer1, true);
    unsafe {
        timers::set_match(&p, Timer16::Timer1, MatchReg::Reg0, 65534u16/4); //120s/4
        p.CT16B1.pr.modify(|_, w| w.pcval().bits(24000u16));
    }
    p.CT16B1.mcr.modify(|_, w| w.mr0s().bit(true));
}

/// Init GPIO interrupt on PIO_3
fn init_pin3(p: &init::Peripherals) {
    gpio::set_dir(&p, Port::Port0, 3, false);
    gpio::set_pin_interrupt(&p, 0u8, Port::Port0, 3u32, Sense::Edge, Event::Falling);
}

/// Configure the PWM
fn init_pwm(p: &init::Peripherals) {

         timers::reset(&p, Timer16::Timer0);
    timers::init(&p, Timer16::Timer0);

    // Set PWM match registers
    timers::set_pwm(&p, Timer16::Timer0, 4000, 3991, 1000, 1000);

    // Configure match properties
    // Here, mr0 determines DC and mr1 when the output goes high
    p.CT16B0.mcr.modify(|_, w| w.mr0r().bit(true));
    p.CT16B0.mcr.modify(|_, w| w.mr1r().bit(false));
    p.CT16B0.mcr.modify(|_, w| w.mr1s().bit(false));

    timers::set_enabled(&p, Timer16::Timer0, true);
    p.CT16B0.tcr.modify(|_, w| w.cen().bit(true));
    unsafe {
        p.CT16B0.pr.modify(|_, w| w.pcval().bits(30));
    }
}

fn init(p: init::Peripherals/*, _r: init::Resources*/) {
    {
        let mut stdout = hio::hstdout().unwrap();
        let _ = writeln!(stdout, "Initializing...");
    }

    // Set pio0_9 as PWM output
    p.SYSCON.sysahbclkctrl.modify(|_, w| w.iocon().bit(true));
    unsafe {
        p.IOCON.pio0_9.modify(|_, w| w.func().bits(0x2));
    }


    // Initialize GPIO and set pio0_7 to output
    gpio::init(&p);
    //gpio::set_dir(&p, Port::Port0, 7, true);
    gpio::set_dir(&p, Port::Port0, 8, true);
    gpio::set_dir(&p, Port::Port0, 9, true);

    // Kill LED during initialization
    //p.GPIO_PORT.not0.write(|w| w.notp7().bit(true));

    init_pwm(&p);
    //init_timer1(&p);
    //init_pin3(&p);

    {
        let mut stdout = hio::hstdout().unwrap();
        let _ = writeln!(stdout, "Done");
    }
    // Put LED back up when up and running
    //p.GPIO_PORT.not0.write(|w| w.notp7().bit(true));
}

fn idle() -> ! {
    loop {
        wfi();
    }
}

/*
fn clock1_tick(_t: &mut Threshold, r: CT16B1::Resources) {
    // Only match register 0
    if r.CT16B1.ir.read().mr0int().bit() {

        let mut stdout = hio::hstdout().unwrap();
        let _ = writeln!(stdout, "Entering power-down mode.");
        unsafe {
            r.PMU.pcon.write(|w| w.pm().bits(0x2));
        }

        // TODO: we need to write 1 to SCR for power-down

        // Reset clock and interrupt
        r.CT16B1.tcr.modify(|_, w| w.crst().bit(true));
        r.CT16B1.ir.write(|w| w.mr0int().bit(true));
    }

}*/
