extern crate linux_embedded_hal as hal;
use hdc2080::{Hdc2080, SlaveAddr};
use nb::block;

fn main() {
    let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
    let address = SlaveAddr::default();
    let mut sensor = Hdc2080::new(dev, address);
    loop {
        let data = block!(sensor.read()).unwrap();
        println!(
            "Temperature: {:2}°C, Humidity: {:2}%",
            data.temperature,
            data.humidity.unwrap()
        );
    }
}
