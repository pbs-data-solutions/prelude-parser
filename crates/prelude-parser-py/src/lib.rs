mod errors;
mod utils;

use std::{collections::HashMap, fs::read_to_string, path::PathBuf, str::from_utf8};

use chrono::{Datelike, NaiveDate};
use prelude_xml_parser::{
    native::{
        common::{Category, Comment, Entry, Field, Form, LockState, Reason, State, Value},
        site_native::{Site, SiteNative},
        subject_native::{Patient, SubjectNative},
        user_native::{User, UserNative},
    },
    parse_site_native_file as parse_site_native_file_rs,
    parse_site_native_string as parse_site_native_string_rs,
    parse_subject_native_file as parse_subject_native_file_rs,
    parse_subject_native_string as parse_subject_native_string_rs,
    parse_user_native_file as parse_user_native_file_rs,
    parse_user_native_string as parse_user_native_string_rs,
};
use pyo3::{
    prelude::*,
    types::{IntoPyDict, PyDict, PyList, PyString},
};
use quick_xml::{
    escape::resolve_predefined_entity,
    events::{BytesRef, Event},
    Reader,
};

use crate::{
    errors::{FileNotFoundError, InvalidFileTypeError, ParsingError, XmlFileValidationError},
    utils::{to_snake, validate_file},
};

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

/// Map a `prelude-xml-parser` error onto the matching Python exception.
///
/// The crate distinguishes a missing file and a wrong file type from an actual parse failure, so
/// the bindings surface the same distinction rather than collapsing everything into
/// `ParsingError`. Messages match those raised by the flat-file path.
fn native_error(e: prelude_xml_parser::errors::Error) -> PyErr {
    use prelude_xml_parser::errors::Error as NativeError;

    match e {
        NativeError::FileNotFound(path) => {
            FileNotFoundError::new_err(format!("File not found: {path:?}"))
        }
        NativeError::InvalidFileType(path) => {
            InvalidFileTypeError::new_err(format!("{path:?} is not an xml file"))
        }
        other => ParsingError::new_err(format!("Error parsing xml file: {other:?}")),
    }
}

fn xml_error(e: impl std::fmt::Display) -> PyErr {
    ParsingError::new_err(format!("Error parsing xml file: {e}"))
}

/// Append a `&...;` reference to the text being accumulated.
///
/// quick-xml reports references as their own events, so a value's text has to be reassembled from
/// the surrounding text events and these. This matches what a DOM parser hands back as the node's
/// text. An unresolvable reference is kept verbatim rather than dropped.
fn push_general_ref(text: &mut String, reference: &BytesRef<'_>) -> PyResult<()> {
    if let Some(character) = reference.resolve_char_ref().map_err(xml_error)? {
        text.push(character);
        return Ok(());
    }

    let name = reference.decode().map_err(xml_error)?;

    match resolve_predefined_entity(&name) {
        Some(resolved) => text.push_str(resolved),
        None => {
            text.push('&');
            text.push_str(&name);
            text.push(';');
        }
    }

    Ok(())
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
    let contents = read_to_string(xml_file).map_err(xml_error)?;

    let mut reader = Reader::from_str(&contents);
    reader.config_mut().trim_text(false);

    let mut data: HashMap<String, Vec<Bound<'_, PyDict>>> = HashMap::new();
    let mut form_names: HashMap<Vec<u8>, String> = HashMap::new();
    let mut field_names: HashMap<Vec<u8>, Py<PyString>> = HashMap::new();

    let mut depth = 0usize;
    let mut saw_root = false;
    let mut form_name: Option<String> = None;
    let mut form_data: Option<Bound<'_, PyDict>> = None;
    let mut field_key: Option<Py<PyString>> = None;
    let mut text: Option<String> = None;

    loop {
        match reader.read_event().map_err(xml_error)? {
            Event::Eof => break,

            Event::Start(e) => {
                depth += 1;
                saw_root = true;
                match depth {
                    2 => {
                        let raw = e.name().into_inner();
                        let name = match form_names.get(raw) {
                            Some(name) => name.clone(),
                            None => {
                                let name =
                                    convert_name(from_utf8(raw).map_err(xml_error)?, short_names);
                                form_names.insert(raw.to_vec(), name.clone());
                                name
                            }
                        };
                        form_name = Some(name);
                        form_data = Some(PyDict::new(py));
                    }
                    3 => {
                        field_key = Some(field_key_for(py, &e, short_names, &mut field_names)?);
                        text = None;
                    }
                    _ => {}
                }
            }

            Event::Empty(e) => match depth + 1 {
                2 => {
                    let raw = e.name().into_inner();
                    let name = match form_names.get(raw) {
                        Some(name) => name.clone(),
                        None => {
                            let name =
                                convert_name(from_utf8(raw).map_err(xml_error)?, short_names);
                            form_names.insert(raw.to_vec(), name.clone());
                            name
                        }
                    };
                    if !name.is_empty() {
                        data.entry(name).or_default().push(PyDict::new(py));
                    }
                }
                3 => {
                    if let Some(ref dict) = form_data {
                        let key = field_key_for(py, &e, short_names, &mut field_names)?;
                        add_item(py, key.bind(py), None, dict, &date)?;
                    }
                }
                _ => {}
            },

            Event::Text(e) if depth == 3 => {
                // xml10_content normalizes \r\n and \r to \n as the XML spec requires; decode does
                // not
                let decoded = e.xml10_content().map_err(xml_error)?;
                text.get_or_insert_with(String::new).push_str(&decoded);
            }

            Event::GeneralRef(ref e) if depth == 3 => {
                push_general_ref(text.get_or_insert_with(String::new), e)?;
            }

            Event::End(_) => {
                match depth {
                    3 => {
                        if let (Some(ref dict), Some(key)) = (&form_data, field_key.take()) {
                            add_item(py, key.bind(py), text.as_deref(), dict, &date)?;
                        }
                        text = None;
                    }
                    2 => {
                        if let (Some(name), Some(dict)) = (form_name.take(), form_data.take()) {
                            if !name.is_empty() {
                                data.entry(name).or_default().push(dict);
                            }
                        }
                    }
                    _ => {}
                }
                depth -= 1;
            }

            _ => {}
        }
    }

    if !saw_root {
        return Err(ParsingError::new_err(
            "Error parsing xml file: no root element found",
        ));
    }

    let data_dict = data.into_py_dict(py)?;
    Ok(data_dict)
}

fn field_key_for<'py>(
    py: Python<'py>,
    e: &quick_xml::events::BytesStart<'_>,
    short_names: bool,
    cache: &mut HashMap<Vec<u8>, Py<PyString>>,
) -> PyResult<Py<PyString>> {
    let raw = e.name().into_inner();
    if let Some(key) = cache.get(raw) {
        return Ok(key.clone_ref(py));
    }

    let key = PyString::new(
        py,
        &convert_name(from_utf8(raw).map_err(xml_error)?, short_names),
    )
    .unbind();
    cache.insert(raw.to_vec(), key.clone_ref(py));

    Ok(key)
}

fn parse_xml_pandas<'py>(
    py: Python<'py>,
    xml_file: &PathBuf,
    short_names: bool,
) -> PyResult<Bound<'py, PyDict>> {
    let date = py.import("datetime")?.getattr("date")?;
    let contents = read_to_string(xml_file).map_err(xml_error)?;

    let mut reader = Reader::from_str(&contents);
    reader.config_mut().trim_text(false);

    let data = PyDict::new(py);
    let mut columns: HashMap<Vec<u8>, Py<PyList>> = HashMap::new();

    let mut depth = 0usize;
    let mut saw_root = false;
    let mut column: Option<Py<PyList>> = None;
    let mut text: Option<String> = None;

    loop {
        match reader.read_event().map_err(xml_error)? {
            Event::Eof => break,

            Event::Start(e) => {
                depth += 1;
                saw_root = true;
                if depth == 3 {
                    column = Some(column_for(py, &e, short_names, &mut columns, &data)?);
                    text = None;
                }
            }

            Event::Empty(e) => {
                if depth + 1 == 3 {
                    let list = column_for(py, &e, short_names, &mut columns, &data)?;
                    py_list_append(py, None, list.bind(py), &date)?;
                }
            }

            Event::Text(e) if depth == 3 => {
                let decoded = e.xml10_content().map_err(xml_error)?;
                text.get_or_insert_with(String::new).push_str(&decoded);
            }

            Event::GeneralRef(ref e) if depth == 3 => {
                push_general_ref(text.get_or_insert_with(String::new), e)?;
            }

            Event::End(_) => {
                if depth == 3 {
                    if let Some(list) = column.take() {
                        py_list_append(py, text.as_deref(), list.bind(py), &date)?;
                    }
                    text = None;
                }
                depth -= 1;
            }

            _ => {}
        }
    }

    if !saw_root {
        return Err(ParsingError::new_err(
            "Error parsing xml file: no root element found",
        ));
    }

    let data_dict = data.into_py_dict(py)?;
    Ok(data_dict)
}

/// Look up (or create) the list backing a column, keyed by the raw tag so the name is converted
/// once per distinct field rather than once per occurrence.
fn column_for<'py>(
    py: Python<'py>,
    e: &quick_xml::events::BytesStart<'_>,
    short_names: bool,
    cache: &mut HashMap<Vec<u8>, Py<PyList>>,
    data: &Bound<'py, PyDict>,
) -> PyResult<Py<PyList>> {
    let raw = e.name().into_inner();
    if let Some(list) = cache.get(raw) {
        return Ok(list.clone_ref(py));
    }

    let name = convert_name(from_utf8(raw).map_err(xml_error)?, short_names);
    let list = PyList::empty(py);
    data.set_item(name, &list)?;

    let list = list.unbind();
    cache.insert(raw.to_vec(), list.clone_ref(py));

    Ok(list)
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
        Err(e) => Err(native_error(e)),
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
        Err(e) => Err(native_error(e)),
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
        Err(e) => Err(native_error(e)),
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
