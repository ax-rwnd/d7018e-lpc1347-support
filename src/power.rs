#![allow(dead_code)]
extern crate lpc1347;
extern crate cortex_m;
extern crate cortex_m_rtfm as rtfm;

use lpc1347::Interrupt::{PIN_INT0, PIN_INT1, PIN_INT2, PIN_INT3, PIN_INT4, PIN_INT5, PIN_INT6, PIN_INT7, WWDT, BOD_IRQ, USBWAKEUP, GINT0, GINT1};

pub enum WakeupInts {
    Pin0,
    Pin1,
    Pin2,
    Pin3,
    Pin4,
    Pin5,
    Pin6,
    Pin7,
    WWDT,
    BOD,
    USBWAKEUP,
    GPIO0,
    GPIO1
}

/// Write deepsleep to the Cortex-M3
/// Note: this is not to be used directly
fn write_sleepdeep() {
    let cs = cortex_m::peripheral::SCB.get();
    unsafe {
        let rval = (*cs).scr.read();
        (*cs).scr.write( rval | (1<<2) | (1<<1));
    }
}

/// Enter deep-sleep mode
///
/// # Arguments
/// * `enabled_bod` - Enable brown-out detector clock, this is useful if low-voltage-induced non-determinism may cause severe errors
/// * `enabled_watchdog` - Enable watchdog clock, use this if you need to employ timed wake-ups
///
/// # Example
/// ```
/// fn my_task (_t: &mut Threshold, r: RESOURCE::Resources) {
///     power::deep_sleep(r.PMU);
///     wfi();
/// }
/// ```
pub fn deep_sleep(pmu: &lpc1347::PMU, syscon: &lpc1347::SYSCON, enable_bod: bool, enable_watchdog: bool) {
    unsafe {
        pmu.pcon.modify(|_, w| w.pm().bits(0x1));
    }

    // Enable bod if you want low-voltage protection, WD if you nned timer-based wakeup
    syscon.pdsleepcfg.modify(|_, w| w.bod_pd().bit(!enable_bod));
    syscon.pdsleepcfg.modify(|_, w| w.wdtosc_pd().bit(!enable_watchdog));

    write_sleepdeep();
}

/// Enter power-down mode
///
/// # Example
/// ```
/// fn my_task (_t: &mut Threshold, r: RESOURCE::Resources) {
///     power::power_down(r.PMU);
///     wfi();
/// }
/// ```
pub fn power_down(pmu: &lpc1347::PMU, syscon: &lpc1347::SYSCON, bod: bool, watchdog_osc: bool) {
    unsafe {
        pmu.pcon.modify(|_, w| w.pm().bits(0x2));
    }
    syscon.pdsleepcfg.modify(|_, w| w.bod_pd().bit(!bod));
    syscon.pdsleepcfg.modify(|_, w| w.wdtosc_pd().bit(!watchdog_osc));

    write_sleepdeep();
    rtfm::wfi();
}

/// Enter deep power-down mode
///
/// # Example
/// ```
/// fn my_task (_t: &mut Threshold, r: RESOURCE::Resources) {
///     // Save data to GPR
///     ...
///
///     // Power down
///     power::power_down(r.PMU);
///     wfi();
/// }
/// ```
pub fn deep_power_down(pmu: &lpc1347::PMU) {
    // Remember to clear the nodpd bit!pmu.pcon.modify(|_, w| w.nodpd().bit(false));
    unsafe {
        pmu.pcon.modify(|_, w| w.pm().bits(0x3));
    }
    write_sleepdeep();
}

/// Set wakeup interrupts
///
///
pub fn set_wakeup_interrupt(syscon: &lpc1347::SYSCON, nvic: &lpc1347::NVIC, interrupt: WakeupInts) {
    match interrupt {
        WakeupInts::Pin0 => {
            syscon.starterp0.modify(|_, w| w.pint0().bit(true));
            nvic.enable(PIN_INT0);
        },
        WakeupInts::Pin1 => {
            syscon.starterp0.modify(|_, w| w.pint1().bit(true));
            nvic.enable(PIN_INT1);
        },
        WakeupInts::Pin2 => {
            syscon.starterp0.modify(|_, w| w.pint2().bit(true));
            nvic.enable(PIN_INT2);
        },
        WakeupInts::Pin3 => {
            syscon.starterp0.modify(|_, w| w.pint3().bit(true));
            nvic.enable(PIN_INT3);
        },
        WakeupInts::Pin4 => {
            syscon.starterp0.modify(|_, w| w.pint4().bit(true));
            nvic.enable(PIN_INT4);
        },
        WakeupInts::Pin5 => {
            syscon.starterp0.modify(|_, w| w.pint5().bit(true));
            nvic.enable(PIN_INT5);
        },
        WakeupInts::Pin6 => {
            syscon.starterp0.modify(|_, w| w.pint6().bit(true));
            nvic.enable(PIN_INT6);
        },
        WakeupInts::Pin7 => {
            syscon.starterp0.modify(|_, w| w.pint7().bit(true));
            nvic.enable(PIN_INT7);
        },
        WakeupInts::WWDT => {
            syscon.starterp1.modify(|_, w| w.wwdtint().bit(true));
            nvic.enable(WWDT);
        },
        WakeupInts::BOD => {
            syscon.starterp1.modify(|_, w| w.bodint().bit(true));
            nvic.enable(BOD_IRQ);
        },
        WakeupInts::USBWAKEUP => {
            syscon.starterp1.modify(|_, w| w.usb_wakeup().bit(true));
            nvic.enable(USBWAKEUP);
        },
        WakeupInts::GPIO0 => {
            syscon.starterp1.modify(|_, w| w.gpioint0().bit(true));
            nvic.enable(GINT0);
        },
        WakeupInts::GPIO1 => {
            syscon.starterp1.modify(|_, w| w.gpioint1().bit(true));
            nvic.enable(GINT1);
        }
    }
}
