#![feature(proc_macro)]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rtfm as rtfm;
extern crate cortex_m_semihosting;
extern crate lpc1347;
extern crate cortex_m_rtfm_macros;
extern crate rtfm_core;
extern crate untagged_option;

use core::u8;

pub use rtfm_core::{Resource, Static, Threshold};
pub use cortex_m::asm::{bkpt, wfi};
pub use cortex_m_rtfm_macros::app;
#[doc(hidden)]
pub use untagged_option::UntaggedOption;

use cortex_m::interrupt::{self, Nr};
#[cfg(not(armv6m))]
use cortex_m::register::basepri;

/// Executes the closure `f` in a preemption free context
///
/// During the execution of the closure no task can preempt the current task.
pub fn atomic<R, F>(t: &mut Threshold, f: F) -> R
where
    F: FnOnce(&mut Threshold) -> R,
{
    if t.value() == u8::MAX {
        f(t)
    } else {
        interrupt::disable();
        let r = f(&mut unsafe { Threshold::max() });
        unsafe { interrupt::enable() };
        r
    }
}

#[inline]
#[doc(hidden)]
pub unsafe fn claim<T, R, F>(
    data: T,
    ceiling: u8,
    _nvic_prio_bits: u8,
    t: &mut Threshold,
    f: F,
) -> R
where
    F: FnOnce(T, &mut Threshold) -> R,
{
    if ceiling > t.value() {
        match () {
            #[cfg(armv6m)]
            () => atomic(t, |t| f(data, t)),

            #[cfg(not(armv6m))]
            () => {
                let max_priority = 1 << _nvic_prio_bits;

                if ceiling == max_priority {
                    atomic(t, |t| f(data, t))
                } else {
                    let old = basepri::read();
                    let hw = (max_priority - ceiling) << (8 - _nvic_prio_bits);
                    basepri::write(hw);
                    let ret = f(data, &mut Threshold::new(ceiling));
                    basepri::write(old);
                    ret
                }
            }
        }
    } else {
        f(data, t)
    }
}

/// Sets an interrupt, that is a task, as pending
///
/// If the task priority is high enough the task will be serviced immediately,
/// otherwise it will be serviced at some point after the current task ends.
pub fn set_pending<I>(interrupt: I)
where
    I: Nr,
{
    // NOTE(safe) atomic write
    let nvic = unsafe { &*cortex_m::peripheral::NVIC.get() };
    nvic.set_pending(interrupt);
}
