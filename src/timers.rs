#![feature(proc_macro)]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rtfm as rtfm;
extern crate cortex_m_semihosting;
extern crate lpc1347;

static mut T16_0_COUNTER: [u8; 4] = [0u8,0u8,0u8,0u8];
static mut T16_0_CAPTURE: [u8; 4] = [0u8,0u8,0u8,0u8];
static mut T16_1_COUNTER: [u8; 4] = [0u8,0u8,0u8,0u8];
static mut T16_1_CAPTURE: [u8; 4] = [0u8,0u8,0u8,0u8];

use lpc1347::Peripherals;
use lpc1347::Interrupt::CT16B0;
use lpc1347::Interrupt::CT16B1;


#[derive(Copy, Clone)]
pub enum Timer16 {
    Timer0,
    Timer1
}

#[derive(Copy, Clone)]
pub enum MatchReg {
    Reg0,
    Reg1,
    Reg2,
    Reg3
}

/*  Example code for setting up clock16/0

    timers::reset(&p, Timer16::Timer0);
    timers::init(&p, Timer16::Timer0);
    timers::set_interrupt(&p, Timer16::Timer0, MatchReg::Reg0, true);
    timers::set_enabled(&p, Timer16::Timer0, true);
    unsafe { timers::set_match(&p, Timer16::Timer0, MatchReg::Reg0, 2u16); }
*/

/// Initialize 16-bit timers
pub fn init(p: &lpc1347::Peripherals, timer: Timer16) {
    match timer {
        Timer16::Timer0 => {
            p.SYSCON.sysahbclkctrl.modify(|_, w| w.ct16b0().bit(true));

            unsafe {
                T16_0_COUNTER = [0u8,0u8,0u8,0u8];
                T16_0_CAPTURE = [0u8,0u8,0u8,0u8];
            }
        }
        Timer16::Timer1 => {
            p.SYSCON.sysahbclkctrl.modify(|_, w| w.ct16b1().bit(true));

            unsafe {
                T16_1_COUNTER = [0u8,0u8,0u8,0u8];
                T16_1_CAPTURE = [0u8,0u8,0u8,0u8];
            }
        }
    }
}

/// Enable or disable interrupts for a timer
pub fn set_interrupt(p: &Peripherals, timer: Timer16, mr: MatchReg, enabled: bool) {
    match timer {
        Timer16::Timer0 => {
            match mr {
                MatchReg::Reg0 => {
                    p.CT16B0.mcr.modify(|_, w| w.mr0i().bit(enabled));
                }
                MatchReg::Reg1 => {
                    p.CT16B0.mcr.modify(|_, w| w.mr1i().bit(enabled));
                }
                MatchReg::Reg2 => {
                    p.CT16B0.mcr.modify(|_, w| w.mr2i().bit(enabled));
                }
                MatchReg::Reg3 => {
                    p.CT16B0.mcr.modify(|_, w| w.mr3i().bit(enabled));
                }
            }
        }

        Timer16::Timer1 => {
            match mr {
                MatchReg::Reg0 => {
                    p.CT16B1.mcr.modify(|_, w| w.mr0i().bit(enabled));
                }
                MatchReg::Reg1 => {
                    p.CT16B1.mcr.modify(|_, w| w.mr1i().bit(enabled));
                }
                MatchReg::Reg2 => {
                    p.CT16B1.mcr.modify(|_, w| w.mr2i().bit(enabled));
                }
                MatchReg::Reg3 => {
                    p.CT16B1.mcr.modify(|_, w| w.mr3i().bit(enabled));
                }
            }
        }
    }
}

/// Enable or disable 16-bit timers
pub fn set_enabled(p: &lpc1347::Peripherals, timer: Timer16, enabled: bool) -> bool {
    match timer {
        Timer16::Timer0 => {
            p.CT16B0.tcr.modify(|_, w| w.cen().bit(enabled));

            if enabled {
                p.NVIC.enable(CT16B0);
            } else {
                p.NVIC.disable(CT16B0);
            }
            return p.NVIC.is_enabled(CT16B0);
        }
        Timer16::Timer1 => {
            p.CT16B1.tcr.modify(|_, w| w.cen().bit(enabled));

            if enabled {
                p.NVIC.enable(CT16B1);
            } else {
                p.NVIC.disable(CT16B1);
            }
            return p.NVIC.is_enabled(CT16B1);
        }
    }
}

/// Reset a 16-bit timer
pub fn reset(p: &lpc1347::Peripherals, timer: Timer16) {
    match timer {
        Timer16::Timer0 => {
            p.CT16B0.tcr.modify(|_, w| w.crst().bit(true));
        }
        Timer16::Timer1 => {
            p.CT16B1.tcr.modify(|_, w| w.crst().bit(true));
        }
    }
}

/// Set the match register
pub unsafe fn set_match(p: &lpc1347::Peripherals, timer: Timer16, reg: MatchReg, value: u16) {
    match timer {
        Timer16::Timer0 => {
            match reg {
                MatchReg::Reg0 => {
                    p.CT16B0.mr0.write(|w| w.match_reg().bits(value));
                }
                MatchReg::Reg1 => {
                    p.CT16B0.mr1.write(|w| w.match_reg().bits(value));
                }
                MatchReg::Reg2 => {
                    p.CT16B0.mr2.write(|w| w.match_reg().bits(value));
                }
                MatchReg::Reg3 => {
                    p.CT16B0.mr3.write(|w| w.match_reg().bits(value));
                }
            }
        }

        Timer16::Timer1 => {
            match reg {
                MatchReg::Reg0 => {
                    p.CT16B1.mr0.write(|w| w.match_reg().bits(value));
                }
                MatchReg::Reg1 => {
                    p.CT16B1.mr1.write(|w| w.match_reg().bits(value));
                }
                MatchReg::Reg2 => {
                    p.CT16B1.mr2.write(|w| w.match_reg().bits(value));
                }
                MatchReg::Reg3 => {
                    p.CT16B1.mr3.write(|w| w.match_reg().bits(value));
                }
            }
        }
    }
}

/// Setup a clock to be used for PWM
pub fn set_pwm(p: &Peripherals, timer: Timer16) {
    // TODO: untested
    set_enabled(&p, timer, false);

    match timer {
        Timer16::Timer0 => {
            p.CT16B0.tcr.modify(|_, w| w.cen().bit(true));

            p.CT16B0.emr.modify(|_, w| w.emc3().bits(0b01));
            p.CT16B0.emr.modify(|_, w| w.emc2().bits(0b01));
            p.CT16B0.emr.modify(|_, w| w.emc1().bits(0b01));
            p.CT16B0.emr.modify(|_, w| w.emc0().bits(0b01));

            p.CT16B0.pwmc.modify(|_, w| w.pwmen0().bit(true));

            reset(&p, timer);
        }

        Timer16::Timer1 => {
            p.CT16B1.tcr.modify(|_, w| w.cen().bit(true));

            p.CT16B1.emr.modify(|_, w| w.emc3().bits(0b01));
            p.CT16B1.emr.modify(|_, w| w.emc2().bits(0b01));
            p.CT16B1.emr.modify(|_, w| w.emc1().bits(0b01));
            p.CT16B1.emr.modify(|_, w| w.emc0().bits(0b01));

            p.CT16B1.pwmc.modify(|_, w| w.pwmen0().bit(true));

            reset(&p, timer.clone());
        }
    }
}
