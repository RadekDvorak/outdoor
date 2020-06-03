use std::future::Future;
use std::sync::Arc;

use futures_util::stream::StreamExt;
use rumq_client::{MqttEventLoop, Notification, Publish, QoS, Request};
use slog::Logger;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::time;
use tokio::time::Duration;
use uom::si::pressure;

use crate::app::publisher::{Humidity, Pressure, Temperature, Topic};
use crate::arguments::Units;
use crate::domain::current_weather::CurrentWeather;
use crate::domain::interfaces::WeatherClient;

pub async fn create_weather_fetcher<T>(
    period: Duration,
    mut channel: Sender<CurrentWeather>,
    api_client: T,
    logger: Arc<Logger>,
) -> Result<(), anyhow::Error>
where
    T: WeatherClient + 'static,
{
    let mut interval = time::interval(period);

    loop {
        interval.tick().await;

        let result = api_client.get_current_weather().await;
        match result {
            Err(e) => {
                slog::slog_error!(logger, "{:#?}", e);
            }
            Ok(v) => {
                channel.send(v).await?;
            }
        };
    }
}

pub async fn run_mqtt_loop(
    mut event_loop: MqttEventLoop,
    logger: Arc<Logger>,
) -> Result<(), anyhow::Error> {
    let mut stream = event_loop.connect().await?;

    while let Some(notification) = stream.next().await {
        match notification {
            Notification::Publish(_p) => {
                slog::slog_debug!(logger, "Publih = {:?}", _p);
            }
            Notification::Puback(_pid) => {
                slog::slog_debug!(logger, "Puback = {:?}", _pid);
            }
            Notification::Pubcomp(_pcm) => {
                slog::slog_debug!(logger, "Pubcomp = {:?}", _pcm);
            }
            Notification::Pubrec(_prc) => {
                slog::slog_debug!(logger, "Pubrec = {:?}", _prc);
            }
            Notification::Suback(_suback) => {
                slog::slog_debug!(logger, "Suback = {:?}", _suback);
            }
            Notification::Unsuback(_usa) => {
                slog::slog_debug!(logger, "Unsuback = {:?}", _usa);
            }
            Notification::Abort(error) => {
                slog::slog_debug!(logger, "Requests abort");
                return Err(error.into());
            }
        }
    }

    Ok(())
}

pub fn create_mqtt_publisher(
    mut weather_rx: Receiver<CurrentWeather>,
    temperature: Temperature,
    mut temperature_tx: Sender<Request>,
    pressure: Pressure,
    humidity: Humidity,
    units: Units,
    logger: Arc<Logger>,
) -> impl Future<Output = ()> + 'static {
    let t_temp = temperature.get_value();
    let t_pressure = pressure.get_value();
    let t_humidity = humidity.get_value();

    let mut pub_pressure_tx = temperature_tx.clone();
    let mut pub_humidity_tx = temperature_tx.clone();

    async move {
        while let Some(v) = weather_rx.recv().await {
            let temperature: f32 = units.convert_temperature(*v.get_temperature());
            let r_temp = temperature_tx.send(create_publish_request(
                format!("{0:.2}", temperature),
                &t_temp,
            ));
            let r_pressure = pub_pressure_tx.send(create_publish_request(
                format!("{0:.2}", v.get_pressure().get::<pressure::pascal>()),
                &t_pressure,
            ));
            let humidity_value: &f32 = v.get_humidity().as_ref();
            let r_humidity = pub_humidity_tx.send(create_publish_request(
                format!("{0:.1}", humidity_value),
                &t_humidity,
            ));

            let completion_status = tokio::join!(r_temp, r_pressure, r_humidity);
            slog::slog_debug!(logger, "Publisher completed with {:?}", completion_status);
        }
    }
}

fn create_publish_request(msg: String, top: &str) -> Request {
    let payload: Vec<u8> = msg.into_bytes();
    let publish = Publish::new(top, QoS::AtLeastOnce, payload);
    Request::Publish(publish)
}
