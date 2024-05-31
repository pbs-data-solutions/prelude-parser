use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer};

fn deserialize_empty_string_as_none_datetime<'de, D>(
    deserializer: D,
) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    if s.is_empty() {
        Ok(None)
    } else {
        // Parse the datetime with a fixed offset, then convert it to UTC
        let dt_with_offset = DateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S %z")
            .map_err(serde::de::Error::custom)?;
        Ok(Some(dt_with_offset.with_timezone(&Utc)))
    }
}

fn deserialize_empty_string_as_none<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    if s.is_empty() {
        Ok(None)
    } else {
        Ok(Some(s))
    }
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Value {
    pub by: String,
    pub by_unique_id: usize,
    pub role: String,
    pub when: DateTime<Utc>,

    #[serde(rename = "$value")]
    pub value: String,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Entry {
    pub id: usize,
    pub value: Value,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Field {
    pub name: String,

    #[serde(alias = "type")]
    pub field_type: String,

    pub data_type: String,
    pub error_code: String,
    pub when_created: DateTime<Utc>,
    pub keep_history: bool,

    #[serde(rename = "$value")]
    pub entries: Vec<Entry>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    pub name: String,

    #[serde(alias = "type")]
    pub category_type: String,

    pub highest_index: usize,

    #[serde(rename = "$value")]
    pub fields: Vec<Field>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct State {
    pub value: String,
    pub signer: String,
    pub signer_unique_id: usize,

    #[serde(deserialize_with = "deserialize_empty_string_as_none_datetime")]
    pub date_signed: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Form {
    pub name: String,
    pub last_modified: DateTime<Utc>,
    pub who_last_modified_name: String,
    pub who_last_modified_role: String,
    pub when_created: usize,
    pub has_errors: bool,
    pub has_warnings: bool,
    pub locked: bool,

    #[serde(deserialize_with = "deserialize_empty_string_as_none")]
    pub user: Option<String>,

    #[serde(deserialize_with = "deserialize_empty_string_as_none_datetime")]
    pub date_time_changed: Option<DateTime<Utc>>,
    pub form_title: String,
    pub form_index: usize,
    pub form_group: String,
    pub form_state: String,
    pub state: Option<State>,
    pub category: Category,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Patient {
    pub patient_id: String,
    pub unique_id: usize,
    pub when_created: DateTime<Utc>,
    pub creator: String,
    pub site_name: String,
    pub site_unique_id: usize,

    #[serde(deserialize_with = "deserialize_empty_string_as_none")]
    pub last_language: Option<String>,

    pub number_of_forms: usize,

    #[serde(rename = "$value")]
    pub forms: Vec<Form>,
}

/// Contains the information from the Prelude native XML.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Native {
    #[serde(rename = "$value")]
    pub patients: Vec<Patient>,
}
