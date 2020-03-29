use std::num::{NonZeroU16, NonZeroU32};
use std::str::FromStr;
use std::string::ParseError;

use crate::app::publisher::PublishingInfo;
use structopt::StructOpt;
use uom::si::f32::ThermodynamicTemperature;
use uom::si::thermodynamic_temperature;
use url::Url;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "outdoor",
    about = "Publishes current weather from OpenWeatherMap to Hardwario/BigClown bus"
)]
pub struct Args {
    #[structopt(short = "v", long, parse(from_occurrences))]
    pub verbose: u8,

    /// API key from openweathermap.com
    #[structopt(env)]
    pub api_key: ApiKey,

    /// OpenWeatherMap city ID
    ///
    /// Use a city ID as recomended in https://openweathermap.org/appid
    /// All city ids should be at http://bulk.openweathermap.org/sample/city.list.json.gz
    #[structopt(env)]
    pub city_id: u32,

    #[structopt(short, long, env, default_value = & Units::CELSIUS.value().unwrap(), possible_values = & Units::variants())]
    pub units: Units,

    #[structopt(flatten)]
    pub publishing: MqttPublishingArgs,

    /// Weather API scraping period in seconds
    ///
    /// OpenWeatherMap update information no more than one time every 10 minutes.
    #[structopt(short, long, env, default_value = "600")]
    pub interval_secs: NonZeroU32,

    /// Base API for weather requests
    #[structopt(long)]
    pub api_base: Option<Url>,

    #[structopt(env)]
    pub mqtt_host: String,

    #[structopt(env, long, default_value = "1883")]
    pub mqtt_port: NonZeroU16,

    #[structopt(long, env)]
    pub mqtt_user: Option<User>,

    #[structopt(long, env)]
    pub mqtt_password: Option<Password>,
}

#[derive(Debug, StructOpt)]
pub struct MqttPublishingArgs {
    /// Identification of this agent in published weather information
    #[structopt(env)]
    pub device_name: String,

    /// MQTT topic prefix
    ///
    /// Prefix for topics that are defined in https://developers.hardwario.com/interfaces/mqtt-protocol
    #[structopt(short, long, env)]
    pub topic_prefix: Option<String>,

    #[structopt(long, env, default_value = "0:0")]
    pub channel_thermometer: String,

    #[structopt(long, env, default_value = "0:0")]
    pub channel_barometer: String,

    #[structopt(long, env, default_value = "0:0")]
    pub channel_hygrometer: String,
}

impl PublishingInfo for MqttPublishingArgs {
    fn get_prefix(&self) -> &Option<String> {
        &self.topic_prefix
    }

    fn get_device_name(&self) -> &str {
        &self.device_name
    }

    fn get_channel_thermometer(&self) -> &str {
        &self.channel_thermometer
    }

    fn get_channel_barometer(&self) -> &str {
        &self.channel_barometer
    }

    fn get_channel_hygrometer(&self) -> &str {
        &self.channel_hygrometer
    }
}

#[derive(Debug)]
pub enum Units {
    KELVIN,
    FAHRENHEIT,
    CELSIUS,
}

impl Units {
    pub fn convert_temperature(&self, temperature: ThermodynamicTemperature) -> f32 {
        match *self {
            Units::KELVIN => temperature.get::<thermodynamic_temperature::kelvin>(),
            Units::FAHRENHEIT => temperature.get::<thermodynamic_temperature::degree_fahrenheit>(),
            Units::CELSIUS => temperature.get::<thermodynamic_temperature::degree_celsius>(),
        }
    }

    pub fn value(&self) -> Option<&'static str> {
        match *self {
            Units::CELSIUS => Some("celsius"),
            Units::FAHRENHEIT => Some("fahrenheit"),
            Units::KELVIN => None,
        }
    }

    fn variants() -> Vec<&'static str> {
        vec!["celsius", "fahrenheit", "kelvin"]
    }
}

impl FromStr for Units {
    type Err = ParseError;
    fn from_str(day: &str) -> Result<Self, Self::Err> {
        match day {
            "celsius" => Ok(Units::CELSIUS),
            "fahrenheit" => Ok(Units::FAHRENHEIT),
            "kelvin" => Ok(Units::KELVIN),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub struct ApiKey {
    value: String,
}

impl Into<String> for ApiKey {
    fn into(self) -> String {
        self.value
    }
}

impl FromStr for ApiKey {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ApiKey {
            value: s.to_owned(),
        })
    }
}

#[derive(Debug)]
pub struct User(String);

impl FromStr for User {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(User(s.to_owned()))
    }
}

impl Into<String> for User {
    fn into(self) -> String {
        self.0
    }
}

#[derive(Debug)]
pub struct Password(String);

impl FromStr for Password {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Password(s.to_owned()))
    }
}

impl Into<String> for Password {
    fn into(self) -> String {
        self.0
    }
}
