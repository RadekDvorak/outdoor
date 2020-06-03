extern crate anyhow;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate slog;
extern crate sloggers;
extern crate uom;
extern crate url;

use std::time::Duration;

use rumq_client::eventloop;
use rumq_client::MqttOptions;
use tokio::sync::mpsc::channel;

use domain::current_weather;
use location_specifier::LocationSpecifier;

use crate::app::publisher::{Humidity, Pressure, Temperature};
use crate::app::tasks::*;
use crate::arguments::MqttConnectionArgs;
use crate::weather_client::OpenWeatherMapClientBuilder;
use std::sync::Arc;

mod app;
mod arguments;
mod domain;
mod location_specifier;
mod weather_client;
mod weather_types;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let settings: arguments::Args = structopt::StructOpt::from_args();

    let logger = Arc::new(app::logging::create_logger(settings.verbose)?);

    let city_id = settings.city_id.to_string();
    let api_key = settings.api_key;
    let period = Duration::from_secs(settings.interval_secs.get().into());
    let api_base = settings.api_base;

    let (weather_tx, weather_rx) = channel::<current_weather::CurrentWeather>(10);

    let mut builder =
        OpenWeatherMapClientBuilder::new(LocationSpecifier::CityId(city_id.as_ref()), api_key);
    if let Some(base) = api_base {
        builder.with_base_url(base);
    }

    let api_client = builder.build()?;

    let weather_fetcher = {
        let builder = WeatherFetcherBuilder::new(weather_tx, api_client, logger.clone());
        builder.build_task(period)
    };

    let weather_handle = tokio::spawn(weather_fetcher);

    let (requests_tx, requests_rx) = channel(10);

    let mqtt_options = create_connection_options(settings.mqtt_connection);
    let eventloop = eventloop(mqtt_options, requests_rx);

    let units = settings.units;

    let temperature = Temperature::from_publishing_args(&settings.publishing);
    let pressure = Pressure::from_publishing_args(&settings.publishing);
    let humidity = Humidity::from_publishing_args(&settings.publishing);

    let publisher_task = create_mqtt_publisher(
        weather_rx,
        temperature,
        requests_tx.clone(),
        pressure,
        humidity,
        units,
        logger.clone(),
    );

    let handle_mqtt = tokio::spawn(publisher_task);

    let handle_mqtt_loop = tokio::spawn(run_mqtt_loop(eventloop, logger.clone()));

    let error_msg: Option<String>;
    tokio::select!(
        v = weather_handle => {error_msg = Some(format!("Weather fetcher finished: {:?}", v));},
        v = handle_mqtt => {error_msg = Some(format!("Publisher task finished: {:?}", v));},
        v = handle_mqtt_loop => {error_msg = Some(format!("MQTT loop finished: {:?}", v));},
    );

    match error_msg {
        None => Ok(()),
        Some(message) => anyhow::bail!("{}", message),
    }
}

fn create_connection_options(mqtt_connection: MqttConnectionArgs) -> MqttOptions {
    let mut mqtt_options = MqttOptions::new(
        mqtt_connection.mqtt_id,
        mqtt_connection.mqtt_host,
        mqtt_connection.mqtt_port.get(),
    );

    if let Some(user) = mqtt_connection.mqtt_user {
        if let Some(password) = mqtt_connection.mqtt_password {
            let u: String = user.into();
            let p: String = password.into();

            mqtt_options.set_credentials(u, p);
        }
    }

    mqtt_options
        .set_keep_alive(mqtt_connection.mqtt_keepalive)
        .set_throttle(Duration::from_millis(mqtt_connection.mqtt_throttle_ms));

    mqtt_options
}
