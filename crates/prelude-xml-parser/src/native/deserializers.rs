#[cfg(feature = "python")]
use chrono::{Datelike, Timelike};

use std::borrow::Cow;
use std::str::from_utf8;

use chrono::{DateTime, FixedOffset, NaiveDate, TimeZone, Utc};

use quick_xml::events::BytesStart;
use serde::{Deserialize, Deserializer};

/// Walk an element's attributes once, handing each raw key and its borrowed value to `visit`.
///
/// Matching on the raw key bytes lets callers pull out the attributes they care about without
/// building an intermediate map, which matters because this runs for every element in the file.
pub(crate) fn visit_attributes<'a>(
    e: &'a BytesStart<'a>,
    mut visit: impl FnMut(&'a [u8], &'a str),
) -> Result<(), crate::errors::Error> {
    for attr in e.attributes() {
        let attr = attr.map_err(|e| {
            crate::errors::Error::ParsingError(quick_xml::de::DeError::Custom(format!(
                "Attribute error: {}",
                e
            )))
        })?;

        let Cow::Borrowed(value) = attr.value else {
            return Err(crate::errors::Error::ParsingError(
                quick_xml::de::DeError::Custom(
                    "Attribute value was not borrowed from the source".to_string(),
                ),
            ));
        };

        let Ok(value) = from_utf8(value) else {
            return Err(crate::errors::Error::ParsingError(
                quick_xml::de::DeError::Custom("Attribute was not valid UTF-8".to_string()),
            ));
        };

        visit(attr.key.into_inner(), value);
    }

    Ok(())
}

/// Parse an attribute that carries a datetime, treating an empty string as absent and an
/// unparsable value as absent.
pub(crate) fn optional_datetime(s: &str) -> Option<DateTime<Utc>> {
    if s.is_empty() {
        None
    } else {
        parse_datetime(s).ok()
    }
}

/// Parse an attribute that carries a datetime, treating an empty string as absent but reporting an
/// unparsable value as an error.
pub(crate) fn checked_datetime(s: &str) -> Result<Option<DateTime<Utc>>, crate::errors::Error> {
    if s.is_empty() {
        Ok(None)
    } else {
        parse_datetime(s).map(Some)
    }
}

/// Take an attribute that must be present, reporting its absence as an error.
pub(crate) fn required_attribute(
    value: Option<&str>,
    name: &str,
) -> Result<String, crate::errors::Error> {
    match value {
        Some(value) => Ok(value.to_string()),
        None => Err(crate::errors::Error::ParsingError(
            quick_xml::de::DeError::Custom(format!("Missing {}", name)),
        )),
    }
}

/// Convert an attribute into an owned `String`, treating an empty string as absent.
pub(crate) fn optional_string(s: &str) -> Option<String> {
    if s.is_empty() {
        None
    } else {
        Some(s.to_string())
    }
}

fn two_digits(tens: u8, ones: u8) -> Option<u32> {
    if !tens.is_ascii_digit() || !ones.is_ascii_digit() {
        return None;
    }

    Some(u32::from(tens - b'0') * 10 + u32::from(ones - b'0'))
}

/// Fast path for the only datetime shape Prelude emits: `YYYY-MM-DD HH:MM:SS ±HHMM`.
///
/// Returns `None` when the input does not match that exact shape so the caller can fall back to
/// the general chrono parsers.
fn parse_prelude_datetime(s: &str) -> Option<DateTime<Utc>> {
    let Ok(b) = <&[u8; 25]>::try_from(s.as_bytes()) else {
        return None;
    };

    if b[4] != b'-' || b[7] != b'-' || b[10] != b' ' || b[13] != b':' || b[16] != b':' {
        return None;
    }

    let offset_sign = match (b[19], b[20]) {
        (b' ', b'+') => 1i32,
        (b' ', b'-') => -1i32,
        _ => return None,
    };

    let year = two_digits(b[0], b[1])? * 100 + two_digits(b[2], b[3])?;
    let month = two_digits(b[5], b[6])?;
    let day = two_digits(b[8], b[9])?;
    let hour = two_digits(b[11], b[12])?;
    let minute = two_digits(b[14], b[15])?;
    let second = two_digits(b[17], b[18])?;
    let offset_hours = two_digits(b[21], b[22])?;
    let offset_minutes = two_digits(b[23], b[24])?;

    let offset_seconds = offset_sign * (offset_hours * 3600 + offset_minutes * 60) as i32;
    let offset = FixedOffset::east_opt(offset_seconds)?;

    let naive =
        NaiveDate::from_ymd_opt(year as i32, month, day)?.and_hms_opt(hour, minute, second)?;

    Some(offset.from_local_datetime(&naive).single()?.to_utc())
}

/// Parse a Prelude datetime attribute, trying the common fixed-shape fast path before falling back
/// to the general chrono format parsers.
pub(crate) fn parse_datetime(s: &str) -> Result<DateTime<Utc>, crate::errors::Error> {
    if let Some(dt) = parse_prelude_datetime(s) {
        Ok(dt)
    } else if let Ok(dt) = DateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S %z") {
        Ok(dt.with_timezone(&Utc))
    } else if let Ok(dt) = DateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S%z") {
        Ok(dt.with_timezone(&Utc))
    } else if let Ok(dt) = DateTime::parse_from_rfc3339(s) {
        Ok(dt.with_timezone(&Utc))
    } else {
        Err(crate::errors::Error::ParsingError(
            quick_xml::de::DeError::Custom(format!("Invalid datetime format: {}", s)),
        ))
    }
}

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
