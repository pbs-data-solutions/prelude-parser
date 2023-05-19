from __future__ import annotations

from pathlib import Path

from prelude_parser._prelude_parser import _parse_flat_file_to_pandas_dict

try:
    import polars as pl
except ImportError:  # pragma: no cover
    raise ImportError("prelude-parser must be installed with the polars or all extra to use pandas")


def to_dataframe(xml_file: str | Path) -> pl.DataFrame:
    """Parse a Prelude flat XML file into a Polars DataFrame.

    This works for Prelude flat XML files that were exported with the "write tables to seperate
    files" option.

    Args:
        xml_file: The path to the XML file to parser.

    Returns:
        A Polars DataFrame the data from the XML file.

    Examples:
        >>> from prelude_parser.polars import to_dataframe
        >>> df = to_dataframe("physical_examination.xml")
    """
    data = _parse_flat_file_to_pandas_dict(xml_file)
    return pl.from_dict(data)
