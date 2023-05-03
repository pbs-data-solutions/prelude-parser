// use std::fs::read_to_string;
use std::fs::read_to_string;
use std::path::PathBuf;

use pyo3::create_exception;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use roxmltree::Document;

create_exception!(_prelude_parser, FileNotFoundError, PyException);
create_exception!(_prelude_parser, InvalidFileType, PyException);
create_exception!(_prelude_parser, ParsingError, PyException);

fn parse_xml<'py>(py: Python<'py>, xml_file: &PathBuf) -> PyResult<&'py PyDict> {
    let reader = read_to_string(xml_file);

    match reader {
        Ok(r) => match Document::parse(&r) {
            Ok(doc) => {
                let data = PyDict::new(py);
                let tree = doc.root_element();
                if let Some(form_name) = tree.first_element_child() {
                    data.set_item("form_name", form_name.tag_name().name())
                        .unwrap();
                    for child in form_name.children() {
                        if child.tag_name().name() != "" {
                            data.set_item(child.tag_name().name(), child.text())
                                .unwrap();
                        }
                    }
                }
                return Ok(data);
            }
            Err(e) => ParsingError::new_err(format!("Error parsing xml file: {:?}", e)).restore(py),
        },
        Err(e) => ParsingError::new_err(format!("Error parsing xml file: {:?}", e)).restore(py),
    }

    Err(ParsingError::new_err("Unable to parse XML file"))
}

fn validate_file(py: Python, xml_file: &PathBuf) -> PyResult<()> {
    if !xml_file.is_file() {
        FileNotFoundError::new_err(format!("File not found: {:?}", xml_file)).restore(py);
    } else if xml_file.extension().unwrap() != "xml" {
        InvalidFileType::new_err(format!("{:?} is not an xml file", xml_file)).restore(py);
    }

    Ok(())
}

#[pyfunction]
fn _parse_flat_file(py: Python, xml_file: PathBuf) -> PyResult<&PyDict> {
    validate_file(py, &xml_file)?;
    let data = parse_xml(py, &xml_file)?;

    Ok(data)
}

/// A Python module implemented in Rust.
#[pymodule]
fn _prelude_parser(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(_parse_flat_file, m)?)?;
    Ok(())
}
