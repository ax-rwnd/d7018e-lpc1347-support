//! Board support crate for NXP's LCP1347
//!
//! # Usage
//! Each module supplies a driver for some functionality, such as GPIO access or
//! clock setup.

#![no_std]

pub extern crate lpc1347;
pub mod power;
pub mod clock;
pub mod gpio;
pub mod timers;
pub mod timers32;
pub mod uart;
pub mod adc;
