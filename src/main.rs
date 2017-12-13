#![feature(proc_macro)]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rtfm as rtfm;
extern crate cortex_m_semihosting;
extern crate lpc1347;

use rtfm::{app, Threshold, wfi};

app! {
    device: lpc1347,

    resources: {
        static ON: bool = false;
    },

    tasks: {
        SYS_TICK: {
            path: sys_tick,
            resources: [GPIO_PORT, ON],
        },
    }
}

fn init(p: init::Peripherals, r: init::Resources) {
}

fn idle() -> ! {
    loop {
        wfi();
    }
}

fn sys_tick(_t: &mut Threshold, r: SYS_TICK::Resources) {
}
