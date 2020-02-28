extern crate anyhow;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate uom;
extern crate url;

use std::num::NonZeroU16;
use std::time::Duration;

use rumq_client;
use rumq_client::eventloop;
use rumq_client::MqttOptions;
use tokio::sync::mpsc::channel;

use domain::current_weather;
use location_specifier::LocationSpecifier;

use crate::app::publisher::{Humidity, Pressure, Temperature};
use crate::app::tasks::*;
use crate::arguments::{Password, User};
use crate::weather_client::OpenWeatherMapClientBuilder;

mod app;
mod arguments;
mod domain;
mod location_specifier;
mod weather_client;
mod weather_types;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let settings: arguments::Args = structopt::StructOpt::from_args();

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

    let handle_api = tokio::spawn(create_weather_fetcher(period, weather_tx, api_client));

    let (requests_tx, requests_rx) = channel(10);

    let mqtt_host = settings.mqtt_host;
    let mqtt_port = settings.mqtt_port;
    let mqtt_credentials = match (settings.mqtt_user, settings.mqtt_password) {
        (Some(u), Some(p)) => Some((u, p)),
        _ => None,
    };
    let mqttoptions = create_connection_options(mqtt_host, mqtt_port, mqtt_credentials);
    let eventloop = eventloop(mqttoptions, requests_rx);

    let units = settings.units;

    let temperature = Temperature::new(
        &settings.topic_prefix,
        &settings.device_name,
        &settings.channel_thermometer,
    );

    let pressure = Pressure::new(
        &settings.topic_prefix,
        &settings.device_name,
        &settings.channel_barometer,
    );

    let humidity = Humidity::new(
        &settings.topic_prefix,
        &settings.device_name,
        &settings.channel_hygrometer,
    );

    let publisher_task = create_mqtt_publisher(
        weather_rx,
        temperature,
        requests_tx.clone(),
        pressure,
        requests_tx.clone(),
        humidity,
        requests_tx.clone(),
        units,
    );

    let handle_mqtt = tokio::spawn(publisher_task);

    let handle_mqtt_loop = tokio::spawn(run_mqtt_loop(eventloop));

    let error_msg: Option<String>;
    tokio::select!(
        v = handle_api => {error_msg = Some(format!("Weather fetcher finished: {:?}", v));},
        v = handle_mqtt => {error_msg = Some(format!("Publisher task finished: {:?}", v));},
        v = handle_mqtt_loop => {error_msg = Some(format!("MQTT loop finished: {:?}", v));},
    );

    match error_msg {
        None => Ok(()),
        Some(message) => anyhow::bail!("{}", message),
    }
}

fn create_connection_options(
    host: String,
    port: NonZeroU16,
    credentials: Option<(User, Password)>,
) -> MqttOptions {
    let mut mqtt_options = MqttOptions::new("weather", host, port.get());

    if let Some((user, password)) = credentials {
        let u: String = user.into();
        let p: String = password.into();

        mqtt_options.set_credentials(u, p);
    }
    mqtt_options
        .set_keep_alive(30)
        .set_throttle(Duration::from_secs(1));

    mqtt_options
}
