// author: Broderick Carlin openweather=0.0.1, https://crates.io/crates/openweather

#[derive(Debug)]
#[allow(dead_code)]
pub enum LocationSpecifier<'a> {
    CityAndCountryName {
        city: &'a str,
        country: &'a str,
    },
    CityId(&'a str),
    Coordinates {
        lat: f32,
        lon: f32,
    },
    ZipCode {
        zip: &'a str,
        country: &'a str,
    },

    // The following location specifiers are used to specify multiple cities or a region
    BoundingBox {
        lon_left: f32,
        lat_bottom: f32,
        lon_right: f32,
        lat_top: f32,
        zoom: f32,
    },
    Circle {
        lat: f32,
        lon: f32,
        count: u16,
    },
    CityIds(Vec<&'a str>),
}

impl<'a> LocationSpecifier<'a> {
    pub fn format(&'a self) -> Vec<(String, String)> {
        match &self {
            LocationSpecifier::CityAndCountryName { city, country } => {
                return if country.is_empty() {
                    vec![("q".to_string(), (*city).to_string())]
                } else {
                    vec![("q".to_string(), format!("{},{}", city, country))]
                }
            }
            LocationSpecifier::CityId(id) => {
                return vec![("id".to_string(), (*id).to_string())];
            }
            LocationSpecifier::Coordinates { lat, lon } => {
                return vec![
                    ("lat".to_string(), format!("{}", lat)),
                    ("lon".to_string(), format!("{}", lon)),
                ];
            }
            LocationSpecifier::ZipCode { zip, country } => {
                return if country.is_empty() {
                    vec![("zip".to_string(), (*zip).to_string())]
                } else {
                    vec![("zip".to_string(), format!("{},{}", zip, country))]
                }
            }
            LocationSpecifier::BoundingBox {
                lon_left,
                lat_bottom,
                lon_right,
                lat_top,
                zoom,
            } => {
                return vec![(
                    "bbox".to_string(),
                    format!(
                        "{},{},{},{},{}",
                        lon_left, lat_bottom, lon_right, lat_top, zoom
                    ),
                )];
            }
            LocationSpecifier::Circle { lat, lon, count } => {
                return vec![
                    ("lat".to_string(), format!("{}", lat)),
                    ("lon".to_string(), format!("{}", lon)),
                    ("cnt".to_string(), format!("{}", count)),
                ];
            }
            LocationSpecifier::CityIds(ids) => {
                let mut locations: String = "".to_string();
                for loc in ids {
                    locations += loc;
                }
                return vec![("id".to_string(), locations)];
            }
        }
    }
}
