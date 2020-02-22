mod common;
use crate::common::{destroy, new, BitFlags as BF, Register, BASE_ADDR};
use embedded_hal_mock::i2c::Transaction as I2cTrans;
use hdc20xx::MeasurementMode;

#[test]
fn can_create_and_destroy() {
    let sensor = new(&[]);
    destroy(sensor);
}

#[test]
fn can_get_device_id() {
    let dev_id = 0xABCD;
    let mut sensor = new(&[I2cTrans::write_read(
        BASE_ADDR,
        vec![Register::DEVICE_ID_L],
        vec![0xCD, 0xAB],
    )]);
    let id = sensor.device_id().unwrap();
    assert_eq!(dev_id, id);
    destroy(sensor);
}

#[test]
fn can_get_manufacturer_id() {
    let manuf_id = 0xABCD;
    let mut sensor = new(&[I2cTrans::write_read(
        BASE_ADDR,
        vec![Register::MANUFACTURER_ID_L],
        vec![0xCD, 0xAB],
    )]);
    let id = sensor.manufacturer_id().unwrap();
    assert_eq!(manuf_id, id);
    destroy(sensor);
}

macro_rules! set_test {
    ($name:ident, $method:ident, $reg:ident, $value:expr $(, $arg:expr)*) => {
        #[test]
        fn $name() {
            let mut sensor = new(&[I2cTrans::write(BASE_ADDR, vec![Register::$reg, $value])]);
            sensor.$method($($arg),*).unwrap();
            destroy(sensor);
        }
    };
}

set_test!(sw_reset_one_shot, software_reset, MEAS_CONF, BF::SOFT_RESET);
set_test!(
    set_temp_and_humidity_mode,
    set_measurement_mode,
    MEAS_CONF,
    0,
    MeasurementMode::TemperatureAndHumidity
);

set_test!(
    set_temp_only_mode,
    set_measurement_mode,
    MEAS_CONF,
    BF::TEMP_ONLY,
    MeasurementMode::TemperatureOnly
);

#[test]
fn can_make_one_shot_measurement_temp_and_humidity() {
    let transactions = [
        I2cTrans::write(BASE_ADDR, vec![Register::MEAS_CONF, BF::MEAS_TRIG]),
        I2cTrans::write_read(BASE_ADDR, vec![Register::DRDY], vec![0]),
        I2cTrans::write_read(BASE_ADDR, vec![Register::DRDY], vec![BF::DRDY_STATUS]),
        I2cTrans::write_read(
            BASE_ADDR,
            vec![Register::TEMP_L],
            vec![0xD9, 0x64, 0xEC, 0x91],
        ),
    ];
    let mut sensor = new(&transactions);
    sensor.read().expect_err("should block");
    sensor.read().expect_err("should block");
    let data = sensor.read().unwrap();
    assert!(data.temperature < 25.5);
    assert!(data.temperature > 24.5);
    let rh = data.humidity.unwrap();
    assert!(rh < 57.5);
    assert!(rh > 56.5);
    assert!(data.status.data_ready);
    destroy(sensor);
}

#[test]
fn can_make_one_shot_measurement_temp_only() {
    let transactions = [
        I2cTrans::write(BASE_ADDR, vec![Register::MEAS_CONF, BF::TEMP_ONLY]),
        I2cTrans::write(
            BASE_ADDR,
            vec![Register::MEAS_CONF, BF::TEMP_ONLY | BF::MEAS_TRIG],
        ),
        I2cTrans::write_read(BASE_ADDR, vec![Register::DRDY], vec![0]),
        I2cTrans::write_read(BASE_ADDR, vec![Register::DRDY], vec![BF::DRDY_STATUS]),
        I2cTrans::write_read(BASE_ADDR, vec![Register::TEMP_L], vec![0xD9, 0x64]),
    ];
    let mut sensor = new(&transactions);
    sensor
        .set_measurement_mode(MeasurementMode::TemperatureOnly)
        .unwrap();
    sensor.read().expect_err("should block");
    sensor.read().expect_err("should block");
    let data = sensor.read().unwrap();
    assert!(data.temperature < 25.5);
    assert!(data.temperature > 24.5);
    assert!(data.humidity.is_none());
    assert!(data.status.data_ready);
    destroy(sensor);
}
