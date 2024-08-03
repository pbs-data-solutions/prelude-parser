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
    use insta::assert_yaml_snapshot;

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

        let result: UserNative = serde_json::from_str(json_str).unwrap();

        assert_yaml_snapshot!(result);
    }
}
