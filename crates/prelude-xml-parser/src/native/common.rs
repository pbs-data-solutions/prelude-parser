use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[cfg(feature = "python")]
use pyo3::{
    prelude::*,
    types::{PyDateTime, PyDict},
};

use crate::native::deserializers::{
    default_datetime_none, default_string_none, deserialize_empty_string_as_none,
    deserialize_empty_string_as_none_datetime,
};

#[cfg(feature = "python")]
use crate::native::deserializers::{to_py_datetime, to_py_datetime_option};

#[cfg(not(feature = "python"))]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Value {
    #[serde(rename = "by")]
    #[serde(alias = "@by")]
    #[serde(alias = "by")]
    pub by: String,

    #[serde(rename = "byUniqueId")]
    #[serde(alias = "@byUniqueId")]
    #[serde(alias = "byUniqueId")]
    #[serde(
        default = "default_string_none",
        deserialize_with = "deserialize_empty_string_as_none"
    )]
    pub by_unique_id: Option<String>,
    #[serde(rename = "role")]
    #[serde(alias = "@role")]
    #[serde(alias = "role")]
    pub role: String,
    #[serde(rename = "when")]
    #[serde(alias = "@when")]
    #[serde(alias = "when")]
    pub when: Option<DateTime<Utc>>,

    #[serde(rename = "value")]
    #[serde(alias = "$text")]
    #[serde(alias = "#text")]
    #[serde(alias = "value")]
    #[serde(default)]
    pub value: String,
}

#[cfg(feature = "python")]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[pyclass]
pub struct Value {
    #[serde(rename = "by")]
    #[serde(alias = "@by")]
    #[serde(alias = "by")]
    pub by: String,

    #[serde(rename = "byUniqueId")]
    #[serde(alias = "@byUniqueId")]
    #[serde(alias = "byUniqueId")]
    #[serde(
        default = "default_string_none",
        deserialize_with = "deserialize_empty_string_as_none"
    )]
    pub by_unique_id: Option<String>,
    #[serde(rename = "role")]
    #[serde(alias = "@role")]
    #[serde(alias = "role")]
    pub role: String,
    #[serde(rename = "when")]
    #[serde(alias = "@when")]
    #[serde(alias = "when")]
    pub when: Option<DateTime<Utc>>,

    #[serde(rename = "value")]
    #[serde(alias = "$text")]
    #[serde(alias = "#text")]
    #[serde(alias = "value")]
    #[serde(default)]
    pub value: String,
}

#[cfg(feature = "python")]
#[pymethods]
impl Value {
    #[getter]
    fn by(&self) -> PyResult<String> {
        Ok(self.by.clone())
    }

    #[getter]
    fn by_unique_id(&self) -> PyResult<Option<String>> {
        Ok(self.by_unique_id.clone())
    }

    #[getter]
    fn role(&self) -> PyResult<String> {
        Ok(self.role.clone())
    }

    #[getter]
    fn when<'py>(&self, py: Python<'py>) -> PyResult<Option<Bound<'py, PyDateTime>>> {
        to_py_datetime_option(py, &self.when)
    }

    #[getter]
    fn value(&self) -> PyResult<String> {
        Ok(self.value.clone())
    }

    pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new(py);
        dict.set_item("by", &self.by)?;
        dict.set_item("by_unique_id", &self.by_unique_id)?;
        dict.set_item("role", &self.role)?;
        dict.set_item("when", to_py_datetime_option(py, &self.when)?)?;
        dict.set_item("value", &self.value)?;

        Ok(dict)
    }
}

#[cfg(not(feature = "python"))]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Reason {
    #[serde(rename = "by")]
    #[serde(alias = "@by")]
    #[serde(alias = "by")]
    pub by: String,

    #[serde(rename = "byUniqueId")]
    #[serde(alias = "@byUniqueId")]
    #[serde(alias = "byUniqueId")]
    #[serde(
        default = "default_string_none",
        deserialize_with = "deserialize_empty_string_as_none"
    )]
    pub by_unique_id: Option<String>,

    #[serde(rename = "role")]
    #[serde(alias = "@role")]
    #[serde(alias = "role")]
    pub role: String,
    #[serde(rename = "when")]
    #[serde(alias = "@when")]
    #[serde(alias = "when")]
    pub when: Option<DateTime<Utc>>,

    #[serde(rename = "value")]
    #[serde(alias = "$text")]
    #[serde(alias = "#text")]
    #[serde(alias = "value")]
    #[serde(default)]
    pub value: String,
}

#[cfg(feature = "python")]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[pyclass]
pub struct Reason {
    #[serde(rename = "by")]
    #[serde(alias = "@by")]
    #[serde(alias = "by")]
    pub by: String,

    #[serde(rename = "byUniqueId")]
    #[serde(alias = "@byUniqueId")]
    #[serde(alias = "byUniqueId")]
    #[serde(
        default = "default_string_none",
        deserialize_with = "deserialize_empty_string_as_none"
    )]
    pub by_unique_id: Option<String>,

    #[serde(rename = "role")]
    #[serde(alias = "@role")]
    #[serde(alias = "role")]
    pub role: String,
    #[serde(rename = "when")]
    #[serde(alias = "@when")]
    #[serde(alias = "when")]
    pub when: Option<DateTime<Utc>>,

    #[serde(rename = "value")]
    #[serde(alias = "$text")]
    #[serde(alias = "#text")]
    #[serde(alias = "value")]
    #[serde(default)]
    pub value: String,
}

#[cfg(feature = "python")]
#[pymethods]
impl Reason {
    #[getter]
    fn by(&self) -> PyResult<String> {
        Ok(self.by.clone())
    }

    #[getter]
    fn by_unique_id(&self) -> PyResult<Option<String>> {
        Ok(self.by_unique_id.clone())
    }

    #[getter]
    fn role(&self) -> PyResult<String> {
        Ok(self.role.clone())
    }

    #[getter]
    fn when<'py>(&self, py: Python<'py>) -> PyResult<Option<Bound<'py, PyDateTime>>> {
        to_py_datetime_option(py, &self.when)
    }

    #[getter]
    fn value(&self) -> PyResult<String> {
        Ok(self.value.clone())
    }

    pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new(py);
        dict.set_item("by", &self.by)?;
        dict.set_item("by_unique_id", &self.by_unique_id)?;
        dict.set_item("role", &self.role)?;
        dict.set_item("when", to_py_datetime_option(py, &self.when)?)?;
        dict.set_item("value", &self.value)?;

        Ok(dict)
    }
}

#[cfg(not(feature = "python"))]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Entry {
    #[serde(rename = "entryId")]
    #[serde(alias = "@id")]
    #[serde(alias = "entryId")]
    pub entry_id: String,
    pub value: Option<Value>,
    pub reason: Option<Reason>,
}

#[cfg(feature = "python")]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[pyclass(get_all)]
pub struct Entry {
    #[serde(rename = "entryId")]
    #[serde(alias = "@id")]
    #[serde(alias = "entryId")]
    pub entry_id: String,
    pub value: Option<Value>,
    pub reason: Option<Reason>,
}

#[cfg(feature = "python")]
#[pymethods]
impl Entry {
    #[getter]
    fn entry_id(&self) -> PyResult<String> {
        Ok(self.entry_id.clone())
    }

    #[getter]
    fn value(&self) -> PyResult<Option<Value>> {
        Ok(self.value.clone())
    }

    #[getter]
    fn reason(&self) -> PyResult<Option<Reason>> {
        Ok(self.reason.clone())
    }

    pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new(py);
        dict.set_item("entry_id", &self.entry_id)?;
        if let Some(value) = &self.value {
            dict.set_item("value", value.to_dict(py)?)?;
        } else {
            dict.set_item("value", py.None())?;
        }
        if let Some(reason) = &self.reason {
            dict.set_item("reason", reason.to_dict(py)?)?;
        } else {
            dict.set_item("reason", py.None())?;
        }

        Ok(dict)
    }
}

#[cfg(not(feature = "python"))]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Comment {
    #[serde(rename = "commentId")]
    #[serde(alias = "@id")]
    #[serde(alias = "commentId")]
    pub comment_id: String,
    pub value: Option<Value>,
}

#[cfg(feature = "python")]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[pyclass(get_all)]
pub struct Comment {
    #[serde(rename = "commentId")]
    #[serde(alias = "@id")]
    #[serde(alias = "commentId")]
    pub comment_id: String,
    pub value: Option<Value>,
}

#[cfg(feature = "python")]
#[pymethods]
impl Comment {
    #[getter]
    fn comment_id(&self) -> PyResult<String> {
        Ok(self.comment_id.clone())
    }

    #[getter]
    fn value(&self) -> PyResult<Option<Value>> {
        Ok(self.value.clone())
    }

    pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new(py);
        dict.set_item("comment_id", &self.comment_id)?;
        if let Some(value) = &self.value {
            dict.set_item("value", value.to_dict(py)?)?;
        } else {
            dict.set_item("value", py.None())?;
        }

        Ok(dict)
    }
}

#[cfg(not(feature = "python"))]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Field {
    #[serde(rename = "name")]
    #[serde(alias = "@name")]
    #[serde(alias = "name")]
    pub name: String,

    #[serde(rename = "fieldType")]
    #[serde(alias = "@type")]
    #[serde(alias = "fieldType")]
    pub field_type: String,

    #[serde(rename = "dataType")]
    #[serde(alias = "@dataType")]
    #[serde(alias = "dataType")]
    #[serde(
        default = "default_string_none",
        deserialize_with = "deserialize_empty_string_as_none"
    )]
    pub data_type: Option<String>,
    #[serde(rename = "errorCode")]
    #[serde(alias = "@errorCode")]
    #[serde(alias = "errorCode")]
    pub error_code: String,
    #[serde(rename = "whenCreated")]
    #[serde(alias = "@whenCreated")]
    #[serde(alias = "whenCreated")]
    pub when_created: Option<DateTime<Utc>>,
    #[serde(rename = "keepHistory")]
    #[serde(alias = "@keepHistory")]
    #[serde(alias = "keepHistory")]
    pub keep_history: bool,

    #[serde(alias = "entry")]
    pub entries: Option<Vec<Entry>>,

    #[serde(alias = "comment")]
    pub comments: Option<Vec<Comment>>,
}

#[cfg(feature = "python")]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[pyclass]
pub struct Field {
    #[serde(rename = "name")]
    #[serde(alias = "@name")]
    #[serde(alias = "name")]
    pub name: String,

    #[serde(rename = "fieldType")]
    #[serde(alias = "@type")]
    #[serde(alias = "fieldType")]
    pub field_type: String,

    #[serde(rename = "dataType")]
    #[serde(alias = "@dataType")]
    #[serde(alias = "dataType")]
    #[serde(
        default = "default_string_none",
        deserialize_with = "deserialize_empty_string_as_none"
    )]
    pub data_type: Option<String>,

    #[serde(rename = "errorCode")]
    #[serde(alias = "@errorCode")]
    #[serde(alias = "errorCode")]
    pub error_code: String,
    #[serde(rename = "whenCreated")]
    #[serde(alias = "@whenCreated")]
    #[serde(alias = "whenCreated")]
    pub when_created: Option<DateTime<Utc>>,
    #[serde(rename = "keepHistory")]
    #[serde(alias = "@keepHistory")]
    #[serde(alias = "keepHistory")]
    pub keep_history: bool,

    #[serde(alias = "entry")]
    pub entries: Option<Vec<Entry>>,

    #[serde(alias = "comment")]
    pub comments: Option<Vec<Comment>>,
}

#[cfg(feature = "python")]
#[pymethods]
impl Field {
    #[getter]
    fn name(&self) -> PyResult<String> {
        Ok(self.name.clone())
    }

    #[getter]
    fn field_type(&self) -> PyResult<String> {
        Ok(self.field_type.clone())
    }

    #[getter]
    fn data_type(&self) -> PyResult<Option<String>> {
        Ok(self.data_type.clone())
    }

    #[getter]
    fn error_code(&self) -> PyResult<String> {
        Ok(self.error_code.clone())
    }

    #[getter]
    fn when_created<'py>(&self, py: Python<'py>) -> PyResult<Option<Bound<'py, PyDateTime>>> {
        self.when_created
            .as_ref()
            .map(|dt| to_py_datetime(py, dt))
            .transpose()
    }

    #[getter]
    fn keep_history(&self) -> PyResult<bool> {
        Ok(self.keep_history)
    }

    #[getter]
    fn entries(&self) -> PyResult<Option<Vec<Entry>>> {
        Ok(self.entries.clone())
    }

    #[getter]
    fn comments(&self) -> PyResult<Option<Vec<Comment>>> {
        Ok(self.comments.clone())
    }

    pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new(py);
        dict.set_item("name", &self.name)?;
        dict.set_item("field_type", &self.field_type)?;
        dict.set_item("data_type", &self.data_type)?;
        dict.set_item("error_code", &self.error_code)?;
        dict.set_item(
            "when_created",
            self.when_created
                .as_ref()
                .map(|dt| to_py_datetime(py, dt))
                .transpose()?,
        )?;
        dict.set_item("keep_history", self.keep_history)?;

        let mut entry_dicts = Vec::new();
        if let Some(entries) = &self.entries {
            for entry in entries {
                let entry_dict = entry.to_dict(py)?;
                entry_dicts.push(entry_dict);
            }
            dict.set_item("entries", entry_dicts)?;
        } else {
            dict.set_item("entries", py.None())?;
        }

        let mut comment_dicts = Vec::new();
        if let Some(comments) = &self.comments {
            for comment in comments {
                let comment_dict = comment.to_dict(py)?;
                comment_dicts.push(comment_dict);
            }
            dict.set_item("comments", comment_dicts)?;
        } else {
            dict.set_item("comments", py.None())?;
        }

        Ok(dict)
    }
}

#[cfg(not(feature = "python"))]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Category {
    #[serde(rename = "name")]
    #[serde(alias = "@name")]
    #[serde(alias = "name")]
    pub name: String,

    #[serde(rename = "categoryType")]
    #[serde(alias = "@type")]
    #[serde(alias = "categoryType")]
    pub category_type: String,

    #[serde(rename = "highestIndex")]
    #[serde(alias = "@highestIndex")]
    #[serde(alias = "highestIndex")]
    pub highest_index: usize,

    #[serde(alias = "field")]
    pub fields: Option<Vec<Field>>,
}

#[cfg(feature = "python")]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[pyclass(get_all)]
pub struct Category {
    #[serde(rename = "name")]
    #[serde(alias = "@name")]
    #[serde(alias = "name")]
    pub name: String,

    #[serde(rename = "categoryType")]
    #[serde(alias = "@type")]
    #[serde(alias = "categoryType")]
    pub category_type: String,

    #[serde(rename = "highestIndex")]
    #[serde(alias = "@highestIndex")]
    #[serde(alias = "highestIndex")]
    pub highest_index: usize,

    #[serde(alias = "field")]
    pub fields: Option<Vec<Field>>,
}

#[cfg(feature = "python")]
#[pymethods]
impl Category {
    #[getter]
    fn name(&self) -> PyResult<String> {
        Ok(self.name.clone())
    }

    #[getter]
    fn category_type(&self) -> PyResult<String> {
        Ok(self.category_type.clone())
    }

    #[getter]
    fn highest_index(&self) -> PyResult<usize> {
        Ok(self.highest_index)
    }

    #[getter]
    fn fields(&self) -> PyResult<Option<Vec<Field>>> {
        Ok(self.fields.clone())
    }

    pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new(py);
        dict.set_item("name", &self.name)?;
        dict.set_item("category_type", &self.category_type)?;
        dict.set_item("highest_index", self.highest_index)?;

        let mut field_dicts = Vec::new();
        if let Some(fields) = &self.fields {
            for field in fields {
                let field_dict = field.to_dict(py)?;
                field_dicts.push(field_dict);
            }
            dict.set_item("fields", field_dicts)?;
        } else {
            dict.set_item("fields", py.None())?;
        }

        Ok(dict)
    }
}

impl Form {
    pub fn from_attributes(
        attrs: std::collections::HashMap<String, String>,
    ) -> Result<Self, crate::errors::Error> {
        let name = attrs.get("name").cloned().unwrap_or_default();

        let last_modified = if let Some(lm) = attrs.get("lastModified") {
            if lm.is_empty() {
                None
            } else {
                parse_datetime_internal(lm).ok()
            }
        } else {
            None
        };

        let who_last_modified_name = attrs
            .get("whoLastModifiedName")
            .filter(|s| !s.is_empty())
            .cloned();
        let who_last_modified_role = attrs
            .get("whoLastModifiedRole")
            .filter(|s| !s.is_empty())
            .cloned();

        let when_created = attrs
            .get("whenCreated")
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);

        let has_errors = attrs.get("hasErrors").map(|s| s == "true").unwrap_or(false);

        let has_warnings = attrs
            .get("hasWarnings")
            .map(|s| s == "true")
            .unwrap_or(false);

        let locked = attrs.get("locked").map(|s| s == "true").unwrap_or(false);

        let user = attrs.get("user").filter(|s| !s.is_empty()).cloned();

        let date_time_changed = if let Some(dtc) = attrs.get("dateTimeChanged") {
            if dtc.is_empty() {
                None
            } else {
                parse_datetime_internal(dtc).ok()
            }
        } else {
            None
        };

        let form_title = attrs.get("formTitle").cloned().unwrap_or_default();

        let form_index = attrs
            .get("formIndex")
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);

        let form_group = attrs.get("formGroup").filter(|s| !s.is_empty()).cloned();
        let form_state = attrs.get("formState").cloned().unwrap_or_default();

        Ok(Form {
            name,
            last_modified,
            who_last_modified_name,
            who_last_modified_role,
            when_created,
            has_errors,
            has_warnings,
            locked,
            user,
            date_time_changed,
            form_title,
            form_index,
            form_group,
            form_state,
            states: None,
            categories: None,
        })
    }
}

fn parse_datetime_internal(s: &str) -> Result<DateTime<Utc>, crate::errors::Error> {
    if let Ok(dt) = chrono::DateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S %z") {
        Ok(dt.with_timezone(&Utc))
    } else if let Ok(dt) = chrono::DateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S%z") {
        Ok(dt.with_timezone(&Utc))
    } else if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(s) {
        Ok(dt.with_timezone(&Utc))
    } else {
        Err(crate::errors::Error::ParsingError(
            quick_xml::de::DeError::Custom(format!("Invalid datetime format: {}", s)),
        ))
    }
}

#[cfg(not(feature = "python"))]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct State {
    #[serde(rename = "value")]
    #[serde(alias = "@value")]
    #[serde(alias = "value")]
    pub value: String,
    #[serde(rename = "signer")]
    #[serde(alias = "@signer")]
    #[serde(alias = "signer")]
    pub signer: String,
    #[serde(rename = "signerUniqueId")]
    #[serde(alias = "@signerUniqueId")]
    #[serde(alias = "signerUniqueId")]
    pub signer_unique_id: String,

    #[serde(rename = "dateSigned")]
    #[serde(alias = "@dateSigned")]
    #[serde(alias = "dateSigned")]
    #[serde(
        default = "default_datetime_none",
        deserialize_with = "deserialize_empty_string_as_none_datetime"
    )]
    pub date_signed: Option<DateTime<Utc>>,
}

#[cfg(feature = "python")]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[pyclass]
pub struct State {
    #[serde(rename = "value")]
    #[serde(alias = "@value")]
    #[serde(alias = "value")]
    pub value: String,
    #[serde(rename = "signer")]
    #[serde(alias = "@signer")]
    #[serde(alias = "signer")]
    pub signer: String,
    #[serde(rename = "signerUniqueId")]
    #[serde(alias = "@signerUniqueId")]
    #[serde(alias = "signerUniqueId")]
    pub signer_unique_id: String,

    #[serde(rename = "dateSigned")]
    #[serde(alias = "@dateSigned")]
    #[serde(alias = "dateSigned")]
    #[serde(
        default = "default_datetime_none",
        deserialize_with = "deserialize_empty_string_as_none_datetime"
    )]
    pub date_signed: Option<DateTime<Utc>>,
}

#[cfg(feature = "python")]
#[pymethods]
impl State {
    #[getter]
    fn value(&self) -> PyResult<String> {
        Ok(self.value.clone())
    }

    #[getter]
    fn signer(&self) -> PyResult<String> {
        Ok(self.signer.clone())
    }

    #[getter]
    fn signer_unique_id(&self) -> PyResult<String> {
        Ok(self.signer_unique_id.clone())
    }

    #[getter]
    fn date_signed<'py>(&self, py: Python<'py>) -> PyResult<Option<Bound<'py, PyDateTime>>> {
        to_py_datetime_option(py, &self.date_signed)
    }

    pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new(py);
        dict.set_item("value", &self.value)?;
        dict.set_item("signer", &self.signer)?;
        dict.set_item("signer_unique_id", &self.signer_unique_id)?;
        dict.set_item("date_signed", to_py_datetime_option(py, &self.date_signed)?)?;

        Ok(dict)
    }
}

#[cfg(not(feature = "python"))]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Form {
    #[serde(rename = "name")]
    #[serde(alias = "@name")]
    #[serde(alias = "name")]
    pub name: String,

    #[serde(rename = "lastModified")]
    #[serde(alias = "@lastModified")]
    #[serde(alias = "lastModified")]
    #[serde(
        default = "default_datetime_none",
        deserialize_with = "deserialize_empty_string_as_none_datetime"
    )]
    pub last_modified: Option<DateTime<Utc>>,

    #[serde(rename = "whoLastModifiedName")]
    #[serde(alias = "@whoLastModifiedName")]
    #[serde(alias = "whoLastModifiedName")]
    #[serde(
        default = "default_string_none",
        deserialize_with = "deserialize_empty_string_as_none"
    )]
    pub who_last_modified_name: Option<String>,

    #[serde(rename = "whoLastModifiedRole")]
    #[serde(alias = "@whoLastModifiedRole")]
    #[serde(alias = "whoLastModifiedRole")]
    #[serde(
        default = "default_string_none",
        deserialize_with = "deserialize_empty_string_as_none"
    )]
    pub who_last_modified_role: Option<String>,

    #[serde(rename = "whenCreated")]
    #[serde(alias = "@whenCreated")]
    #[serde(alias = "whenCreated")]
    pub when_created: usize,
    #[serde(rename = "hasErrors")]
    #[serde(alias = "@hasErrors")]
    #[serde(alias = "hasErrors")]
    pub has_errors: bool,
    #[serde(rename = "hasWarnings")]
    #[serde(alias = "@hasWarnings")]
    #[serde(alias = "hasWarnings")]
    pub has_warnings: bool,
    #[serde(rename = "locked")]
    #[serde(alias = "@locked")]
    #[serde(alias = "locked")]
    pub locked: bool,

    #[serde(rename = "user")]
    #[serde(alias = "@user")]
    #[serde(alias = "user")]
    #[serde(
        default = "default_string_none",
        deserialize_with = "deserialize_empty_string_as_none"
    )]
    pub user: Option<String>,

    #[serde(rename = "dateTimeChanged")]
    #[serde(alias = "@dateTimeChanged")]
    #[serde(alias = "dateTimeChanged")]
    #[serde(
        default = "default_datetime_none",
        deserialize_with = "deserialize_empty_string_as_none_datetime"
    )]
    pub date_time_changed: Option<DateTime<Utc>>,

    #[serde(rename = "formTitle")]
    #[serde(alias = "@formTitle")]
    #[serde(alias = "formTitle")]
    pub form_title: String,
    #[serde(rename = "formIndex")]
    #[serde(alias = "@formIndex")]
    #[serde(alias = "formIndex")]
    pub form_index: usize,

    #[serde(rename = "formGroup")]
    #[serde(alias = "@formGroup")]
    #[serde(alias = "formGroup")]
    #[serde(
        default = "default_string_none",
        deserialize_with = "deserialize_empty_string_as_none"
    )]
    pub form_group: Option<String>,

    #[serde(rename = "formState")]
    #[serde(alias = "@formState")]
    #[serde(alias = "formState")]
    pub form_state: String,

    #[serde(alias = "state")]
    pub states: Option<Vec<State>>,

    #[serde(alias = "category")]
    pub categories: Option<Vec<Category>>,
}

#[cfg(feature = "python")]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[pyclass]
pub struct Form {
    #[serde(rename = "name")]
    #[serde(alias = "@name")]
    #[serde(alias = "name")]
    pub name: String,

    #[serde(rename = "lastModified")]
    #[serde(alias = "@lastModified")]
    #[serde(alias = "lastModified")]
    #[serde(
        default = "default_datetime_none",
        deserialize_with = "deserialize_empty_string_as_none_datetime"
    )]
    pub last_modified: Option<DateTime<Utc>>,

    #[serde(rename = "whoLastModifiedName")]
    #[serde(alias = "@whoLastModifiedName")]
    #[serde(alias = "whoLastModifiedName")]
    #[serde(
        default = "default_string_none",
        deserialize_with = "deserialize_empty_string_as_none"
    )]
    pub who_last_modified_name: Option<String>,

    #[serde(rename = "whoLastModifiedRole")]
    #[serde(alias = "@whoLastModifiedRole")]
    #[serde(alias = "whoLastModifiedRole")]
    #[serde(
        default = "default_string_none",
        deserialize_with = "deserialize_empty_string_as_none"
    )]
    pub who_last_modified_role: Option<String>,

    #[serde(rename = "whenCreated")]
    #[serde(alias = "@whenCreated")]
    #[serde(alias = "whenCreated")]
    pub when_created: usize,
    #[serde(rename = "hasErrors")]
    #[serde(alias = "@hasErrors")]
    #[serde(alias = "hasErrors")]
    pub has_errors: bool,
    #[serde(rename = "hasWarnings")]
    #[serde(alias = "@hasWarnings")]
    #[serde(alias = "hasWarnings")]
    pub has_warnings: bool,
    #[serde(rename = "locked")]
    #[serde(alias = "@locked")]
    #[serde(alias = "locked")]
    pub locked: bool,

    #[serde(rename = "user")]
    #[serde(alias = "@user")]
    #[serde(alias = "user")]
    #[serde(
        default = "default_string_none",
        deserialize_with = "deserialize_empty_string_as_none"
    )]
    pub user: Option<String>,

    #[serde(rename = "dateTimeChanged")]
    #[serde(alias = "@dateTimeChanged")]
    #[serde(alias = "dateTimeChanged")]
    #[serde(
        default = "default_datetime_none",
        deserialize_with = "deserialize_empty_string_as_none_datetime"
    )]
    pub date_time_changed: Option<DateTime<Utc>>,

    #[serde(rename = "formTitle")]
    #[serde(alias = "@formTitle")]
    #[serde(alias = "formTitle")]
    pub form_title: String,
    #[serde(rename = "formIndex")]
    #[serde(alias = "@formIndex")]
    #[serde(alias = "formIndex")]
    pub form_index: usize,

    #[serde(rename = "formGroup")]
    #[serde(alias = "@formGroup")]
    #[serde(alias = "formGroup")]
    #[serde(
        default = "default_string_none",
        deserialize_with = "deserialize_empty_string_as_none"
    )]
    pub form_group: Option<String>,

    #[serde(rename = "formState")]
    #[serde(alias = "@formState")]
    #[serde(alias = "formState")]
    pub form_state: String,

    #[serde(alias = "state")]
    pub states: Option<Vec<State>>,

    #[serde(alias = "category")]
    pub categories: Option<Vec<Category>>,
}

#[cfg(feature = "python")]
#[pymethods]
impl Form {
    #[getter]
    fn name(&self) -> PyResult<String> {
        Ok(self.name.clone())
    }

    #[getter]
    fn last_modified<'py>(&self, py: Python<'py>) -> PyResult<Option<Bound<'py, PyDateTime>>> {
        to_py_datetime_option(py, &self.last_modified)
    }

    #[getter]
    fn who_last_modified_name(&self) -> PyResult<Option<String>> {
        Ok(self.who_last_modified_name.clone())
    }

    #[getter]
    fn who_last_modified_role(&self) -> PyResult<Option<String>> {
        Ok(self.who_last_modified_role.clone())
    }

    #[getter]
    fn when_created(&self) -> PyResult<usize> {
        Ok(self.when_created)
    }

    #[getter]
    fn has_errors(&self) -> PyResult<bool> {
        Ok(self.has_errors)
    }

    #[getter]
    fn has_warnings(&self) -> PyResult<bool> {
        Ok(self.has_warnings)
    }

    #[getter]
    fn locked(&self) -> PyResult<bool> {
        Ok(self.locked)
    }

    #[getter]
    fn user(&self) -> PyResult<Option<String>> {
        Ok(self.user.clone())
    }

    #[getter]
    fn date_time_changed<'py>(&self, py: Python<'py>) -> PyResult<Option<Bound<'py, PyDateTime>>> {
        to_py_datetime_option(py, &self.date_time_changed)
    }

    #[getter]
    fn form_title(&self) -> PyResult<String> {
        Ok(self.form_title.clone())
    }

    #[getter]
    fn form_index(&self) -> PyResult<usize> {
        Ok(self.form_index)
    }

    #[getter]
    fn form_group(&self) -> PyResult<Option<String>> {
        Ok(self.form_group.clone())
    }

    #[getter]
    fn form_state(&self) -> PyResult<String> {
        Ok(self.form_state.clone())
    }

    #[getter]
    fn states(&self) -> PyResult<Option<Vec<State>>> {
        Ok(self.states.clone())
    }

    #[getter]
    fn categories(&self) -> PyResult<Option<Vec<Category>>> {
        Ok(self.categories.clone())
    }

    pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new(py);
        dict.set_item("name", &self.name)?;
        dict.set_item(
            "last_modified",
            to_py_datetime_option(py, &self.last_modified)?,
        )?;
        dict.set_item("who_last_modified_name", &self.who_last_modified_name)?;
        dict.set_item("who_last_modified_role", &self.who_last_modified_role)?;
        dict.set_item("when_created", self.when_created)?;
        dict.set_item("has_errors", self.has_errors)?;
        dict.set_item("has_warnings", self.has_warnings)?;
        dict.set_item("locked", self.locked)?;
        dict.set_item("user", &self.user)?;
        dict.set_item(
            "date_time_changed",
            to_py_datetime_option(py, &self.date_time_changed)?,
        )?;
        dict.set_item("form_title", &self.form_title)?;
        dict.set_item("form_index", self.form_index)?;
        dict.set_item("form_group", &self.form_group)?;
        dict.set_item("form_state", &self.form_state)?;

        let mut state_dicts = Vec::new();
        if let Some(states) = &self.states {
            for state in states {
                let state_dict = state.to_dict(py)?;
                state_dicts.push(state_dict);
            }
            dict.set_item("states", state_dicts)?;
        } else {
            dict.set_item("states", py.None())?;
        }

        if let Some(categories) = &self.categories {
            let mut category_dicts = Vec::new();
            for category in categories {
                let category_dict = category.to_dict(py)?;
                category_dicts.push(category_dict);
            }
            dict.set_item("categories", category_dicts)?;
        } else {
            dict.set_item("categories", py.None())?;
        }

        Ok(dict)
    }
}

impl State {
    pub fn from_attributes(
        attrs: std::collections::HashMap<String, String>,
    ) -> Result<Self, crate::errors::Error> {
        let value = attrs.get("value").cloned().unwrap_or_default();
        let signer = attrs.get("signer").cloned().unwrap_or_default();
        let signer_unique_id = attrs.get("signerUniqueId").cloned().unwrap_or_default();

        let date_signed = if let Some(ds) = attrs.get("dateSigned") {
            if ds.is_empty() {
                None
            } else {
                parse_datetime_internal(ds).ok()
            }
        } else {
            None
        };

        Ok(State {
            value,
            signer,
            signer_unique_id,
            date_signed,
        })
    }
}

impl Category {
    pub fn from_attributes(
        attrs: std::collections::HashMap<String, String>,
    ) -> Result<Self, crate::errors::Error> {
        let name = attrs.get("name").cloned().unwrap_or_default();
        let category_type = attrs.get("type").cloned().unwrap_or_default();
        let highest_index = attrs
            .get("highestIndex")
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);

        Ok(Category {
            name,
            category_type,
            highest_index,
            fields: None,
        })
    }
}

impl Field {
    pub fn from_attributes(
        attrs: std::collections::HashMap<String, String>,
    ) -> Result<Self, crate::errors::Error> {
        let name = attrs.get("name").cloned().unwrap_or_default();
        let field_type = attrs.get("type").cloned().unwrap_or_default();
        let data_type = attrs.get("dataType").filter(|s| !s.is_empty()).cloned();
        let error_code = attrs.get("errorCode").cloned().unwrap_or_default();

        let when_created = if let Some(wc) = attrs.get("whenCreated") {
            if wc.is_empty() {
                None
            } else {
                Some(parse_datetime_internal(wc)?)
            }
        } else {
            None
        };

        let keep_history = attrs
            .get("keepHistory")
            .map(|s| s == "true")
            .unwrap_or(false);

        Ok(Field {
            name,
            field_type,
            data_type,
            error_code,
            when_created,
            keep_history,
            entries: None,
            comments: None,
        })
    }
}

impl Entry {
    pub fn from_attributes(
        attrs: std::collections::HashMap<String, String>,
    ) -> Result<Self, crate::errors::Error> {
        let entry_id = attrs
            .get("id")
            .or_else(|| attrs.get("entryId"))
            .cloned()
            .unwrap_or_default();

        Ok(Entry {
            entry_id,
            value: None,
            reason: None,
        })
    }
}

impl Value {
    pub fn from_attributes(
        attrs: std::collections::HashMap<String, String>,
    ) -> Result<Self, crate::errors::Error> {
        let by = attrs.get("by").cloned().unwrap_or_default();
        let by_unique_id = attrs.get("byUniqueId").filter(|s| !s.is_empty()).cloned();
        let role = attrs.get("role").cloned().unwrap_or_default();

        let when = if let Some(w) = attrs.get("when") {
            if w.is_empty() {
                None
            } else {
                Some(parse_datetime_internal(w)?)
            }
        } else {
            None
        };

        Ok(Value {
            by,
            by_unique_id,
            role,
            when,
            value: String::new(),
        })
    }
}

impl Reason {
    pub fn from_attributes(
        attrs: std::collections::HashMap<String, String>,
    ) -> Result<Self, crate::errors::Error> {
        let by = attrs.get("by").cloned().unwrap_or_default();
        let by_unique_id = attrs.get("byUniqueId").filter(|s| !s.is_empty()).cloned();
        let role = attrs.get("role").cloned().unwrap_or_default();

        let when = if let Some(w) = attrs.get("when") {
            if w.is_empty() {
                None
            } else {
                Some(parse_datetime_internal(w)?)
            }
        } else {
            None
        };

        Ok(Reason {
            by,
            by_unique_id,
            role,
            when,
            value: String::new(),
        })
    }
}
