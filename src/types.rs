use crate::BASE_ADDR;

/// All possible errors in this crate
#[derive(Debug)]
pub enum Error<E> {
    /// I²C communication error
    I2C(E),
    /// Invalid input data provided
    InvalidInputData,
}

/// Measurement result
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Measurement {
    /// Temperature (°C)
    pub temperature: f32,
    /// Relative Humidity (%RH)
    ///
    /// Optionally read depending on the measurement configuration
    pub humidity: Option<f32>,
    /// Last status
    pub status: Status,
}

/// Status
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Status {
    /// Whether data is ready
    pub data_ready: bool,
    /// Whether the temperature high threshold was exceeded
    pub high_temp_threshold_exceeded: bool,
    /// Whether the temperature low threshold was exceeded
    pub low_temp_threshold_exceeded: bool,
    /// Whether the humidity high threshold was exceeded
    pub high_humidity_threshold_exceeded: bool,
    /// Whether the humidity low threshold was exceeded
    pub low_humidity_threshold_exceeded: bool,
}

/// Measurement mode
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MeasurementMode {
    /// Temperature and humidity (default)
    TemperatureAndHumidity,
    /// Temperature only
    TemperatureOnly,
}

impl Default for MeasurementMode {
    fn default() -> Self {
        MeasurementMode::TemperatureAndHumidity
    }
}

/// Possible slave addresses
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SlaveAddr {
    /// Default slave address
    Default,
    /// Alternative slave address providing bit value for the SDO pin
    Alternative(bool),
}

impl Default for SlaveAddr {
    /// Default slave address
    fn default() -> Self {
        SlaveAddr::Default
    }
}

impl SlaveAddr {
    pub(crate) fn addr(self) -> u8 {
        match self {
            SlaveAddr::Default => BASE_ADDR,
            SlaveAddr::Alternative(false) => BASE_ADDR,
            SlaveAddr::Alternative(true) => BASE_ADDR | 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::BASE_ADDR as ADDR;
    use super::{MeasurementMode, SlaveAddr};

    #[test]
    fn can_get_default_address() {
        let addr = SlaveAddr::default();
        assert_eq!(ADDR, addr.addr());
    }

    #[test]
    fn can_generate_alternative_addresses() {
        assert_eq!(ADDR, SlaveAddr::Alternative(false).addr());
        assert_eq!(ADDR | 1, SlaveAddr::Alternative(true).addr());
    }

    #[test]
    fn can_get_default_measurement_mode() {
        assert_eq!(
            MeasurementMode::TemperatureAndHumidity,
            MeasurementMode::default()
        );
    }
}
