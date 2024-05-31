mod native_parser;

use std::{fs::read_to_string, path::Path};

use anyhow::Result;

use native_parser::{parse_native_string, Native};

pub fn parse_native_file(xml_path: &Path) -> Result<Native> {
    let xml_file = read_to_string(xml_path)?;
    let native = parse_native_string(&xml_file)?;

    Ok(native)
}
