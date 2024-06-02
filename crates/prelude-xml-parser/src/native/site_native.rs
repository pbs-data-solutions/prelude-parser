use chrono::{DateTime, Utc};

#[cfg(feature = "python")]
use pyo3::{prelude::*, types::PyDateTime};

use serde::Deserialize;

pub use crate::native::common::{Category, Entry, Field, Reason, State, Value};
use crate::native::deserializers::{
    default_datetime_none, default_string_none, deserialize_empty_string_as_none,
    deserialize_empty_string_as_none_datetime,
};

#[cfg(feature = "python")]
use crate::native::deserializers::{to_py_datetime, to_py_datetime_option};

#[cfg(not(feature = "python"))]
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Form {
    pub name: String,

    #[serde(
        default = "default_datetime_none",
        deserialize_with = "deserialize_empty_string_as_none_datetime"
    )]
    pub last_modified: Option<DateTime<Utc>>,

    #[serde(
        default = "default_string_none",
        deserialize_with = "deserialize_empty_string_as_none"
    )]
    pub who_last_modified_name: Option<String>,

    #[serde(
        default = "default_string_none",
        deserialize_with = "deserialize_empty_string_as_none"
    )]
    pub who_last_modified_role: Option<String>,

    pub when_created: usize,
    pub has_errors: bool,
    pub has_warnings: bool,
    pub locked: bool,

    #[serde(
        default = "default_string_none",
        deserialize_with = "deserialize_empty_string_as_none"
    )]
    pub user: Option<String>,

    #[serde(
        default = "default_datetime_none",
        deserialize_with = "deserialize_empty_string_as_none_datetime"
    )]
    pub date_time_changed: Option<DateTime<Utc>>,

    pub form_title: String,
    pub form_index: usize,
    pub form_group: String,
    pub form_state: String,

    #[serde(rename = "state", default)]
    pub states: Option<Vec<State>>,

    #[serde(rename = "category", default)]
    pub categories: Option<Vec<Category>>,
}

#[cfg(feature = "python")]
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[pyclass]
pub struct Form {
    pub name: String,

    #[serde(
        default = "default_datetime_none",
        deserialize_with = "deserialize_empty_string_as_none_datetime"
    )]
    pub last_modified: Option<DateTime<Utc>>,

    #[serde(
        default = "default_string_none",
        deserialize_with = "deserialize_empty_string_as_none"
    )]
    pub who_last_modified_name: Option<String>,

    #[serde(
        default = "default_string_none",
        deserialize_with = "deserialize_empty_string_as_none"
    )]
    pub who_last_modified_role: Option<String>,

    pub when_created: usize,
    pub has_errors: bool,
    pub has_warnings: bool,
    pub locked: bool,

    #[serde(
        default = "default_string_none",
        deserialize_with = "deserialize_empty_string_as_none"
    )]
    pub user: Option<String>,

    #[serde(
        default = "default_datetime_none",
        deserialize_with = "deserialize_empty_string_as_none_datetime"
    )]
    pub date_time_changed: Option<DateTime<Utc>>,

    pub form_title: String,
    pub form_index: usize,
    pub form_group: String,
    pub form_state: String,

    #[serde(rename = "state", default)]
    pub states: Option<Vec<State>>,

    #[serde(rename = "category", default)]
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
    fn form_group(&self) -> PyResult<String> {
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
}

#[cfg(not(feature = "python"))]
#[derive(Debug, Deserialize, PartialEq)]
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
#[derive(Clone, Debug, Deserialize, PartialEq)]
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
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SiteNative {
    #[serde(rename = "site", default)]
    pub sites: Vec<Site>,
}

#[cfg(feature = "python")]
/// Contains the information from the Prelude native site XML.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[pyclass(get_all)]
pub struct SiteNative {
    #[serde(rename = "site", default)]
    pub sites: Vec<Site>,
}
