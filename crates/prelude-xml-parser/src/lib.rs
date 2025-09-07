pub mod errors;
pub mod native;

use std::{
    collections::HashMap,
    fs::{read_to_string, File},
    io::{BufReader, Cursor},
    path::Path,
};

use crate::errors::Error;
use crate::native::{
    site_native::SiteNative,
    subject_native::{Form, Patient, SubjectNative},
    user_native::UserNative,
};
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;

/// Parses a Prelude native XML file into a `Native` stuct.
///
/// # Example
///
/// ```
/// use std::path::Path;
///
/// use prelude_xml_parser::parse_site_native_file;
///
/// let file_path = Path::new("tests/assets/site_native.xml");
/// let native = parse_site_native_file(&file_path).unwrap();
///
/// assert!(native.sites.len() >= 1, "Vector length is less than 1");
/// ```
pub fn parse_site_native_file(xml_path: &Path) -> Result<SiteNative, Error> {
    check_valid_xml_file(xml_path)?;

    let xml_file = read_to_string(xml_path)?;
    let native = parse_site_native_string(&xml_file)?;

    Ok(native)
}

/// Parse a string of Prelude native site XML into a `SiteNative` struct.
///
/// # Example
///
/// ```
/// use chrono::{DateTime, Utc};
/// use prelude_xml_parser::parse_site_native_string;
/// use prelude_xml_parser::native::site_native::*;
///
/// let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
/// <export_from_vision_EDC date="01-Jun-2024 18:17 -0500" createdBy="Paul Sanders" role="Project Manager" numberSubjectsProcessed="2">
///
///   <site name="Some Site" uniqueId="1681574834910" numberOfPatients="4" countOfRandomizedPatients="0" whenCreated="2023-04-15 12:08:19 -0400" creator="Paul Sanders" numberOfForms="1">
///     <form name="demographic.form.name.site.demographics" lastModified="2023-04-15 12:08:19 -0400" whoLastModifiedName="Paul Sanders" whoLastModifiedRole="Project Manager" whenCreated="1681574834930" hasErrors="false" hasWarnings="false" locked="false" user="" dateTimeChanged="" formTitle="Site Demographics" formIndex="1" formGroup="Demographic" formState="In-Work">
///       <state value="form.state.in.work" signer="Paul Sanders - Project Manager" signerUniqueId="1681162687395" dateSigned="2023-04-15 12:08:19 -0400" />
///       <category name="Demographics" type="normal" highestIndex="0">
///         <field name="address" type="text" dataType="string" errorCode="valid" whenCreated="2023-04-15 11:07:14 -0500" keepHistory="true" />
///         <field name="company" type="text" dataType="string" errorCode="valid" whenCreated="2023-04-15 11:07:14 -0500" keepHistory="true">
///           <entry id="1">
///             <value by="Paul Sanders" byUniqueId="1681162687395" role="Project Manager" when="2023-04-15 12:08:19 -0400" xml:space="preserve">Some Company</value>
///           </entry>
///         </field>
///         <field name="site_code_name" type="hidden" dataType="string" errorCode="valid" whenCreated="2023-04-15 11:07:14 -0500" keepHistory="true">
///           <entry id="1">
///             <value by="set from calculation" byUniqueId="" role="System" when="2023-04-15 12:08:19 -0400" xml:space="preserve">ABC-Some Site</value>
///             <reason by="set from calculation" byUniqueId="" role="System" when="2023-04-15 12:08:19 -0400" xml:space="preserve">calculated value</reason>
///           </entry>
///           <entry id="2">
///             <value by="set from calculation" byUniqueId="" role="System" when="2023-04-15 12:07:24 -0400" xml:space="preserve">Some Site</value>
///             <reason by="set from calculation" byUniqueId="" role="System" when="2023-04-15 12:07:24 -0400" xml:space="preserve">calculated value</reason>
///           </entry>
///         </field>
///       </category>
///       <category name="Enrollment" type="normal" highestIndex="0">
///         <field name="enrollment_closed_date" type="popUpCalendar" dataType="date" errorCode="valid" whenCreated="2023-04-15 11:07:14 -0500" keepHistory="true" />
///         <field name="enrollment_open" type="radio" dataType="string" errorCode="valid" whenCreated="2023-04-15 11:07:14 -0500" keepHistory="true">
///           <entry id="1">
///             <value by="Paul Sanders" byUniqueId="1681162687395" role="Project Manager" when="2023-04-15 12:08:19 -0400" xml:space="preserve">Yes</value>
///           </entry>
///         </field>
///         <field name="enrollment_open_date" type="popUpCalendar" dataType="date" errorCode="valid" whenCreated="2023-04-15 11:07:14 -0500" keepHistory="true" />
///       </category>
///     </form>
///   </site>
///
///   <site name="Artemis" uniqueId="1691420994591" numberOfPatients="0" countOfRandomizedPatients="0" whenCreated="2023-08-07 08:14:23 -0700" creator="Paul Sanders" numberOfForms="1">
///     <form name="demographic.form.name.site.demographics" lastModified="2023-08-07 08:14:23 -0700" whoLastModifiedName="Paul Sanders" whoLastModifiedRole="Project Manager" whenCreated="1691420994611" hasErrors="false" hasWarnings="false" locked="false" user="" dateTimeChanged="" formTitle="Site Demographics" formIndex="1" formGroup="Demographic" formState="In-Work">
///       <state value="form.state.in.work" signer="Paul Sanders - Project Manager" signerUniqueId="1681162687395" dateSigned="2023-08-07 08:14:23 -0700" />
///       <category name="Demographics" type="normal" highestIndex="0">
///         <field name="address" type="text" dataType="string" errorCode="valid" whenCreated="2023-08-07 10:09:54 -0500" keepHistory="true">
///           <entry id="1">
///             <value by="Paul Sanders" byUniqueId="1681162687395" role="Project Manager" when="2023-08-07 08:14:21 -0700" xml:space="preserve">1111 Moon Drive</value>
///           </entry>
///           <comment id="1">
///             <value by="Paul Sanders" byUniqueId="1681162687395" role="Project Manager" when="2023-08-07 08:14:21 -0700" xml:space="preserve">Some comment</value>
///           </comment>
///         </field>
///       </category>
///     </form>
///   </site>
///
/// </export_from_vision_EDC>
/// "#;
///
/// let expected = SiteNative {
///     sites: vec![
///         Site {
///             name: "Some Site".to_string(),
///             unique_id: "1681574834910".to_string(),
///             number_of_patients: 4,
///             count_of_randomized_patients: 0,
///             when_created: Some(DateTime::parse_from_rfc3339("2023-04-15T16:08:19Z")
///                 .unwrap()
///                 .with_timezone(&Utc)),
///             creator: "Paul Sanders".to_string(),
///             number_of_forms: 1,
///             forms: Some(vec![Form {
///                 name: "demographic.form.name.site.demographics".to_string(),
///                 last_modified: Some(
///                     DateTime::parse_from_rfc3339("2023-04-15T16:08:19Z")
///                         .unwrap()
///                         .with_timezone(&Utc),
///                 ),
///                 who_last_modified_name: Some("Paul Sanders".to_string()),
///                 who_last_modified_role: Some("Project Manager".to_string()),
///                 when_created: 1681574834930,
///                 has_errors: false,
///                 has_warnings: false,
///                 locked: false,
///                 user: None,
///                 date_time_changed: None,
///                 form_title: "Site Demographics".to_string(),
///                 form_index: 1,
///                 form_group: Some("Demographic".to_string()),
///                 form_state: "In-Work".to_string(),
///                 states: Some(vec![State {
///                     value: "form.state.in.work".to_string(),
///                     signer: "Paul Sanders - Project Manager".to_string(),
///                     signer_unique_id: "1681162687395".to_string(),
///                     date_signed: Some(
///                         DateTime::parse_from_rfc3339("2023-04-15T16:08:19Z")
///                             .unwrap()
///                             .with_timezone(&Utc),
///                     ),
///                 }]),
///                 categories: Some(vec![
///                     Category {
///                         name: "Demographics".to_string(),
///                         category_type: "normal".to_string(),
///                         highest_index: 0,
///                         fields: Some(vec![
///                             Field {
///                                 name: "address".to_string(),
///                                 field_type: "text".to_string(),
///                                 data_type: Some("string".to_string()),
///                                 error_code: "valid".to_string(),
///                                 when_created: Some(DateTime::parse_from_rfc3339(
///                                     "2023-04-15T16:07:14Z",
///                                 )
///                                 .unwrap()
///                                 .with_timezone(&Utc)),
///                                 keep_history: true,
///                                 entries: None,
///                                 comments: None,
///                             },
///                             Field {
///                                 name: "company".to_string(),
///                                 field_type: "text".to_string(),
///                                 data_type: Some("string".to_string()),
///                                 error_code: "valid".to_string(),
///                                 when_created: Some(DateTime::parse_from_rfc3339(
///                                     "2023-04-15T16:07:14Z",
///                                 )
///                                 .unwrap()
///                                 .with_timezone(&Utc)),
///                                 keep_history: true,
///                                 entries: Some(vec![Entry {
///                                     entry_id: "1".to_string(),
///                                     value: Some(Value {
///                                         by: "Paul Sanders".to_string(),
///                                         by_unique_id: Some("1681162687395".to_string()),
///                                         role: "Project Manager".to_string(),
///                                         when: Some(DateTime::parse_from_rfc3339(
///                                             "2023-04-15T16:08:19Z",
///                                         )
///                                         .unwrap()
///                                         .with_timezone(&Utc)),
///                                         value: "Some Company".to_string(),
///                                     }),
///                                     reason: None,
///                                 }]),
///                                 comments: None,
///                             },
///                             Field {
///                                 name: "site_code_name".to_string(),
///                                 field_type: "hidden".to_string(),
///                                 data_type: Some("string".to_string()),
///                                 error_code: "valid".to_string(),
///                                 when_created: Some(DateTime::parse_from_rfc3339(
///                                     "2023-04-15T16:07:14Z",
///                                 )
///                                 .unwrap()
///                                 .with_timezone(&Utc)),
///                                 keep_history: true,
///                                 entries: Some(vec![
///                                     Entry {
///                                         entry_id: "1".to_string(),
///                                         value: Some(Value {
///                                             by: "set from calculation".to_string(),
///                                             by_unique_id: None,
///                                             role: "System".to_string(),
///                                             when: Some(DateTime::parse_from_rfc3339(
///                                                 "2023-04-15T16:08:19Z",
///                                             )
///                                             .unwrap()
///                                             .with_timezone(&Utc)),
///                                             value: "ABC-Some Site".to_string(),
///                                         }),
///                                         reason: Some(Reason {
///                                             by: "set from calculation".to_string(),
///                                             by_unique_id: None,
///                                             role: "System".to_string(),
///                                             when: Some(DateTime::parse_from_rfc3339(
///                                                 "2023-04-15T16:08:19Z",
///                                             )
///                                             .unwrap()
///                                             .with_timezone(&Utc)),
///                                             value: "calculated value".to_string(),
///                                         }),
///                                     },
///                                     Entry {
///                                         entry_id: "2".to_string(),
///                                         value: Some(Value {
///                                             by: "set from calculation".to_string(),
///                                             by_unique_id: None,
///                                             role: "System".to_string(),
///                                             when: Some(DateTime::parse_from_rfc3339(
///                                                 "2023-04-15T16:07:24Z",
///                                             )
///                                             .unwrap()
///                                             .with_timezone(&Utc)),
///                                             value: "Some Site".to_string(),
///                                         }),
///                                         reason: Some(Reason {
///                                             by: "set from calculation".to_string(),
///                                             by_unique_id: None,
///                                             role: "System".to_string(),
///                                             when: Some(DateTime::parse_from_rfc3339(
///                                                 "2023-04-15T16:07:24Z",
///                                             )
///                                             .unwrap()
///                                             .with_timezone(&Utc)),
///                                             value: "calculated value".to_string(),
///                                         }),
///                                     },
///                                 ]),
///                                 comments: None,
///                             },
///                         ]),
///                     },
///                     Category {
///                         name: "Enrollment".to_string(),
///                         category_type: "normal".to_string(),
///                         highest_index: 0,
///                         fields: Some(vec![
///                             Field {
///                                 name: "enrollment_closed_date".to_string(),
///                                 field_type: "popUpCalendar".to_string(),
///                                 data_type: Some("date".to_string()),
///                                 error_code: "valid".to_string(),
///                                 when_created: Some(DateTime::parse_from_rfc3339(
///                                     "2023-04-15T16:07:14Z",
///                                 )
///                                 .unwrap()
///                                 .with_timezone(&Utc)),
///                                 keep_history: true,
///                                 entries: None,
///                                 comments: None,
///                             },
///                             Field {
///                                 name: "enrollment_open".to_string(),
///                                 field_type: "radio".to_string(),
///                                 data_type: Some("string".to_string()),
///                                 error_code: "valid".to_string(),
///                                 when_created: Some(DateTime::parse_from_rfc3339(
///                                     "2023-04-15T16:07:14Z",
///                                 )
///                                 .unwrap()
///                                 .with_timezone(&Utc)),
///                                 keep_history: true,
///                                 entries: Some(vec![Entry {
///                                     entry_id: "1".to_string(),
///                                     value: Some(Value {
///                                         by: "Paul Sanders".to_string(),
///                                         by_unique_id: Some("1681162687395".to_string()),
///                                         role: "Project Manager".to_string(),
///                                         when: Some(DateTime::parse_from_rfc3339(
///                                             "2023-04-15T16:08:19Z",
///                                         )
///                                         .unwrap()
///                                         .with_timezone(&Utc)),
///                                         value: "Yes".to_string(),
///                                     }),
///                                     reason: None,
///                                 }]),
///                                 comments: None,
///                             },
///                             Field {
///                                 name: "enrollment_open_date".to_string(),
///                                 field_type: "popUpCalendar".to_string(),
///                                 data_type: Some("date".to_string()),
///                                 error_code: "valid".to_string(),
///                                 when_created: Some(DateTime::parse_from_rfc3339(
///                                     "2023-04-15T16:07:14Z",
///                                 )
///                                 .unwrap()
///                                 .with_timezone(&Utc)),
///                                 keep_history: true,
///                                 entries: None,
///                                 comments: None,
///                             },
///                         ]),
///                     },
///                 ]),
///             }]),
///         },
///         Site {
///             name: "Artemis".to_string(),
///             unique_id: "1691420994591".to_string(),
///             number_of_patients: 0,
///             count_of_randomized_patients: 0,
///             when_created: Some(DateTime::parse_from_rfc3339("2023-08-07T15:14:23Z")
///                 .unwrap()
///                 .with_timezone(&Utc)),
///             creator: "Paul Sanders".to_string(),
///             number_of_forms: 1,
///             forms: Some(vec![Form {
///                 name: "demographic.form.name.site.demographics".to_string(),
///                 last_modified: Some(
///                     DateTime::parse_from_rfc3339("2023-08-07T15:14:23Z")
///                         .unwrap()
///                         .with_timezone(&Utc),
///                 ),
///                 who_last_modified_name: Some("Paul Sanders".to_string()),
///                 who_last_modified_role: Some("Project Manager".to_string()),
///                 when_created: 1691420994611,
///                 has_errors: false,
///                 has_warnings: false,
///                 locked: false,
///                 user: None,
///                 date_time_changed: None,
///                 form_title: "Site Demographics".to_string(),
///                 form_index: 1,
///                 form_group: Some("Demographic".to_string()),
///                 form_state: "In-Work".to_string(),
///                 states: Some(vec![State {
///                     value: "form.state.in.work".to_string(),
///                     signer: "Paul Sanders - Project Manager".to_string(),
///                     signer_unique_id: "1681162687395".to_string(),
///                     date_signed: Some(
///                         DateTime::parse_from_rfc3339("2023-08-07T15:14:23Z")
///                             .unwrap()
///                             .with_timezone(&Utc),
///                     ),
///                 }]),
///                 categories: Some(vec![Category {
///                     name: "Demographics".to_string(),
///                     category_type: "normal".to_string(),
///                     highest_index: 0,
///                     fields: Some(vec![Field {
///                         name: "address".to_string(),
///                         field_type: "text".to_string(),
///                         data_type: Some("string".to_string()),
///                         error_code: "valid".to_string(),
///                         when_created: Some(DateTime::parse_from_rfc3339("2023-08-07T15:09:54Z")
///                             .unwrap()
///                             .with_timezone(&Utc)),
///                         keep_history: true,
///                         entries: Some(vec![Entry {
///                             entry_id: "1".to_string(),
///                             value: Some(Value {
///                                 by: "Paul Sanders".to_string(),
///                                 by_unique_id: Some("1681162687395".to_string()),
///                                 role: "Project Manager".to_string(),
///                                 when: Some(DateTime::parse_from_rfc3339("2023-08-07T15:14:21Z")
///                                     .unwrap()
///                                     .with_timezone(&Utc)),
///                                 value: "1111 Moon Drive".to_string(),
///                             }),
///                             reason: None,
///                         }]),
///                         comments: Some(vec![Comment {
///                             comment_id: "1".to_string(),
///                             value: Some(Value {
///                                 by: "Paul Sanders".to_string(),
///                                 by_unique_id: Some("1681162687395".to_string()),
///                                 role: "Project Manager".to_string(),
///                                 when: Some(DateTime::parse_from_rfc3339("2023-08-07T15:14:21Z")
///                                     .unwrap()
///                                     .with_timezone(&Utc)),
///                                 value: "Some comment".to_string(),
///                             }),
///                         }]),
///                     }]),
///                 }]),
///             }]),
///         },
///     ],
/// };
/// let result = parse_site_native_string(xml).unwrap();
/// assert_eq!(result, expected);
pub fn parse_site_native_string(xml_str: &str) -> Result<SiteNative, Error> {
    let native: SiteNative = quick_xml::de::from_str(xml_str)?;

    Ok(native)
}

/// Parses a Prelude native subject XML file into a `SubjectNative` stuct.
///
/// # Example
///
/// ```
/// use std::path::Path;
///
/// use prelude_xml_parser::parse_subject_native_file;
///
/// let file_path = Path::new("tests/assets/subject_native.xml");
/// let native = parse_subject_native_file(&file_path).unwrap();
///
/// assert!(native.patients.len() >= 1, "Vector length is less than 1");
/// ```
pub fn parse_subject_native_file(xml_path: &Path) -> Result<SubjectNative, Error> {
    check_valid_xml_file(xml_path)?;

    let file = File::open(xml_path)?;
    let buf_reader = BufReader::new(file);
    parse_subject_native_streaming(buf_reader)
}

/// Parse a string of Prelude native subject XML into a `SubjectNative` struct.
///
/// # Example
///
/// ```
/// use chrono::{DateTime, Utc};
/// use prelude_xml_parser::parse_subject_native_string;
/// use prelude_xml_parser::native::subject_native::*;
///
/// let xml = r#"<export_from_vision_EDC date="30-May-2024 10:35 -0500" createdBy="Paul Sanders" role="Project Manager" numberSubjectsProcessed="4">
///     <patient patientId="ABC-001" uniqueId="1681574905819" whenCreated="2023-04-15 12:09:02 -0400" creator="Paul Sanders" siteName="Some Site" siteUniqueId="1681574834910" lastLanguage="English" numberOfForms="6">
///       <form name="day.0.form.name.demographics" lastModified="2023-04-15 12:09:15 -0400" whoLastModifiedName="Paul Sanders" whoLastModifiedRole="Project Manager" whenCreated="1681574905839" hasErrors="false" hasWarnings="false" locked="false" user="" dateTimeChanged="" formTitle="Demographics" formIndex="1" formGroup="Day 0" formState="In-Work">
///         <state value="form.state.in.work" signer="Paul Sanders - Project Manager" signerUniqueId="1681162687395" dateSigned="2023-04-15 12:09:02 -0400"/>
///         <category name="Demographics" type="normal" highestIndex="0">
///           <field name="breed" type="combo-box" dataType="string" errorCode="valid" whenCreated="2023-04-15 12:08:26 -0400" keepHistory="true">
///             <entry id="1">
///               <value by="Paul Sanders" byUniqueId="1681162687395" role="Project Manager" when="2023-04-15 12:09:02 -0400" xml:space="preserve">Labrador</value>
///             </entry>
///           </field>
///         </category>
///       </form>
///     </patient>
///     <patient patientId="DEF-002" uniqueId="1681574905820" whenCreated="2023-04-16 12:10:02 -0400" creator="Wade Watts" siteName="Another Site" siteUniqueId="1681574834911" lastLanguage="" numberOfForms="8">
///       <form name="day.0.form.name.demographics" lastModified="2023-04-16 12:10:15 -0400" whoLastModifiedName="Barney Rubble" whoLastModifiedRole="Technician" whenCreated="1681574905838" hasErrors="false" hasWarnings="false" locked="false" user="" dateTimeChanged="" formTitle="Demographics" formIndex="1" formGroup="Day 0" formState="In-Work">
///         <state value="form.state.in.work" signer="Paul Sanders - Project Manager" signerUniqueId="1681162687395" dateSigned="2023-04-16 12:10:02 -0400"/>
///         <category name="Demographics" type="normal" highestIndex="0">
///           <field name="breed" type="combo-box" dataType="string" errorCode="valid" whenCreated="2023-04-15 12:08:26 -0400" keepHistory="true">
///             <entry id="1">
///               <value by="Paul Sanders" byUniqueId="1681162687395" role="Project Manager" when="2023-04-15 12:09:02 -0400" xml:space="preserve">Labrador</value>
///             </entry>
///           </field>
///         </category>
///       </form>
///     </patient>
/// </export_from_vision_EDC>
/// "#;
///
/// let expected = SubjectNative {
///     patients: vec![
///         Patient {
///             patient_id: "ABC-001".to_string(),
///             unique_id: "1681574905819".to_string(),
///             when_created: Some(DateTime::parse_from_rfc3339("2023-04-15T16:09:02Z")
///                 .unwrap()
///                 .with_timezone(&Utc)),
///             creator: "Paul Sanders".to_string(),
///             site_name: "Some Site".to_string(),
///             site_unique_id: "1681574834910".to_string(),
///             last_language: Some("English".to_string()),
///             number_of_forms: 6,
///             forms: Some(vec![Form {
///                 name: "day.0.form.name.demographics".to_string(),
///                 last_modified: Some(DateTime::parse_from_rfc3339("2023-04-15T16:09:15Z")
///                     .unwrap()
///                     .with_timezone(&Utc)),
///                 who_last_modified_name: Some("Paul Sanders".to_string()),
///                 who_last_modified_role: Some("Project Manager".to_string()),
///                 when_created: 1681574905839,
///                 has_errors: false,
///                 has_warnings: false,
///                 locked: false,
///                 user: None,
///                 date_time_changed: None,
///                 form_title: "Demographics".to_string(),
///                 form_index: 1,
///                 form_group: Some("Day 0".to_string()),
///                 form_state: "In-Work".to_string(),
///                 states: Some(vec![State {
///                     value: "form.state.in.work".to_string(),
///                     signer: "Paul Sanders - Project Manager".to_string(),
///                     signer_unique_id: "1681162687395".to_string(),
///                     date_signed: Some(
///                         DateTime::parse_from_rfc3339("2023-04-15T16:09:02Z")
///                             .unwrap()
///                             .with_timezone(&Utc),
///                     ),
///                 }]),
///                 categories: Some(vec![Category {
///                     name: "Demographics".to_string(),
///                     category_type: "normal".to_string(),
///                     highest_index: 0,
///                     fields: Some(vec![Field {
///                         name: "breed".to_string(),
///                         field_type: "combo-box".to_string(),
///                         data_type: Some("string".to_string()),
///                         error_code: "valid".to_string(),
///                         when_created: Some(DateTime::parse_from_rfc3339("2023-04-15T16:08:26Z")
///                             .unwrap()
///                             .with_timezone(&Utc)),
///                         keep_history: true,
///                         entries: Some(vec![Entry {
///                             entry_id: "1".to_string(),
///                             value: Some(Value {
///                                 by: "Paul Sanders".to_string(),
///                                 by_unique_id: Some("1681162687395".to_string()),
///                                 role: "Project Manager".to_string(),
///                                 when: Some(DateTime::parse_from_rfc3339("2023-04-15T16:09:02Z")
///                                     .unwrap()
///                                     .with_timezone(&Utc)),
///                                 value: "Labrador".to_string(),
///                             }),
///                             reason: None,
///                         }]),
///                         comments: None,
///                     }]),
///                 }]),
///             }]),
///         },
///         Patient {
///             patient_id: "DEF-002".to_string(),
///             unique_id: "1681574905820".to_string(),
///             when_created: Some(DateTime::parse_from_rfc3339("2023-04-16T16:10:02Z")
///                 .unwrap()
///                 .with_timezone(&Utc)),
///             creator: "Wade Watts".to_string(),
///             site_name: "Another Site".to_string(),
///             site_unique_id: "1681574834911".to_string(),
///             last_language: None,
///             number_of_forms: 8,
///             forms: Some(vec![Form {
///                 name: "day.0.form.name.demographics".to_string(),
///                 last_modified: Some(DateTime::parse_from_rfc3339("2023-04-16T16:10:15Z")
///                     .unwrap()
///                     .with_timezone(&Utc)),
///                 who_last_modified_name: Some("Barney Rubble".to_string()),
///                 who_last_modified_role: Some("Technician".to_string()),
///                 when_created: 1681574905838,
///                 has_errors: false,
///                 has_warnings: false,
///                 locked: false,
///                 user: None,
///                 date_time_changed: None,
///                 form_title: "Demographics".to_string(),
///                 form_index: 1,
///                 form_group: Some("Day 0".to_string()),
///                 form_state: "In-Work".to_string(),
///                 states: Some(vec![State {
///                     value: "form.state.in.work".to_string(),
///                     signer: "Paul Sanders - Project Manager".to_string(),
///                     signer_unique_id: "1681162687395".to_string(),
///                     date_signed: Some(
///                         DateTime::parse_from_rfc3339("2023-04-16T16:10:02Z")
///                             .unwrap()
///                             .with_timezone(&Utc),
///                     ),
///                 }]),
///                 categories: Some(vec![Category {
///                     name: "Demographics".to_string(),
///                     category_type: "normal".to_string(),
///                     highest_index: 0,
///                     fields: Some(vec![Field {
///                         name: "breed".to_string(),
///                         field_type: "combo-box".to_string(),
///                         data_type: Some("string".to_string()),
///                         error_code: "valid".to_string(),
///                         when_created: Some(DateTime::parse_from_rfc3339("2023-04-15T16:08:26Z")
///                             .unwrap()
///                             .with_timezone(&Utc)),
///                         keep_history: true,
///                         entries: Some(vec![Entry {
///                             entry_id: "1".to_string(),
///                             value: Some(Value {
///                                 by: "Paul Sanders".to_string(),
///                                 by_unique_id: Some("1681162687395".to_string()),
///                                 role: "Project Manager".to_string(),
///                                 when: Some(DateTime::parse_from_rfc3339("2023-04-15T16:09:02Z")
///                                     .unwrap()
///                                     .with_timezone(&Utc)),
///                                 value: "Labrador".to_string(),
///                             }),
///                             reason: None,
///                         }]),
///                         comments: None,
///                     }]),
///                 }]),
///             }]),
///         },
///     ],
/// };
/// let result = parse_subject_native_string(xml).unwrap();
///
/// assert_eq!(result, expected);
/// ```
pub fn parse_subject_native_string(xml_str: &str) -> Result<SubjectNative, Error> {
    parse_subject_native_streaming(Cursor::new(xml_str.as_bytes()))
}

use crate::native::common::{Category, Comment, Entry, Field, Reason, State, Value};

fn parse_subject_native_streaming<R: std::io::BufRead>(reader: R) -> Result<SubjectNative, Error> {
    let mut xml_reader = Reader::from_reader(reader);
    xml_reader.config_mut().trim_text(true);

    let mut patients = Vec::new();
    let mut buf = Vec::new();
    let mut text_content = String::new();

    let mut current_patient: Option<Patient> = None;
    let mut current_forms: Vec<Form> = Vec::new();
    let mut current_form: Option<Form> = None;
    let mut current_states: Vec<State> = Vec::new();
    let mut current_categories: Vec<Category> = Vec::new();
    let mut current_category: Option<Category> = None;
    let mut current_fields: Vec<Field> = Vec::new();
    let mut current_field: Option<Field> = None;
    let mut current_entries: Vec<Entry> = Vec::new();
    let mut current_entry: Option<Entry> = None;
    let mut current_comments: Vec<Comment> = Vec::new();
    let mut current_comment: Option<Comment> = None;
    let mut current_value: Option<Value> = None;
    let mut current_reason: Option<Reason> = None;

    let mut in_patient = false;
    let mut in_form = false;
    let mut in_category = false;
    let mut in_field = false;
    let mut in_entry = false;
    let mut in_comment = false;
    let mut in_value = false;
    let mut in_reason = false;

    loop {
        match xml_reader.read_event_into(&mut buf) {
            Err(e) => {
                return Err(Error::ParsingError(quick_xml::de::DeError::Custom(
                    format!("XML reading error: {}", e),
                )))
            }
            Ok(Event::Eof) => break,

            Ok(Event::Start(ref e)) => {
                let name_bytes = e.local_name();
                if let Ok(name) = std::str::from_utf8(name_bytes.as_ref()) {
                    match name {
                        "patient" => {
                            let attrs = extract_attributes(e)?;
                            current_patient = Some(Patient::from_attributes(attrs)?);
                            in_patient = true;
                            current_forms.clear();
                        }
                        "form" if in_patient => {
                            let attrs = extract_attributes(e)?;
                            current_form = Some(Form::from_attributes(attrs)?);
                            in_form = true;
                            current_states.clear();
                            current_categories.clear();
                        }
                        "category" if in_form => {
                            let attrs = extract_attributes(e)?;
                            current_category = Some(Category::from_attributes(attrs)?);
                            in_category = true;
                            current_fields.clear();
                        }
                        "field" if in_category => {
                            let attrs = extract_attributes(e)?;
                            current_field = Some(Field::from_attributes(attrs)?);
                            in_field = true;
                            current_entries.clear();
                            current_comments.clear();
                        }
                        "entry" if in_field => {
                            let attrs = extract_attributes(e)?;
                            current_entry = Some(Entry::from_attributes(attrs)?);
                            in_entry = true;
                        }
                        "comment" if in_field => {
                            let attrs = extract_attributes(e)?;
                            let comment_id = attrs.get("id").cloned().unwrap_or_default();
                            current_comment = Some(Comment {
                                comment_id,
                                value: None,
                            });
                            in_comment = true;
                        }
                        "value" if in_entry || in_comment => {
                            let attrs = extract_attributes(e)?;
                            current_value = Some(Value::from_attributes(attrs)?);
                            in_value = true;
                            text_content.clear();
                        }
                        "reason" if in_entry => {
                            let attrs = extract_attributes(e)?;
                            current_reason = Some(Reason::from_attributes(attrs)?);
                            in_reason = true;
                            text_content.clear();
                        }
                        _ => {}
                    }
                }
            }

            Ok(Event::Text(e)) => {
                if in_value || in_reason {
                    text_content.push_str(&String::from_utf8_lossy(&e));
                }
            }

            Ok(Event::End(ref e)) => {
                let name_bytes = e.local_name();
                if let Ok(name) = std::str::from_utf8(name_bytes.as_ref()) {
                    match name {
                        "patient" => {
                            if let Some(mut patient) = current_patient.take() {
                                if !current_forms.is_empty() {
                                    patient.set_forms(current_forms.clone());
                                }
                                patients.push(patient);
                            }
                            in_patient = false;
                            current_forms.clear();
                        }
                        "form" if in_form => {
                            if let Some(mut form) = current_form.take() {
                                if !current_states.is_empty() {
                                    form.states = Some(current_states.clone());
                                }
                                if !current_categories.is_empty() {
                                    form.categories = Some(current_categories.clone());
                                }
                                current_forms.push(form);
                            }
                            in_form = false;
                            current_states.clear();
                            current_categories.clear();
                        }
                        "category" if in_category => {
                            if let Some(mut category) = current_category.take() {
                                if !current_fields.is_empty() {
                                    category.fields = Some(current_fields.clone());
                                }
                                current_categories.push(category);
                            }
                            in_category = false;
                            current_fields.clear();
                        }
                        "field" if in_field => {
                            if let Some(mut field) = current_field.take() {
                                if !current_entries.is_empty() {
                                    field.entries = Some(current_entries.clone());
                                }
                                if !current_comments.is_empty() {
                                    field.comments = Some(current_comments.clone());
                                }
                                current_fields.push(field);
                            }
                            in_field = false;
                            current_entries.clear();
                            current_comments.clear();
                        }
                        "entry" if in_entry => {
                            if let Some(entry) = current_entry.take() {
                                current_entries.push(entry);
                            }
                            in_entry = false;
                        }
                        "comment" if in_comment => {
                            if let Some(comment) = current_comment.take() {
                                current_comments.push(comment);
                            }
                            in_comment = false;
                        }
                        "value" if in_value => {
                            if let Some(mut value) = current_value.take() {
                                value.value = text_content.clone();
                                if let Some(ref mut entry) = current_entry {
                                    entry.value = Some(value.clone());
                                }
                                if let Some(ref mut comment) = current_comment {
                                    comment.value = Some(value);
                                }
                            }
                            in_value = false;
                            text_content.clear();
                        }
                        "reason" if in_reason => {
                            if let Some(mut reason) = current_reason.take() {
                                reason.value = text_content.clone();
                                if let Some(ref mut entry) = current_entry {
                                    entry.reason = Some(reason);
                                }
                            }
                            in_reason = false;
                            text_content.clear();
                        }
                        _ => {}
                    }
                }
            }

            Ok(Event::Empty(ref e)) => {
                let name_bytes = e.local_name();
                if let Ok(name) = std::str::from_utf8(name_bytes.as_ref()) {
                    match name {
                        "state" if in_form => {
                            let attrs = extract_attributes(e)?;
                            let state = State::from_attributes(attrs)?;
                            current_states.push(state);
                        }
                        "value" if in_entry => {
                            let attrs = extract_attributes(e)?;
                            let value = Value::from_attributes(attrs)?;
                            if let Some(ref mut entry) = current_entry {
                                entry.value = Some(value);
                            }
                        }
                        "reason" if in_entry => {
                            let attrs = extract_attributes(e)?;
                            let reason = Reason::from_attributes(attrs)?;
                            if let Some(ref mut entry) = current_entry {
                                entry.reason = Some(reason);
                            }
                        }
                        _ => {}
                    }
                }
            }

            _ => {}
        }

        buf.clear();
    }

    Ok(SubjectNative { patients })
}

fn extract_attributes(e: &BytesStart) -> Result<HashMap<String, String>, Error> {
    let mut attrs = HashMap::new();
    for attr in e.attributes() {
        let attr = attr.map_err(|e| {
            Error::ParsingError(quick_xml::de::DeError::Custom(format!(
                "Attribute error: {}",
                e
            )))
        })?;
        let key = String::from_utf8_lossy(attr.key.as_ref()).to_string();
        let value = String::from_utf8_lossy(&attr.value).to_string();
        attrs.insert(key, value);
    }
    Ok(attrs)
}

/// Parses a Prelude native user XML file into a `UserNative` stuct.
///
/// # Example
///
/// ```
/// use std::path::Path;
///
/// use prelude_xml_parser::parse_user_native_file;
///
/// let file_path = Path::new("tests/assets/user_native.xml");
/// let native = parse_user_native_file(&file_path).unwrap();
///
/// assert!(native.users.len() >= 1, "Vector length is less than 1");
/// ```
pub fn parse_user_native_file(xml_path: &Path) -> Result<UserNative, Error> {
    check_valid_xml_file(xml_path)?;

    let xml_file = read_to_string(xml_path)?;
    let native = parse_user_native_string(&xml_file)?;

    Ok(native)
}

/// Parse a string of Prelude native user XML into a `UserNative` struct.
///
/// # Example
///
/// ```
/// use chrono::{DateTime, Utc};
/// use prelude_xml_parser::parse_user_native_string;
/// use prelude_xml_parser::native::user_native::*;
///
/// let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
///   <export_from_vision_EDC date="02-Jun-2024 06:59 -0500" createdBy="Paul Sanders" role="Project Manager" numberSubjectsProcessed="3">
///     <user uniqueId="1691421275437" lastLanguage="" creator="Paul Sanders(1681162687395)" numberOfForms="1">
///       <form name="form.name.demographics" lastModified="2023-08-07 10:15:41 -0500" whoLastModifiedName="Paul Sanders" whoLastModifiedRole="Project Manager" whenCreated="1691421341578" hasErrors="false" hasWarnings="false" locked="false" user="" dateTimeChanged="" formTitle="User Demographics" formIndex="1" formGroup="" formState="In-Work">
///         <state value="form.state.in.work" signer="Paul Sanders - Project Manager" signerUniqueId="1681162687395" dateSigned="2023-08-07 10:15:41 -0500" />
///         <category name="demographics" type="normal" highestIndex="0">
///           <field name="address" type="text" dataType="string" errorCode="undefined" whenCreated="2024-01-12 14:14:09 -0600" keepHistory="true" />
///           <field name="email" type="text" dataType="string" errorCode="undefined" whenCreated="2023-08-07 10:15:41 -0500" keepHistory="true">
///             <entry id="1">
///               <value by="Paul Sanders" byUniqueId="1681162687395" role="Project Manager" when="2023-08-07 10:15:41 -0500" xml:space="preserve">jazz@artemis.com</value>
///             </entry>
///           </field>
///         </category>
///         <category name="Administrative" type="normal" highestIndex="0">
///           <field name="study_assignment" type="text" dataType="" errorCode="undefined" whenCreated="2023-08-07 10:15:41 -0500" keepHistory="true">
///             <entry id="1">
///               <value by="set from calculation" byUniqueId="" role="System" when="2023-08-07 10:15:41 -0500" xml:space="preserve">On 07-Aug-2023 10:15 -0500, Paul Sanders assigned user from another study</value>
///               <reason by="set from calculation" byUniqueId="" role="System" when="2023-08-07 10:15:41 -0500" xml:space="preserve">calculated value</reason>
///             </entry>
///           </field>
///         </category>
///       </form>
///     </user>
///   </export_from_vision_EDC>
/// "#;
///
/// let expected = UserNative {
///     users: vec![User {
///         unique_id: "1691421275437".to_string(),
///         last_language: None,
///         creator: "Paul Sanders(1681162687395)".to_string(),
///         number_of_forms: 1,
///         forms: Some(vec![Form {
///             name: "form.name.demographics".to_string(),
///             last_modified: Some(
///                 DateTime::parse_from_rfc3339("2023-08-07T15:15:41Z")
///                     .unwrap()
///                     .with_timezone(&Utc),
///             ),
///             who_last_modified_name: Some("Paul Sanders".to_string()),
///             who_last_modified_role: Some("Project Manager".to_string()),
///             when_created: 1691421341578,
///             has_errors: false,
///             has_warnings: false,
///             locked: false,
///             user: None,
///             date_time_changed: None,
///             form_title: "User Demographics".to_string(),
///             form_index: 1,
///             form_group: None,
///             form_state: "In-Work".to_string(),
///             states: Some(vec![State {
///                 value: "form.state.in.work".to_string(),
///                 signer: "Paul Sanders - Project Manager".to_string(),
///                 signer_unique_id: "1681162687395".to_string(),
///                 date_signed: Some(
///                     DateTime::parse_from_rfc3339("2023-08-07T15:15:41Z")
///                         .unwrap()
///                         .with_timezone(&Utc),
///                 ),
///             }]),
///             categories: Some(vec![
///                         Category {
///                             name: "demographics".to_string(),
///                             category_type: "normal".to_string(),
///                             highest_index: 0,
///                             fields: Some(vec![
///                                 Field {
///                                     name: "address".to_string(),
///                                     field_type: "text".to_string(),
///                                     data_type: Some("string".to_string()),
///                                     error_code: "undefined".to_string(),
///                                     when_created: Some(DateTime::parse_from_rfc3339("2024-01-12T20:14:09Z")
///                                         .unwrap()
///                                         .with_timezone(&Utc)),
///                                     keep_history: true,
///                                     entries: None,
///                                     comments: None,
///                                 },
///                                 Field {
///                                     name: "email".to_string(),
///                                     field_type: "text".to_string(),
///                                     data_type: Some("string".to_string()),
///                                     error_code: "undefined".to_string(),
///                                     when_created: Some(DateTime::parse_from_rfc3339("2023-08-07T15:15:41Z")
///                                         .unwrap()
///                                         .with_timezone(&Utc)),
///                                     keep_history: true,
///                                     entries: Some(vec![Entry {
///                                         entry_id: "1".to_string(),
///                                         value: Some(Value {
///                                             by: "Paul Sanders".to_string(),
///                                             by_unique_id: Some("1681162687395".to_string()),
///                                             role: "Project Manager".to_string(),
///                                             when: Some(DateTime::parse_from_rfc3339("2023-08-07T15:15:41Z")
///                                                 .unwrap()
///                                                 .with_timezone(&Utc)),
///                                             value: "jazz@artemis.com".to_string(),
///                                         }),
///                                         reason: None,
///                                     }]),
///                                     comments: None,
///                                 },
///                             ]),
///                         },
///                         Category {
///                             name: "Administrative".to_string(),
///                             category_type: "normal".to_string(),
///                             highest_index: 0,
///                             fields: Some(vec![
///                                 Field {
///                                     name: "study_assignment".to_string(),
///                                     field_type: "text".to_string(),
///                                     data_type: None,
///                                     error_code: "undefined".to_string(),
///                                     when_created: Some(DateTime::parse_from_rfc3339("2023-08-07T15:15:41Z")
///                                         .unwrap()
///                                         .with_timezone(&Utc)),
///                                     keep_history: true,
///                                     entries: Some(vec![
///                                         Entry {
///                                             entry_id: "1".to_string(),
///                                             value: Some(Value {
///                                                 by: "set from calculation".to_string(),
///                                                 by_unique_id: None,
///                                                 role: "System".to_string(),
///                                                 when: Some(DateTime::parse_from_rfc3339("2023-08-07T15:15:41Z")
///                                                     .unwrap()
///                                                     .with_timezone(&Utc)),
///                                                 value: "On 07-Aug-2023 10:15 -0500, Paul Sanders assigned user from another study".to_string(),
///                                             }),
///                                             reason: Some(Reason {
///                                                 by: "set from calculation".to_string(),
///                                                 by_unique_id: None,
///                                                 role: "System".to_string(),
///                                                 when: Some(DateTime::parse_from_rfc3339("2023-08-07T15:15:41Z")
///                                                     .unwrap()
///                                                     .with_timezone(&Utc)),
///                                                 value: "calculated value".to_string(),
///                                             }),
///                                         },
///                                     ]),
///                                     comments: None,
///                                 },
///                             ]),
///                         },
///             ]),
///         }]),
///     }],
/// };
///
/// let result = parse_user_native_string(xml).unwrap();
///
/// assert_eq!(result, expected);
/// ```
pub fn parse_user_native_string(xml_str: &str) -> Result<UserNative, Error> {
    let native: UserNative = quick_xml::de::from_str(xml_str)?;

    Ok(native)
}

fn check_valid_xml_file(xml_path: &Path) -> Result<(), Error> {
    if !xml_path.exists() {
        return Err(Error::FileNotFound(xml_path.to_path_buf()));
    }

    if let Some(extension) = xml_path.extension() {
        if extension != "xml" {
            return Err(Error::InvalidFileType(xml_path.to_owned()));
        }
    } else {
        return Err(Error::Unknown);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::{tempdir, Builder};

    #[test]
    fn test_site_file_not_found_error() {
        let dir = tempdir().unwrap().path().to_path_buf();
        let result = parse_site_native_file(&dir);
        assert!(result.is_err());
        assert!(matches!(result, Err(Error::FileNotFound(_))));
    }

    #[test]
    fn test_site_invaid_file_type_error() {
        let file = Builder::new()
            .prefix("test")
            .suffix(".csv")
            .tempfile()
            .unwrap();
        let result = parse_site_native_file(file.path());

        assert!(result.is_err());
        assert!(matches!(result, Err(Error::InvalidFileType(_))));
    }

    #[test]
    fn test_subject_file_not_found_error() {
        let dir = tempdir().unwrap().path().to_path_buf();
        let result = parse_subject_native_file(&dir);
        assert!(result.is_err());
        assert!(matches!(result, Err(Error::FileNotFound(_))));
    }

    #[test]
    fn test_subject_invaid_file_type_error() {
        let file = Builder::new()
            .prefix("test")
            .suffix(".csv")
            .tempfile()
            .unwrap();
        let result = parse_subject_native_file(file.path());

        assert!(result.is_err());
        assert!(matches!(result, Err(Error::InvalidFileType(_))));
    }

    #[test]
    fn test_user_file_not_found_error() {
        let dir = tempdir().unwrap().path().to_path_buf();
        let result = parse_user_native_file(&dir);
        assert!(result.is_err());
        assert!(matches!(result, Err(Error::FileNotFound(_))));
    }

    #[test]
    fn test_user_invaid_file_type_error() {
        let file = Builder::new()
            .prefix("test")
            .suffix(".csv")
            .tempfile()
            .unwrap();
        let result = parse_user_native_file(file.path());

        assert!(result.is_err());
        assert!(matches!(result, Err(Error::InvalidFileType(_))));
    }

    #[test]
    fn test_forms_parsing_regression() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<export_from_vision_EDC date="30-May-2024 10:35 -0500" createdBy="Test User" role="Project Manager" numberSubjectsProcessed="1">
    <patient patientId="TEST-001" uniqueId="123456789" whenCreated="2023-04-15 12:09:02 -0400" creator="Test User" siteName="Test Site" siteUniqueId="987654321" lastLanguage="English" numberOfForms="2">
        <form name="test.form.1" lastModified="2023-04-15 12:09:15 -0400" whoLastModifiedName="Test User" whoLastModifiedRole="Tester" whenCreated="123456789" hasErrors="false" hasWarnings="false" locked="false" user="" dateTimeChanged="" formTitle="Test Form 1" formIndex="1" formGroup="Test Group" formState="In-Work">
            <state value="form.state.in.work" signer="Test User - Tester" signerUniqueId="111111111" dateSigned="2023-04-15 12:09:02 -0400"/>
            <category name="Test Category" type="normal" highestIndex="0">
                <field name="test_field" type="text" dataType="string" errorCode="valid" whenCreated="2023-04-15 12:08:26 -0400" keepHistory="true">
                    <entry id="1">
                        <value by="Test User" byUniqueId="111111111" role="Tester" when="2023-04-15 12:09:02 -0400" xml:space="preserve">Test Value</value>
                    </entry>
                </field>
            </category>
        </form>
        <form name="test.form.2" lastModified="2023-04-15 12:10:15 -0400" whoLastModifiedName="Test User" whoLastModifiedRole="Tester" whenCreated="123456790" hasErrors="false" hasWarnings="false" locked="false" user="" dateTimeChanged="" formTitle="Test Form 2" formIndex="2" formGroup="Test Group" formState="Complete">
            <state value="form.state.complete" signer="Test User - Tester" signerUniqueId="111111111" dateSigned="2023-04-15 12:10:02 -0400"/>
        </form>
    </patient>
</export_from_vision_EDC>"#;

        let result = parse_subject_native_string(xml).expect("Should parse successfully");

        assert_eq!(result.patients.len(), 1, "Should have exactly 1 patient");

        let patient = &result.patients[0];
        assert_eq!(patient.patient_id, "TEST-001");
        assert_eq!(patient.number_of_forms, 2);

        let forms = patient.forms.as_ref().expect("Patient should have forms");
        assert_eq!(forms.len(), 2, "Patient should have exactly 2 forms");

        let form1 = &forms[0];
        assert_eq!(form1.name, "test.form.1");
        assert_eq!(form1.form_title, "Test Form 1");
        assert_eq!(form1.form_index, 1);
        assert_eq!(form1.form_state, "In-Work");

        let states1 = form1.states.as_ref().expect("Form 1 should have states");
        assert_eq!(states1.len(), 1);
        assert_eq!(states1[0].value, "form.state.in.work");

        let categories1 = form1
            .categories
            .as_ref()
            .expect("Form 1 should have categories");
        assert_eq!(categories1.len(), 1);
        assert_eq!(categories1[0].name, "Test Category");

        let fields1 = categories1[0]
            .fields
            .as_ref()
            .expect("Category should have fields");
        assert_eq!(fields1.len(), 1);
        assert_eq!(fields1[0].name, "test_field");

        let entries1 = fields1[0]
            .entries
            .as_ref()
            .expect("Field should have entries");
        assert_eq!(entries1.len(), 1);
        assert_eq!(entries1[0].entry_id, "1");

        let value1 = entries1[0].value.as_ref().expect("Entry should have value");
        assert_eq!(value1.value, "Test Value");
        assert_eq!(value1.by, "Test User");
        assert_eq!(value1.role, "Tester");

        let form2 = &forms[1];
        assert_eq!(form2.name, "test.form.2");
        assert_eq!(form2.form_title, "Test Form 2");
        assert_eq!(form2.form_index, 2);
        assert_eq!(form2.form_state, "Complete");

        let states2 = form2.states.as_ref().expect("Form 2 should have states");
        assert_eq!(states2.len(), 1);
        assert_eq!(states2[0].value, "form.state.complete");
    }

    #[test]
    fn test_comments_parsing_regression() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<export_from_vision_EDC date="30-May-2024 10:35 -0500" createdBy="Test User" role="Project Manager" numberSubjectsProcessed="1">
    <patient patientId="TEST-002" uniqueId="123456790" whenCreated="2023-04-15 12:09:02 -0400" creator="Test User" siteName="Test Site" siteUniqueId="987654321" lastLanguage="English" numberOfForms="1">
        <form name="test.form.with.comments" lastModified="2023-04-15 12:09:15 -0400" whoLastModifiedName="Test User" whoLastModifiedRole="Tester" whenCreated="123456789" hasErrors="false" hasWarnings="false" locked="false" user="" dateTimeChanged="" formTitle="Test Form With Comments" formIndex="1" formGroup="Test Group" formState="In-Work">
            <category name="Test Category" type="normal" highestIndex="0">
                <field name="field_with_comments" type="text" dataType="string" errorCode="valid" whenCreated="2023-04-15 12:08:26 -0400" keepHistory="true">
                    <entry id="1">
                        <value by="Test User" byUniqueId="111111111" role="Tester" when="2023-04-15 12:09:02 -0400" xml:space="preserve">Field Value</value>
                    </entry>
                    <comment id="1">
                        <value by="Test User" byUniqueId="111111111" role="Tester" when="2023-04-15 12:09:05 -0400" xml:space="preserve">First comment</value>
                    </comment>
                    <comment id="2">
                        <value by="Another User" byUniqueId="222222222" role="Reviewer" when="2023-04-15 12:10:00 -0400" xml:space="preserve">Second comment</value>
                    </comment>
                </field>
                <field name="field_without_comments" type="text" dataType="string" errorCode="valid" whenCreated="2023-04-15 12:08:30 -0400" keepHistory="true">
                    <entry id="1">
                        <value by="Test User" byUniqueId="111111111" role="Tester" when="2023-04-15 12:09:10 -0400" xml:space="preserve">Another Value</value>
                    </entry>
                </field>
            </category>
        </form>
    </patient>
</export_from_vision_EDC>"#;

        let result = parse_subject_native_string(xml).expect("Should parse successfully");

        assert_eq!(result.patients.len(), 1, "Should have exactly 1 patient");

        let patient = &result.patients[0];
        let forms = patient.forms.as_ref().expect("Patient should have forms");
        let form = &forms[0];
        let categories = form
            .categories
            .as_ref()
            .expect("Form should have categories");
        let fields = categories[0]
            .fields
            .as_ref()
            .expect("Category should have fields");
        assert_eq!(fields.len(), 2, "Should have 2 fields");

        let field_with_comments = &fields[0];
        assert_eq!(field_with_comments.name, "field_with_comments");

        let comments = field_with_comments
            .comments
            .as_ref()
            .expect("Field should have comments");
        assert_eq!(comments.len(), 2, "Should have exactly 2 comments");

        let comment1 = &comments[0];
        assert_eq!(comment1.comment_id, "1");
        let comment1_value = comment1
            .value
            .as_ref()
            .expect("Comment 1 should have value");
        assert_eq!(comment1_value.value, "First comment");
        assert_eq!(comment1_value.by, "Test User");
        assert_eq!(comment1_value.role, "Tester");

        let comment2 = &comments[1];
        assert_eq!(comment2.comment_id, "2");
        let comment2_value = comment2
            .value
            .as_ref()
            .expect("Comment 2 should have value");
        assert_eq!(comment2_value.value, "Second comment");
        assert_eq!(comment2_value.by, "Another User");
        assert_eq!(comment2_value.role, "Reviewer");

        let field_without_comments = &fields[1];
        assert_eq!(field_without_comments.name, "field_without_comments");
        assert!(
            field_without_comments.comments.is_none(),
            "Field without comments should have no comments"
        );
    }

    #[test]
    fn test_empty_forms_handling() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<export_from_vision_EDC date="30-May-2024 10:35 -0500" createdBy="Test User" role="Project Manager" numberSubjectsProcessed="1">
    <patient patientId="TEST-003" uniqueId="123456791" whenCreated="2023-04-15 12:09:02 -0400" creator="Test User" siteName="Test Site" siteUniqueId="987654321" lastLanguage="English" numberOfForms="0">
    </patient>
</export_from_vision_EDC>"#;

        let result = parse_subject_native_string(xml).expect("Should parse successfully");

        assert_eq!(result.patients.len(), 1, "Should have exactly 1 patient");

        let patient = &result.patients[0];
        assert_eq!(patient.patient_id, "TEST-003");
        assert_eq!(patient.number_of_forms, 0);
        assert!(
            patient.forms.is_none(),
            "Patient with 0 forms should have None for forms"
        );
    }

    #[test]
    fn test_large_patient_forms_regression() {
        let mut xml = String::from(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<export_from_vision_EDC date="30-May-2024 10:35 -0500" createdBy="Test User" role="Project Manager" numberSubjectsProcessed="1">
    <patient patientId="LARGE-TEST" uniqueId="123456792" whenCreated="2023-04-15 12:09:02 -0400" creator="Test User" siteName="Test Site" siteUniqueId="987654321" lastLanguage="English" numberOfForms="50">"#,
        );

        for i in 1..=50 {
            xml.push_str(&format!(r#"
        <form name="test.form.{}" lastModified="2023-04-15 12:09:15 -0400" whoLastModifiedName="Test User" whoLastModifiedRole="Tester" whenCreated="12345678{}" hasErrors="false" hasWarnings="false" locked="false" user="" dateTimeChanged="" formTitle="Test Form {}" formIndex="{}" formGroup="Test Group" formState="In-Work">
            <state value="form.state.in.work" signer="Test User - Tester" signerUniqueId="111111111" dateSigned="2023-04-15 12:09:02 -0400"/>
            <category name="Category {}" type="normal" highestIndex="0">
                <field name="field_{}" type="text" dataType="string" errorCode="valid" whenCreated="2023-04-15 12:08:26 -0400" keepHistory="true">
                    <entry id="1">
                        <value by="Test User" byUniqueId="111111111" role="Tester" when="2023-04-15 12:09:02 -0400" xml:space="preserve">Value {}</value>
                    </entry>
                    <comment id="1">
                        <value by="Test User" byUniqueId="111111111" role="Tester" when="2023-04-15 12:09:05 -0400" xml:space="preserve">Comment for form {}</value>
                    </comment>
                </field>
            </category>
        </form>"#, i, i, i, i, i, i, i, i));
        }

        xml.push_str(
            r#"
    </patient>
</export_from_vision_EDC>"#,
        );

        let result =
            parse_subject_native_string(&xml).expect("Should parse large patient successfully");

        assert_eq!(result.patients.len(), 1, "Should have exactly 1 patient");

        let patient = &result.patients[0];
        assert_eq!(patient.patient_id, "LARGE-TEST");
        assert_eq!(patient.number_of_forms, 50);

        let forms = patient.forms.as_ref().expect("Patient should have forms");
        assert_eq!(forms.len(), 50, "Patient should have exactly 50 forms");

        for (i, form) in forms.iter().enumerate() {
            let form_num = i + 1;
            assert_eq!(form.name, format!("test.form.{}", form_num));
            assert_eq!(form.form_title, format!("Test Form {}", form_num));
            assert_eq!(form.form_index, form_num);

            let categories = form
                .categories
                .as_ref()
                .expect("Form should have categories");
            assert_eq!(categories.len(), 1);

            let fields = categories[0]
                .fields
                .as_ref()
                .expect("Category should have fields");
            assert_eq!(fields.len(), 1);

            let entries = fields[0]
                .entries
                .as_ref()
                .expect("Field should have entries");
            assert_eq!(entries.len(), 1);
            assert_eq!(
                entries[0].value.as_ref().unwrap().value,
                format!("Value {}", form_num)
            );

            let comments = fields[0]
                .comments
                .as_ref()
                .expect("Field should have comments");
            assert_eq!(comments.len(), 1);
            assert_eq!(
                comments[0].value.as_ref().unwrap().value,
                format!("Comment for form {}", form_num)
            );
        }
    }

    #[test]
    fn test_malformed_datetime_handling() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<export_from_vision_EDC date="30-May-2024 10:35 -0500" createdBy="Test User" role="Project Manager" numberSubjectsProcessed="1">
    <patient patientId="TEST-004" uniqueId="123456793" whenCreated="" creator="Test User" siteName="Test Site" siteUniqueId="987654321" lastLanguage="English" numberOfForms="1">
        <form name="test.form.malformed.dates" lastModified="" whoLastModifiedName="Test User" whoLastModifiedRole="Tester" whenCreated="123456789" hasErrors="false" hasWarnings="false" locked="false" user="" dateTimeChanged="" formTitle="Test Form" formIndex="1" formGroup="Test Group" formState="In-Work">
            <category name="Test Category" type="normal" highestIndex="0">
                <field name="test_field" type="text" dataType="string" errorCode="valid" whenCreated="" keepHistory="true">
                    <entry id="1">
                        <value by="Test User" byUniqueId="111111111" role="Tester" when="2023-04-15 12:09:02 -0400" xml:space="preserve">Test Value</value>
                    </entry>
                </field>
            </category>
        </form>
    </patient>
</export_from_vision_EDC>"#;

        let result =
            parse_subject_native_string(xml).expect("Should handle malformed datetimes gracefully");

        assert_eq!(result.patients.len(), 1, "Should have exactly 1 patient");

        let patient = &result.patients[0];
        assert!(
            patient.when_created.is_none(),
            "Empty whenCreated should be None"
        );

        let forms = patient.forms.as_ref().expect("Patient should have forms");
        let form = &forms[0];
        assert!(
            form.last_modified.is_none(),
            "Empty lastModified should be None"
        );

        let categories = form
            .categories
            .as_ref()
            .expect("Form should have categories");
        let fields = categories[0]
            .fields
            .as_ref()
            .expect("Category should have fields");
        let field = &fields[0];
        assert!(
            field.when_created.is_none(),
            "Empty whenCreated in field should be None"
        );
    }

    #[test]
    fn test_empty_datetime_in_value_and_reason() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<export_from_vision_EDC date="30-May-2024 10:35 -0500" createdBy="Test User" role="Project Manager" numberSubjectsProcessed="1">
  <patient patientId="TEST-001" uniqueId="123456" whenCreated="2023-04-15 12:09:02 -0400" creator="Test User" siteName="Test Site" siteUniqueId="654321" lastLanguage="" numberOfForms="1">
    <form name="test.form" lastModified="2023-04-15 12:09:15 -0400" whoLastModifiedName="Test User" whoLastModifiedRole="Manager" whenCreated="1681574905839" hasErrors="false" hasWarnings="false" locked="false" user="" dateTimeChanged="" formTitle="Test Form" formIndex="1" formGroup="Test" formState="In-Work">
      <state value="form.state.in.work" signer="Test User - Manager" signerUniqueId="123456" dateSigned="2023-04-15 12:09:02 -0400" />
      <category name="Test Category" type="normal" highestIndex="0">
        <field name="test_field" type="text" dataType="string" errorCode="valid" whenCreated="2023-04-15 12:08:26 -0400" keepHistory="true">
          <entry id="1">
            <value by="Test User" byUniqueId="123456" role="Manager" when="" xml:space="preserve">Test Value</value>
            <reason by="Test User" byUniqueId="123456" role="Manager" when="" xml:space="preserve">Test Reason</reason>
          </entry>
        </field>
      </category>
    </form>
  </patient>
</export_from_vision_EDC>"#;

        let result = parse_subject_native_string(xml);
        assert!(result.is_ok(), "Should parse successfully: {:?}", result);

        let native = result.unwrap();
        assert_eq!(native.patients.len(), 1, "Should have 1 patient");

        let patient = &native.patients[0];
        let forms = patient.forms.as_ref().expect("Patient should have forms");
        let form = &forms[0];
        let categories = form
            .categories
            .as_ref()
            .expect("Form should have categories");
        let fields = categories[0]
            .fields
            .as_ref()
            .expect("Category should have fields");
        let field = &fields[0];
        let entries = field.entries.as_ref().expect("Field should have entries");
        let entry = &entries[0];

        let value = entry.value.as_ref().expect("Entry should have value");
        assert!(
            value.when.is_none(),
            "Empty when attribute in value should be None"
        );
        assert_eq!(value.value, "Test Value");

        let reason = entry.reason.as_ref().expect("Entry should have reason");
        assert!(
            reason.when.is_none(),
            "Empty when attribute in reason should be None"
        );
        assert_eq!(reason.value, "Test Reason");
    }
}
