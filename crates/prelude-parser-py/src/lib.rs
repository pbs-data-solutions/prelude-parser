mod errors;
mod utils;

use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::PathBuf;

use chrono::{Datelike, NaiveDate};
pub use prelude_xml_parser::native::{
    common::{Category, Comment, Entry, Field, Form, LockState, Reason, State, Value},
    site_native::{Site, SiteNative},
    subject_native::{Patient, SubjectNative},
    user_native::{User, UserNative},
};
use prelude_xml_parser::parse_site_native_file as parse_site_native_file_rs;
use prelude_xml_parser::parse_site_native_string as parse_site_native_string_rs;
use prelude_xml_parser::parse_subject_native_file as parse_subject_native_file_rs;
use prelude_xml_parser::parse_subject_native_string as parse_subject_native_string_rs;
use prelude_xml_parser::parse_user_native_file as parse_user_native_file_rs;
use prelude_xml_parser::parse_user_native_string as parse_user_native_string_rs;
use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyDict, PyList, PyString};
use roxmltree::Document;

use crate::errors::{
    FileNotFoundError, InvalidFileTypeError, ParsingError, XmlFileValidationError,
};
use crate::utils::{to_snake, validate_file};

fn check_valid_file(xml_file: &PathBuf) -> PyResult<()> {
    if let Err(e) = validate_file(xml_file) {
        match e {
            XmlFileValidationError::FileNotFound(_) => {
                return Err(FileNotFoundError::new_err(format!(
                    "File not found: {xml_file:?}"
                )))
            }
            XmlFileValidationError::InvalidFileType(_) => {
                return Err(InvalidFileTypeError::new_err(format!(
                    "{xml_file:?} is not an xml file"
                )))
            }
            XmlFileValidationError::NoFileExtension(_) => {
                return Err(InvalidFileTypeError::new_err(
                    "No file extension found in file: {xml_file:?}",
                ))
            }
        };
    };

    Ok(())
}

fn py_list_append<'py>(
    py: Python<'py>,
    value: Option<&str>,
    list: &'py Bound<'py, PyList>,
    date: &Bound<'py, PyAny>,
) -> PyResult<&'py Bound<'py, PyList>> {
    match value {
        Some(t) => match t.parse::<usize>() {
            Ok(int_val) => list.append(int_val)?,
            Err(_) => match t.parse::<f64>() {
                Ok(float_val) => list.append(float_val)?,
                Err(_) => match NaiveDate::parse_from_str(t, "%d-%b-%Y") {
                    Ok(dt) => {
                        let py_date = date.call1((dt.year(), dt.month(), dt.day()))?;
                        list.append(py_date)?;
                    }
                    Err(_) => list.append(t)?,
                },
            },
        },
        None => list.append(py.None())?,
    };

    Ok(list)
}

fn add_item<'py>(
    py: Python<'py>,
    key: &Bound<'py, PyString>,
    value: Option<&str>,
    form_data: &'py Bound<'py, PyDict>,
    date: &Bound<'py, PyAny>,
) -> PyResult<&'py Bound<'py, PyDict>> {
    match value {
        Some(t) => match t.parse::<usize>() {
            Ok(int_val) => form_data.set_item(key, int_val)?,
            Err(_) => match t.parse::<f64>() {
                Ok(float_val) => form_data.set_item(key, float_val)?,
                Err(_) => match NaiveDate::parse_from_str(t, "%d-%b-%Y") {
                    Ok(dt) => {
                        let py_date = date.call1((dt.year(), dt.month(), dt.day()))?;
                        form_data.set_item(key, py_date)?;
                    }
                    Err(_) => form_data.set_item(key, t)?,
                },
            },
        },
        None => form_data.set_item(key, py.None())?,
    };

    Ok(form_data)
}

fn convert_name(raw: &str, short_names: bool) -> String {
    if short_names {
        raw.to_lowercase()
    } else {
        to_snake(raw)
    }
}

fn parse_xml<'py>(
    py: Python<'py>,
    xml_file: &PathBuf,
    short_names: bool,
) -> PyResult<Bound<'py, PyDict>> {
    let date = py.import("datetime")?.getattr("date")?;
    let reader = read_to_string(xml_file);

    match reader {
        Ok(r) => match Document::parse(&r) {
            Ok(doc) => {
                let mut data: HashMap<String, Vec<Bound<'_, PyDict>>> = HashMap::new();
                let mut form_names: HashMap<&str, String> = HashMap::new();
                let mut field_names: HashMap<&str, Py<PyString>> = HashMap::new();
                let tree = doc.root_element();

                for form in tree.children() {
                    let raw_form = form.tag_name().name();
                    let form_name = match form_names.get(raw_form) {
                        Some(name) => name,
                        None => form_names
                            .entry(raw_form)
                            .or_insert_with(|| convert_name(raw_form, short_names)),
                    };

                    if form_name.is_empty() {
                        continue;
                    }

                    let form_data = PyDict::new(py);
                    for child in form.children() {
                        if !child.is_element() {
                            continue;
                        }

                        let raw_field = child.tag_name().name();
                        if raw_field.is_empty() {
                            continue;
                        }

                        let key = match field_names.get(raw_field) {
                            Some(key) => key.clone_ref(py),
                            None => {
                                let key = PyString::new(py, &convert_name(raw_field, short_names))
                                    .unbind();
                                field_names.insert(raw_field, key.clone_ref(py));
                                key
                            }
                        };

                        add_item(py, key.bind(py), child.text(), &form_data, &date)?;
                    }

                    data.entry(form_name.clone()).or_default().push(form_data);
                }

                let data_dict = data.into_py_dict(py)?;
                Ok(data_dict)
            }
            Err(e) => Err(ParsingError::new_err(format!(
                "Error parsing xml file: {e:?}"
            ))),
        },
        Err(e) => Err(ParsingError::new_err(format!(
            "Error parsing xml file: {e:?}"
        ))),
    }
}

fn parse_xml_pandas<'py>(
    py: Python<'py>,
    xml_file: &PathBuf,
    short_names: bool,
) -> PyResult<Bound<'py, PyDict>> {
    let date = py.import("datetime")?.getattr("date")?;
    let reader = read_to_string(xml_file);

    match reader {
        Ok(r) => match Document::parse(&r) {
            Ok(doc) => {
                let data = PyDict::new(py);
                let tree = doc.root_element();

                for form in tree.children() {
                    for child in form.children() {
                        if child.is_element() && child.tag_name().name() != "" {
                            let column = if short_names {
                                child.tag_name().name().to_owned().to_lowercase()
                            } else {
                                to_snake(child.tag_name().name())
                            };
                            if let Ok(Some(c)) = data.get_item(&column) {
                                py_list_append(py, child.text(), &c.extract()?, &date)?;
                                data.set_item(column, c)?;
                            } else {
                                let list = PyList::empty(py);
                                py_list_append(py, child.text(), &list, &date)?;
                                data.set_item(column, list)?;
                            }
                        }
                    }
                }
                let data_dict = data.into_py_dict(py)?;
                Ok(data_dict)
            }
            Err(e) => Err(ParsingError::new_err(format!(
                "Error parsing xml file: {e:?}"
            ))),
        },
        Err(e) => Err(ParsingError::new_err(format!(
            "Error parsing xml file: {e:?}"
        ))),
    }
}

#[pyfunction]
#[pyo3(signature = (xml_file, *, short_names=false))]
fn _parse_flat_file_to_dict<'py>(
    py: Python<'py>,
    xml_file: PathBuf,
    short_names: bool,
) -> PyResult<Bound<'py, PyDict>> {
    check_valid_file(&xml_file)?;
    let data = parse_xml(py, &xml_file, short_names)?;

    Ok(data)
}

#[pyfunction]
#[pyo3(signature = (xml_file, *, short_names=false))]
fn _parse_flat_file_to_pandas_dict<'py>(
    py: Python<'py>,
    xml_file: PathBuf,
    short_names: bool,
) -> PyResult<Bound<'py, PyDict>> {
    check_valid_file(&xml_file)?;
    let data = parse_xml_pandas(py, &xml_file, short_names)?;

    Ok(data)
}

#[pyfunction]
#[pyo3(signature = (xml_file))]
fn parse_site_native_file(py: Python, xml_file: PathBuf) -> PyResult<SiteNative> {
    let result = py.detach(|| parse_site_native_file_rs(&xml_file));
    match result {
        Ok(native) => Ok(native),
        Err(e) => Err(ParsingError::new_err(format!(
            "Error parsing xml file: {e:?}"
        ))),
    }
}

#[pyfunction]
#[pyo3(signature = (xml_str))]
fn parse_site_native_string(py: Python, xml_str: &str) -> PyResult<SiteNative> {
    let result = py.detach(|| parse_site_native_string_rs(xml_str));

    match result {
        Ok(native) => Ok(native),
        Err(e) => Err(ParsingError::new_err(format!("Error parsing xml: {e:?}"))),
    }
}

#[pyfunction]
#[pyo3(signature = (xml_file))]
fn parse_subject_native_file(py: Python, xml_file: PathBuf) -> PyResult<SubjectNative> {
    let result = py.detach(|| parse_subject_native_file_rs(&xml_file));

    match result {
        Ok(native) => Ok(native),
        Err(e) => Err(ParsingError::new_err(format!(
            "Error parsing xml file: {e:?}"
        ))),
    }
}

#[pyfunction]
#[pyo3(signature = (xml_str))]
fn parse_subject_native_string(py: Python, xml_str: &str) -> PyResult<SubjectNative> {
    let result = py.detach(|| parse_subject_native_string_rs(xml_str));
    match result {
        Ok(native) => Ok(native),
        Err(e) => Err(ParsingError::new_err(format!("Error parsing xml: {e:?}"))),
    }
}

#[pyfunction]
#[pyo3(signature = (xml_file))]
fn parse_user_native_file(py: Python, xml_file: PathBuf) -> PyResult<UserNative> {
    let result = py.detach(|| parse_user_native_file_rs(&xml_file));

    match result {
        Ok(native) => Ok(native),
        Err(e) => Err(ParsingError::new_err(format!(
            "Error parsing xml file: {e:?}"
        ))),
    }
}

#[pyfunction]
#[pyo3(signature = (xml_str))]
fn parse_user_native_string(py: Python, xml_str: &str) -> PyResult<UserNative> {
    let result = py.detach(|| parse_user_native_string_rs(xml_str));

    match result {
        Ok(native) => Ok(native),
        Err(e) => Err(ParsingError::new_err(format!("Error parsing xml: {e:?}"))),
    }
}

#[pymodule]
fn _prelude_parser(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Category>()?;
    m.add_class::<Comment>()?;
    m.add_class::<Entry>()?;
    m.add_class::<Field>()?;
    m.add_class::<Form>()?;
    m.add_class::<LockState>()?;
    m.add_class::<Patient>()?;
    m.add_class::<Reason>()?;
    m.add_class::<Site>()?;
    m.add_class::<SiteNative>()?;
    m.add_class::<State>()?;
    m.add_class::<SubjectNative>()?;
    m.add_class::<User>()?;
    m.add_class::<UserNative>()?;
    m.add_class::<Value>()?;
    m.add_function(wrap_pyfunction!(_parse_flat_file_to_dict, m)?)?;
    m.add_function(wrap_pyfunction!(_parse_flat_file_to_pandas_dict, m)?)?;
    m.add_function(wrap_pyfunction!(parse_site_native_file, m)?)?;
    m.add_function(wrap_pyfunction!(parse_site_native_string, m)?)?;
    m.add_function(wrap_pyfunction!(parse_subject_native_file, m)?)?;
    m.add_function(wrap_pyfunction!(parse_subject_native_string, m)?)?;
    m.add_function(wrap_pyfunction!(parse_user_native_file, m)?)?;
    m.add_function(wrap_pyfunction!(parse_user_native_string, m)?)?;
    m.add("FileNotFoundError", py.get_type::<FileNotFoundError>())?;
    m.add(
        "InvalidFileTypeError",
        py.get_type::<InvalidFileTypeError>(),
    )?;
    m.add("ParsingError", py.get_type::<ParsingError>())?;
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_snake() {
        assert_eq!(
            to_snake("i_communications_Details"),
            String::from("i_communications_details")
        );
    }
}
