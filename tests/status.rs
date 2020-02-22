mod common;
use crate::common::{destroy, new, BitFlags as BF, Register, BASE_ADDR};
use embedded_hal_mock::i2c::Transaction as I2cTrans;
use hdc20xx::Status;

macro_rules! test {
    ($name:ident, $value:expr, $drdy:expr, $htemp:expr, $ltemp:expr,
     $hrh:expr, $lrh:expr) => {
        #[test]
        fn $name() {
            let mut sensor = new(&[I2cTrans::write_read(
                BASE_ADDR,
                vec![Register::DRDY],
                vec![$value],
            )]);
            let st = sensor.status().unwrap();
            assert_eq!(
                st,
                Status {
                    data_ready: $drdy,
                    high_temp_threshold_exceeded: $htemp,
                    low_temp_threshold_exceeded: $ltemp,
                    high_humidity_threshold_exceeded: $hrh,
                    low_humidity_threshold_exceeded: $lrh,
                }
            );
            destroy(sensor);
        }
    };
}

test!(all_false, 0, false, false, false, false, false);
test!(drdy, BF::DRDY_STATUS, true, false, false, false, false);
test!(htemp, BF::TH_STATUS, false, true, false, false, false);
test!(ltemp, BF::TL_STATUS, false, false, true, false, false);
test!(hrh, BF::HH_STATUS, false, false, false, true, false);
test!(lrh, BF::HL_STATUS, false, false, false, false, true);
test!(
    all_true,
    BF::DRDY_STATUS | BF::TH_STATUS | BF::TL_STATUS | BF::HH_STATUS | BF::HL_STATUS,
    true,
    true,
    true,
    true,
    true
);
