from datetime import datetime

from prelude_parser.parser import parse_flat_file


def test_parse_flat_file(test_file_1):
    result = parse_flat_file(test_file_1)
    assert result.study_name == "PBS"
    assert result.site_name == "Some Site"
    assert result.site_id == 1681574834910
    assert result.patient_name == "ABC-001"
    assert result.patient_id == 1681574905819
    assert result.form_title == "Communications"
    assert result.base_form == "communications.form.name.communications"
    assert result.form_number is None
    assert result.form_group == "Communications"
    assert result.form_state == "In-Work"
    assert result.communications_made == "Yes"


def test_parse_flat_file_with_float(test_file_2):
    result = parse_flat_file(test_file_2)
    assert result.weight == 80.2
    assert result.dob == datetime(2020, 4, 15)


def test_parse_flat_file_i_form(test_file_3):
    result = parse_flat_file(test_file_3)
    assert result.study_name == "PBS"
    assert result.site_name == "Some Site"
    assert result.site_id == 1681574834910
    assert result.patient_name == "ABC-001"
    assert result.patient_id == 1681574905819
    assert result.form_title == "Communications"
    assert result.base_form == "communications.form.name.communications"
    assert result.form_number is None
    assert result.form_group == "Communications"
    assert result.form_state == "In-Work"
    assert result.i == 1
    assert result.contacted_by == "You"
    assert result.investigator == "Dr. Me"
    assert result.communication == "Some random talk"
