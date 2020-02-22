//! This is a platform agnostic Rust driver for the HDC2080, HDC2021 and
//! HDC2010 low-power humidity and temperature digital sensor using
//! the [`embedded-hal`] traits.
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
//! [`set_measurement_mode()`]: struct.Hdc20xx.html#method.set_measurement_mode
//! [`read()`]: struct.Hdc20xx.html#method.read
//! [`status()`]: struct.Hdc20xx.html#method.status
//! [`software_reset()`]: struct.Hdc20xx.html#method.software_reset
//! [`manufacturer_id()`]: struct.Hdc20xx.html#method.manufacturer_id
//! [`device_id()`]: struct.Hdc20xx.html#method.device_id
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
//! ## Usage examples (see also examples folder)
//!
//! To use this driver, import this crate and an `embedded_hal` implementation,
//! then instantiate the device.
//!
//! Please find additional examples using hardware in this repository: [driver-examples]
//!
//! [driver-examples]: https://github.com/eldruin/driver-examples
//!
//! ### Make a one-shot temperature and humidity measurement
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! use hdc20xx::{Hdc20xx, SlaveAddr};
//! use nb::block;
//! 
//! # fn main() {
//! let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
//! let address = SlaveAddr::default();
//! let mut sensor = Hdc20xx::new(dev, address);
//! loop {
//!     let data = block!(sensor.read()).unwrap();
//!     println!(
//!         "Temperature: {:2}°C, Humidity: {:2}%",
//!         data.temperature,
//!         data.humidity.unwrap()
//!     );
//! }
//! # }
//! ```
//! 
//! ### Use an alternative address
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! use hdc20xx::{Hdc20xx, SlaveAddr};
//! 
//! # fn main() {
//! let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
//! let address = SlaveAddr::Alternative(true);
//! let sensor = Hdc20xx::new(dev, address);
//! # }
//! ```
//! 
//! ### Configure measuring only the temperature
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! use hdc20xx::{Hdc20xx, MeasurementMode, SlaveAddr};
//! 
//! # fn main() {
//! let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
//! let address = SlaveAddr::default();
//! let mut sensor = Hdc20xx::new(dev, address);
//! sensor.set_measurement_mode(MeasurementMode::TemperatureOnly).unwrap();
//! # }
//! ```
//! 
//! ### Read the manufacturer and device ID
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! use hdc20xx::{Hdc20xx, SlaveAddr};
//! 
//! # fn main() {
//! let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
//! let address = SlaveAddr::default();
//! let mut sensor = Hdc20xx::new(dev, address);
//! let manuf_id = sensor.manufacturer_id().unwrap();
//! let dev_id = sensor.device_id().unwrap();
//! println!(
//!     "Manufacturer ID: {}, Device ID: {}",
//!     manuf_id, dev_id
//! );
//! # }
//! ```
//! 
//! ### Read the data and interrupt status
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! use hdc20xx::{Hdc20xx, SlaveAddr};
//! 
//! # fn main() {
//! let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
//! let address = SlaveAddr::default();
//! let mut sensor = Hdc20xx::new(dev, address);
//! let status = sensor.status().unwrap();
//! println!("Status: {:?}", status);
//! # }
//! ```
//! 
#![deny(unsafe_code, missing_docs)]
#![no_std]

use core::marker::PhantomData;
mod device_impl;
mod types;
pub use crate::types::{Error, Measurement, MeasurementMode, SlaveAddr, Status};
mod register_address;
use crate::register_address::{BitFlags, Register, BASE_ADDR};

/// HDC2080, HDC2021 and HDC2010 device driver
#[derive(Debug)]
pub struct Hdc20xx<I2C, MODE> {
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
