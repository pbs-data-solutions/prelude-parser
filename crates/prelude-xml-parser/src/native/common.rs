use std::sync::Arc;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[cfg(feature = "python")]
use pyo3::{
    prelude::*,
    types::{PyDateTime, PyDict},
};

use quick_xml::events::BytesStart;

use crate::native::deserializers::{
    attribute_string, checked_datetime, deserialize_empty_string_as_none,
    deserialize_empty_string_as_none_arc, deserialize_empty_string_as_none_datetime,
    optional_datetime, optional_string, visit_attributes, Interner,
};

#[cfg(feature = "python")]
use crate::native::deserializers::{to_py_datetime, to_py_datetime_option};

#[cfg(not(feature = "python"))]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Value {
    #[serde(rename = "by")]
    #[serde(alias = "@by")]
    #[serde(alias = "by")]
    pub by: Arc<str>,

    #[serde(rename = "byUniqueId")]
    #[serde(alias = "@byUniqueId")]
    #[serde(alias = "byUniqueId")]
    #[serde(default, deserialize_with = "deserialize_empty_string_as_none_arc")]
    pub by_unique_id: Option<Arc<str>>,
    #[serde(rename = "role")]
    #[serde(alias = "@role")]
    #[serde(alias = "role")]
    pub role: Arc<str>,
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
#[pyclass(skip_from_py_object)]
pub struct Value {
    #[serde(rename = "by")]
    #[serde(alias = "@by")]
    #[serde(alias = "by")]
    pub by: Arc<str>,

    #[serde(rename = "byUniqueId")]
    #[serde(alias = "@byUniqueId")]
    #[serde(alias = "byUniqueId")]
    #[serde(default, deserialize_with = "deserialize_empty_string_as_none_arc")]
    pub by_unique_id: Option<Arc<str>>,
    #[serde(rename = "role")]
    #[serde(alias = "@role")]
    #[serde(alias = "role")]
    pub role: Arc<str>,
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
        Ok(self.by.to_string())
    }

    #[getter]
    fn by_unique_id(&self) -> PyResult<Option<String>> {
        Ok(self.by_unique_id.as_deref().map(str::to_string))
    }

    #[getter]
    fn role(&self) -> PyResult<String> {
        Ok(self.role.to_string())
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
        dict.set_item("by", &*self.by)?;
        dict.set_item("by_unique_id", self.by_unique_id.as_deref())?;
        dict.set_item("role", &*self.role)?;
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
    pub by: Arc<str>,

    #[serde(rename = "byUniqueId")]
    #[serde(alias = "@byUniqueId")]
    #[serde(alias = "byUniqueId")]
    #[serde(default, deserialize_with = "deserialize_empty_string_as_none_arc")]
    pub by_unique_id: Option<Arc<str>>,

    #[serde(rename = "role")]
    #[serde(alias = "@role")]
    #[serde(alias = "role")]
    pub role: Arc<str>,
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
#[pyclass(skip_from_py_object)]
pub struct Reason {
    #[serde(rename = "by")]
    #[serde(alias = "@by")]
    #[serde(alias = "by")]
    pub by: Arc<str>,

    #[serde(rename = "byUniqueId")]
    #[serde(alias = "@byUniqueId")]
    #[serde(alias = "byUniqueId")]
    #[serde(default, deserialize_with = "deserialize_empty_string_as_none_arc")]
    pub by_unique_id: Option<Arc<str>>,

    #[serde(rename = "role")]
    #[serde(alias = "@role")]
    #[serde(alias = "role")]
    pub role: Arc<str>,
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
        Ok(self.by.to_string())
    }

    #[getter]
    fn by_unique_id(&self) -> PyResult<Option<String>> {
        Ok(self.by_unique_id.as_deref().map(str::to_string))
    }

    #[getter]
    fn role(&self) -> PyResult<String> {
        Ok(self.role.to_string())
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
        dict.set_item("by", &*self.by)?;
        dict.set_item("by_unique_id", self.by_unique_id.as_deref())?;
        dict.set_item("role", &*self.role)?;
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
    pub entry_id: Arc<str>,

    #[serde(rename = "reviewedBy")]
    #[serde(alias = "@reviewedBy")]
    #[serde(alias = "reviewedBy")]
    #[serde(default, deserialize_with = "deserialize_empty_string_as_none")]
    pub reviewed_by: Option<String>,

    #[serde(rename = "reviewedByUniqueId")]
    #[serde(alias = "@reviewedByUniqueId")]
    #[serde(alias = "reviewedByUniqueId")]
    #[serde(default, deserialize_with = "deserialize_empty_string_as_none")]
    pub reviewed_by_unique_id: Option<String>,

    #[serde(rename = "reviewedByWhen")]
    #[serde(alias = "@reviewedByWhen")]
    #[serde(alias = "reviewedByWhen")]
    #[serde(
        default,
        deserialize_with = "deserialize_empty_string_as_none_datetime"
    )]
    pub reviewed_by_when: Option<DateTime<Utc>>,

    pub value: Option<Value>,
    pub reason: Option<Reason>,
}

#[cfg(feature = "python")]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[pyclass(skip_from_py_object)]
pub struct Entry {
    #[serde(rename = "entryId")]
    #[serde(alias = "@id")]
    #[serde(alias = "entryId")]
    pub entry_id: Arc<str>,

    #[serde(rename = "reviewedBy")]
    #[serde(alias = "@reviewedBy")]
    #[serde(alias = "reviewedBy")]
    #[serde(default, deserialize_with = "deserialize_empty_string_as_none")]
    pub reviewed_by: Option<String>,

    #[serde(rename = "reviewedByUniqueId")]
    #[serde(alias = "@reviewedByUniqueId")]
    #[serde(alias = "reviewedByUniqueId")]
    #[serde(default, deserialize_with = "deserialize_empty_string_as_none")]
    pub reviewed_by_unique_id: Option<String>,

    #[serde(rename = "reviewedByWhen")]
    #[serde(alias = "@reviewedByWhen")]
    #[serde(alias = "reviewedByWhen")]
    #[serde(
        default,
        deserialize_with = "deserialize_empty_string_as_none_datetime"
    )]
    pub reviewed_by_when: Option<DateTime<Utc>>,

    pub value: Option<Value>,
    pub reason: Option<Reason>,
}

#[cfg(feature = "python")]
#[pymethods]
impl Entry {
    #[getter]
    fn entry_id(&self) -> PyResult<String> {
        Ok(self.entry_id.to_string())
    }

    #[getter]
    fn reviewed_by(&self) -> PyResult<Option<String>> {
        Ok(self.reviewed_by.clone())
    }

    #[getter]
    fn reviewed_by_unique_id(&self) -> PyResult<Option<String>> {
        Ok(self.reviewed_by_unique_id.clone())
    }

    #[getter]
    fn reviewed_by_when<'py>(&self, py: Python<'py>) -> PyResult<Option<Bound<'py, PyDateTime>>> {
        to_py_datetime_option(py, &self.reviewed_by_when)
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
        dict.set_item("entry_id", &*self.entry_id)?;
        dict.set_item("reviewed_by", &self.reviewed_by)?;
        dict.set_item("reviewed_by_unique_id", &self.reviewed_by_unique_id)?;
        dict.set_item(
            "reviewed_by_when",
            to_py_datetime_option(py, &self.reviewed_by_when)?,
        )?;
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
#[pyclass(get_all, skip_from_py_object)]
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
    pub name: Arc<str>,

    #[serde(rename = "fieldType")]
    #[serde(alias = "@type")]
    #[serde(alias = "fieldType")]
    pub field_type: Arc<str>,

    #[serde(rename = "dataType")]
    #[serde(alias = "@dataType")]
    #[serde(alias = "dataType")]
    #[serde(default, deserialize_with = "deserialize_empty_string_as_none_arc")]
    pub data_type: Option<Arc<str>>,
    #[serde(rename = "errorCode")]
    #[serde(alias = "@errorCode")]
    #[serde(alias = "errorCode")]
    pub error_code: Arc<str>,
    #[serde(rename = "whenCreated")]
    #[serde(alias = "@whenCreated")]
    #[serde(alias = "whenCreated")]
    pub when_created: Option<DateTime<Utc>>,
    #[serde(rename = "keepHistory")]
    #[serde(alias = "@keepHistory")]
    #[serde(alias = "keepHistory")]
    pub keep_history: bool,

    #[serde(alias = "entry")]
    pub entries: Option<Arc<Vec<Entry>>>,

    #[serde(alias = "comment")]
    pub comments: Option<Arc<Vec<Comment>>>,
}

#[cfg(feature = "python")]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[pyclass(skip_from_py_object)]
pub struct Field {
    #[serde(rename = "name")]
    #[serde(alias = "@name")]
    #[serde(alias = "name")]
    pub name: Arc<str>,

    #[serde(rename = "fieldType")]
    #[serde(alias = "@type")]
    #[serde(alias = "fieldType")]
    pub field_type: Arc<str>,

    #[serde(rename = "dataType")]
    #[serde(alias = "@dataType")]
    #[serde(alias = "dataType")]
    #[serde(default, deserialize_with = "deserialize_empty_string_as_none_arc")]
    pub data_type: Option<Arc<str>>,

    #[serde(rename = "errorCode")]
    #[serde(alias = "@errorCode")]
    #[serde(alias = "errorCode")]
    pub error_code: Arc<str>,
    #[serde(rename = "whenCreated")]
    #[serde(alias = "@whenCreated")]
    #[serde(alias = "whenCreated")]
    pub when_created: Option<DateTime<Utc>>,
    #[serde(rename = "keepHistory")]
    #[serde(alias = "@keepHistory")]
    #[serde(alias = "keepHistory")]
    pub keep_history: bool,

    #[serde(alias = "entry")]
    pub entries: Option<Arc<Vec<Entry>>>,

    #[serde(alias = "comment")]
    pub comments: Option<Arc<Vec<Comment>>>,
}

#[cfg(feature = "python")]
#[pymethods]
impl Field {
    #[getter]
    fn name(&self) -> PyResult<String> {
        Ok(self.name.to_string())
    }

    #[getter]
    fn field_type(&self) -> PyResult<String> {
        Ok(self.field_type.to_string())
    }

    #[getter]
    fn data_type(&self) -> PyResult<Option<String>> {
        Ok(self.data_type.as_deref().map(str::to_string))
    }

    #[getter]
    fn error_code(&self) -> PyResult<String> {
        Ok(self.error_code.to_string())
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
        Ok(self.entries.as_deref().cloned())
    }

    #[getter]
    fn comments(&self) -> PyResult<Option<Vec<Comment>>> {
        Ok(self.comments.as_deref().cloned())
    }

    pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new(py);
        dict.set_item("name", &*self.name)?;
        dict.set_item("field_type", &*self.field_type)?;
        dict.set_item("data_type", self.data_type.as_deref())?;
        dict.set_item("error_code", &*self.error_code)?;
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
            for entry in entries.iter() {
                let entry_dict = entry.to_dict(py)?;
                entry_dicts.push(entry_dict);
            }
            dict.set_item("entries", entry_dicts)?;
        } else {
            dict.set_item("entries", py.None())?;
        }

        let mut comment_dicts = Vec::new();
        if let Some(comments) = &self.comments {
            for comment in comments.iter() {
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
    pub name: Arc<str>,

    #[serde(rename = "categoryType")]
    #[serde(alias = "@type")]
    #[serde(alias = "categoryType")]
    pub category_type: Arc<str>,

    #[serde(rename = "highestIndex")]
    #[serde(alias = "@highestIndex")]
    #[serde(alias = "highestIndex")]
    pub highest_index: usize,

    #[serde(alias = "field")]
    pub fields: Option<Arc<Vec<Field>>>,
}

#[cfg(feature = "python")]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[pyclass(skip_from_py_object)]
pub struct Category {
    #[serde(rename = "name")]
    #[serde(alias = "@name")]
    #[serde(alias = "name")]
    pub name: Arc<str>,

    #[serde(rename = "categoryType")]
    #[serde(alias = "@type")]
    #[serde(alias = "categoryType")]
    pub category_type: Arc<str>,

    #[serde(rename = "highestIndex")]
    #[serde(alias = "@highestIndex")]
    #[serde(alias = "highestIndex")]
    pub highest_index: usize,

    #[serde(alias = "field")]
    pub fields: Option<Arc<Vec<Field>>>,
}

#[cfg(feature = "python")]
#[pymethods]
impl Category {
    #[getter]
    fn name(&self) -> PyResult<String> {
        Ok(self.name.to_string())
    }

    #[getter]
    fn category_type(&self) -> PyResult<String> {
        Ok(self.category_type.to_string())
    }

    #[getter]
    fn highest_index(&self) -> PyResult<usize> {
        Ok(self.highest_index)
    }

    #[getter]
    fn fields(&self) -> PyResult<Option<Vec<Field>>> {
        Ok(self.fields.as_deref().cloned())
    }

    pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new(py);
        dict.set_item("name", &*self.name)?;
        dict.set_item("category_type", &*self.category_type)?;
        dict.set_item("highest_index", self.highest_index)?;

        let mut field_dicts = Vec::new();
        if let Some(fields) = &self.fields {
            for field in fields.iter() {
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
    pub(crate) fn from_attributes(e: &BytesStart<'_>) -> Result<Self, crate::errors::Error> {
        let mut name = "";
        let mut last_modified = "";
        let mut who_last_modified_name = "";
        let mut who_last_modified_role = "";
        let mut when_created = "";
        let mut has_errors = "";
        let mut has_warnings = "";
        let mut locked = "";
        let mut user = "";
        let mut date_time_changed = "";
        let mut form_title = "";
        let mut form_index = "";
        let mut form_group = "";
        let mut form_state = "";

        visit_attributes(e, |key, attr| match key {
            b"name" => name = attr,
            b"lastModified" => last_modified = attr,
            b"whoLastModifiedName" => who_last_modified_name = attr,
            b"whoLastModifiedRole" => who_last_modified_role = attr,
            b"whenCreated" => when_created = attr,
            b"hasErrors" => has_errors = attr,
            b"hasWarnings" => has_warnings = attr,
            b"locked" => locked = attr,
            b"user" => user = attr,
            b"dateTimeChanged" => date_time_changed = attr,
            b"formTitle" => form_title = attr,
            b"formIndex" => form_index = attr,
            b"formGroup" => form_group = attr,
            b"formState" => form_state = attr,
            _ => {}
        })?;

        Ok(Form {
            name: attribute_string(name),
            last_modified: optional_datetime(last_modified),
            who_last_modified_name: optional_string(who_last_modified_name),
            who_last_modified_role: optional_string(who_last_modified_role),
            when_created: when_created.parse().unwrap_or(0),
            has_errors: has_errors == "true",
            has_warnings: has_warnings == "true",
            locked: locked == "true",
            user: optional_string(user),
            date_time_changed: optional_datetime(date_time_changed),
            form_title: attribute_string(form_title),
            form_index: form_index.parse().unwrap_or(0),
            form_group: optional_string(form_group),
            form_state: attribute_string(form_state),
            states: None,
            lock_states: None,
            categories: None,
        })
    }
}

#[cfg(not(feature = "python"))]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct State {
    #[serde(rename = "value")]
    #[serde(alias = "@value")]
    #[serde(alias = "value")]
    pub value: Arc<str>,
    #[serde(rename = "signer")]
    #[serde(alias = "@signer")]
    #[serde(alias = "signer")]
    pub signer: Arc<str>,
    #[serde(rename = "signerUniqueId")]
    #[serde(alias = "@signerUniqueId")]
    #[serde(alias = "signerUniqueId")]
    pub signer_unique_id: Arc<str>,

    #[serde(rename = "dateSigned")]
    #[serde(alias = "@dateSigned")]
    #[serde(alias = "dateSigned")]
    #[serde(
        default,
        deserialize_with = "deserialize_empty_string_as_none_datetime"
    )]
    pub date_signed: Option<DateTime<Utc>>,
}

#[cfg(feature = "python")]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[pyclass(skip_from_py_object)]
pub struct State {
    #[serde(rename = "value")]
    #[serde(alias = "@value")]
    #[serde(alias = "value")]
    pub value: Arc<str>,
    #[serde(rename = "signer")]
    #[serde(alias = "@signer")]
    #[serde(alias = "signer")]
    pub signer: Arc<str>,
    #[serde(rename = "signerUniqueId")]
    #[serde(alias = "@signerUniqueId")]
    #[serde(alias = "signerUniqueId")]
    pub signer_unique_id: Arc<str>,

    #[serde(rename = "dateSigned")]
    #[serde(alias = "@dateSigned")]
    #[serde(alias = "dateSigned")]
    #[serde(
        default,
        deserialize_with = "deserialize_empty_string_as_none_datetime"
    )]
    pub date_signed: Option<DateTime<Utc>>,
}

#[cfg(feature = "python")]
#[pymethods]
impl State {
    #[getter]
    fn value(&self) -> PyResult<String> {
        Ok(self.value.to_string())
    }

    #[getter]
    fn signer(&self) -> PyResult<String> {
        Ok(self.signer.to_string())
    }

    #[getter]
    fn signer_unique_id(&self) -> PyResult<String> {
        Ok(self.signer_unique_id.to_string())
    }

    #[getter]
    fn date_signed<'py>(&self, py: Python<'py>) -> PyResult<Option<Bound<'py, PyDateTime>>> {
        to_py_datetime_option(py, &self.date_signed)
    }

    pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new(py);
        dict.set_item("value", &*self.value)?;
        dict.set_item("signer", &*self.signer)?;
        dict.set_item("signer_unique_id", &*self.signer_unique_id)?;
        dict.set_item("date_signed", to_py_datetime_option(py, &self.date_signed)?)?;

        Ok(dict)
    }
}

#[cfg(not(feature = "python"))]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct LockState {
    #[serde(rename = "locked")]
    #[serde(alias = "@locked")]
    #[serde(alias = "locked")]
    pub locked: bool,

    #[serde(rename = "user")]
    #[serde(alias = "@user")]
    #[serde(alias = "user")]
    #[serde(default, deserialize_with = "deserialize_empty_string_as_none")]
    pub user: Option<String>,

    #[serde(rename = "userUniqueId")]
    #[serde(alias = "@userUniqueId")]
    #[serde(alias = "userUniqueId")]
    #[serde(default, deserialize_with = "deserialize_empty_string_as_none")]
    pub user_unique_id: Option<String>,

    #[serde(rename = "dateTimeChanged")]
    #[serde(alias = "@dateTimeChanged")]
    #[serde(alias = "dateTimeChanged")]
    #[serde(
        default,
        deserialize_with = "deserialize_empty_string_as_none_datetime"
    )]
    pub date_time_changed: Option<DateTime<Utc>>,
}

#[cfg(feature = "python")]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[pyclass(skip_from_py_object)]
pub struct LockState {
    #[serde(rename = "locked")]
    #[serde(alias = "@locked")]
    #[serde(alias = "locked")]
    pub locked: bool,

    #[serde(rename = "user")]
    #[serde(alias = "@user")]
    #[serde(alias = "user")]
    #[serde(default, deserialize_with = "deserialize_empty_string_as_none")]
    pub user: Option<String>,

    #[serde(rename = "userUniqueId")]
    #[serde(alias = "@userUniqueId")]
    #[serde(alias = "userUniqueId")]
    #[serde(default, deserialize_with = "deserialize_empty_string_as_none")]
    pub user_unique_id: Option<String>,

    #[serde(rename = "dateTimeChanged")]
    #[serde(alias = "@dateTimeChanged")]
    #[serde(alias = "dateTimeChanged")]
    #[serde(
        default,
        deserialize_with = "deserialize_empty_string_as_none_datetime"
    )]
    pub date_time_changed: Option<DateTime<Utc>>,
}

#[cfg(feature = "python")]
#[pymethods]
impl LockState {
    #[getter]
    fn locked(&self) -> PyResult<bool> {
        Ok(self.locked)
    }

    #[getter]
    fn user(&self) -> PyResult<Option<String>> {
        Ok(self.user.clone())
    }

    #[getter]
    fn user_unique_id(&self) -> PyResult<Option<String>> {
        Ok(self.user_unique_id.clone())
    }

    #[getter]
    fn date_time_changed<'py>(&self, py: Python<'py>) -> PyResult<Option<Bound<'py, PyDateTime>>> {
        to_py_datetime_option(py, &self.date_time_changed)
    }

    pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new(py);
        dict.set_item("locked", self.locked)?;
        dict.set_item("user", &self.user)?;
        dict.set_item("user_unique_id", &self.user_unique_id)?;
        dict.set_item(
            "date_time_changed",
            to_py_datetime_option(py, &self.date_time_changed)?,
        )?;

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
        default,
        deserialize_with = "deserialize_empty_string_as_none_datetime"
    )]
    pub last_modified: Option<DateTime<Utc>>,

    #[serde(rename = "whoLastModifiedName")]
    #[serde(alias = "@whoLastModifiedName")]
    #[serde(alias = "whoLastModifiedName")]
    #[serde(default, deserialize_with = "deserialize_empty_string_as_none")]
    pub who_last_modified_name: Option<String>,

    #[serde(rename = "whoLastModifiedRole")]
    #[serde(alias = "@whoLastModifiedRole")]
    #[serde(alias = "whoLastModifiedRole")]
    #[serde(default, deserialize_with = "deserialize_empty_string_as_none")]
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
    #[serde(default, deserialize_with = "deserialize_empty_string_as_none")]
    pub user: Option<String>,

    #[serde(rename = "dateTimeChanged")]
    #[serde(alias = "@dateTimeChanged")]
    #[serde(alias = "dateTimeChanged")]
    #[serde(
        default,
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
    #[serde(default, deserialize_with = "deserialize_empty_string_as_none")]
    pub form_group: Option<String>,

    #[serde(rename = "formState")]
    #[serde(alias = "@formState")]
    #[serde(alias = "formState")]
    pub form_state: String,

    #[serde(alias = "state")]
    pub states: Option<Arc<Vec<State>>>,

    #[serde(alias = "lockState")]
    pub lock_states: Option<Arc<Vec<LockState>>>,

    #[serde(alias = "category")]
    pub categories: Option<Arc<Vec<Category>>>,
}

#[cfg(feature = "python")]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[pyclass(skip_from_py_object)]
pub struct Form {
    #[serde(rename = "name")]
    #[serde(alias = "@name")]
    #[serde(alias = "name")]
    pub name: String,

    #[serde(rename = "lastModified")]
    #[serde(alias = "@lastModified")]
    #[serde(alias = "lastModified")]
    #[serde(
        default,
        deserialize_with = "deserialize_empty_string_as_none_datetime"
    )]
    pub last_modified: Option<DateTime<Utc>>,

    #[serde(rename = "whoLastModifiedName")]
    #[serde(alias = "@whoLastModifiedName")]
    #[serde(alias = "whoLastModifiedName")]
    #[serde(default, deserialize_with = "deserialize_empty_string_as_none")]
    pub who_last_modified_name: Option<String>,

    #[serde(rename = "whoLastModifiedRole")]
    #[serde(alias = "@whoLastModifiedRole")]
    #[serde(alias = "whoLastModifiedRole")]
    #[serde(default, deserialize_with = "deserialize_empty_string_as_none")]
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
    #[serde(default, deserialize_with = "deserialize_empty_string_as_none")]
    pub user: Option<String>,

    #[serde(rename = "dateTimeChanged")]
    #[serde(alias = "@dateTimeChanged")]
    #[serde(alias = "dateTimeChanged")]
    #[serde(
        default,
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
    #[serde(default, deserialize_with = "deserialize_empty_string_as_none")]
    pub form_group: Option<String>,

    #[serde(rename = "formState")]
    #[serde(alias = "@formState")]
    #[serde(alias = "formState")]
    pub form_state: String,

    #[serde(alias = "state")]
    pub states: Option<Arc<Vec<State>>>,

    #[serde(alias = "lockState")]
    pub lock_states: Option<Arc<Vec<LockState>>>,

    #[serde(alias = "category")]
    pub categories: Option<Arc<Vec<Category>>>,
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
        Ok(self.states.as_deref().cloned())
    }

    #[getter]
    fn lock_states(&self) -> PyResult<Option<Vec<LockState>>> {
        Ok(self.lock_states.as_deref().cloned())
    }

    #[getter]
    fn categories(&self) -> PyResult<Option<Vec<Category>>> {
        Ok(self.categories.as_deref().cloned())
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
            for state in states.iter() {
                let state_dict = state.to_dict(py)?;
                state_dicts.push(state_dict);
            }
            dict.set_item("states", state_dicts)?;
        } else {
            dict.set_item("states", py.None())?;
        }

        let mut lock_state_dicts = Vec::new();
        if let Some(lock_states) = &self.lock_states {
            for lock_state in lock_states.iter() {
                let lock_state_dict = lock_state.to_dict(py)?;
                lock_state_dicts.push(lock_state_dict);
            }
            dict.set_item("lock_states", lock_state_dicts)?;
        } else {
            dict.set_item("lock_states", py.None())?;
        }

        if let Some(categories) = &self.categories {
            let mut category_dicts = Vec::new();
            for category in categories.iter() {
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
    pub(crate) fn from_attributes(
        e: &BytesStart<'_>,
        interner: &mut Interner,
    ) -> Result<Self, crate::errors::Error> {
        let mut value = "";
        let mut signer = "";
        let mut signer_unique_id = "";
        let mut date_signed = "";

        visit_attributes(e, |key, attr| match key {
            b"value" => value = attr,
            b"signer" => signer = attr,
            b"signerUniqueId" => signer_unique_id = attr,
            b"dateSigned" => date_signed = attr,
            _ => {}
        })?;

        Ok(State {
            value: interner.intern(value),
            signer: interner.intern(signer),
            signer_unique_id: interner.intern(signer_unique_id),
            date_signed: optional_datetime(date_signed),
        })
    }
}

impl LockState {
    pub(crate) fn from_attributes(e: &BytesStart<'_>) -> Result<Self, crate::errors::Error> {
        let mut locked = "";
        let mut user = "";
        let mut user_unique_id = "";
        let mut date_time_changed = "";

        visit_attributes(e, |key, attr| match key {
            b"locked" => locked = attr,
            b"user" => user = attr,
            b"userUniqueId" => user_unique_id = attr,
            b"dateTimeChanged" => date_time_changed = attr,
            _ => {}
        })?;

        Ok(LockState {
            locked: locked == "true",
            user: optional_string(user),
            user_unique_id: optional_string(user_unique_id),
            date_time_changed: optional_datetime(date_time_changed),
        })
    }
}

impl Category {
    pub(crate) fn from_attributes(
        e: &BytesStart<'_>,
        interner: &mut Interner,
    ) -> Result<Self, crate::errors::Error> {
        let mut name = "";
        let mut category_type = "";
        let mut highest_index = "";

        visit_attributes(e, |key, attr| match key {
            b"name" => name = attr,
            b"type" => category_type = attr,
            b"highestIndex" => highest_index = attr,
            _ => {}
        })?;

        Ok(Category {
            name: interner.intern(name),
            category_type: interner.intern(category_type),
            highest_index: highest_index.parse().unwrap_or(0),
            fields: None,
        })
    }
}

impl Field {
    pub(crate) fn from_attributes(
        e: &BytesStart<'_>,
        interner: &mut Interner,
    ) -> Result<Self, crate::errors::Error> {
        let mut name = "";
        let mut field_type = "";
        let mut data_type = "";
        let mut error_code = "";
        let mut when_created = "";
        let mut keep_history = "";

        visit_attributes(e, |key, attr| match key {
            b"name" => name = attr,
            b"type" => field_type = attr,
            b"dataType" => data_type = attr,
            b"errorCode" => error_code = attr,
            b"whenCreated" => when_created = attr,
            b"keepHistory" => keep_history = attr,
            _ => {}
        })?;

        Ok(Field {
            name: interner.intern(name),
            field_type: interner.intern(field_type),
            data_type: interner.intern_optional(data_type),
            error_code: interner.intern(error_code),
            when_created: checked_datetime(when_created)?,
            keep_history: keep_history == "true",
            entries: None,
            comments: None,
        })
    }
}

impl Entry {
    pub(crate) fn from_attributes(
        e: &BytesStart<'_>,
        interner: &mut Interner,
    ) -> Result<Self, crate::errors::Error> {
        let mut id: Option<&str> = None;
        let mut entry_id: Option<&str> = None;
        let mut reviewed_by = "";
        let mut reviewed_by_unique_id = "";
        let mut reviewed_by_when = "";

        visit_attributes(e, |key, attr| match key {
            b"id" => id = Some(attr),
            b"entryId" => entry_id = Some(attr),
            b"reviewedBy" => reviewed_by = attr,
            b"reviewedByUniqueId" => reviewed_by_unique_id = attr,
            b"reviewedByWhen" => reviewed_by_when = attr,
            _ => {}
        })?;

        Ok(Entry {
            entry_id: interner.intern(id.or(entry_id).unwrap_or_default()),
            reviewed_by: optional_string(reviewed_by),
            reviewed_by_unique_id: optional_string(reviewed_by_unique_id),
            reviewed_by_when: optional_datetime(reviewed_by_when),
            value: None,
            reason: None,
        })
    }
}

impl Value {
    pub(crate) fn from_attributes(
        e: &BytesStart<'_>,
        interner: &mut Interner,
    ) -> Result<Self, crate::errors::Error> {
        let mut by = "";
        let mut by_unique_id = "";
        let mut role = "";
        let mut when = "";

        visit_attributes(e, |key, attr| match key {
            b"by" => by = attr,
            b"byUniqueId" => by_unique_id = attr,
            b"role" => role = attr,
            b"when" => when = attr,
            _ => {}
        })?;

        Ok(Value {
            by: interner.intern(by),
            by_unique_id: interner.intern_optional(by_unique_id),
            role: interner.intern(role),
            when: checked_datetime(when)?,
            value: String::new(),
        })
    }
}

impl Reason {
    pub(crate) fn from_attributes(
        e: &BytesStart<'_>,
        interner: &mut Interner,
    ) -> Result<Self, crate::errors::Error> {
        let mut by = "";
        let mut by_unique_id = "";
        let mut role = "";
        let mut when = "";

        visit_attributes(e, |key, attr| match key {
            b"by" => by = attr,
            b"byUniqueId" => by_unique_id = attr,
            b"role" => role = attr,
            b"when" => when = attr,
            _ => {}
        })?;

        Ok(Reason {
            by: interner.intern(by),
            by_unique_id: interner.intern_optional(by_unique_id),
            role: interner.intern(role),
            when: checked_datetime(when)?,
            value: String::new(),
        })
    }
}

impl Comment {
    pub(crate) fn from_attributes(e: &BytesStart<'_>) -> Result<Self, crate::errors::Error> {
        let mut comment_id = "";

        visit_attributes(e, |key, attr| {
            if key == b"id" {
                comment_id = attr;
            }
        })?;

        Ok(Comment {
            comment_id: attribute_string(comment_id),
            value: None,
        })
    }
}
