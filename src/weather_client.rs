use url::Url;

use async_trait::async_trait;

use crate::domain::current_weather::CurrentWeather;
use crate::domain::interfaces::WeatherClient;
use crate::location_specifier::LocationSpecifier;
use crate::weather_types::{ErrorReport, WeatherReportCurrent};

pub struct OpenWeatherMapClient {
    url: Url,
    http_client: reqwest::Client,
}

#[async_trait]
impl WeatherClient for OpenWeatherMapClient {
    async fn get_current_weather(&self) -> Result<CurrentWeather, anyhow::Error> {
        let body = self
            .http_client
            .get(self.url.as_str())
            .send()
            .await?
            .text()
            .await?;

        serde_json::from_str::<WeatherReportCurrent>(body.as_ref())
            .map(|v| -> CurrentWeather { v.into() })
            .map_err(|bad_error| -> String {
                let parsed_error = serde_json::from_str::<ErrorReport>(body.as_ref());
                match parsed_error {
                    Ok(parsed_e) => format!(
                        "Error code {} with message \"{}\"",
                        parsed_e.cod, parsed_e.message
                    ),
                    Err(_) => bad_error.to_string(),
                }
            })
            .map_err(anyhow::Error::msg)
    }
}

#[derive(Debug)]
pub struct OpenWeatherMapClientBuilder<'a, T>
where
    T: Into<String>,
{
    location_specifier: LocationSpecifier<'a>,
    api_key: T,
    base_url: Url,
}

impl<'a, T> OpenWeatherMapClientBuilder<'a, T>
where
    T: Into<String>,
{
    pub fn new(location_specifier: LocationSpecifier<'a>, api_key: T) -> Self {
        let default_base_url = "https://api.openweathermap.org/data/2.5/";
        let base_url: Url = Url::parse(default_base_url)
            .unwrap_or_else(|_| panic!("Broken default hardcoded base URL {}", &default_base_url));

        OpenWeatherMapClientBuilder {
            location_specifier,
            api_key,
            base_url,
        }
    }

    #[allow(dead_code)]
    pub fn with_base_url(&mut self, url: Url) {
        self.base_url = url;
    }

    pub fn build(self) -> Result<OpenWeatherMapClient, anyhow::Error> {
        let cb = reqwest::ClientBuilder::new();

        let client = OpenWeatherMapClient {
            url: Self::get_current_weather_url(
                &self.location_specifier,
                self.api_key,
                self.base_url,
            )?,
            http_client: cb.build()?,
        };

        Ok(client)
    }

    fn get_current_weather_url(
        location: &LocationSpecifier,
        key: T,
        base_url: Url,
    ) -> Result<Url, anyhow::Error> {
        let mut base = base_url.into_string();
        let mut params = location.format();

        base.push_str("weather");
        params.push(("APPID".to_string(), key.into()));

        let url = Url::parse_with_params(&base, params)?;
        Ok(url)
    }
}
