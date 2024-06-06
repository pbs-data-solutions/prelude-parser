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

    #[serde(alias = "form")]
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
    #[serde(alias = "site")]
    // #[serde(alias = "site")]
    pub sites: Vec<Site>,
}

#[cfg(feature = "python")]
/// Contains the information from the Prelude native site XML.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[pyclass(get_all)]
pub struct SiteNative {
    #[serde(rename = "site")]
    pub sites: Vec<Site>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_site_native_json() {
        let json_str = r#"{
  "sites": [
    {
      "name": "Some Site",
      "uniqueId": "1681574834910",
      "numberOfPatients": 4,
      "countOfRandomizedPatients": 0,
      "whenCreated": "2023-04-15T16:08:19Z",
      "creator": "Paul Sanders",
      "numberOfForms": 1,
      "forms": [
        {
          "name": "demographic.form.name.site.demographics",
          "lastModified": "2023-04-15T16:08:19Z",
          "whoLastModifiedName": "Paul Sanders",
          "whoLastModifiedRole": "Project Manager",
          "whenCreated": 1681574834930,
          "hasErrors": false,
          "hasWarnings": false,
          "locked": false,
          "user": null,
          "dateTimeChanged": null,
          "formTitle": "Site Demographics",
          "formIndex": 1,
          "formGroup": "Demographic",
          "formState": "In-Work",
          "states": [
            {
              "value": "form.state.in.work",
              "signer": "Paul Sanders - Project Manager",
              "signerUniqueId": "1681162687395",
              "dateSigned": "2023-04-15T16:08:19Z"
            }
          ],
          "categories": [
            {
              "name": "Demographics",
              "categoryType": "normal",
              "highestIndex": 0,
              "fields": [
                {
                  "name": "address",
                  "fieldType": "text",
                  "dataType": "string",
                  "errorCode": "valid",
                  "whenCreated": "2023-04-15T16:07:14Z",
                  "keepHistory": true,
                  "entry": null
                },
                {
                  "name": "company",
                  "fieldType": "text",
                  "dataType": "string",
                  "errorCode": "valid",
                  "whenCreated": "2023-04-15T16:07:14Z",
                  "keepHistory": true,
                  "entries": [
                    {
                      "entryId": "1",
                      "value": {
                        "by": "Paul Sanders",
                        "byUniqueId": "1681162687395",
                        "role": "Project Manager",
                        "when": "2023-04-15T16:08:19Z",
                        "$value": "Some Company"
                      },
                      "reason": null
                    }
                  ]
                },
                {
                  "name": "site_code_name",
                  "fieldType": "hidden",
                  "dataType": "string",
                  "errorCode": "valid",
                  "whenCreated": "2023-04-15T16:07:14Z",
                  "keepHistory": true,
                  "entry": [
                    {
                      "entryId": "1",
                      "value": {
                        "by": "set from calculation",
                        "byUniqueId": null,
                        "role": "System",
                        "when": "2023-04-15T16:08:19Z",
                        "$value": "ABC-Some Site"
                      },
                      "reason": {
                        "by": "set from calculation",
                        "byUniqueId": null,
                        "role": "System",
                        "when": "2023-04-15T16:08:19Z",
                        "$value": "calculated value"
                      }
                    },
                    {
                      "entryId": "2",
                      "value": {
                        "by": "set from calculation",
                        "byUniqueId": null,
                        "role": "System",
                        "when": "2023-04-15T16:07:24Z",
                        "$value": "Some Site"
                      },
                      "reason": {
                        "by": "set from calculation",
                        "byUniqueId": null,
                        "role": "System",
                        "when": "2023-04-15T16:07:24Z",
                        "$value": "calculated value"
                      }
                    }
                  ]
                }
              ]
            },
            {
              "name": "Enrollment",
              "categoryType": "normal",
              "highestIndex": 0,
              "field": [
                {
                  "name": "enrollment_closed_date",
                  "fieldType": "popUpCalendar",
                  "dataType": "date",
                  "errorCode": "valid",
                  "whenCreated": "2023-04-15T16:07:14Z",
                  "keepHistory": true,
                  "entry": null
                },
                {
                  "name": "enrollment_open",
                  "fieldType": "radio",
                  "dataType": "string",
                  "errorCode": "valid",
                  "whenCreated": "2023-04-15T16:07:14Z",
                  "keepHistory": true,
                  "entry": [
                    {
                      "entryId": "1",
                      "value": {
                        "by": "Paul Sanders",
                        "byUniqueId": "1681162687395",
                        "role": "Project Manager",
                        "when": "2023-04-15T16:08:19Z",
                        "$value": "Yes"
                      },
                      "reason": null
                    }
                  ]
                },
                {
                  "name": "enrollment_open_date",
                  "fieldType": "popUpCalendar",
                  "dataType": "date",
                  "errorCode": "valid",
                  "whenCreated": "2023-04-15T16:07:14Z",
                  "keepHistory": true,
                  "entry": null
                }
              ]
            }
          ]
        }
      ]
    },
    {
      "name": "Artemis",
      "uniqueId": "1691420994591",
      "numberOfPatients": 0,
      "countOfRandomizedPatients": 0,
      "whenCreated": "2023-08-07T15:14:23Z",
      "creator": "Paul Sanders",
      "numberOfForms": 1,
      "forms": [
        {
          "name": "demographic.form.name.site.demographics",
          "lastModified": "2023-08-07T15:14:23Z",
          "whoLastModifiedName": "Paul Sanders",
          "whoLastModifiedRole": "Project Manager",
          "whenCreated": 1691420994611,
          "hasErrors": false,
          "hasWarnings": false,
          "locked": false,
          "user": null,
          "dateTimeChanged": null,
          "formTitle": "Site Demographics",
          "formIndex": 1,
          "formGroup": "Demographic",
          "formState": "In-Work",
          "states": [
            {
              "value": "form.state.in.work",
              "signer": "Paul Sanders - Project Manager",
              "signerUniqueId": "1681162687395",
              "dateSigned": "2023-08-07T15:14:23Z"
            }
          ],
          "categories": [
            {
              "name": "Demographics",
              "categoryType": "normal",
              "highestIndex": 0,
              "fields": [
                {
                  "name": "address",
                  "fieldType": "text",
                  "dataType": "string",
                  "errorCode": "valid",
                  "whenCreated": "2023-08-07T15:09:54Z",
                  "keepHistory": true,
                  "entries": [
                    {
                      "entryId": "1",
                      "value": {
                        "by": "Paul Sanders",
                        "byUniqueId": "1681162687395",
                        "role": "Project Manager",
                        "when": "2023-08-07T15:14:21Z",
                        "$value": "1111 Moon Drive"
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

        let expected = SiteNative {
            sites: vec![
                Site {
                    name: "Some Site".to_string(),
                    unique_id: "1681574834910".to_string(),
                    number_of_patients: 4,
                    count_of_randomized_patients: 0,
                    when_created: DateTime::parse_from_rfc3339("2023-04-15T16:08:19Z")
                        .unwrap()
                        .with_timezone(&Utc),
                    creator: "Paul Sanders".to_string(),
                    number_of_forms: 1,
                    forms: Some(vec![Form {
                        name: "demographic.form.name.site.demographics".to_string(),
                        last_modified: Some(
                            DateTime::parse_from_rfc3339("2023-04-15T16:08:19Z")
                                .unwrap()
                                .with_timezone(&Utc),
                        ),
                        who_last_modified_name: Some("Paul Sanders".to_string()),
                        who_last_modified_role: Some("Project Manager".to_string()),
                        when_created: 1681574834930,
                        has_errors: false,
                        has_warnings: false,
                        locked: false,
                        user: None,
                        date_time_changed: None,
                        form_title: "Site Demographics".to_string(),
                        form_index: 1,
                        form_group: Some("Demographic".to_string()),
                        form_state: "In-Work".to_string(),
                        states: Some(vec![State {
                            value: "form.state.in.work".to_string(),
                            signer: "Paul Sanders - Project Manager".to_string(),
                            signer_unique_id: "1681162687395".to_string(),
                            date_signed: Some(
                                DateTime::parse_from_rfc3339("2023-04-15T16:08:19Z")
                                    .unwrap()
                                    .with_timezone(&Utc),
                            ),
                        }]),
                        categories: Some(vec![
                            Category {
                                name: "Demographics".to_string(),
                                category_type: "normal".to_string(),
                                highest_index: 0,
                                fields: Some(vec![
                                    Field {
                                        name: "address".to_string(),
                                        field_type: "text".to_string(),
                                        data_type: Some("string".to_string()),
                                        error_code: "valid".to_string(),
                                        when_created: DateTime::parse_from_rfc3339(
                                            "2023-04-15T16:07:14Z",
                                        )
                                        .unwrap()
                                        .with_timezone(&Utc),
                                        keep_history: true,
                                        entries: None,
                                    },
                                    Field {
                                        name: "company".to_string(),
                                        field_type: "text".to_string(),
                                        data_type: Some("string".to_string()),
                                        error_code: "valid".to_string(),
                                        when_created: DateTime::parse_from_rfc3339(
                                            "2023-04-15T16:07:14Z",
                                        )
                                        .unwrap()
                                        .with_timezone(&Utc),
                                        keep_history: true,
                                        entries: Some(vec![Entry {
                                            entry_id: "1".to_string(),
                                            value: Some(Value {
                                                by: "Paul Sanders".to_string(),
                                                by_unique_id: Some("1681162687395".to_string()),
                                                role: "Project Manager".to_string(),
                                                when: DateTime::parse_from_rfc3339(
                                                    "2023-04-15T16:08:19Z",
                                                )
                                                .unwrap()
                                                .with_timezone(&Utc),
                                                value: "Some Company".to_string(),
                                            }),
                                            reason: None,
                                        }]),
                                    },
                                    Field {
                                        name: "site_code_name".to_string(),
                                        field_type: "hidden".to_string(),
                                        data_type: Some("string".to_string()),
                                        error_code: "valid".to_string(),
                                        when_created: DateTime::parse_from_rfc3339(
                                            "2023-04-15T16:07:14Z",
                                        )
                                        .unwrap()
                                        .with_timezone(&Utc),
                                        keep_history: true,
                                        entries: Some(vec![
                                            Entry {
                                                entry_id: "1".to_string(),
                                                value: Some(Value {
                                                    by: "set from calculation".to_string(),
                                                    by_unique_id: None,
                                                    role: "System".to_string(),
                                                    when: DateTime::parse_from_rfc3339(
                                                        "2023-04-15T16:08:19Z",
                                                    )
                                                    .unwrap()
                                                    .with_timezone(&Utc),
                                                    value: "ABC-Some Site".to_string(),
                                                }),
                                                reason: Some(Reason {
                                                    by: "set from calculation".to_string(),
                                                    by_unique_id: None,
                                                    role: "System".to_string(),
                                                    when: DateTime::parse_from_rfc3339(
                                                        "2023-04-15T16:08:19Z",
                                                    )
                                                    .unwrap()
                                                    .with_timezone(&Utc),
                                                    value: "calculated value".to_string(),
                                                }),
                                            },
                                            Entry {
                                                entry_id: "2".to_string(),
                                                value: Some(Value {
                                                    by: "set from calculation".to_string(),
                                                    by_unique_id: None,
                                                    role: "System".to_string(),
                                                    when: DateTime::parse_from_rfc3339(
                                                        "2023-04-15T16:07:24Z",
                                                    )
                                                    .unwrap()
                                                    .with_timezone(&Utc),
                                                    value: "Some Site".to_string(),
                                                }),
                                                reason: Some(Reason {
                                                    by: "set from calculation".to_string(),
                                                    by_unique_id: None,
                                                    role: "System".to_string(),
                                                    when: DateTime::parse_from_rfc3339(
                                                        "2023-04-15T16:07:24Z",
                                                    )
                                                    .unwrap()
                                                    .with_timezone(&Utc),
                                                    value: "calculated value".to_string(),
                                                }),
                                            },
                                        ]),
                                    },
                                ]),
                            },
                            Category {
                                name: "Enrollment".to_string(),
                                category_type: "normal".to_string(),
                                highest_index: 0,
                                fields: Some(vec![
                                    Field {
                                        name: "enrollment_closed_date".to_string(),
                                        field_type: "popUpCalendar".to_string(),
                                        data_type: Some("date".to_string()),
                                        error_code: "valid".to_string(),
                                        when_created: DateTime::parse_from_rfc3339(
                                            "2023-04-15T16:07:14Z",
                                        )
                                        .unwrap()
                                        .with_timezone(&Utc),
                                        keep_history: true,
                                        entries: None,
                                    },
                                    Field {
                                        name: "enrollment_open".to_string(),
                                        field_type: "radio".to_string(),
                                        data_type: Some("string".to_string()),
                                        error_code: "valid".to_string(),
                                        when_created: DateTime::parse_from_rfc3339(
                                            "2023-04-15T16:07:14Z",
                                        )
                                        .unwrap()
                                        .with_timezone(&Utc),
                                        keep_history: true,
                                        entries: Some(vec![Entry {
                                            entry_id: "1".to_string(),
                                            value: Some(Value {
                                                by: "Paul Sanders".to_string(),
                                                by_unique_id: Some("1681162687395".to_string()),
                                                role: "Project Manager".to_string(),
                                                when: DateTime::parse_from_rfc3339(
                                                    "2023-04-15T16:08:19Z",
                                                )
                                                .unwrap()
                                                .with_timezone(&Utc),
                                                value: "Yes".to_string(),
                                            }),
                                            reason: None,
                                        }]),
                                    },
                                    Field {
                                        name: "enrollment_open_date".to_string(),
                                        field_type: "popUpCalendar".to_string(),
                                        data_type: Some("date".to_string()),
                                        error_code: "valid".to_string(),
                                        when_created: DateTime::parse_from_rfc3339(
                                            "2023-04-15T16:07:14Z",
                                        )
                                        .unwrap()
                                        .with_timezone(&Utc),
                                        keep_history: true,
                                        entries: None,
                                    },
                                ]),
                            },
                        ]),
                    }]),
                },
                Site {
                    name: "Artemis".to_string(),
                    unique_id: "1691420994591".to_string(),
                    number_of_patients: 0,
                    count_of_randomized_patients: 0,
                    when_created: DateTime::parse_from_rfc3339("2023-08-07T15:14:23Z")
                        .unwrap()
                        .with_timezone(&Utc),
                    creator: "Paul Sanders".to_string(),
                    number_of_forms: 1,
                    forms: Some(vec![Form {
                        name: "demographic.form.name.site.demographics".to_string(),
                        last_modified: Some(
                            DateTime::parse_from_rfc3339("2023-08-07T15:14:23Z")
                                .unwrap()
                                .with_timezone(&Utc),
                        ),
                        who_last_modified_name: Some("Paul Sanders".to_string()),
                        who_last_modified_role: Some("Project Manager".to_string()),
                        when_created: 1691420994611,
                        has_errors: false,
                        has_warnings: false,
                        locked: false,
                        user: None,
                        date_time_changed: None,
                        form_title: "Site Demographics".to_string(),
                        form_index: 1,
                        form_group: Some("Demographic".to_string()),
                        form_state: "In-Work".to_string(),
                        states: Some(vec![State {
                            value: "form.state.in.work".to_string(),
                            signer: "Paul Sanders - Project Manager".to_string(),
                            signer_unique_id: "1681162687395".to_string(),
                            date_signed: Some(
                                DateTime::parse_from_rfc3339("2023-08-07T15:14:23Z")
                                    .unwrap()
                                    .with_timezone(&Utc),
                            ),
                        }]),
                        categories: Some(vec![Category {
                            name: "Demographics".to_string(),
                            category_type: "normal".to_string(),
                            highest_index: 0,
                            fields: Some(vec![Field {
                                name: "address".to_string(),
                                field_type: "text".to_string(),
                                data_type: Some("string".to_string()),
                                error_code: "valid".to_string(),
                                when_created: DateTime::parse_from_rfc3339("2023-08-07T15:09:54Z")
                                    .unwrap()
                                    .with_timezone(&Utc),
                                keep_history: true,
                                entries: Some(vec![Entry {
                                    entry_id: "1".to_string(),
                                    value: Some(Value {
                                        by: "Paul Sanders".to_string(),
                                        by_unique_id: Some("1681162687395".to_string()),
                                        role: "Project Manager".to_string(),
                                        when: DateTime::parse_from_rfc3339("2023-08-07T15:14:21Z")
                                            .unwrap()
                                            .with_timezone(&Utc),
                                        value: "1111 Moon Drive".to_string(),
                                    }),
                                    reason: None,
                                }]),
                            }]),
                        }]),
                    }]),
                },
            ],
        };

        let result: SiteNative = serde_json::from_str(json_str).unwrap();

        assert_eq!(result, expected);
    }
}
