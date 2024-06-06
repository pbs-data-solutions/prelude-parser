use chrono::{DateTime, Utc};

#[cfg(feature = "python")]
use pyo3::{prelude::*, types::PyDateTime};

#[cfg(feature = "python")]
use crate::native::deserializers::to_py_datetime;

use serde::{Deserialize, Serialize};

pub use crate::native::{
    common::{Category, Entry, Field, Form, Reason, State, Value},
    deserializers::{
        default_datetime_none, default_string_none, deserialize_empty_string_as_none,
        deserialize_empty_string_as_none_datetime,
    },
};

#[cfg(not(feature = "python"))]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
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

    #[serde(alias = "form")]
    pub forms: Option<Vec<Form>>,
}

#[cfg(feature = "python")]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
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
}

#[cfg(not(feature = "python"))]
/// Contains the information from the Prelude native subject XML.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SubjectNative {
    #[serde(alias = "patient")]
    pub patients: Vec<Patient>,
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

#[cfg(test)]
mod tests {
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

        let expected = SubjectNative {
            patients: vec![
                Patient {
                    patient_id: "ABC-001".to_string(),
                    unique_id: "1681574905819".to_string(),
                    when_created: DateTime::parse_from_rfc3339("2023-04-15T16:09:02Z")
                        .unwrap()
                        .with_timezone(&Utc),
                    creator: "Paul Sanders".to_string(),
                    site_name: "Some Site".to_string(),
                    site_unique_id: "1681574834910".to_string(),
                    last_language: Some("English".to_string()),
                    number_of_forms: 6,
                    forms: Some(vec![Form {
                        name: "day.0.form.name.demographics".to_string(),
                        last_modified: Some(
                            DateTime::parse_from_rfc3339("2023-04-15T16:09:15Z")
                                .unwrap()
                                .with_timezone(&Utc),
                        ),
                        who_last_modified_name: Some("Paul Sanders".to_string()),
                        who_last_modified_role: Some("Project Manager".to_string()),
                        when_created: 1681574905839,
                        has_errors: false,
                        has_warnings: false,
                        locked: false,
                        user: None,
                        date_time_changed: None,
                        form_title: "Demographics".to_string(),
                        form_index: 1,
                        form_group: Some("Day 0".to_string()),
                        form_state: "In-Work".to_string(),
                        states: Some(vec![State {
                            value: "form.state.in.work".to_string(),
                            signer: "Paul Sanders - Project Manager".to_string(),
                            signer_unique_id: "1681162687395".to_string(),
                            date_signed: Some(
                                DateTime::parse_from_rfc3339("2023-04-15T16:09:02Z")
                                    .unwrap()
                                    .with_timezone(&Utc),
                            ),
                        }]),
                        categories: Some(vec![Category {
                            name: "Demographics".to_string(),
                            category_type: "normal".to_string(),
                            highest_index: 0,
                            fields: Some(vec![Field {
                                name: "breed".to_string(),
                                field_type: "combo-box".to_string(),
                                data_type: Some("string".to_string()),
                                error_code: "valid".to_string(),
                                when_created: DateTime::parse_from_rfc3339("2023-04-15T16:08:26Z")
                                    .unwrap()
                                    .with_timezone(&Utc),
                                keep_history: true,
                                entries: Some(vec![Entry {
                                    entry_id: "1".to_string(),
                                    value: Some(Value {
                                        by: "Paul Sanders".to_string(),
                                        by_unique_id: Some("1681162687395".to_string()),
                                        role: "Project Manager".to_string(),
                                        when: DateTime::parse_from_rfc3339("2023-04-15T16:09:02Z")
                                            .unwrap()
                                            .with_timezone(&Utc),
                                        value: "Labrador".to_string(),
                                    }),
                                    reason: None,
                                }]),
                            }]),
                        }]),
                    }]),
                },
                Patient {
                    patient_id: "DEF-002".to_string(),
                    unique_id: "1681574905820".to_string(),
                    when_created: DateTime::parse_from_rfc3339("2023-04-16T16:10:02Z")
                        .unwrap()
                        .with_timezone(&Utc),
                    creator: "Wade Watts".to_string(),
                    site_name: "Another Site".to_string(),
                    site_unique_id: "1681574834911".to_string(),
                    last_language: None,
                    number_of_forms: 8,
                    forms: Some(vec![Form {
                        name: "day.0.form.name.demographics".to_string(),
                        last_modified: Some(
                            DateTime::parse_from_rfc3339("2023-04-16T16:10:15Z")
                                .unwrap()
                                .with_timezone(&Utc),
                        ),
                        who_last_modified_name: Some("Barney Rubble".to_string()),
                        who_last_modified_role: Some("Technician".to_string()),
                        when_created: 1681574905838,
                        has_errors: false,
                        has_warnings: false,
                        locked: false,
                        user: None,
                        date_time_changed: None,
                        form_title: "Demographics".to_string(),
                        form_index: 1,
                        form_group: Some("Day 0".to_string()),
                        form_state: "In-Work".to_string(),
                        states: Some(vec![State {
                            value: "form.state.in.work".to_string(),
                            signer: "Paul Sanders - Project Manager".to_string(),
                            signer_unique_id: "1681162687395".to_string(),
                            date_signed: Some(
                                DateTime::parse_from_rfc3339("2023-04-16T16:10:02Z")
                                    .unwrap()
                                    .with_timezone(&Utc),
                            ),
                        }]),
                        categories: Some(vec![Category {
                            name: "Demographics".to_string(),
                            category_type: "normal".to_string(),
                            highest_index: 0,
                            fields: Some(vec![Field {
                                name: "breed".to_string(),
                                field_type: "combo-box".to_string(),
                                data_type: Some("string".to_string()),
                                error_code: "valid".to_string(),
                                when_created: DateTime::parse_from_rfc3339("2023-04-15T16:08:26Z")
                                    .unwrap()
                                    .with_timezone(&Utc),
                                keep_history: true,
                                entries: Some(vec![Entry {
                                    entry_id: "1".to_string(),
                                    value: Some(Value {
                                        by: "Paul Sanders".to_string(),
                                        by_unique_id: Some("1681162687395".to_string()),
                                        role: "Project Manager".to_string(),
                                        when: DateTime::parse_from_rfc3339("2023-04-15T16:09:02Z")
                                            .unwrap()
                                            .with_timezone(&Utc),
                                        value: "Labrador".to_string(),
                                    }),
                                    reason: None,
                                }]),
                            }]),
                        }]),
                    }]),
                },
            ],
        };

        let result: SubjectNative = serde_json::from_str(json_str).unwrap();

        assert_eq!(result, expected);
    }
}
