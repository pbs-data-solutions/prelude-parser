use chrono::{DateTime, Utc};

#[cfg(feature = "python")]
use pyo3::{
    exceptions::PyValueError,
    prelude::*,
    types::{PyDateTime, PyDict},
};

#[cfg(feature = "python")]
use crate::native::deserializers::to_py_datetime;

use serde::{Deserialize, Serialize};

pub use crate::native::common::{Category, Comment, Entry, Field, Form, Reason, State, Value};

use crate::native::deserializers::{default_string_none, deserialize_empty_string_as_none};

#[cfg(not(feature = "python"))]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Patient {
    #[serde(rename = "@patientId", alias = "patientId")]
    pub patient_id: String,
    #[serde(rename = "@uniqueId", alias = "uniqueId")]
    pub unique_id: String,
    #[serde(rename = "@whenCreated", alias = "whenCreated")]
    pub when_created: DateTime<Utc>,
    #[serde(rename = "@creator", alias = "creator")]
    pub creator: String,
    #[serde(rename = "@siteName", alias = "siteName")]
    pub site_name: String,
    #[serde(rename = "@siteUniqueId", alias = "siteUniqueId")]
    pub site_unique_id: String,

    #[serde(
        rename = "@lastLanguage",
        alias = "lastLanguage",
        default = "default_string_none",
        deserialize_with = "deserialize_empty_string_as_none"
    )]
    pub last_language: Option<String>,

    #[serde(rename = "@numberOfForms", alias = "numberOfForms")]
    pub number_of_forms: usize,

    #[serde(alias = "form")]
    pub forms: Option<Vec<Form>>,
}

#[cfg(feature = "python")]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[pyclass]
pub struct Patient {
    #[serde(rename = "@patientId", alias = "patientId")]
    pub patient_id: String,
    #[serde(rename = "@uniqueId", alias = "uniqueId")]
    pub unique_id: String,
    #[serde(rename = "@whenCreated", alias = "whenCreated")]
    pub when_created: DateTime<Utc>,
    #[serde(rename = "@creator", alias = "creator")]
    pub creator: String,
    #[serde(rename = "@siteName", alias = "siteName")]
    pub site_name: String,
    #[serde(rename = "@siteUniqueId", alias = "siteUniqueId")]
    pub site_unique_id: String,

    #[serde(
        rename = "@lastLanguage",
        alias = "lastLanguage",
        default = "default_string_none",
        deserialize_with = "deserialize_empty_string_as_none"
    )]
    pub last_language: Option<String>,

    #[serde(rename = "@numberOfForms", alias = "numberOfForms")]
    pub number_of_forms: usize,

    #[serde(alias = "form")]
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

    pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new(py);
        dict.set_item("patient_id", &self.patient_id)?;
        dict.set_item("unique_id", &self.unique_id)?;
        dict.set_item("when_created", to_py_datetime(py, &self.when_created)?)?;
        dict.set_item("creator", &self.creator)?;
        dict.set_item("site_name", &self.site_name)?;
        dict.set_item("site_unique_id", &self.site_unique_id)?;
        dict.set_item("last_language", &self.last_language)?;
        dict.set_item("number_of_forms", self.number_of_forms)?;

        let mut form_dicts = Vec::new();
        if let Some(forms) = &self.forms {
            for form in forms {
                let form_dict = form.to_dict(py)?;
                form_dicts.push(form_dict);
            }
            dict.set_item("forms", form_dicts)?;
        } else {
            dict.set_item("forms", py.None())?;
        }

        Ok(dict)
    }
}

#[cfg(not(feature = "python"))]
/// Contains the information from the Prelude native subject XML.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SubjectNative {
    #[serde(alias = "patient")]
    pub patients: Vec<Patient>,
}

#[cfg(not(feature = "python"))]
impl SubjectNative {
    /// Convert to a JSON string
    ///
    /// # Example
    ///
    /// ```
    /// use std::path::Path;
    ///
    /// use prelude_xml_parser::parse_subject_native_file;
    ///
    /// let file_path = Path::new("tests/assets/subject_native_small.xml");
    /// let native = parse_subject_native_file(&file_path).unwrap();
    /// let json = native.to_json().unwrap();
    /// // Verify the JSON contains expected patient data
    /// assert!(json.contains("ABC-001"));
    /// assert!(json.contains("Paul Sanders"));
    /// assert!(json.contains("Labrador"));
    /// ```
    pub fn to_json(&self) -> serde_json::Result<String> {
        let json = serde_json::to_string(&self)?;

        Ok(json)
    }
}

#[cfg(feature = "python")]
/// Contains the information from the Prelude native subject XML.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[pyclass(get_all)]
pub struct SubjectNative {
    #[serde(alias = "patient")]
    pub patients: Vec<Patient>,
}

#[cfg(feature = "python")]
#[pymethods]
impl SubjectNative {
    #[getter]
    fn sites(&self) -> PyResult<Vec<Patient>> {
        Ok(self.patients.clone())
    }

    /// Convert the class instance to a dictionary
    fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new(py);
        let mut patient_dicts = Vec::new();
        for patient in &self.patients {
            let patient_dict = patient.to_dict(py)?;
            patient_dicts.push(patient_dict);
        }
        dict.set_item("patients", patient_dicts)?;
        Ok(dict)
    }

    /// Convert the class instance to a JSON string
    fn to_json(&self) -> PyResult<String> {
        serde_json::to_string(&self)
            .map_err(|_| PyErr::new::<PyValueError, _>("Error converting to JSON"))
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_yaml_snapshot;

    use super::*;

    #[test]
    fn deserialize_subject_native_json() {
        let json_str = r#"{
    "patients": [
        {
            "patientId": "ABC-001",
            "uniqueId": "1681574905819",
            "whenCreated": "2023-04-15T16:09:02Z",
            "creator": "Paul Sanders",
            "siteName": "Some Site",
            "siteUniqueId": "1681574834910",
            "lastLanguage": "English",
            "numberOfForms": 6,
            "forms": [
                {
                    "name": "day.0.form.name.demographics",
                    "lastModified": "2023-04-15T16:09:15Z",
                    "whoLastModifiedName": "Paul Sanders",
                    "whoLastModifiedRole": "Project Manager",
                    "whenCreated": 1681574905839,
                    "hasErrors": false,
                    "hasWarnings": false,
                    "locked": false,
                    "user": null,
                    "dateTimeChanged": null,
                    "formTitle": "Demographics",
                    "formIndex": 1,
                    "formGroup": "Day 0",
                    "formState": "In-Work",
                    "states": [
                        {
                            "value": "form.state.in.work",
                            "signer": "Paul Sanders - Project Manager",
                            "signerUniqueId": "1681162687395",
                            "dateSigned": "2023-04-15T16:09:02Z"
                        }
                    ],
                    "categories": [
                        {
                            "name": "Demographics",
                            "categoryType": "normal",
                            "highestIndex": 0,
                            "fields": [
                                {
                                    "name": "breed",
                                    "fieldType": "combo-box",
                                    "dataType": "string",
                                    "errorCode": "valid",
                                    "whenCreated": "2023-04-15T16:08:26Z",
                                    "keepHistory": true,
                                    "entries": [
                                        {
                                            "entryId": "1",
                                            "value": {
                                                "by": "Paul Sanders",
                                                "byUniqueId": "1681162687395",
                                                "role": "Project Manager",
                                                "when": "2023-04-15T16:09:02Z",
                                                "value": "Labrador"
                                            },
                                            "reason": null
                                        }
                                    ]
                                }
                            ]
                        }
                    ]
                }
            ]
        },
        {
            "patientId": "DEF-002",
            "uniqueId": "1681574905820",
            "whenCreated": "2023-04-16T16:10:02Z",
            "creator": "Wade Watts",
            "siteName": "Another Site",
            "siteUniqueId": "1681574834911",
            "lastLanguage": null,
            "numberOfForms": 8,
            "forms": [
                {
                    "name": "day.0.form.name.demographics",
                    "lastModified": "2023-04-16T16:10:15Z",
                    "whoLastModifiedName": "Barney Rubble",
                    "whoLastModifiedRole": "Technician",
                    "whenCreated": 1681574905838,
                    "hasErrors": false,
                    "hasWarnings": false,
                    "locked": false,
                    "user": null,
                    "dateTimeChanged": null,
                    "formTitle": "Demographics",
                    "formIndex": 1,
                    "formGroup": "Day 0",
                    "formState": "In-Work",
                    "states": [
                        {
                            "value": "form.state.in.work",
                            "signer": "Paul Sanders - Project Manager",
                            "signerUniqueId": "1681162687395",
                            "dateSigned": "2023-04-16T16:10:02Z"
                        }
                    ],
                    "categories": [
                        {
                            "name": "Demographics",
                            "categoryType": "normal",
                            "highestIndex": 0,
                            "fields": [
                                {
                                    "name": "breed",
                                    "fieldType": "combo-box",
                                    "dataType": "string",
                                    "errorCode": "valid",
                                    "whenCreated": "2023-04-15T16:08:26Z",
                                    "keepHistory": true,
                                    "entries": [
                                        {
                                            "entryId": "1",
                                            "value": {
                                                "by": "Paul Sanders",
                                                "byUniqueId": "1681162687395",
                                                "role": "Project Manager",
                                                "when": "2023-04-15T16:09:02Z",
                                                "value": "Labrador"
                                            },
                                            "reason": null
                                        }
                                    ]
                                }
                            ]
                        }
                    ]
                }
            ]
        }
    ]
}
        "#;

        let result: SubjectNative = serde_json::from_str(json_str).unwrap();

        assert_yaml_snapshot!(result);
    }
}
