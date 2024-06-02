use chrono::{DateTime, Utc};

#[cfg(feature = "python")]
use pyo3::{prelude::*, types::PyDateTime};

#[cfg(feature = "python")]
use crate::native::deserializers::to_py_datetime;

use serde::Deserialize;

pub use crate::native::{
    common::{Category, Entry, Field, Form, Reason, State, Value},
    deserializers::{
        default_datetime_none, default_string_none, deserialize_empty_string_as_none,
        deserialize_empty_string_as_none_datetime,
    },
};

#[cfg(not(feature = "python"))]
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Patient {
    pub patient_id: String,
    pub unique_id: String,
    pub when_created: DateTime<Utc>,
    pub creator: String,
    pub site_name: String,
    pub site_unique_id: String,

    #[serde(
        default = "default_string_none",
        deserialize_with = "deserialize_empty_string_as_none"
    )]
    pub last_language: Option<String>,

    pub number_of_forms: usize,

    #[serde(rename = "form")]
    pub forms: Option<Vec<Form>>,
}

#[cfg(feature = "python")]
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[pyclass]
pub struct Patient {
    pub patient_id: String,
    pub unique_id: String,
    pub when_created: DateTime<Utc>,
    pub creator: String,
    pub site_name: String,
    pub site_unique_id: String,

    #[serde(
        default = "default_string_none",
        deserialize_with = "deserialize_empty_string_as_none"
    )]
    pub last_language: Option<String>,

    pub number_of_forms: usize,

    #[serde(rename = "form")]
    pub forms: Option<Vec<Form>>,
}

#[cfg(feature = "python")]
#[pymethods]
impl Patient {
    #[getter]
    fn patient_id(&self) -> PyResult<String> {
        Ok(self.patient_id.clone())
    }

    #[getter]
    fn unique_id(&self) -> PyResult<String> {
        Ok(self.unique_id.clone())
    }

    #[getter]
    fn when_created<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDateTime>> {
        to_py_datetime(py, &self.when_created)
    }

    #[getter]
    fn creator(&self) -> PyResult<String> {
        Ok(self.creator.clone())
    }

    #[getter]
    fn site_name(&self) -> PyResult<String> {
        Ok(self.site_name.clone())
    }

    #[getter]
    fn site_unique_id(&self) -> PyResult<String> {
        Ok(self.site_unique_id.clone())
    }

    #[getter]
    fn last_language(&self) -> PyResult<Option<String>> {
        Ok(self.last_language.clone())
    }

    #[getter]
    fn number_of_forms(&self) -> PyResult<usize> {
        Ok(self.number_of_forms)
    }

    #[getter]
    fn forms(&self) -> PyResult<Option<Vec<Form>>> {
        Ok(self.forms.clone())
    }
}

#[cfg(not(feature = "python"))]
/// Contains the information from the Prelude native subject XML.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SubjectNative {
    #[serde(rename = "patient", default)]
    pub patients: Vec<Patient>,
}

#[cfg(feature = "python")]
/// Contains the information from the Prelude native subject XML.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[pyclass(get_all)]
pub struct SubjectNative {
    #[serde(rename = "patient", default)]
    pub patients: Vec<Patient>,
}
