use serde::{Deserialize, Serialize};

#[cfg(feature = "python")]
use pyo3::prelude::*;

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
pub struct User {
    pub unique_id: String,

    #[serde(
        default = "default_string_none",
        deserialize_with = "deserialize_empty_string_as_none"
    )]
    pub last_language: Option<String>,
    pub creator: String,
    pub number_of_forms: usize,

    #[serde(alias = "form")]
    pub forms: Option<Vec<Form>>,
}

#[cfg(feature = "python")]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[pyclass(get_all)]
pub struct User {
    pub unique_id: String,

    #[serde(
        default = "default_string_none",
        deserialize_with = "deserialize_empty_string_as_none"
    )]
    pub last_language: Option<String>,
    pub creator: String,
    pub number_of_forms: usize,

    #[serde(alias = "form")]
    pub forms: Option<Vec<Form>>,
}

#[cfg(not(feature = "python"))]
/// Contains the information from the Prelude native user XML.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UserNative {
    #[serde(alias = "user")]
    pub users: Vec<User>,
}

#[cfg(feature = "python")]
/// Contains the information from the Prelude native user XML.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[pyclass(get_all)]
pub struct UserNative {
    #[serde(alias = "user")]
    pub users: Vec<User>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{DateTime, Utc};

    #[test]
    fn deserialize_user_native_json() {
        let json_str = r#"{
    "users": [
        {
            "uniqueId": "1691421275437",
            "lastLanguage": null,
            "creator": "Paul Sanders(1681162687395)",
            "numberOfForms": 1,
            "forms": [
                {
                    "name": "form.name.demographics",
                    "lastModified": "2023-08-07T15:15:41Z",
                    "whoLastModifiedName": "Paul Sanders",
                    "whoLastModifiedRole": "Project Manager",
                    "whenCreated": 1691421341578,
                    "hasErrors": false,
                    "hasWarnings": false,
                    "locked": false,
                    "user": null,
                    "dateTimeChanged": null,
                    "formTitle": "User Demographics",
                    "formIndex": 1,
                    "formGroup": null,
                    "formState": "In-Work",
                    "states": [
                        {
                            "value": "form.state.in.work",
                            "signer": "Paul Sanders - Project Manager",
                            "signerUniqueId": "1681162687395",
                            "dateSigned": "2023-08-07T15:15:41Z"
                        }
                    ],
                    "categories": [
                        {
                            "name": "demographics",
                            "categoryType": "normal",
                            "highestIndex": 0,
                            "fields": [
                                {
                                    "name": "address",
                                    "fieldType": "text",
                                    "dataType": "string",
                                    "errorCode": "undefined",
                                    "whenCreated": "2024-01-12T20:14:09Z",
                                    "keepHistory": true,
                                    "entries": null
                                },
                                {
                                    "name": "email",
                                    "fieldType": "text",
                                    "dataType": "string",
                                    "errorCode": "undefined",
                                    "whenCreated": "2023-08-07T15:15:41Z",
                                    "keepHistory": true,
                                    "entries": [
                                        {
                                            "entryId": "1",
                                            "value": {
                                                "by": "Paul Sanders",
                                                "byUniqueId": "1681162687395",
                                                "role": "Project Manager",
                                                "when": "2023-08-07T15:15:41Z",
                                                "value": "jazz@artemis.com"
                                            },
                                            "reason": null
                                        }
                                    ]
                                }
                            ]
                        },
                        {
                            "name": "Administrative",
                            "categoryType": "normal",
                            "highestIndex": 0,
                            "fields": [
                                {
                                    "name": "study_assignment",
                                    "fieldType": "text",
                                    "dataType": null,
                                    "errorCode": "undefined",
                                    "whenCreated": "2023-08-07T15:15:41Z",
                                    "keepHistory": true,
                                    "entries": [
                                        {
                                            "entryId": "1",
                                            "value": {
                                                "by": "set from calculation",
                                                "byUniqueId": null,
                                                "role": "System",
                                                "when": "2023-08-07T15:15:41Z",
                                                "value": "On 07-Aug-2023 10:15 -0500, Paul Sanders assigned user from another study"
                                            },
                                            "reason": {
                                                "by": "set from calculation",
                                                "byUniqueId": null,
                                                "role": "System",
                                                "when": "2023-08-07T15:15:41Z",
                                                "value": "calculated value"
                                            }
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

        let expected = UserNative {
            users: vec![User {
                unique_id: "1691421275437".to_string(),
                last_language: None,
                creator: "Paul Sanders(1681162687395)".to_string(),
                number_of_forms: 1,
                forms: Some(vec![Form {
                    name: "form.name.demographics".to_string(),
                    last_modified: Some(
                        DateTime::parse_from_rfc3339("2023-08-07T15:15:41Z")
                            .unwrap()
                            .with_timezone(&Utc),
                    ),
                    who_last_modified_name: Some("Paul Sanders".to_string()),
                    who_last_modified_role: Some("Project Manager".to_string()),
                    when_created: 1691421341578,
                    has_errors: false,
                    has_warnings: false,
                    locked: false,
                    user: None,
                    date_time_changed: None,
                    form_title: "User Demographics".to_string(),
                    form_index: 1,
                    form_group: None,
                    form_state: "In-Work".to_string(),
                    states: Some(vec![State {
                        value: "form.state.in.work".to_string(),
                        signer: "Paul Sanders - Project Manager".to_string(),
                        signer_unique_id: "1681162687395".to_string(),
                        date_signed: Some(
                            DateTime::parse_from_rfc3339("2023-08-07T15:15:41Z")
                                .unwrap()
                                .with_timezone(&Utc),
                        ),
                    }]),
                    categories: Some(vec![
                                Category {
                                    name: "demographics".to_string(),
                                    category_type: "normal".to_string(),
                                    highest_index: 0,
                                    fields: Some(vec![
                                        Field {
                                            name: "address".to_string(),
                                            field_type: "text".to_string(),
                                            data_type: Some("string".to_string()),
                                            error_code: "undefined".to_string(),
                                            when_created: DateTime::parse_from_rfc3339("2024-01-12T20:14:09Z")
                                                .unwrap()
                                                .with_timezone(&Utc),
                                            keep_history: true,
                                            entries: None,
                                        },
                                        Field {
                                            name: "email".to_string(),
                                            field_type: "text".to_string(),
                                            data_type: Some("string".to_string()),
                                            error_code: "undefined".to_string(),
                                            when_created: DateTime::parse_from_rfc3339("2023-08-07T15:15:41Z")
                                                .unwrap()
                                                .with_timezone(&Utc),
                                            keep_history: true,
                                            entries: Some(vec![Entry {
                                                entry_id: "1".to_string(),
                                                value: Some(Value {
                                                    by: "Paul Sanders".to_string(),
                                                    by_unique_id: Some("1681162687395".to_string()),
                                                    role: "Project Manager".to_string(),
                                                    when: DateTime::parse_from_rfc3339("2023-08-07T15:15:41Z")
                                                        .unwrap()
                                                        .with_timezone(&Utc),
                                                    value: "jazz@artemis.com".to_string(),
                                                }),
                                                reason: None,
                                            }]),
                                        },
                                    ]),
                                },
                                Category {
                                    name: "Administrative".to_string(),
                                    category_type: "normal".to_string(),
                                    highest_index: 0,
                                    fields: Some(vec![
                                        Field {
                                            name: "study_assignment".to_string(),
                                            field_type: "text".to_string(),
                                            data_type: None,
                                            error_code: "undefined".to_string(),
                                            when_created: DateTime::parse_from_rfc3339("2023-08-07T15:15:41Z")
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
                                                        when: DateTime::parse_from_rfc3339("2023-08-07T15:15:41Z")
                                                            .unwrap()
                                                            .with_timezone(&Utc),
                                                        value: "On 07-Aug-2023 10:15 -0500, Paul Sanders assigned user from another study".to_string(),
                                                    }),
                                                    reason: Some(Reason {
                                                        by: "set from calculation".to_string(),
                                                        by_unique_id: None,
                                                        role: "System".to_string(),
                                                        when: DateTime::parse_from_rfc3339("2023-08-07T15:15:41Z")
                                                            .unwrap()
                                                            .with_timezone(&Utc),
                                                        value: "calculated value".to_string(),
                                                    }),
                                                },
                                            ]),
                                        },
                                    ]),
                                },
                    ]),
                }]),
            }],
        };

        let result: UserNative = serde_json::from_str(json_str).unwrap();

        assert_eq!(result, expected);
    }
}
