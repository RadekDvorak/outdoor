use std::future::Future;

use futures_util::stream::StreamExt;
use rumq_client::{EventLoopError, MqttEventLoop, Notification, QoS, Request};
use serde::export::Formatter;
use slog::Logger;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::time;
use tokio::time::Duration;
use uom::si::pressure;

use crate::app::publisher::{Humidity, Pressure, Temperature, Topic};
use crate::arguments::Units;
use crate::domain::current_weather::CurrentWeather;
use crate::domain::interfaces::WeatherClient;
use std::sync::Arc;

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
    let mut stream = event_loop.stream();

    while let Some(notification) = stream.next().await {
        match notification {
            Notification::Connected => {
                slog::slog_debug!(logger, "Connected");
            }
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
            Notification::RequestsDone => {
                slog::slog_debug!(logger, "Requests Done");
            }
            Notification::NetworkClosed => {
                slog::slog_debug!(logger, "Network closed");
            }
            Notification::StreamEnd(_err) => {
                return Err(MqttTaskError(_err).into());
            }
        }
    }

    Ok(())
}

#[derive(Debug)]
pub struct MqttTaskError(EventLoopError);

impl std::fmt::Display for MqttTaskError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            EventLoopError::MqttState(state_error) => {
                write!(f, "Mqtt State Error: {:?}", state_error).unwrap_or_else(|_| {
                    panic!("Failed to write error: Mqtt State Error: {:?}", state_error)
                });
            }
            EventLoopError::Timeout(elapsed) => {
                elapsed.fmt(f).unwrap_or_else(|_| {
                    panic!("Failed to write: Timeout: {:?}", elapsed.to_string())
                });
            }
            EventLoopError::Rumq(rumq_core_error) => {
                write!(f, "Rumq Error: {:?}", rumq_core_error).unwrap_or_else(|_| {
                    panic!("Failed to write: Rumq Error: {:?}", rumq_core_error)
                });
            }
            EventLoopError::Network(network_error) => {
                write!(f, "Network Error: {:?}", network_error).unwrap_or_else(|_| {
                    panic!("Failed to write: Network Error: {:?}", network_error)
                });
            }
            EventLoopError::Io(io_error) => {
                write!(f, "IO Error: {:?}", io_error)
                    .unwrap_or_else(|_| panic!("Failed to write: IO Error: {:?}", io_error));
            }
            EventLoopError::StreamDone => {
                write!(f, "Stream is done - whatever that means")
                    .unwrap_or_else(|_| panic!("Failed to write: Stream is done"));
            }
        };

        Ok(())
    }
}

impl std::error::Error for MqttTaskError {}

#[allow(clippy::too_many_arguments)]
pub fn create_mqtt_publisher(
    mut weather_rx: Receiver<CurrentWeather>,
    temperature: Temperature,
    mut temperature_tx: Sender<Request>,
    pressure: Pressure,
    mut pub_pressure_tx: Sender<Request>,
    humidity: Humidity,
    mut pub_humidity_tx: Sender<Request>,
    units: Units,
    logger: Arc<Logger>,
) -> impl Future<Output = ()> + 'static {
    let t_temp = temperature.get_value();
    let t_pressure = pressure.get_value();
    let t_humidity = humidity.get_value();

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
    let topic = top.to_owned();
    let payload: Vec<u8> = msg.into_bytes();
    let publish = rumq_client::publish(&topic, QoS::AtLeastOnce, payload);
    Request::Publish(publish)
}
