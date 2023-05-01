use std::fs::read_to_string;
use std::path::PathBuf;

use pyo3::create_exception;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use quick_xml::reader::Reader;

create_exception!(_prelude_parser, FileNotFoundError, PyException);
create_exception!(_prelude_parser, InvalidFileType, PyException);
create_exception!(_prelude_parser, ParsingError, PyException);

fn parse_xml<'py>(py: Python<'py>, xml_file: &PathBuf) -> PyResult<&'py PyDict> {
    let data = PyDict::new(py);
    let reader = Reader::from_file(xml_file);
    let mut buff = Vec::new();
    let reader_data: Reader<BufReader<File>>;

    match reader {
        Ok(r) => let reader_data = r,
        Err(e) => ParsingError:new_err(format!("An error occurred parsing the file: {:?}", e)).restore(py),
    }

    loop {
        match reader.read_event_into(&mut buff) {
            Err(e) => ParsingError:new_err(format!("An error occurred parsing the file: {:?}", e)).restore(py);
        }
    }
    /*let content = read_to_string(xml_file)?;

    for line in content.split('\n').skip(2) {
        if line.len() > 0 {
            println!("{line}");
        }
    }*/

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
