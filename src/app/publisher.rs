pub trait Topic {
    fn get_value(&self) -> String;
}

pub trait PublishingInfo {
    fn get_prefix(&self) -> &Option<String>;
    fn get_device_name(&self) -> &str;
    fn get_channel_thermometer(&self) -> &str;
    fn get_channel_barometer(&self) -> &str;
    fn get_channel_hygrometer(&self) -> &str;
}

#[derive(Debug)]
pub struct Temperature<'a> {
    prefix: &'a str,
    device: &'a str,
    channel: &'a str,
}

impl<'a> Temperature<'a> {
    pub fn new(prefix: &'a Option<String>, device: &'a str, channel: &'a str) -> Self {
        let prefixed = prefix.as_deref().unwrap_or("");

        Temperature {
            prefix: prefixed,
            device,
            channel,
        }
    }

    pub fn from_publishing_args(args: &'a dyn PublishingInfo) -> Self {
        Self::new(
            &args.get_prefix(),
            &args.get_device_name(),
            &args.get_channel_thermometer(),
        )
    }
}

impl<'a> Topic for Temperature<'a> {
    fn get_value(&self) -> String {
        format!(
            "{}node/{}/thermometer/{}/temperature",
            self.prefix, self.device, self.channel
        )
    }
}

#[derive(Debug)]
pub struct Pressure<'a> {
    prefix: &'a str,
    device: &'a str,
    channel: &'a str,
}

impl<'a> Pressure<'a> {
    pub fn new(prefix: &'a Option<String>, device: &'a str, channel: &'a str) -> Self {
        let prefixed = prefix.as_deref().unwrap_or("");
        Pressure {
            prefix: prefixed,
            device,
            channel,
        }
    }

    pub fn from_publishing_args(args: &'a dyn PublishingInfo) -> Self {
        Self::new(
            &args.get_prefix(),
            &args.get_device_name(),
            &args.get_channel_barometer(),
        )
    }
}

impl<'a> Topic for Pressure<'a> {
    fn get_value(&self) -> String {
        format!(
            "{}node/{}/barometer/{}/pressure",
            self.prefix, self.device, self.channel
        )
    }
}

#[derive(Debug)]
pub struct Humidity<'a> {
    prefix: &'a str,
    device: &'a str,
    channel: &'a str,
}

impl<'a> Humidity<'a> {
    pub fn new(prefix: &'a Option<String>, device: &'a str, channel: &'a str) -> Self {
        let prefixed = prefix.as_deref().unwrap_or("");
        Humidity {
            prefix: prefixed,
            device,
            channel,
        }
    }

    pub fn from_publishing_args(args: &'a dyn PublishingInfo) -> Self {
        Self::new(
            &args.get_prefix(),
            &args.get_device_name(),
            &args.get_channel_hygrometer(),
        )
    }
}

impl<'a> Topic for Humidity<'a> {
    fn get_value(&self) -> String {
        format!(
            "{}node/{}/hygrometer/{}/relative-humidity",
            self.prefix, self.device, self.channel
        )
    }
}
