mod errors;
mod utils;

use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::PathBuf;

use chrono::{Datelike, NaiveDate};
pub use prelude_xml_parser::native::{site_native::SiteNative, subject_native::SubjectNative};
use prelude_xml_parser::parse_site_native_file as parse_site_native_file_rs;
use prelude_xml_parser::parse_subject_native_file as parse_subject_native_file_rs;
use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyDict, PyList};
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
                    "File not found: {:?}",
                    xml_file
                )))
            }
            XmlFileValidationError::InvalidFileType(_) => {
                return Err(InvalidFileTypeError::new_err(format!(
                    "{:?} is not an xml file",
                    xml_file
                )))
            }
        };
    };

    Ok(())
}

fn py_list_append<'py>(
    py: Python<'py>,
    value: Option<&str>,
    list: &'py Bound<'py, PyList>,
) -> PyResult<&'py Bound<'py, PyList>> {
    let datetime = py.import_bound("datetime")?;
    let date = datetime.getattr("date")?;

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
    key: &str,
    value: Option<&str>,
    form_data: &'py Bound<'py, PyDict>,
) -> PyResult<&'py Bound<'py, PyDict>> {
    let datetime = py.import_bound("datetime")?;
    let date = datetime.getattr("date")?;

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

fn parse_xml<'py>(
    py: Python<'py>,
    xml_file: &PathBuf,
    short_names: bool,
) -> PyResult<Bound<'py, PyDict>> {
    let reader = read_to_string(xml_file);

    match reader {
        Ok(r) => match Document::parse(&r) {
            Ok(doc) => {
                let mut data: HashMap<String, Vec<Bound<'_, PyDict>>> = HashMap::new();
                let tree = doc.root_element();
                for form in tree.children() {
                    let form_name = if short_names {
                        form.tag_name().name().to_owned().to_lowercase()
                    } else {
                        to_snake(form.tag_name().name())
                    };
                    if !form_name.is_empty() {
                        if let Some(d) = data.get_mut(&form_name) {
                            let form_data = PyDict::new_bound(py);
                            for child in form.children() {
                                if child.is_element() && child.tag_name().name() != "" {
                                    let key = if short_names {
                                        child.tag_name().name().to_owned().to_lowercase()
                                    } else {
                                        to_snake(child.tag_name().name())
                                    };
                                    add_item(py, &key, child.text(), &form_data)?;
                                };
                            }
                            d.push(form_data);
                        } else {
                            let mut items: Vec<Bound<'_, PyDict>> = Vec::new();
                            let form_data = PyDict::new_bound(py);
                            for child in form.children() {
                                if child.is_element() && child.tag_name().name() != "" {
                                    let key = if short_names {
                                        child.tag_name().name().to_owned().to_lowercase()
                                    } else {
                                        to_snake(child.tag_name().name())
                                    };
                                    add_item(py, &key, child.text(), &form_data)?;
                                }
                            }
                            items.push(form_data.into_py_dict_bound(py));
                            data.insert(form_name, items);
                        }
                    }
                }
                return Ok(data.into_py_dict_bound(py));
            }
            Err(e) => Err(ParsingError::new_err(format!(
                "Error parsing xml file: {:?}",
                e
            ))),
        },
        Err(e) => Err(ParsingError::new_err(format!(
            "Error parsing xml file: {:?}",
            e
        ))),
    }
}

fn parse_xml_pandas<'py>(
    py: Python<'py>,
    xml_file: &PathBuf,
    short_names: bool,
) -> PyResult<Bound<'py, PyDict>> {
    let reader = read_to_string(xml_file);

    match reader {
        Ok(r) => match Document::parse(&r) {
            Ok(doc) => {
                let data = PyDict::new_bound(py);
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
                                py_list_append(py, child.text(), &c.extract()?)?;
                                data.set_item(column, c)?;
                            } else {
                                let list = PyList::empty_bound(py);
                                py_list_append(py, child.text(), &list)?;
                                data.set_item(column, list)?;
                            }
                        }
                    }
                }
                return Ok(data.into_py_dict_bound(py));
            }
            Err(e) => Err(ParsingError::new_err(format!(
                "Error parsing xml file: {:?}",
                e
            ))),
        },
        Err(e) => Err(ParsingError::new_err(format!(
            "Error parsing xml file: {:?}",
            e
        ))),
    }
}

#[pyfunction]
fn _parse_flat_file_to_dict(
    py: Python,
    xml_file: PathBuf,
    short_names: bool,
) -> PyResult<Bound<'_, PyDict>> {
    check_valid_file(&xml_file)?;
    let data = parse_xml(py, &xml_file, short_names)?;

    Ok(data)
}

#[pyfunction]
fn _parse_flat_file_to_pandas_dict(
    py: Python,
    xml_file: PathBuf,
    short_names: bool,
) -> PyResult<Bound<'_, PyDict>> {
    check_valid_file(&xml_file)?;
    let data = parse_xml_pandas(py, &xml_file, short_names)?;

    Ok(data)
}

#[pyfunction]
fn parse_site_native_file(_py: Python, xml_file: PathBuf) -> PyResult<SiteNative> {
    match parse_site_native_file_rs(&xml_file) {
        Ok(native) => Ok(native),
        Err(e) => Err(ParsingError::new_err(format!(
            "Error parsing xml file: {:?}",
            e
        ))),
    }
}

#[pyfunction]
fn parse_subject_native_file(_py: Python, xml_file: PathBuf) -> PyResult<SubjectNative> {
    match parse_subject_native_file_rs(&xml_file) {
        Ok(native) => Ok(native),
        Err(e) => Err(ParsingError::new_err(format!(
            "Error parsing xml file: {:?}",
            e
        ))),
    }
}

#[pymodule]
fn _prelude_parser(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<SiteNative>()?;
    m.add_class::<SubjectNative>()?;
    m.add_function(wrap_pyfunction!(_parse_flat_file_to_dict, m)?)?;
    m.add_function(wrap_pyfunction!(_parse_flat_file_to_pandas_dict, m)?)?;
    m.add_function(wrap_pyfunction!(parse_site_native_file, m)?)?;
    m.add_function(wrap_pyfunction!(parse_subject_native_file, m)?)?;
    m.add(
        "FileNotFoundError",
        py.get_type_bound::<FileNotFoundError>(),
    )?;
    m.add(
        "InvalidFileTypeError",
        py.get_type_bound::<InvalidFileTypeError>(),
    )?;
    m.add("ParsingError", py.get_type_bound::<ParsingError>())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use super::*;

    #[test]
    fn test_to_snake() {
        assert_eq!(
            to_snake("i_communications_Details"),
            String::from("i_communications_details")
        );
    }
}
