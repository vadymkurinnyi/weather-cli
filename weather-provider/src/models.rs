use thiserror::Error;

#[derive(Debug)]
/// Struct that contains information about the weather at a certain point in time.
pub struct Weather {
    pub kind: WeatherKind,
    pub temp: Temperature,
    pub condition: String,
}
#[derive(Debug, PartialEq, Eq)]
/// Enum that contains the different kinds of weather information available.
pub enum WeatherKind {
    History,
    Current,
    Forecast,
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// Enum that represents a temperature in different scales.
pub enum Temperature {
    Kelvin(f32),
    Celsius(f32),
    Fahrenheit(f32),
}
#[derive(Debug, Clone, Copy)]
/// Enum that contains different temperature unit scales.
pub enum Units {
    /// Fahrenheit temperature unit scale.
    Imperial,
    /// Celsius temperature unit scale.
    Metric,
    /// Kelvin temperature unit scale.
    SI,
}

#[derive(Error, Debug)]
pub enum TemperatureError {
    #[error("The Kelvin temperature scale {0} must be greater than 0°K")]
    Kelvin(f32),
    #[error("The Celsius temperature scale {0} must be greater than -273.15°C")]
    Celsius(f32),
    #[error("The Fahrenheit temperature scale {0} must be greater than -459.67°F")]
    Fahrenheit(f32),
}

/// Constant that represents 0°C in Kelvin.
static K_ZERO_C: f32 = 273.15;
/// Constant that represents 0°F in Kelvin.
static K_ZERO_F: f32 = 459.67;
/// Constant that represents 0°C in Kelvin.
type TemperatureResult = Result<Temperature, TemperatureError>;

impl Temperature {
    /// Creates a `Temperature` instance from a Celsius value.
    /// 
    /// Returns a `TemperatureError` if the temperature value is less than -273.15°C.
    pub fn from_c(celsius: f32) -> TemperatureResult {
        if celsius < -K_ZERO_C {
            return Err(TemperatureError::Celsius(celsius));
        }
        Ok(Temperature::Celsius(celsius))
    }
    /// Creates a `Temperature` instance from a Fahrenheit value.
    /// 
    /// Returns a `TemperatureError` if the temperature value is less than -459.67°F.
    pub fn from_f(fahrenheit: f32) -> TemperatureResult {
        if fahrenheit < -K_ZERO_F {
            return Err(TemperatureError::Fahrenheit(fahrenheit));
        }
        Ok(Temperature::Fahrenheit(fahrenheit))
    }
    /// Creates a `Temperature` instance from a Kelvin value.
    /// 
    /// Returns a `TemperatureError` if the temperature value is less than 0°K.
    pub fn from_k(kelvin: f32) -> TemperatureResult {
        if kelvin < 0.0 {
            return Err(TemperatureError::Kelvin(kelvin));
        }
        Ok(Temperature::Kelvin(kelvin))
    }
    /// Converts the temperature value to a string representation based on the desired units.
    ///
    /// # Arguments
    ///
    /// * `units` - The units to use for the string representation of the temperature.
    ///
    /// # Returns
    ///
    /// A string representation of the temperature in the desired units.
    ///
    /// # Examples
    ///
    /// ```
    /// use weather_provider::{Temperature, Units};
    ///
    /// let temperature = Temperature::Kelvin(273.15);
    /// let string_value = temperature.to_string_value(Units::Metric);
    ///
    /// assert_eq!(string_value, "0.0°C");
    /// ```
    pub fn to_string_value(self, units: Units) -> String {
        match self {
            Temperature::Kelvin(k) => match units {
                Units::Imperial => {
                    let f = (k - K_ZERO_C) * 9.0 / 5.0 + 32.0;
                    format!("{:.1}°F", f)
                }
                Units::Metric => {
                    let c = k - K_ZERO_C;
                    format!("{:.1}°C", c)
                }
                Units::SI => format!("{:.1}°K", k),
            },
            Temperature::Celsius(c) => match units {
                Units::Imperial => {
                    let f = (c * 1.8) + 32.0;
                    format!("{:.1}°F", f)
                }
                Units::Metric => {
                    format!("{:.1}°C", c)
                }
                Units::SI => {
                    let k = c + K_ZERO_C;
                    format!("{:.1}°K", k)
                }
            },
            Temperature::Fahrenheit(f) => match units {
                Units::Imperial => {
                    format!("{:.1}°F", f)
                }
                Units::Metric => {
                    let c = (f - 32.0) * 5.0 / 9.0;
                    format!("{:.1}°C", c)
                }
                Units::SI => {
                    let k = (f + K_ZERO_F) * 5.0 / 9.0;
                    format!("{:.1}°K", k)
                }
            },
        }
    }
}
impl Weather {
    /// Creates a new `Weather` instance with type `WeatherKind::History` and specified temperature and weather condition.
    ///
    /// # Examples
    ///
    /// ```
    /// use weather_provider::{Temperature, Units, Weather, WeatherKind};
    /// let temp = Temperature::from_c(22.0).unwrap();
    /// let weather = Weather::history(temp, "Sunny");
    /// assert_eq!(weather.kind, WeatherKind::History);
    /// assert_eq!(weather.temp.to_string_value(Units::Metric), "22.0°C");
    /// assert_eq!(weather.condition, "Sunny");
    /// ```
    pub fn history(temp: Temperature, condition: impl Into<String>) -> Self {
        Self {
            kind: WeatherKind::History,
            temp,
            condition: condition.into(),
        }
    }
    /// Creates a new `Weather` instance with type `WeatherKind::Current` and specified temperature and weather condition.
    ///
    /// # Examples
    ///
    /// ```
    /// use weather_provider::{Temperature, Units, Weather, WeatherKind};
    /// let temp = Temperature::from_c(22.0).unwrap();
    /// let weather = Weather::current(temp, "Sunny");
    /// assert_eq!(weather.kind, WeatherKind::Current);
    /// assert_eq!(weather.temp.to_string_value(Units::Metric), "22.0°C");
    /// assert_eq!(weather.condition, "Sunny");
    /// ```
    pub fn current(temp: Temperature, condition: impl Into<String>) -> Self {
        Self {
            kind: WeatherKind::Current,
            temp,
            condition: condition.into(),
        }
    }
    /// Creates a new `Weather` instance with type `WeatherKind::Forecast` and specified temperature and weather condition.
    ///
    /// # Examples
    ///
    /// ```
    /// use weather_provider::{Temperature, Units, Weather, WeatherKind};
    /// let temp = Temperature::from_c(22.0).unwrap();
    /// let weather = Weather::forecast(temp, "Sunny");
    /// assert_eq!(weather.kind, WeatherKind::Forecast);
    /// assert_eq!(weather.temp.to_string_value(Units::Metric), "22.0°C");
    /// assert_eq!(weather.condition, "Sunny");
    /// ```
    pub fn forecast(temp: Temperature, condition: impl Into<String>) -> Self {
        Self {
            kind: WeatherKind::Forecast,
            temp,
            condition: condition.into(),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(-273.151)]
    #[case(-300.0)]
    #[case(f32::MIN)]
    fn test_temperature_from_c_negative(#[case] val: f32) {
        Temperature::from_c(val).expect_err(&format!("{} is not valid value for Celcius", val));
    }
    #[rstest]
    #[case(-273.15)]
    #[case(-100.0)]
    #[case(0.0)]
    #[case(20.0)]
    #[case(f32::MAX)]
    fn test_temperature_from_c_positive(#[case] val: f32) {
        Temperature::from_c(val).expect(&format!("{} valid value for Celcius", val));
    }
    #[rstest]
    #[case(-0.001)]
    #[case(-100.0)]
    #[case(f32::MIN)]
    fn test_temperature_from_k_negative(#[case] val: f32) {
        Temperature::from_k(val).expect_err(&format!("{} is not a valid value for Kelvin", val));
    }
    #[rstest]
    #[case(273.15)]
    #[case(373.15)]
    #[case(f32::MAX)]
    fn test_temperature_from_k_positive(#[case] val: f32) {
        Temperature::from_k(val).expect(&format!("{} is a valid value for Kelvin", val));
    }

    #[rstest]
    #[case(-459.6701)]
    #[case(-500.0)]
    #[case(f32::MIN)]
    fn test_temperature_from_f_negative(#[case] val: f32) {
        Temperature::from_f(val)
            .expect_err(&format!("{} is not a valid value for Fahrenheit", val));
    }
    #[rstest]
    #[case(-459.67)]
    #[case(32.0)]
    #[case(212.0)]
    #[case(f32::MAX)]
    fn test_temperature_from_f_positive(#[case] val: f32) {
        Temperature::from_f(val).expect(&format!("{} is a valid value for Fahrenheit", val));
    }

    #[rstest]
    #[case(10.099, "10.1°C", "10.1°F", "10.1°K")]
    #[case(20.0001, "20.0°C", "20.0°F", "20.0°K")]
    #[case(30.999, "31.0°C", "31.0°F", "31.0°K")]
    #[case(50.499, "50.5°C", "50.5°F", "50.5°K")]
    fn test_temperature_to_string_value(
        #[case] val: f32,
        #[case] c: &str,
        #[case] f: &str,
        #[case] k: &str,
    ) {
        let temp = Temperature::Celsius(val);
        let string_value = temp.to_string_value(Units::Metric);
        assert_eq!(string_value, c);

        let temp = Temperature::Fahrenheit(val);
        let string_value = temp.to_string_value(Units::Imperial);
        assert_eq!(string_value, f);

        let temp = Temperature::Kelvin(val);
        let string_value = temp.to_string_value(Units::SI);
        assert_eq!(string_value, k);
    }
    #[rstest]
    #[case(Temperature::Kelvin(0.0), Units::Imperial, "-459.7°F")]
    #[case(Temperature::Kelvin(0.0), Units::Metric, "-273.1°C")]
    #[case(Temperature::Kelvin(0.0), Units::SI, "0.0°K")]
    #[case(Temperature::Celsius(0.0), Units::Imperial, "32.0°F")]
    #[case(Temperature::Celsius(0.0), Units::Metric, "0.0°C")]
    #[case(Temperature::Celsius(0.0), Units::SI, "273.1°K")]
    #[case(Temperature::Fahrenheit(0.0), Units::Imperial, "0.0°F")]
    #[case(Temperature::Fahrenheit(0.0), Units::Metric, "-17.8°C")]
    #[case(Temperature::Fahrenheit(0.0), Units::SI, "255.4°K")]
    fn test_to_string_value(
        #[case] temp: Temperature,
        #[case] units: Units,
        #[case] expected: &str,
    ) {
        assert_eq!(temp.to_string_value(units), expected);
    }
}
