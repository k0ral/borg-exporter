use chrono::{DateTime, Utc, TimeZone};
use serde::{Deserialize, Serializer, Deserializer};

const FORMAT: &'static str = "%Y-%m-%dT%H:%M:%S%.6f";

pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where S: Serializer {
    let s = format!("{}", date.format(FORMAT));
    serializer.serialize_str(&s)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where D: Deserializer<'de> {
    let s = String::deserialize(deserializer)?;
    Utc.datetime_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
}

