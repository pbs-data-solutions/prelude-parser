use chrono::{DateTime, Utc};
use serde::Deserialize;

#[cfg(feature = "python")]
use pyo3::{prelude::*, types::PyDateTime};

use crate::native::deserializers::{
    default_datetime_none, default_string_none, deserialize_empty_string_as_none,
    deserialize_empty_string_as_none_datetime,
};

#[cfg(feature = "python")]
use crate::native::deserializers::{to_py_datetime, to_py_datetime_option};

#[cfg(not(feature = "python"))]
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Value {
    pub by: String,

    #[serde(
        default = "default_string_none",
        deserialize_with = "deserialize_empty_string_as_none"
    )]
    pub by_unique_id: Option<String>,
    pub role: String,
    pub when: DateTime<Utc>,

    #[serde(rename = "$value")]
    pub value: String,
}

#[cfg(feature = "python")]
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[pyclass]
pub struct Value {
    pub by: String,

    #[serde(
        default = "default_string_none",
        deserialize_with = "deserialize_empty_string_as_none"
    )]
    pub by_unique_id: Option<String>,
    pub role: String,
    pub when: DateTime<Utc>,

    #[serde(rename = "$value")]
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
    fn when<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDateTime>> {
        to_py_datetime(py, &self.when)
    }

    #[getter]
    fn value(&self) -> PyResult<String> {
        Ok(self.value.clone())
    }
}

#[cfg(not(feature = "python"))]
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Reason {
    pub by: String,

    #[serde(
        default = "default_string_none",
        deserialize_with = "deserialize_empty_string_as_none"
    )]
    pub by_unique_id: Option<String>,

    pub role: String,
    pub when: DateTime<Utc>,

    #[serde(rename = "$value")]
    pub value: String,
}

#[cfg(feature = "python")]
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[pyclass]
pub struct Reason {
    pub by: String,

    #[serde(
        default = "default_string_none",
        deserialize_with = "deserialize_empty_string_as_none"
    )]
    pub by_unique_id: Option<String>,

    pub role: String,
    pub when: DateTime<Utc>,

    #[serde(rename = "$value")]
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
    fn when<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDateTime>> {
        to_py_datetime(py, &self.when)
    }

    #[getter]
    fn value(&self) -> PyResult<String> {
        Ok(self.value.clone())
    }
}

#[cfg(not(feature = "python"))]
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Entry {
    #[serde(alias = "id")]
    pub entry_id: String,
    pub value: Option<Value>,
    pub reason: Option<Reason>,
}

#[cfg(feature = "python")]
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[pyclass(get_all)]
pub struct Entry {
    #[serde(alias = "id")]
    pub entry_id: String,
    pub value: Option<Value>,
    pub reason: Option<Reason>,
}

#[cfg(not(feature = "python"))]
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

    #[serde(rename = "entry")]
    pub entries: Option<Vec<Entry>>,
}

#[cfg(feature = "python")]
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[pyclass]
pub struct Field {
    pub name: String,

    #[serde(alias = "type")]
    pub field_type: String,

    pub data_type: String,
    pub error_code: String,
    pub when_created: DateTime<Utc>,
    pub keep_history: bool,

    #[serde(rename = "entry")]
    pub entries: Option<Vec<Entry>>,
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
    fn data_type(&self) -> PyResult<String> {
        Ok(self.data_type.clone())
    }

    #[getter]
    fn error_code(&self) -> PyResult<String> {
        Ok(self.error_code.clone())
    }

    #[getter]
    fn when_created<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDateTime>> {
        to_py_datetime(py, &self.when_created)
    }

    #[getter]
    fn keep_history(&self) -> PyResult<bool> {
        Ok(self.keep_history)
    }

    #[getter]
    fn entries(&self) -> PyResult<Option<Vec<Entry>>> {
        Ok(self.entries.clone())
    }
}

#[cfg(not(feature = "python"))]
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    pub name: String,

    #[serde(alias = "type")]
    pub category_type: String,

    pub highest_index: usize,

    #[serde(rename = "field", default)]
    pub fields: Vec<Field>,
}

#[cfg(feature = "python")]
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[pyclass(get_all)]
pub struct Category {
    pub name: String,

    #[serde(alias = "type")]
    pub category_type: String,

    pub highest_index: usize,

    #[serde(rename = "field", default)]
    pub fields: Vec<Field>,
}

#[cfg(not(feature = "python"))]
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct State {
    pub value: String,
    pub signer: String,
    pub signer_unique_id: String,

    #[serde(
        default = "default_datetime_none",
        deserialize_with = "deserialize_empty_string_as_none_datetime"
    )]
    pub date_signed: Option<DateTime<Utc>>,
}

#[cfg(feature = "python")]
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[pyclass]
pub struct State {
    pub value: String,
    pub signer: String,
    pub signer_unique_id: String,

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
}
