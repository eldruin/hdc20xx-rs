use crate::{Error, Hdc20xx};
use embedded_hal::blocking::i2c;

pub const BASE_ADDR: u8 = 0x40;

pub struct Register;
impl Register {
    pub const TEMP_L: u8 = 0x00;
    pub const DRDY: u8 = 0x04;
    pub const MEAS_CONF: u8 = 0x0F;
    pub const MANUFACTURER_ID_L: u8 = 0xFC;
    pub const DEVICE_ID_L: u8 = 0xFE;
}

pub struct BitFlags;
impl BitFlags {
    pub const SOFT_RESET: u8 = 1 << 7;
    pub const TEMP_ONLY: u8 = 1 << 1;
    pub const MEAS_TRIG: u8 = 1;
    pub const DRDY_STATUS: u8 = 1 << 7;
    pub const TH_STATUS: u8 = 1 << 6;
    pub const TL_STATUS: u8 = 1 << 5;
    pub const HH_STATUS: u8 = 1 << 4;
    pub const HL_STATUS: u8 = 1 << 3;
}

impl<I2C, E, MODE> Hdc20xx<I2C, MODE>
where
    I2C: i2c::Write<Error = E>,
{
    pub(crate) fn write_register(&mut self, register: u8, data: u8) -> Result<(), Error<E>> {
        let payload: [u8; 2] = [register, data];
        let addr = self.address;
        self.i2c.write(addr, &payload).map_err(Error::I2C)
    }
}

impl<I2C, E, MODE> Hdc20xx<I2C, MODE>
where
    I2C: i2c::WriteRead<Error = E>,
{
    pub(crate) fn read_double_register(&mut self, register: u8) -> Result<u16, Error<E>> {
        let mut data = [0, 0];
        self.read_data(register, &mut data)
            .and(Ok(u16::from(data[0]) | (u16::from(data[1]) << 8)))
    }

    pub(crate) fn read_register(&mut self, register: u8) -> Result<u8, Error<E>> {
        let mut data = [0];
        self.read_data(register, &mut data).and(Ok(data[0]))
    }

    pub(crate) fn read_data(&mut self, register: u8, data: &mut [u8]) -> Result<(), Error<E>> {
        let addr = self.address;
        self.i2c
            .write_read(addr, &[register], data)
            .map_err(Error::I2C)
    }
}
