use embedded_hal::blocking::delay::DelayMs;
use hdc20xx::{Hdc20xx, SlaveAddr};
use linux_embedded_hal::{Delay, I2cdev};

fn main() {
    let mut delay = Delay {};
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let address = SlaveAddr::default();
    let mut sensor = Hdc20xx::new(dev, address);
    loop {
        loop {
            let result = sensor.read();
            match result {
                Err(nb::Error::WouldBlock) => delay.delay_ms(100_u8),
                Err(e) => {
                    println!("Error! {:?}", e);
                }
                Ok(data) => {
                    println!(
                        "Temperature: {:2}°C, Humidity: {:2}%",
                        data.temperature,
                        data.humidity.unwrap()
                    );
                }
            }
        }
    }
}
