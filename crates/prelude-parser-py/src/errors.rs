use pyo3::{create_exception, exceptions::PyException};

create_exception!(_prelude_parser, FileNotFoundError, PyException);
create_exception!(_prelude_parser, InvalidFileTypeError, PyException);
create_exception!(_prelude_parser, ParsingError, PyException);
