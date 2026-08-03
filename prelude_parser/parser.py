from __future__ import annotations

from pathlib import Path
from typing import TYPE_CHECKING, Any

from camel_converter import to_pascal

from prelude_parser._prelude_parser import _parse_flat_file_to_dict

if TYPE_CHECKING:
    from prelude_parser.types import FlatFormInfo


def parse_to_dict(xml_file: str | Path, *, short_names: bool = False) -> dict[str, FlatFormInfo]:
    """Parse a Prelude flat XML file into a dict.

    Args:
        xml_file: The path to the XML file to parser.
        short_names: Set to True if short names were used in the export.

    Returns:
        A Python dictionary containing the data from the XML file.

    Examples:
        >>> from prelude_parser import parse_to_dict
        >>> data = parse_to_dict("physical_examination.xml")
    """
    return _parse_flat_file_to_dict(xml_file, short_names=short_names)


def parse_to_classes(xml_file: str | Path, short_names: bool = False) -> list[Any]:
    """Parse a Prelude flat XML file into a list of Python objects.

    One class is created per form, named after the form name node in the XML file converted to
    pascal case, and each record in that form becomes an instance of it. For example a
    <physical_examination> node will result in PhysicalExamination instances.

    Args:
        xml_file: The path to the XML file to parser.
        short_names: Set to True if short names were used in the export.

    Returns:
        A list of Python objects containing the data from the XML file.

    Examples:
        >>> from prelude_parser import parse_to_classes
        >>> data = parse_to_classes("physical_examination.xml")
    """
    parsed = parse_to_dict(xml_file, short_names=short_names)
    formatted: list[Any] = []
    for form, data in parsed.items():
        form_class = type(to_pascal(form), (object,), {})
        for d in data:
            instance = object.__new__(form_class)
            instance.__dict__ = d
            formatted.append(instance)

    return formatted
