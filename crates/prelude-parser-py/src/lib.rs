mod errors;
mod utils;

use std::{collections::HashMap, fs::read_to_string, path::PathBuf, str::from_utf8};

use chrono::{Datelike, NaiveDate};
use prelude_xml_parser::{
    native::{
        common::{
            Category, Comment, Entry, Export, Field, File, Form, LockState, Query, Reason, State,
            Value,
        },
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
    types::{PyDict, PyList, PyString},
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

/// The type a whole column is given, decided from every value in it rather than value by value.
///
/// Per-value typing lets one column hold several Python types and, worse, merges distinct
/// identifiers: `"0067"` and `"67"` both become `67`. Deciding per column keeps a column's type
/// stable and keeps zero-padded identifiers intact.
#[derive(Clone, Copy, PartialEq, Eq)]
enum ColumnType {
    /// No non-empty value seen yet.
    Unknown,
    Integer,
    Float,
    Date,
    Text,
}

/// Classify a single value in isolation.
fn classify(value: &str) -> ColumnType {
    if value.is_empty() {
        return ColumnType::Unknown;
    }

    // A zero-padded number is an identifier, not a quantity: parsing it loses both the padding
    // and the distinction between "0067" and "67".
    let digits = value.strip_prefix(['+', '-']).unwrap_or(value);
    if digits.len() > 1 && digits.starts_with('0') && digits.bytes().all(|b| b.is_ascii_digit()) {
        return ColumnType::Text;
    }

    if value.parse::<i64>().is_ok() {
        return ColumnType::Integer;
    }

    // Requiring a digit keeps "nan", "inf" and "infinity" as the text they almost certainly are.
    if value.bytes().any(|b| b.is_ascii_digit()) && value.parse::<f64>().is_ok() {
        return ColumnType::Float;
    }

    if NaiveDate::parse_from_str(value, "%d-%b-%Y").is_ok() {
        return ColumnType::Date;
    }

    ColumnType::Text
}

/// Widen a column's type to also admit `value`.
fn widen(current: ColumnType, value: &str) -> ColumnType {
    let seen = classify(value);

    match (current, seen) {
        (ColumnType::Unknown, other) | (other, ColumnType::Unknown) => other,
        (a, b) if a == b => a,
        (ColumnType::Integer, ColumnType::Float) | (ColumnType::Float, ColumnType::Integer) => {
            ColumnType::Float
        }
        _ => ColumnType::Text,
    }
}

/// Convert a value according to the type decided for its column.
fn to_py_value<'py>(
    py: Python<'py>,
    value: Option<&str>,
    column_type: ColumnType,
    date: &Bound<'py, PyAny>,
) -> PyResult<Py<PyAny>> {
    let Some(text) = value else {
        return Ok(py.None());
    };

    if text.is_empty() {
        return Ok(py.None());
    }

    let converted = match column_type {
        ColumnType::Integer => match text.parse::<i64>() {
            Ok(v) => v.into_pyobject(py)?.into_any().unbind(),
            Err(_) => text.into_pyobject(py)?.into_any().unbind(),
        },
        ColumnType::Float => match text.parse::<f64>() {
            Ok(v) => v.into_pyobject(py)?.into_any().unbind(),
            Err(_) => text.into_pyobject(py)?.into_any().unbind(),
        },
        ColumnType::Date => match NaiveDate::parse_from_str(text, "%d-%b-%Y") {
            Ok(d) => date
                .call1((d.year(), d.month(), d.day()))?
                .into_any()
                .unbind(),
            Err(_) => text.into_pyobject(py)?.into_any().unbind(),
        },
        _ => text.into_pyobject(py)?.into_any().unbind(),
    };

    Ok(converted)
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

    let name = reference.xml10_content().map_err(xml_error)?;

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

/// A form's accumulated rows plus the columns discovered for it.
struct FormTable {
    name: String,
    keys: Vec<Py<PyString>>,
    types: Vec<ColumnType>,
    index: HashMap<Vec<u8>, usize>,
    rows: Vec<Vec<(usize, Option<String>)>>,
}

impl FormTable {
    fn column<'py>(&mut self, py: Python<'py>, raw: &[u8], short_names: bool) -> PyResult<usize> {
        if let Some(index) = self.index.get(raw) {
            return Ok(*index);
        }

        let name = convert_name(from_utf8(raw).map_err(xml_error)?, short_names);
        self.keys.push(PyString::new(py, &name).unbind());
        self.types.push(ColumnType::Unknown);
        self.index.insert(raw.to_vec(), self.keys.len() - 1);

        Ok(self.keys.len() - 1)
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

    // Forms are kept in the order they appear so the resulting dict is reproducible.
    let mut tables: Vec<FormTable> = Vec::new();
    let mut table_index: HashMap<Vec<u8>, usize> = HashMap::new();

    let mut depth = 0usize;
    let mut saw_root = false;
    let mut current_table: Option<usize> = None;
    let mut row: Vec<(usize, Option<String>)> = Vec::new();
    let mut column: Option<usize> = None;
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
                        let index = match table_index.get(raw) {
                            Some(index) => *index,
                            None => {
                                let name =
                                    convert_name(from_utf8(raw).map_err(xml_error)?, short_names);
                                tables.push(FormTable {
                                    name,
                                    keys: Vec::new(),
                                    types: Vec::new(),
                                    index: HashMap::new(),
                                    rows: Vec::new(),
                                });
                                table_index.insert(raw.to_vec(), tables.len() - 1);
                                tables.len() - 1
                            }
                        };
                        current_table = Some(index);
                        row = Vec::new();
                    }
                    3 => {
                        if let Some(index) = current_table {
                            column = Some(tables[index].column(
                                py,
                                e.name().into_inner(),
                                short_names,
                            )?);
                        }
                        text = None;
                    }
                    _ => {}
                }
            }

            Event::Empty(e) => match depth + 1 {
                2 => {
                    let raw = e.name().into_inner();
                    if !table_index.contains_key(raw) {
                        let name = convert_name(from_utf8(raw).map_err(xml_error)?, short_names);
                        tables.push(FormTable {
                            name,
                            keys: Vec::new(),
                            types: Vec::new(),
                            index: HashMap::new(),
                            rows: Vec::new(),
                        });
                        table_index.insert(raw.to_vec(), tables.len() - 1);
                    }
                    if let Some(index) = table_index.get(raw) {
                        let index = *index;
                        tables[index].rows.push(Vec::new());
                    }
                }
                3 => {
                    if let Some(index) = current_table {
                        let column =
                            tables[index].column(py, e.name().into_inner(), short_names)?;
                        row.push((column, None));
                    }
                }
                _ => {}
            },

            Event::Text(e) if depth == 3 => {
                let decoded = e.xml10_content().map_err(xml_error)?;
                text.get_or_insert_with(String::new).push_str(&decoded);
            }

            Event::GeneralRef(ref e) if depth == 3 => {
                push_general_ref(text.get_or_insert_with(String::new), e)?;
            }

            Event::End(_) => {
                match depth {
                    3 => {
                        if let (Some(index), Some(column)) = (current_table, column.take()) {
                            let value = text.take();
                            if let Some(ref value) = value {
                                tables[index].types[column] =
                                    widen(tables[index].types[column], value);
                            }
                            row.push((column, value));
                        }
                        text = None;
                    }
                    2 => {
                        if let Some(index) = current_table.take() {
                            tables[index].rows.push(std::mem::take(&mut row));
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

    let data = PyDict::new(py);
    for table in &tables {
        if table.name.is_empty() {
            continue;
        }

        let records = PyList::empty(py);
        for row in &table.rows {
            let record = PyDict::new(py);
            for (column, value) in row {
                let converted = to_py_value(py, value.as_deref(), table.types[*column], &date)?;
                record.set_item(table.keys[*column].bind(py), converted)?;
            }
            records.append(record)?;
        }
        data.set_item(&table.name, records)?;
    }

    Ok(data)
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

    let mut keys: Vec<Py<PyString>> = Vec::new();
    let mut types: Vec<ColumnType> = Vec::new();
    let mut values: Vec<Vec<Option<String>>> = Vec::new();
    let mut index: HashMap<Vec<u8>, usize> = HashMap::new();

    let mut depth = 0usize;
    let mut saw_root = false;
    let mut column: Option<usize> = None;
    let mut text: Option<String> = None;

    let column_for = |py: Python<'py>,
                      raw: &[u8],
                      keys: &mut Vec<Py<PyString>>,
                      types: &mut Vec<ColumnType>,
                      values: &mut Vec<Vec<Option<String>>>,
                      index: &mut HashMap<Vec<u8>, usize>|
     -> PyResult<usize> {
        if let Some(found) = index.get(raw) {
            return Ok(*found);
        }

        let name = convert_name(from_utf8(raw).map_err(xml_error)?, short_names);
        keys.push(PyString::new(py, &name).unbind());
        types.push(ColumnType::Unknown);
        values.push(Vec::new());
        index.insert(raw.to_vec(), keys.len() - 1);

        Ok(keys.len() - 1)
    };

    loop {
        match reader.read_event().map_err(xml_error)? {
            Event::Eof => break,

            Event::Start(e) => {
                depth += 1;
                saw_root = true;
                if depth == 3 {
                    column = Some(column_for(
                        py,
                        e.name().into_inner(),
                        &mut keys,
                        &mut types,
                        &mut values,
                        &mut index,
                    )?);
                    text = None;
                }
            }

            Event::Empty(e) => {
                if depth + 1 == 3 {
                    let column = column_for(
                        py,
                        e.name().into_inner(),
                        &mut keys,
                        &mut types,
                        &mut values,
                        &mut index,
                    )?;
                    values[column].push(None);
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
                    if let Some(column) = column.take() {
                        let value = text.take();
                        if let Some(ref value) = value {
                            types[column] = widen(types[column], value);
                        }
                        values[column].push(value);
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

    let data = PyDict::new(py);
    for (position, key) in keys.iter().enumerate() {
        let list = PyList::empty(py);
        for value in &values[position] {
            list.append(to_py_value(py, value.as_deref(), types[position], &date)?)?;
        }
        data.set_item(key.bind(py), list)?;
    }

    Ok(data)
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
    m.add_class::<Export>()?;
    m.add_class::<Field>()?;
    m.add_class::<File>()?;
    m.add_class::<Form>()?;
    m.add_class::<LockState>()?;
    m.add_class::<Patient>()?;
    m.add_class::<Query>()?;
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
