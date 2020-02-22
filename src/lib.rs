//! This is a platform agnostic Rust driver for the HDC2080 low-power humidity
//! and temperature digital sensor using the [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
//!
//! This driver allows you to:
//! - Set the measurement mode. Temperature only or temperature and humidity. See: [`set_measurement_mode()`].
//! - Make one shot measurement. See: [`read()`].
//! - Read the data and interrupt status. See: [`status()`].
//! - Trigger a software reset. See: [`software_reset()`].
//! - Read the manufacturer ID. See: [`manufacturer_id()`].
//! - Read the device ID. See: [`device_id()`].
//! 
//! [`set_measurement_mode()`]: struct.Hdc2080.html#method.set_measurement_mode
//! [`read()`]: struct.Hdc2080.html#method.read
//! [`status()`]: struct.Hdc2080.html#method.status
//! [`software_reset()`]: struct.Hdc2080.html#method.software_reset
//! [`manufacturer_id()`]: struct.Hdc2080.html#method.manufacturer_id
//! [`device_id()`]: struct.Hdc2080.html#method.device_id
//!
//! <!-- TODO
//! [Introductory blog post](TODO)
//! -->
//!
//! ## The devices
//!
//! The HDC2080 device is an integrated humidity and temperature sensor that
//! provides high accuracy measurements with very low power consumption in a
//! small DFN package. The capacitive-based sensor includes new integrated
//! digital features and a heating element to dissipate condensation and moisture.
//!
//! The HDC2080 digital features include programmable interrupt thresholds to
//! provide alerts and system wake-ups without requiring a microcontroller to
//! be continuously monitoring the system. Combined with programmable sampling
//! intervals, a low power consumption, and a support for a 1.8-V supply voltage,
//! the HDC2080 is designed for battery-operated systems.
//!
//! This driver is compatible with HDC2080, HDC2021 and HDC2010.
//!
//! Datasheets: [HDC2080](https://www.ti.com/lit/ds/symlink/hdc2080.pdf), [HDC2021](https://www.ti.com/lit/ds/symlink/hdc2021.pdf), [HDC2010](https://www.ti.com/lit/ds/symlink/hdc2010.pdf)
//!
#![deny(unsafe_code, missing_docs)]
#![no_std]

use core::marker::PhantomData;
mod device_impl;
mod types;
pub use crate::types::{Error, Measurement, MeasurementMode, SlaveAddr, Status};
mod register_address;
use crate::register_address::{BitFlags, Register, BASE_ADDR};

/// HDC2080 device driver
#[derive(Debug)]
pub struct Hdc2080<I2C, MODE> {
    i2c: I2C,
    address: u8,
    meas_config: Config,
    was_measurement_started: bool,
    _mode: PhantomData<MODE>,
}

#[derive(Debug, Default, Clone, Copy)]
struct Config {
    bits: u8,
}

/// Mode marker
pub mod mode {
    /// One shot measurement mode
    pub struct OneShot(());
    /// Continuous measurement mode
    pub struct Continuous(());
}

mod private {
    use super::mode;
    pub trait Sealed {}
    impl Sealed for mode::OneShot {}
    impl Sealed for mode::Continuous {}
}
