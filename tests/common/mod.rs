use embedded_hal_mock::i2c::{Mock as I2cMock, Transaction as I2cTrans};
use hdc20xx::{mode, Hdc20xx, SlaveAddr};

pub struct Register;
#[allow(unused)]
impl Register {
    pub const TEMP_L: u8 = 0x00;
    pub const HUMIDITY_L: u8 = 0x02;
    pub const DRDY: u8 = 0x04;
    pub const MEAS_CONF: u8 = 0x0F;
    pub const MANUFACTURER_ID_L: u8 = 0xFC;
    pub const DEVICE_ID_L: u8 = 0xFE;
}

pub struct BitFlags;
#[allow(unused)]
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
pub const BASE_ADDR: u8 = 0x40;

#[allow(unused)]
pub fn new(transactions: &[I2cTrans]) -> Hdc20xx<I2cMock, mode::OneShot> {
    Hdc20xx::new(I2cMock::new(transactions), SlaveAddr::default())
}

#[allow(unused)]
pub fn destroy<MODE>(sensor: Hdc20xx<I2cMock, MODE>) {
    sensor.destroy().done();
}
