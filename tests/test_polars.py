import polars as pl
from polars.testing import assert_frame_equal

from prelude_parser.polars import to_dataframe


def test_pandas_to_dataframe(test_file_1):
    result = to_dataframe(test_file_1)
    data = {
        "BaseForm": [
            "communications.form.name.communications",
            "communications.form.name.communications",
        ],
        "FormGroup": ["Communications", "Communications"],
        "FormNumber": [None, None],
        "FormState": ["In-Work", "In-Work"],
        "FormTitle": ["Communications", "Communications"],
        "PatientId": ["1681574905819", "1681574994823"],
        "PatientName": ["ABC-001", "ABC-002"],
        "SiteId": ["1681574834910", "1681574834910"],
        "SiteName": ["Some Site", "Some Site"],
        "StudyName": ["PBS", "PBS"],
        "communications_made": ["Yes", "Yes"],
    }
    expected = pl.from_dict(data)  # type: ignore
    result = result.pipe(lambda x: x.select(sorted(x.columns)))
    assert_frame_equal(expected, result)
