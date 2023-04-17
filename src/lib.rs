use std::fs::read_to_string;
use std::path::PathBuf;

use pyo3::create_exception;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use pyo3::types::PyDict;

create_exception!(_prelude_parser, FileNotFoundError, PyException);
create_exception!(_prelude_parser, InvalidFileType, PyException);

fn parse_xml<'py>(py: Python<'py>, xml_file: &PathBuf) -> PyResult<&'py PyDict> {
    let content = read_to_string(xml_file)?;
    let data = PyDict::new(py);
    data.set_item("form_name", "demographics")?;

    Ok(data)
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
fn parse_flat_file(py: Python, xml_file: PathBuf) -> PyResult<&PyDict> {
    validate_file(py, &xml_file)?;
    let data = parse_xml(py, &xml_file)?;

    Ok(data)
}

/// A Python module implemented in Rust.
#[pymodule]
fn _prelude_parser(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_flat_file, m)?)?;
    Ok(())
}
