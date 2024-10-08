import pandas as pd

from prelude_parser.pandas import to_dataframe


def test_pandas_to_dataframe(test_file_1):
    result = to_dataframe(test_file_1)
    data = {
        "base_form": [
            "communications.form.name.communications",
            "communications.form.name.communications",
        ],
        "communications_made": ["Yes", "Yes"],
        "form_group": ["Communications", "Communications"],
        "form_number": [None, None],
        "form_state": ["In-Work", "In-Work"],
        "form_title": ["Communications", "Communications"],
        "patient_id": [1681574905819, 1681574994823],
        "patient_name": ["ABC-001", "ABC-002"],
        "site_id": [1681574834910, 1681574834910],
        "site_name": ["Some Site", "Some Site"],
        "study_name": ["PBS", "PBS"],
    }
    expected = pd.DataFrame.from_dict(data)
    result = result.reindex(sorted(result.columns), axis=1)
    assert expected.equals(result)


def test_pandas_to_dataframe_short_names(test_file_4):
    result = to_dataframe(test_file_4, short_names=True)
    data = {
        "sitename": ["Some Site", "Another Site"],
        "studyname": ["PBS", "PBS"],
    }
    expected = pd.DataFrame.from_dict(data)
    result = result.reindex(sorted(result.columns), axis=1)
    assert expected.equals(result)
