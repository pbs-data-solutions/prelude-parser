import pandas as pd

from prelude_parser.pandas import to_dataframe


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
    expected = pd.DataFrame.from_dict(data)
    result = result.reindex(sorted(result.columns), axis=1)
    assert expected.equals(result)
