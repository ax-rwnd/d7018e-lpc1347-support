#![allow(dead_code)]

extern crate lpc1347;
use lpc1347::Interrupt::CT32B0;
use lpc1347::Interrupt::CT32B1;

#[derive(Copy, Clone)]
pub enum Timer {
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

#[derive(Copy, Clone)]
pub enum Control {
    Nothing = 0x0,
    Clear = 0x1,
    Set = 0x2,
    Toggle = 0x3
}

pub fn init(syscon: &lpc1347::SYSCON, nvic: &lpc1347::NVIC, timer: Timer) {
    match timer {
        Timer::Timer0 => {
            syscon.sysahbclkctrl.modify(|_, w| w.ct32b0().bit(true));
            nvic.enable(CT32B0);
        },
        Timer::Timer1 => {
            syscon.sysahbclkctrl.modify(|_, w| w.ct32b1().bit(true));
            nvic.enable(CT32B1);
        }
    }
}

pub fn set_match_t0 (ct32b0: &lpc1347::CT32B0, mr: MatchReg, value: u32) {
    unsafe {
        match mr {
            MatchReg::Reg0 => {
                ct32b0.mr0.modify(|_, w| w.match_reg().bits(value));
            },
            MatchReg::Reg1 => {
                ct32b0.mr1.modify(|_, w| w.match_reg().bits(value));
            },
            MatchReg::Reg2 => {
                ct32b0.mr2.modify(|_, w| w.match_reg().bits(value));
            },
            MatchReg::Reg3 => {
                ct32b0.mr3.modify(|_, w| w.match_reg().bits(value));
            }
        }
    }
}

pub fn set_interrupt_t0 (ct32b0: &lpc1347::CT32B0, mr: MatchReg, interrupt: bool, reset: bool, stop: bool) {
    match mr {
        MatchReg::Reg0 => {
            ct32b0.mcr.modify(|_, w| w.mr0i().bit(interrupt));
            ct32b0.mcr.modify(|_, w| w.mr0r().bit(reset));
            ct32b0.mcr.modify(|_, w| w.mr0s().bit(stop));
        },
        MatchReg::Reg1 => {
            ct32b0.mcr.modify(|_, w| w.mr1i().bit(interrupt));
            ct32b0.mcr.modify(|_, w| w.mr1r().bit(reset));
            ct32b0.mcr.modify(|_, w| w.mr1s().bit(stop));
        },
        MatchReg::Reg2 => {
            ct32b0.mcr.modify(|_, w| w.mr2i().bit(interrupt));
            ct32b0.mcr.modify(|_, w| w.mr2r().bit(reset));
            ct32b0.mcr.modify(|_, w| w.mr2s().bit(stop));
        },
        MatchReg::Reg3 => {
            ct32b0.mcr.modify(|_, w| w.mr3i().bit(interrupt));
            ct32b0.mcr.modify(|_, w| w.mr3r().bit(reset));
            ct32b0.mcr.modify(|_, w| w.mr3s().bit(stop));
        }
    }
}

pub fn set_match_t1 (ct32b1: &lpc1347::CT32B1, mr: MatchReg, value: u32) {
    unsafe {
        match mr {
            MatchReg::Reg0 => {
                ct32b1.mr0.modify(|_, w| w.match_reg().bits(value));
            },
            MatchReg::Reg1 => {
                ct32b1.mr1.modify(|_, w| w.match_reg().bits(value));
            },
            MatchReg::Reg2 => {
                ct32b1.mr2.modify(|_, w| w.match_reg().bits(value));
            },
            MatchReg::Reg3 => {
                ct32b1.mr3.modify(|_, w| w.match_reg().bits(value));
            }
        }
    }
}

pub fn set_interrupt_t1 (ct32b1: &lpc1347::CT32B1, mr: MatchReg, interrupt: bool, reset: bool, stop: bool) {
    match mr {
        MatchReg::Reg0 => {
            ct32b1.mcr.modify(|_, w| w.mr0i().bit(interrupt));
            ct32b1.mcr.modify(|_, w| w.mr0r().bit(reset));
            ct32b1.mcr.modify(|_, w| w.mr0s().bit(stop));
        },
        MatchReg::Reg1 => {
            ct32b1.mcr.modify(|_, w| w.mr1i().bit(interrupt));
            ct32b1.mcr.modify(|_, w| w.mr1r().bit(reset));
            ct32b1.mcr.modify(|_, w| w.mr1s().bit(stop));
        },
        MatchReg::Reg2 => {
            ct32b1.mcr.modify(|_, w| w.mr2i().bit(interrupt));
            ct32b1.mcr.modify(|_, w| w.mr2r().bit(reset));
            ct32b1.mcr.modify(|_, w| w.mr2s().bit(stop));
        },
        MatchReg::Reg3 => {
            ct32b1.mcr.modify(|_, w| w.mr3i().bit(interrupt));
            ct32b1.mcr.modify(|_, w| w.mr3r().bit(reset));
            ct32b1.mcr.modify(|_, w| w.mr3s().bit(stop));
        }
    }
}

pub fn set_enabled_t0(ct32b0: &lpc1347::CT32B0, enabled: bool) {
    ct32b0.tcr.modify(|_, w| w.cen().bit(enabled));
}

pub fn set_enabled_t1(ct32b1: &lpc1347::CT32B1, enabled: bool) {
    ct32b1.tcr.modify(|_, w| w.cen().bit(enabled));
}

pub fn reset_t0(ct32b0: &lpc1347::CT32B0) {
    ct32b0.tcr.modify(|_, w| w.crst().bit(true));
}

pub fn reset_t1(ct32b1: &lpc1347::CT32B1) {
    ct32b1.tcr.modify(|_, w| w.crst().bit(true));
}

pub unsafe fn set_pwm_output_pin(iocon: &lpc1347::IOCON, timer: Timer, mr: MatchReg) {
    match timer {
        Timer::Timer0 => {
            match mr {
                MatchReg::Reg0 => {
                    iocon.pio0_18.modify(|_, w| w.func().bits(0x2));
                    iocon.pio0_18.modify(|_, w| w.mode().bits(0x2));
                },
                MatchReg::Reg1 => {
                    iocon.pio0_19.modify(|_, w| w.func().bits(0x2));
                    iocon.pio0_19.modify(|_, w| w.mode().bits(0x2));
                },
                MatchReg::Reg2 => {
                    iocon.pio0_1.modify(|_, w| w.func().bits(0x2));
                    iocon.pio0_1.modify(|_, w| w.mode().bits(0x2));
                },
                MatchReg::Reg3 => {
                    iocon.tdi_pio0_11.modify(|_, w| w.func().bits(0x3));
                    iocon.tdi_pio0_11.modify(|_, w| w.mode().bits(0x2));
                }
            }
        },
        Timer::Timer1 => {
            match mr {
                MatchReg::Reg0 => {
                    iocon.tdo_pio0_13.modify(|_, w| w.func().bits(0x3));
                    iocon.tdo_pio0_13.modify(|_, w| w.mode().bits(0x2));
                },
                MatchReg::Reg1 => {
                    iocon.trst_pio0_14.modify(|_, w| w.func().bits(0x3));
                    iocon.trst_pio0_14.modify(|_, w| w.mode().bits(0x2));
                },
                MatchReg::Reg2 => {
                    iocon.swdio_pio0_15.modify(|_, w| w.func().bits(0x3));
                    iocon.swdio_pio0_15.modify(|_, w| w.mode().bits(0x2));
                },
                MatchReg::Reg3 => {
                    iocon.pio0_16.modify(|_, w| w.func().bits(0x2));
                    iocon.pio0_16.modify(|_, w| w.mode().bits(0x2));
                }
            }
        }
    }
}

pub fn set_pwm_opts_t0(ct32b0: &lpc1347::CT32B0, mr: MatchReg, control: Control, mat: bool, enabled: bool) {
    match mr {
        MatchReg::Reg0 => {
            ct32b0.emr.modify(|_, w| w.emc0().bits(control as u8));
            ct32b0.emr.modify(|_, w| w.em0().bit(mat));
            ct32b0.pwmc.modify(|_, w| w.pwmen0().bit(enabled));
        },
        MatchReg::Reg1 => {
            ct32b0.emr.modify(|_, w| w.emc1().bits(control as u8));
            ct32b0.emr.modify(|_, w| w.em1().bit(mat));
            ct32b0.pwmc.modify(|_, w| w.pwmen1().bit(enabled));
        },
        MatchReg::Reg2 => {
            ct32b0.emr.modify(|_, w| w.emc2().bits(control as u8));
            ct32b0.emr.modify(|_, w| w.em2().bit(mat));
            ct32b0.pwmc.modify(|_, w| w.pwmen2().bit(enabled));
        },
        MatchReg::Reg3 => {
            ct32b0.emr.modify(|_, w| w.emc3().bits(control as u8));
            ct32b0.emr.modify(|_, w| w.em3().bit(mat));
            ct32b0.pwmc.modify(|_, w| w.pwmen3().bit(enabled));
        }
    }
}

pub fn set_pwm_opts_t1(ct32b1: &lpc1347::CT32B1, mr: MatchReg, control: Control, mat: bool, enabled: bool) {
    match mr {
        MatchReg::Reg0 => {
            ct32b1.emr.modify(|_, w| w.emc0().bits(control as u8));
            ct32b1.emr.modify(|_, w| w.em0().bit(mat));
            ct32b1.pwmc.modify(|_, w| w.pwmen0().bit(enabled));
        },
        MatchReg::Reg1 => {
            ct32b1.emr.modify(|_, w| w.emc1().bits(control as u8));
            ct32b1.emr.modify(|_, w| w.em1().bit(mat));
            ct32b1.pwmc.modify(|_, w| w.pwmen1().bit(enabled));
        },
        MatchReg::Reg2 => {
            ct32b1.emr.modify(|_, w| w.emc2().bits(control as u8));
            ct32b1.emr.modify(|_, w| w.em2().bit(mat));
            ct32b1.pwmc.modify(|_, w| w.pwmen2().bit(enabled));
        },
        MatchReg::Reg3 => {
            ct32b1.emr.modify(|_, w| w.emc3().bits(control as u8));
            ct32b1.emr.modify(|_, w| w.em3().bit(mat));
            ct32b1.pwmc.modify(|_, w| w.pwmen3().bit(enabled));
        }
    }
}
/*
pub fn set_pwm_t0(ct32b0: &lpc1347::CT32B0) {
    set_enabled_t0(&ct32b0, false);

    ct32b0.emr.modify(|_, w| w.emc3().bits(0x1));
    ct32b0.emr.modify(|_, w| w.emc2().bits(0x1));
    ct32b0.emr.modify(|_, w| w.emc1().bits(0x1));
    ct32b0.emr.modify(|_, w| w.emc0().bits(0x1));

    ct32b0.emr.modify(|_, w| w.em3().bit(false));
    ct32b0.emr.modify(|_, w| w.em2().bit(false));
    ct32b0.emr.modify(|_, w| w.em1().bit(true));
    ct32b0.emr.modify(|_, w| w.em0().bit(true));

    ct32b0.pwmc.modify(|_, w| w.pwmen3().bit(false));
    ct32b0.pwmc.modify(|_, w| w.pwmen2().bit(false));
    ct32b0.pwmc.modify(|_, w| w.pwmen1().bit(true));
    ct32b0.pwmc.modify(|_, w| w.pwmen0().bit(true));

    set_match_t0(&ct32b0, MatchReg::Reg0, m0);
    set_match_t0(&ct32b0, MatchReg::Reg1, m1);
    set_match_t0(&ct32b0, MatchReg::Reg2, m2);
    set_match_t0(&ct32b0, MatchReg::Reg3, m3);

    // Reset on clock 0 -> period
    //ct16b0.mcr.modify(|_, w| w.mr0r().bit(true));

}
*/
