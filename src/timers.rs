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

use lpc1347::Interrupt::CT16B0;
use lpc1347::Interrupt::CT16B1;


pub enum Timer16 {
    Timer0,
    Timer1
}

pub enum MatchReg {
    Reg0,
    Reg1,
    Reg2,
    Reg3
}

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

/// Enable or disable 16-bit timers
pub fn set_enabled(p: &lpc1347::Peripherals, timer: Timer16, enabled: bool) -> bool {
    match timer {
        Timer16::Timer0 => {
            p.CT16B0.tcr.modify(|_, w| w.cen().bit(enabled));
            p.CT16B0.mcr.modify(|_, w| w.mr0i().bit(enabled));
            p.NVIC.enable(CT16B0);
            return p.NVIC.is_enabled(CT16B0);
        }
        Timer16::Timer1 => {
            p.CT16B1.tcr.modify(|_, w| w.cen().bit(enabled));
            p.NVIC.enable(CT16B1);
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
