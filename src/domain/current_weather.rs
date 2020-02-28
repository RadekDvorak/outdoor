use std::convert::Into;

use uom::si::f32::*;
use uom::si::{pressure, thermodynamic_temperature};

use crate::weather_types::{Main, WeatherReportCurrent};

#[derive(Debug)]
pub struct CurrentWeather {
    temperature: ThermodynamicTemperature,
    pressure: Pressure,
    humidity: Humidity,
}

impl CurrentWeather {
    pub fn new(temperature: f32, pressure: f32, humidity: f32) -> Self {
        CurrentWeather {
            temperature: ThermodynamicTemperature::new::<thermodynamic_temperature::kelvin>(
                temperature,
            ),
            pressure: Pressure::new::<pressure::hectopascal>(pressure),
            humidity: Humidity::new(humidity),
        }
    }
}

impl From<Main> for CurrentWeather {
    fn from(main: Main) -> Self {
        CurrentWeather::new(main.temp, main.pressure, main.humidity)
    }
}
impl From<WeatherReportCurrent> for CurrentWeather {
    fn from(report: WeatherReportCurrent) -> Self {
        report.main.into()
    }
}

impl CurrentWeather {
    pub fn get_temperature(&self) -> &ThermodynamicTemperature {
        &self.temperature
    }

    pub fn get_pressure(&self) -> &Pressure {
        &self.pressure
    }

    pub fn get_humidity(&self) -> &Humidity {
        &self.humidity
    }
}

#[derive(Debug)]
pub struct Humidity {
    value: f32,
}

impl Humidity {
    pub fn new(value: f32) -> Self {
        assert!(value <= 100.0, "Humidity may be at most 100.0%.");
        assert!(value >= 0.0, "Humidity must be at least 0%.");

        Humidity {
            value: Self::round(value),
        }
    }

    fn round(value: f32) -> f32 {
        (value * 100.0).round() / 100.0
    }

    #[allow(dead_code)]
    pub fn is_valid(humidity: f32) -> bool {
        let rounded = Self::round(humidity);
        rounded >= 0.0 && rounded <= 100.0
    }
}

impl AsRef<f32> for Humidity {
    fn as_ref(&self) -> &f32 {
        &self.value
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static EPSILON: f32 = 0.0001;

    #[test]
    fn weather_pressure_ok() {
        let t = CurrentWeather::new(15.7, 1001.0, 55.1);

        let v: f32 = t.get_pressure().get::<pressure::pascal>();
        assert!(
            (100_100.0 - v).abs() < EPSILON,
            "This pressure {} should be 100100",
            v
        );

        let v2: f32 = t.get_pressure().get::<pressure::pascal>();
        assert!(
            (100_100.0 - v2).abs() < EPSILON,
            "This pressure {} should be 100100",
            v
        );
    }

    #[test]
    fn weather_temperature_ok() {
        let t = CurrentWeather::new(283.3, 1001.0, 55.1);

        let v: f32 = t
            .get_temperature()
            .get::<thermodynamic_temperature::degree_celsius>();
        assert!(
            (10.15 - v).abs() < EPSILON,
            "This temperature {} should be 10.15",
            v
        );

        let v2: f32 = t
            .get_temperature()
            .get::<thermodynamic_temperature::degree_celsius>();
        assert!(
            (10.15 - v2).abs() < EPSILON,
            "This temperature {} should be 10.15",
            v
        );
    }

    #[test]
    fn weather_humidity_ok() {
        let t = CurrentWeather::new(283.3, 1001.0, 55.1);

        let v: &f32 = t.get_humidity().as_ref();
        assert!(
            (55.1 - *v).abs() < EPSILON,
            "This humidity {} should be 55.1",
            v
        );
    }

    #[test]
    fn weather_from_main() {
        let m = Main {
            temp: 199.0,
            temp_min: 0.0,
            temp_max: 0.0,
            pressure: 998.0,
            sea_level: None,
            grnd_level: None,
            humidity: 0.0,
            temp_kf: None,
        };
        let _: CurrentWeather = m.into();
    }

    #[test]
    fn humidity_ok() {
        let t = Humidity::new(32.0);
        let v: f32 = t.into();

        assert!((32.0 - v).abs() < EPSILON);
    }

    #[test]
    #[should_panic(expected = "Humidity may be at most 100.0%.")]
    fn humidity_high() {
        Humidity::new(132.0);
    }

    #[test]
    #[should_panic(expected = "Humidity must be at least 0%.")]
    fn humidity_low() {
        Humidity::new(-1.0);
    }

    #[test]
    fn humidity_valid() {
        let v = 0.45 - 0.15 - 0.15 - 0.15;
        assert!(
            Humidity::is_valid(v),
            "Hodnota musí být <0.0; 100.0> a ne {}.",
            v
        )
    }
}
