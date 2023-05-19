from __future__ import annotations

from datetime import datetime
from pathlib import Path
from typing import Any

from camel_converter import to_pascal

from prelude_parser._prelude_parser import _parse_flat_file_to_dict


class _MetaCls(type):
    def __new__(
        cls, clsname: str, superclasses: tuple[type, ...], attributedict: dict[str, Any]
    ) -> _MetaCls:
        return super(_MetaCls, cls).__new__(cls, clsname, superclasses, attributedict)


def parse_to_dict(xml_file: str | Path) -> dict[str, list[dict[str, Any]]]:
    """Parse a Prelude flat XML file into a dict.

    Args:
        xml_file: The path to the XML file to parser.

    Returns:
        A Python dictionary containing the data from the XML file.

    Examples:
        >>> from prelude_parser import parse_to_dict
        >>> data = parse_to_dict("physical_examination.xml")
    """
    parsed = _parse_flat_file_to_dict(xml_file)
    for _, data in parsed.items():
        for d in data:
            for k, v in d.items():
                try:
                    if "." in v:
                        d[k] = float(v)
                    else:
                        d[k] = int(v)
                    continue
                except (TypeError, ValueError):
                    pass

                try:
                    d[k] = datetime.strptime(v, "%d-%b-%Y")
                except (TypeError, ValueError):
                    pass

    return parsed


def parse_to_classes(xml_file: str | Path) -> list[Any]:
    """Parse a Prelude flat XML file into a list of Python class.

    The name of the class is taken from the form name node in the XML file converted to pascal case.
    For example a <physical_examination> node will result in a PhysicalExamination class being
    created.

    Args:
        xml_file: The path to the XML file to parser.

    Returns:
        A list of Python classes containing the data from the XML file.

    Examples:
        >>> from prelude_parser import parse_to_classes
        >>> data = parse_to_classes("physical_examination.xml")
    """
    parsed = parse_to_dict(xml_file)
    formatted: list[Any] = []
    for form, data in parsed.items():
        class_name = to_pascal(form)
        for d in data:
            formatted.append(_MetaCls(class_name, (object,), d))

    return formatted
