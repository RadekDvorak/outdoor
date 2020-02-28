use crate::domain::current_weather::CurrentWeather;
use async_trait::async_trait;

#[async_trait]
pub trait WeatherClient {
    async fn get_current_weather(&self) -> Result<CurrentWeather, anyhow::Error>;
}
