#[cfg(feature = "python")]
use chrono::{Datelike, Timelike};

use chrono::{DateTime, Utc};

use serde::{Deserialize, Deserializer};

#[cfg(feature = "python")]
use pyo3::{prelude::*, types::PyDateTime};

pub fn deserialize_empty_string_as_none_datetime<'de, D>(
    deserializer: D,
) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Deserialize::deserialize(deserializer)?;
    match s {
        Some(v) => {
            if v.is_empty() {
                Ok(None)
            } else {
                // Parse the datetime with a fixed offset, then convert it to UTC

                let dt_with_offset = if v.ends_with('Z') {
                    DateTime::parse_from_rfc3339(&v).map_err(serde::de::Error::custom)?
                } else {
                    DateTime::parse_from_str(&v, "%Y-%m-%d %H:%M:%S %z")
                        .map_err(serde::de::Error::custom)?
                };
                Ok(Some(dt_with_offset.with_timezone(&Utc)))
            }
        }
        None => Ok(None),
    }
}

pub fn deserialize_empty_string_as_none<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = Option::<String>::deserialize(deserializer)?;
    match s {
        Some(v) if v.is_empty() => Ok(None),
        Some(v) => Ok(Some(v)),
        None => Ok(None),
    }
}

pub fn default_datetime_none() -> Option<DateTime<Utc>> {
    None
}

pub fn default_string_none() -> Option<String> {
    None
}

#[cfg(feature = "python")]
pub fn to_py_datetime<'py>(
    py: Python<'py>,
    date_time: &DateTime<Utc>,
) -> PyResult<Bound<'py, PyDateTime>> {
    let py_datetime = PyDateTime::new(
        py,
        date_time.year(),
        date_time.month() as u8,
        date_time.day() as u8,
        date_time.hour() as u8,
        date_time.minute() as u8,
        date_time.second() as u8,
        date_time.timestamp_subsec_micros(),
        None,
    )?;
    Ok(py_datetime)
}

#[cfg(feature = "python")]
pub fn to_py_datetime_option<'py>(
    py: Python<'py>,
    date_time: &Option<DateTime<Utc>>,
) -> PyResult<Option<Bound<'py, PyDateTime>>> {
    if let Some(d) = date_time {
        let py_datetime = Some(PyDateTime::new(
            py,
            d.year(),
            d.month() as u8,
            d.day() as u8,
            d.hour() as u8,
            d.minute() as u8,
            d.second() as u8,
            d.timestamp_subsec_micros(),
            None,
        )?);
        Ok(py_datetime)
    } else {
        Ok(None)
    }
}
