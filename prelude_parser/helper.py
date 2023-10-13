from __future__ import annotations

from prelude_parser._prelude_parser import _merge_i_data
from prelude_parser.types import FlatFormInfo


def merge_i_data(
    main_data: FlatFormInfo,
    i_data: FlatFormInfo,
    *,
    short_names: bool = False,
    common_fields: list[str] | None = None,
) -> FlatFormInfo:
    """Combines a main table with an i table.

    The two datasets are joined on a combination of subject id and form number.

    Args:
        main_data: The dataset that contains the non-repeating data.
        i_data: The dataset that contains the repeating data.
        short_names: Set to True if short names were used in the export.
        common_fields: These fields are present in both the main_data and the i_data. Before merging
            the common fields are dropped from the i_data so they aren't duplicated in the merged
            dataset. If no common_fields are provided default values will be used for the common
            fields, this will generally be what you want.

    Returns:
        A dictionary with the data from the two datasets merged

    Examples:
        >>> from prelude_parser import merge_i_data
        >>> merged = merge_i_data(main_data, i_data)
    """
    return _merge_i_data(main_data, i_data, short_names, common_fields)
