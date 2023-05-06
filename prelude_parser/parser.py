from __future__ import annotations

from datetime import datetime
from pathlib import Path
from typing import Any

from camel_converter import to_pascal, to_snake

from prelude_parser._prelude_parser import _parse_flat_file


class _MetaCls(type):
    def __new__(cls, clsname: str, superclasses: tuple[type, ...], attributedict: dict) -> _MetaCls:
        return super(_MetaCls, cls).__new__(cls, clsname, superclasses, attributedict)


def parse_flat_file(xml_file: str | Path) -> list[Any]:
    """Parse a Prelude flat XML file into a Python class.

    The name of the class is taken from the form name node in the XML file converted to pascal case.
    For example a <physical_examination> node will result in a PhysicalExamination class being
    created.

    Args:
        xml_file: The path to the XML file to parser.

    Returns:
        A list of Python classes containing the data from the XML file.

    Examples:
        >>> from prelude_parser import parse_flat_file
        >>> data = parse_flat_file("physical_examination.xml")
    """
    parsed = _parse_flat_file(xml_file)
    formatted: list[Any] = []
    for form, data in parsed.items():
        class_name = to_pascal(form)
        formatted_data: dict[str, Any] = {}

        for d in data:
            for k, v in d.items():
                key = to_snake(k)
                try:
                    if "." in v:
                        formatted_data[key] = float(v)
                    else:
                        formatted_data[key] = int(v)
                    continue
                except (TypeError, ValueError):
                    pass

                try:
                    formatted_data[key] = datetime.strptime(v, "%d-%b-%Y")
                    continue
                except (TypeError, ValueError):
                    pass

                formatted_data[key] = v
            formatted.append(_MetaCls(class_name, (object,), formatted_data))

    return formatted
