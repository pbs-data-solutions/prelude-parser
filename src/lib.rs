use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::PathBuf;

use pyo3::create_exception;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyDict};
use roxmltree::Document;

create_exception!(_prelude_parser, FileNotFoundError, PyException);
create_exception!(_prelude_parser, InvalidFileTypeError, PyException);
create_exception!(_prelude_parser, ParsingError, PyException);

fn parse_xml<'py>(py: Python<'py>, xml_file: &PathBuf) -> PyResult<&'py PyDict> {
    let reader = read_to_string(xml_file);

    match reader {
        Ok(r) => match Document::parse(&r) {
            Ok(doc) => {
                let mut data: HashMap<&str, Vec<&PyDict>> = HashMap::new();
                let tree = doc.root_element();
                for form in tree.children() {
                    let form_name = form.tag_name().name();
                    if !form_name.is_empty() {
                        if let Some(d) = data.get_mut(form_name) {
                            let mut form_data: HashMap<&str, Option<&str>> = HashMap::new();
                            for child in form.children() {
                                if child.is_element() && child.tag_name().name() != "" {
                                    form_data.insert(child.tag_name().name(), child.text());
                                }
                            }
                            d.push(form_data.into_py_dict(py));
                        } else {
                            let mut items: Vec<&PyDict> = Vec::new();
                            let mut form_data: HashMap<&str, Option<&str>> = HashMap::new();
                            for child in form.children() {
                                if child.is_element() && child.tag_name().name() != "" {
                                    form_data.insert(child.tag_name().name(), child.text());
                                }
                            }
                            items.push(form_data.into_py_dict(py));
                            data.insert(form_name, items);
                        }
                    }
                }
                return Ok(data.into_py_dict(py));
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

fn validate_file(xml_file: &PathBuf) -> PyResult<()> {
    if !xml_file.is_file() {
        return Err(FileNotFoundError::new_err(format!(
            "File not found: {:?}",
            xml_file
        )));
    } else if xml_file.extension().unwrap() != "xml" {
        return Err(InvalidFileTypeError::new_err(format!(
            "{:?} is not an xml file",
            xml_file
        )));
    }

    Ok(())
}

#[pyfunction]
fn _parse_flat_file(py: Python, xml_file: PathBuf) -> PyResult<&PyDict> {
    validate_file(&xml_file)?;
    let data = parse_xml(py, &xml_file)?;

    Ok(data)
}

#[pymodule]
fn _prelude_parser(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(_parse_flat_file, m)?)?;
    m.add("FileNotFoundError", py.get_type::<FileNotFoundError>())?;
    m.add(
        "InvalidFileTypeError",
        py.get_type::<InvalidFileTypeError>(),
    )?;
    m.add("ParsingError", py.get_type::<ParsingError>())?;
    Ok(())
}
