// author: Broderick Carlin openweather=0.0.1, https://crates.io/crates/openweather
// slightly adapted

#[derive(Serialize, Deserialize, Debug)]
pub struct Coordinates {
    pub lat: f32,
    pub lon: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Main {
    pub temp: f32,
    pub temp_min: f32,
    pub temp_max: f32,
    pub pressure: f32,
    pub sea_level: Option<f32>,
    pub grnd_level: Option<f32>,
    pub humidity: f32,
    pub temp_kf: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Weather {
    pub id: u32,
    pub main: String,
    pub description: String,
    pub icon: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Clouds {
    pub all: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Wind {
    pub speed: f32,
    pub deg: Option<f32>,
    pub gust: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Rain {
    #[serde(rename = "3h")]
    pub three_h: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct System {
    pub pod: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorReport {
    pub cod: u32,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sys {
    #[serde(rename = "type", skip_deserializing)]
    pub message_type: u32,
    #[serde(skip_deserializing)]
    pub id: u32,
    #[serde(skip_deserializing)]
    pub message: Option<f32>,
    pub country: String,
    pub sunrise: u64,
    pub sunset: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherReportCurrent {
    pub coord: Coordinates,
    pub weather: Vec<Weather>,
    pub base: String,
    pub main: Main,
    pub visibility: u32,
    pub wind: Wind,
    pub clouds: Clouds,
    pub dt: u64,
    pub sys: Sys,
    pub id: u64,
    pub name: String,
    #[serde(skip_deserializing)]
    pub cod: u16,
}
