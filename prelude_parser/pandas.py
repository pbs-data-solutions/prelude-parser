from __future__ import annotations

import sys
from pathlib import Path

from prelude_parser._prelude_parser import _parse_flat_file_to_pandas_dict


class UnsupportedPythonVersionError(Exception):
    pass


try:
    import pandas as pd
except ImportError:  # pragma: no cover
    if sys.version_info < (3, 9):  # pargma: no cover
        raise UnsupportedPythonVersionError("Pandas only supports Python 3.9+")

    raise ImportError("prelude-parser must be installed with the pandas or all extra to use pandas")


def to_dataframe(xml_file: str | Path) -> pd.DataFrame:
    """Parse a Prelude flat XML file into a Pandas DataFrame.

    This works for Prelude flat XML files that were exported with the "write tables to seperate
    files" option.

    Args:
        xml_file: The path to the XML file to parser.

    Returns:
        A Pandas DataFrame the data from the XML file.

    Examples:
        >>> from prelude_parser.pandas import to_dataframe
        >>> df = to_dataframe("physical_examination.xml")
    """
    data = _parse_flat_file_to_pandas_dict(xml_file)
    return pd.DataFrame.from_dict(data)
