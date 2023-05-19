from datetime import datetime

import pandas as pd
import pytest

from prelude_parser._prelude_parser import FileNotFoundError, InvalidFileTypeError, ParsingError
from prelude_parser.pandas import to_dataframe
from prelude_parser.parser import parse_flat_file


def test_parse_flat_file(test_file_1):
    result = parse_flat_file(test_file_1)
    assert len(result) == 2
    assert result[0].__name__ == "Communications"
    assert result[0].study_name == "PBS"
    assert result[0].site_name == "Some Site"
    assert result[0].site_id == 1681574834910
    assert result[0].patient_name == "ABC-001"
    assert result[0].patient_id == 1681574905819
    assert result[0].form_title == "Communications"
    assert result[0].base_form == "communications.form.name.communications"
    assert result[0].form_number is None
    assert result[0].form_group == "Communications"
    assert result[0].form_state == "In-Work"
    assert result[0].communications_made == "Yes"


def test_parse_flat_file_with_float(test_file_2):
    result = parse_flat_file(test_file_2)
    assert len(result) == 2
    assert result[0].__name__ == "Demographics"
    assert result[0].weight == 80.2
    assert result[0].dob == datetime(2020, 4, 15)


def test_parse_flat_file_i_form(test_file_3):
    result = parse_flat_file(test_file_3)
    assert len(result) == 3
    assert result[0].__name__ == "ICommunicationsDetails"
    assert result[0].study_name == "PBS"
    assert result[0].site_name == "Some Site"
    assert result[0].site_id == 1681574834910
    assert result[0].patient_name == "ABC-001"
    assert result[0].patient_id == 1681574905819
    assert result[0].form_title == "Communications"
    assert result[0].base_form == "communications.form.name.communications"
    assert result[0].form_number is None
    assert result[0].form_group == "Communications"
    assert result[0].form_state == "In-Work"
    assert result[0].i == 1
    assert result[0].contacted_by == "You"
    assert result[0].investigator == "Dr. Me"
    assert result[0].communication == "Some random talk"


def test_parse_flat_file_not_found_error():
    with pytest.raises(FileNotFoundError):
        parse_flat_file("bad.xml")


def test_parse_flat_file_invalid_file_type_error(tmp_path):
    bad = tmp_path / "bad.txt"
    bad.touch()
    with pytest.raises(InvalidFileTypeError):
        parse_flat_file(bad)


def test_parse_flat_file_parsing_error(tmp_path):
    bad = tmp_path / "bad.xml"
    bad.touch()
    with pytest.raises(ParsingError):
        parse_flat_file(bad)


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
