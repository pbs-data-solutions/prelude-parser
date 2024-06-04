use chrono::{DateTime, Utc};

#[cfg(feature = "python")]
use pyo3::{prelude::*, types::PyDateTime};

use serde::{Deserialize, Serialize};

pub use crate::native::{
    common::{Category, Entry, Field, Form, Reason, State, Value},
    deserializers::{
        default_datetime_none, default_string_none, deserialize_empty_string_as_none,
        deserialize_empty_string_as_none_datetime,
    },
};

#[cfg(feature = "python")]
use crate::native::deserializers::to_py_datetime;

#[cfg(not(feature = "python"))]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Site {
    pub name: String,
    pub unique_id: String,
    pub number_of_patients: usize,
    pub count_of_randomized_patients: usize,
    pub when_created: DateTime<Utc>,
    pub creator: String,
    pub number_of_forms: usize,

    #[serde(rename = "form")]
    pub forms: Option<Vec<Form>>,
}

#[cfg(feature = "python")]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[pyclass]
pub struct Site {
    pub name: String,
    pub unique_id: String,
    pub number_of_patients: usize,
    pub count_of_randomized_patients: usize,
    pub when_created: DateTime<Utc>,
    pub creator: String,
    pub number_of_forms: usize,

    #[serde(rename = "form")]
    pub forms: Option<Vec<Form>>,
}

#[cfg(feature = "python")]
#[pymethods]
impl Site {
    #[getter]
    fn name(&self) -> PyResult<String> {
        Ok(self.name.clone())
    }

    #[getter]
    fn unique_id(&self) -> PyResult<String> {
        Ok(self.unique_id.clone())
    }

    #[getter]
    fn number_of_patients(&self) -> PyResult<usize> {
        Ok(self.number_of_patients)
    }

    #[getter]
    fn count_of_randomized_patients(&self) -> PyResult<usize> {
        Ok(self.count_of_randomized_patients)
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
    fn number_of_forms(&self) -> PyResult<usize> {
        Ok(self.number_of_forms)
    }

    #[getter]
    fn forms(&self) -> PyResult<Option<Vec<Form>>> {
        Ok(self.forms.clone())
    }
}

#[cfg(not(feature = "python"))]
/// Contains the information from the Prelude native site XML.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SiteNative {
    #[serde(rename = "site", default)]
    pub sites: Vec<Site>,
}

#[cfg(feature = "python")]
/// Contains the information from the Prelude native site XML.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[pyclass(get_all)]
pub struct SiteNative {
    #[serde(rename = "site", default)]
    pub sites: Vec<Site>,
}
