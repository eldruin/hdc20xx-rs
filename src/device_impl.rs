use crate::{
    mode, BitFlags, Config, Error, Hdc2080, Measurement, MeasurementMode, Register, SlaveAddr,
    Status,
};
use core::marker::PhantomData;
use embedded_hal::blocking::i2c;

impl<I2C> Hdc2080<I2C, mode::OneShot> {
    /// Create new instance of the HDC2080 device.
    pub fn new(i2c: I2C, address: SlaveAddr) -> Self {
        Hdc2080 {
            i2c,
            address: address.addr(),
            meas_config: Config { bits: 0 },
            was_measurement_started: false,
            _mode: PhantomData,
        }
    }
}

impl<I2C, MODE> Hdc2080<I2C, MODE> {
    /// Destroy driver instance, return I2C bus.
    pub fn destroy(self) -> I2C {
        self.i2c
    }
}

impl<I2C, E, MODE> Hdc2080<I2C, MODE>
where
    I2C: i2c::WriteRead<Error = E> + i2c::Write<Error = E>,
{
    /// Set measurement mode
    pub fn set_measurement_mode(&mut self, mode: MeasurementMode) -> Result<(), Error<E>> {
        let config = match mode {
            MeasurementMode::TemperatureAndHumidity => {
                self.meas_config.with_low(BitFlags::TEMP_ONLY)
            }
            MeasurementMode::TemperatureOnly => self.meas_config.with_high(BitFlags::TEMP_ONLY),
        };
        self.write_register(Register::MEAS_CONF, config.bits)?;
        self.meas_config = config;
        Ok(())
    }

    /// Read status
    pub fn status(&mut self) -> Result<Status, Error<E>> {
        let status = self.read_register(Register::DRDY)?;
        Ok(Status {
            data_ready: (status & BitFlags::DRDY_STATUS) != 0,
            high_temp_threshold_exceeded: (status & BitFlags::TH_STATUS) != 0,
            low_temp_threshold_exceeded: (status & BitFlags::TL_STATUS) != 0,
            high_humidity_threshold_exceeded: (status & BitFlags::HH_STATUS) != 0,
            low_humidity_threshold_exceeded: (status & BitFlags::HL_STATUS) != 0,
        })
    }

    /// Get device ID
    pub fn device_id(&mut self) -> Result<u16, Error<E>> {
        self.read_double_register(Register::DEVICE_ID_L)
    }

    /// Get manufacturer ID
    pub fn manufacturer_id(&mut self) -> Result<u16, Error<E>> {
        self.read_double_register(Register::MANUFACTURER_ID_L)
    }
}

impl<I2C, E> Hdc2080<I2C, mode::OneShot>
where
    I2C: i2c::WriteRead<Error = E> + i2c::Write<Error = E>,
{
    /// Make measurement of temperature or temperature and humidity according
    /// to the configuration.
    ///
    /// Note that all status except the last one once data becomes available
    /// are discarded.
    pub fn read(&mut self) -> nb::Result<Measurement, Error<E>> {
        if self.was_measurement_started {
            let status = self.status()?;
            if status.data_ready {
                let include_humidity = !self.meas_config.is_high(BitFlags::TEMP_ONLY);
                let mut data = [0; 4];
                if include_humidity {
                    self.read_data(Register::TEMP_L, &mut data)?;
                } else {
                    self.read_data(Register::TEMP_L, &mut data[..2])?;
                }
                self.was_measurement_started = false;
                let temp_raw = u16::from(data[0]) | (u16::from(data[1]) << 8);
                let temp = f32::from(temp_raw) / 65536.0 * 165.0 - 40.0;
                if include_humidity {
                    let rh_raw = u16::from(data[2]) | (u16::from(data[3]) << 8);
                    let rh = f32::from(rh_raw) / 65536.0 * 100.0;
                    Ok(Measurement {
                        temperature: temp,
                        humidity: Some(rh),
                        status,
                    })
                } else {
                    Ok(Measurement {
                        temperature: temp,
                        humidity: None,
                        status,
                    })
                }
            } else {
                Err(nb::Error::WouldBlock)
            }
        } else {
            let meas_conf = self.meas_config.with_high(BitFlags::MEAS_TRIG);
            self.write_register(Register::MEAS_CONF, meas_conf.bits)?;
            self.was_measurement_started = true;
            Err(nb::Error::WouldBlock)
        }
    }

    /// Software reset
    pub fn software_reset(&mut self) -> Result<(), Error<E>> {
        let conf = self.meas_config.with_high(BitFlags::SOFT_RESET);
        self.write_register(Register::MEAS_CONF, conf.bits)
    }
}

impl Config {
    fn with_high(self, mask: u8) -> Self {
        Config {
            bits: self.bits | mask,
        }
    }
    fn with_low(self, mask: u8) -> Self {
        Config {
            bits: self.bits & !mask,
        }
    }
    fn is_high(self, mask: u8) -> bool {
        (self.bits & mask) != 0
    }
}
