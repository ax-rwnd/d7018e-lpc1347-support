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
use gpio::{Event, Sense, Port};

mod timers;
use timers::{Timer16, MatchReg};

app! {
    device: lpc1347,

    resources: {
        static DC: u16 = 50;
        static PERIOD: u16 = 60000;
    },

    tasks: {
        PIN_INT0: {
            path: pwm_up,
            resources: [CT16B0, GPIO_PIN_INT, PERIOD, DC],
        },
        PIN_INT1: {
            path: pwm_down,
            resources: [CT16B0, GPIO_PIN_INT, PERIOD, DC],
        }
    }
}

fn init(p: init::Peripherals, r: init::Resources) {
    {
        let mut stdout = hio::hstdout().unwrap();
        let _ = writeln!(stdout, "Infitializing...");
    }

    unsafe {
        p.SYSCON.pdawakecfg.modify(|_,w| w.sysosc_pd().bit(false));
        p.SYSCON.pdruncfg.modify(|_,w| w.sysosc_pd().bit(false));

        p.SYSCON.pdruncfg.modify(|_,w| w.syspll_pd().bit(true));
        p.SYSCON.syspllclksel.modify(|_, w| w.sel().bits(0x1));
        p.SYSCON.syspllctrl.modify(|_, w| w.msel().bits(0b11));
        p.SYSCON.syspllctrl.modify(|_, w| w.psel().bits(0b01));
        p.SYSCON.pdruncfg.modify(|_,w| w.syspll_pd().bit(false));
        while !p.SYSCON.syspllstat.read().lock().bit() {}
        p.SYSCON.mainclksel.modify(|_, w| w.sel().bits(0x3));
        p.SYSCON.clkoutsel.modify(|_, w| w.sel().bits(0x3));
    }

    // Configure PIO0_2 and PIO0_3 for buttons
    gpio::init(&p);
    gpio::set_dir(&p, Port::Port0, 2, false);
    gpio::set_dir(&p, Port::Port0, 3, false);
    unsafe {
        p.IOCON.pio0_3.modify(|_, w| w.func().bits(0x0));
    }
    gpio::set_pin_interrupt(&p, 0u8, Port::Port0, 2u32, Sense::Edge, Event::Rising);
    gpio::set_pin_interrupt(&p, 1u8, Port::Port0, 3u32, Sense::Edge, Event::Rising);

    // Set pio0_9 as PWM output
    p.SYSCON.sysahbclkctrl.modify(|_, w| w.iocon().bit(true));
    unsafe {
        p.IOCON.pio0_9.modify(|_, w| w.func().bits(0x2));
        p.IOCON.pio0_9.modify(|_, w| w.mode().bits(0x2));
    }
    

    // Set PWM match registers
    timers::set_pwm(&p, Timer16::Timer0, **r.PERIOD, **r.PERIOD - (**r.DC * (**r.PERIOD/100)), 1000, 1000);
    unsafe {
        p.CT16B0.pr.modify(|_, w| w.pcval().bits(9));
    }

    // Configure match properties
    // Here, mr0 determines DC and mr1 when the output goes high
    p.CT16B0.mcr.modify(|_, w| w.mr0r().bit(true));
    p.CT16B0.mcr.modify(|_, w| w.mr1r().bit(false));
    p.CT16B0.mcr.modify(|_, w| w.mr1s().bit(false));

    // Enable the PWM
    timers::set_enabled(&p, Timer16::Timer0, false);

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

/// Turn the duty cycle up
fn pwm_up (_t: &mut Threshold, r: PIN_INT0::Resources) {
    {
        let mut stdout = hio::hstdout().unwrap();
        let _ = writeln!(stdout, "Up ({})", **r.DC);
    }
    if **r.DC < 100 {
        **r.DC += 1;
    }

    unsafe {
        r.CT16B0.pr.modify(|_, w| w.pcval().bits(**r.PERIOD - (**r.DC * (**r.PERIOD/100))));
        r.GPIO_PIN_INT.ist.modify(|_, w| w.pstat0().bit(true));
    }

}

/// Turn the duty cycle down
fn pwm_down (_t: &mut Threshold, r: PIN_INT1::Resources) {
    {
        let mut stdout = hio::hstdout().unwrap();
        let _ = writeln!(stdout, "Down ({})", **r.DC);
    }
    if **r.DC > 0 {
        **r.DC -= 1;
    }

    unsafe {
        r.CT16B0.pr.modify(|_, w| w.pcval().bits(**r.PERIOD - (**r.DC * (**r.PERIOD/100))));
        r.GPIO_PIN_INT.ist.modify(|_, w| w.pstat1().bit(true));
    }
}
