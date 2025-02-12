use serde::{Deserialize, Serialize};

#[cfg(feature = "python")]
use pyo3::{exceptions::PyValueError, prelude::*, types::PyDict};

pub use crate::native::{
    common::{Category, Comment, Entry, Field, Form, Reason, State, Value},
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

#[cfg(feature = "python")]
#[pymethods]
impl User {
    #[getter]
    fn unique_id(&self) -> PyResult<String> {
        Ok(self.unique_id.clone())
    }

    #[getter]
    fn last_language(&self) -> PyResult<Option<String>> {
        Ok(self.last_language.clone())
    }

    #[getter]
    fn creator(&self) -> PyResult<String> {
        Ok(self.creator.clone())
    }

    #[getter]
    fn forms(&self) -> PyResult<Option<Vec<Form>>> {
        Ok(self.forms.clone())
    }

    pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new(py);
        dict.set_item("unique_id", &self.unique_id)?;
        dict.set_item("last_language", &self.last_language)?;
        dict.set_item("creator", &self.creator)?;
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
/// Contains the information from the Prelude native user XML.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UserNative {
    #[serde(alias = "user")]
    pub users: Vec<User>,
}

#[cfg(not(feature = "python"))]
impl UserNative {
    /// Convert to a JSON string
    ///
    /// # Example
    ///
    /// ```
    /// use std::path::Path;
    ///
    /// use prelude_xml_parser::parse_user_native_file;
    ///
    /// let file_path = Path::new("tests/assets/user_native_small.xml");
    /// let native = parse_user_native_file(&file_path).unwrap();
    /// let expected = r#"{"users":[{"uniqueId":"1691421275437","lastLanguage":null,"creator":"Paul Sanders(1681162687395)","numberOfForms":1,"forms":[{"name":"form.name.demographics","lastModified":"2023-08-07T15:15:41Z","whoLastModifiedName":"Paul Sanders","whoLastModifiedRole":"Project Manager","whenCreated":1691421341578,"hasErrors":false,"hasWarnings":false,"locked":false,"user":null,"dateTimeChanged":null,"formTitle":"User Demographics","formIndex":1,"formGroup":null,"formState":"In-Work","states":[{"value":"form.state.in.work","signer":"Paul Sanders - Project Manager","signerUniqueId":"1681162687395","dateSigned":"2023-08-07T15:15:41Z"}],"categories":[{"name":"demographics","categoryType":"normal","highestIndex":0,"fields":[{"name":"address","fieldType":"text","dataType":"string","errorCode":"undefined","whenCreated":"2024-01-12T20:14:09Z","keepHistory":true,"entries":null,"comments":null},{"name":"email","fieldType":"text","dataType":"string","errorCode":"undefined","whenCreated":"2023-08-07T15:15:41Z","keepHistory":true,"entries":[{"entryId":"1","value":{"by":"Paul Sanders","byUniqueId":"1681162687395","role":"Project Manager","when":"2023-08-07T15:15:41Z","value":"jazz@artemis.com"},"reason":null}],"comments":[{"commentId":"1","value":{"by":"Paul Sanders","byUniqueId":"1681162687395","role":"Project Manager","when":"2023-04-15T16:09:02Z","value":"Some Comment"}}]}]},{"name":"Administrative","categoryType":"normal","highestIndex":0,"fields":[{"name":"study_assignment","fieldType":"text","dataType":null,"errorCode":"undefined","whenCreated":"2023-08-07T15:15:41Z","keepHistory":true,"entries":[{"entryId":"1","value":{"by":"set from calculation","byUniqueId":null,"role":"System","when":"2023-08-07T15:15:41Z","value":"On 07-Aug-2023 10:15 -0500, Paul Sanders assigned user from another study"},"reason":{"by":"set from calculation","byUniqueId":null,"role":"System","when":"2023-08-07T15:15:41Z","value":"calculated value"}}],"comments":null}]}]}]},{"uniqueId":"1681162687395","lastLanguage":null,"creator":"1609858291483(1609858291483)","numberOfForms":2,"forms":[{"name":"form.name.demographics","lastModified":"2023-04-10T21:39:30Z","whoLastModifiedName":"1609858291483","whoLastModifiedRole":"role.administrator","whenCreated":1681162770378,"hasErrors":false,"hasWarnings":false,"locked":false,"user":null,"dateTimeChanged":null,"formTitle":"User Demographics","formIndex":1,"formGroup":null,"formState":"In-Work","states":[{"value":"form.state.in.work","signer":"1609858291483 - Administrator","signerUniqueId":"1609858291483","dateSigned":"2023-04-10T21:39:30Z"}],"categories":[{"name":"demographics","categoryType":"normal","highestIndex":0,"fields":[{"name":"address","fieldType":"text","dataType":"string","errorCode":"undefined","whenCreated":"2023-04-15T15:21:14Z","keepHistory":true,"entries":null,"comments":null},{"name":"email","fieldType":"text","dataType":"string","errorCode":"undefined","whenCreated":"2023-04-10T21:39:30Z","keepHistory":true,"entries":[{"entryId":"1","value":{"by":"1609858291483","byUniqueId":"1609858291483","role":"Administrator","when":"2023-04-10T21:39:30Z","value":"paul@pbsdatasolutions.com"},"reason":null}],"comments":null}]},{"name":"Administrative","categoryType":"normal","highestIndex":0,"fields":[{"name":"study_assignment","fieldType":"text","dataType":null,"errorCode":"undefined","whenCreated":"2023-04-10T21:39:30Z","keepHistory":true,"entries":[{"entryId":"1","value":{"by":"set from calculation","byUniqueId":null,"role":"System","when":"2023-04-10T21:39:30Z","value":"On 10-Apr-2023 16:39 -0500, 1609858291483 assigned user from another study"},"reason":{"by":"set from calculation","byUniqueId":null,"role":"System","when":"2023-04-10T21:39:30Z","value":"calculated value"}}],"comments":null}]}]},{"name":"training.form.name.checklist","lastModified":null,"whoLastModifiedName":null,"whoLastModifiedRole":null,"whenCreated":1702998014133,"hasErrors":false,"hasWarnings":false,"locked":false,"user":null,"dateTimeChanged":null,"formTitle":"Training Checklist","formIndex":1,"formGroup":"Training","formState":"New","states":[{"value":"form.state.new","signer":"System set","signerUniqueId":"system","dateSigned":"2023-12-19T15:00:14Z"}],"categories":[{"name":"Training_Modules","categoryType":"normal","highestIndex":0,"fields":[{"name":"date(1)","fieldType":"setToday","dataType":"date","errorCode":"undefined","whenCreated":"2023-12-19T15:00:15Z","keepHistory":true,"entries":null,"comments":null}]},{"name":"Other_Training_Modules","categoryType":"indexed","highestIndex":1,"fields":[{"name":"date_field(1)","fieldType":"setToday","dataType":"date","errorCode":"undefined","whenCreated":"2023-12-19T15:00:15Z","keepHistory":true,"entries":null,"comments":null}]},{"name":"21CFR11","categoryType":"normal","highestIndex":0,"fields":null}]}]}]}"#;
    /// let json = native.to_json().unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn to_json(&self) -> serde_json::Result<String> {
        let json = serde_json::to_string(&self)?;

        Ok(json)
    }
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

#[cfg(feature = "python")]
#[pymethods]
impl UserNative {
    #[getter]
    fn users(&self) -> PyResult<Vec<User>> {
        Ok(self.users.clone())
    }

    /// Convert the class instance to a dictionary
    fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new(py);
        let mut user_dicts = Vec::new();
        for user in &self.users {
            let user_dict = user.to_dict(py)?;
            user_dicts.push(user_dict);
        }
        dict.set_item("users", user_dicts)?;
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
