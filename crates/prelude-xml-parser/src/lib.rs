/// Module containing the [errors].
pub mod errors;
/// Module containing the [native] structs.
pub mod native;

use std::{fs::read_to_string, path::Path};

use crate::errors::Error;
use crate::native::{
    site_native::SiteNative, subject_native::SubjectNative, user_native::UserNative,
};

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

/// Parse a string of Preliude native site XML into a `SiteNative` struct.
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
///             when_created: DateTime::parse_from_rfc3339("2023-04-15T16:08:19Z")
///                 .unwrap()
///                 .with_timezone(&Utc),
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
///                         fields: vec![
///                             Field {
///                                 name: "address".to_string(),
///                                 field_type: "text".to_string(),
///                                 data_type: Some("string".to_string()),
///                                 error_code: "valid".to_string(),
///                                 when_created: DateTime::parse_from_rfc3339(
///                                     "2023-04-15T16:07:14Z",
///                                 )
///                                 .unwrap()
///                                 .with_timezone(&Utc),
///                                 keep_history: true,
///                                 entries: None,
///                             },
///                             Field {
///                                 name: "company".to_string(),
///                                 field_type: "text".to_string(),
///                                 data_type: Some("string".to_string()),
///                                 error_code: "valid".to_string(),
///                                 when_created: DateTime::parse_from_rfc3339(
///                                     "2023-04-15T16:07:14Z",
///                                 )
///                                 .unwrap()
///                                 .with_timezone(&Utc),
///                                 keep_history: true,
///                                 entries: Some(vec![Entry {
///                                     entry_id: "1".to_string(),
///                                     value: Some(Value {
///                                         by: "Paul Sanders".to_string(),
///                                         by_unique_id: Some("1681162687395".to_string()),
///                                         role: "Project Manager".to_string(),
///                                         when: DateTime::parse_from_rfc3339(
///                                             "2023-04-15T16:08:19Z",
///                                         )
///                                         .unwrap()
///                                         .with_timezone(&Utc),
///                                         value: "Some Company".to_string(),
///                                     }),
///                                     reason: None,
///                                 }]),
///                             },
///                             Field {
///                                 name: "site_code_name".to_string(),
///                                 field_type: "hidden".to_string(),
///                                 data_type: Some("string".to_string()),
///                                 error_code: "valid".to_string(),
///                                 when_created: DateTime::parse_from_rfc3339(
///                                     "2023-04-15T16:07:14Z",
///                                 )
///                                 .unwrap()
///                                 .with_timezone(&Utc),
///                                 keep_history: true,
///                                 entries: Some(vec![
///                                     Entry {
///                                         entry_id: "1".to_string(),
///                                         value: Some(Value {
///                                             by: "set from calculation".to_string(),
///                                             by_unique_id: None,
///                                             role: "System".to_string(),
///                                             when: DateTime::parse_from_rfc3339(
///                                                 "2023-04-15T16:08:19Z",
///                                             )
///                                             .unwrap()
///                                             .with_timezone(&Utc),
///                                             value: "ABC-Some Site".to_string(),
///                                         }),
///                                         reason: Some(Reason {
///                                             by: "set from calculation".to_string(),
///                                             by_unique_id: None,
///                                             role: "System".to_string(),
///                                             when: DateTime::parse_from_rfc3339(
///                                                 "2023-04-15T16:08:19Z",
///                                             )
///                                             .unwrap()
///                                             .with_timezone(&Utc),
///                                             value: "calculated value".to_string(),
///                                         }),
///                                     },
///                                     Entry {
///                                         entry_id: "2".to_string(),
///                                         value: Some(Value {
///                                             by: "set from calculation".to_string(),
///                                             by_unique_id: None,
///                                             role: "System".to_string(),
///                                             when: DateTime::parse_from_rfc3339(
///                                                 "2023-04-15T16:07:24Z",
///                                             )
///                                             .unwrap()
///                                             .with_timezone(&Utc),
///                                             value: "Some Site".to_string(),
///                                         }),
///                                         reason: Some(Reason {
///                                             by: "set from calculation".to_string(),
///                                             by_unique_id: None,
///                                             role: "System".to_string(),
///                                             when: DateTime::parse_from_rfc3339(
///                                                 "2023-04-15T16:07:24Z",
///                                             )
///                                             .unwrap()
///                                             .with_timezone(&Utc),
///                                             value: "calculated value".to_string(),
///                                         }),
///                                     },
///                                 ]),
///                             },
///                         ],
///                     },
///                     Category {
///                         name: "Enrollment".to_string(),
///                         category_type: "normal".to_string(),
///                         highest_index: 0,
///                         fields: vec![
///                             Field {
///                                 name: "enrollment_closed_date".to_string(),
///                                 field_type: "popUpCalendar".to_string(),
///                                 data_type: Some("date".to_string()),
///                                 error_code: "valid".to_string(),
///                                 when_created: DateTime::parse_from_rfc3339(
///                                     "2023-04-15T16:07:14Z",
///                                 )
///                                 .unwrap()
///                                 .with_timezone(&Utc),
///                                 keep_history: true,
///                                 entries: None,
///                             },
///                             Field {
///                                 name: "enrollment_open".to_string(),
///                                 field_type: "radio".to_string(),
///                                 data_type: Some("string".to_string()),
///                                 error_code: "valid".to_string(),
///                                 when_created: DateTime::parse_from_rfc3339(
///                                     "2023-04-15T16:07:14Z",
///                                 )
///                                 .unwrap()
///                                 .with_timezone(&Utc),
///                                 keep_history: true,
///                                 entries: Some(vec![Entry {
///                                     entry_id: "1".to_string(),
///                                     value: Some(Value {
///                                         by: "Paul Sanders".to_string(),
///                                         by_unique_id: Some("1681162687395".to_string()),
///                                         role: "Project Manager".to_string(),
///                                         when: DateTime::parse_from_rfc3339(
///                                             "2023-04-15T16:08:19Z",
///                                         )
///                                         .unwrap()
///                                         .with_timezone(&Utc),
///                                         value: "Yes".to_string(),
///                                     }),
///                                     reason: None,
///                                 }]),
///                             },
///                             Field {
///                                 name: "enrollment_open_date".to_string(),
///                                 field_type: "popUpCalendar".to_string(),
///                                 data_type: Some("date".to_string()),
///                                 error_code: "valid".to_string(),
///                                 when_created: DateTime::parse_from_rfc3339(
///                                     "2023-04-15T16:07:14Z",
///                                 )
///                                 .unwrap()
///                                 .with_timezone(&Utc),
///                                 keep_history: true,
///                                 entries: None,
///                             },
///                         ],
///                     },
///                 ]),
///             }]),
///         },
///         Site {
///             name: "Artemis".to_string(),
///             unique_id: "1691420994591".to_string(),
///             number_of_patients: 0,
///             count_of_randomized_patients: 0,
///             when_created: DateTime::parse_from_rfc3339("2023-08-07T15:14:23Z")
///                 .unwrap()
///                 .with_timezone(&Utc),
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
///                     fields: vec![Field {
///                         name: "address".to_string(),
///                         field_type: "text".to_string(),
///                         data_type: Some("string".to_string()),
///                         error_code: "valid".to_string(),
///                         when_created: DateTime::parse_from_rfc3339("2023-08-07T15:09:54Z")
///                             .unwrap()
///                             .with_timezone(&Utc),
///                         keep_history: true,
///                         entries: Some(vec![Entry {
///                             entry_id: "1".to_string(),
///                             value: Some(Value {
///                                 by: "Paul Sanders".to_string(),
///                                 by_unique_id: Some("1681162687395".to_string()),
///                                 role: "Project Manager".to_string(),
///                                 when: DateTime::parse_from_rfc3339("2023-08-07T15:14:21Z")
///                                     .unwrap()
///                                     .with_timezone(&Utc),
///                                 value: "1111 Moon Drive".to_string(),
///                             }),
///                             reason: None,
///                         }]),
///                     }],
///                 }]),
///             }]),
///         },
///     ],
/// };
/// let result = parse_site_native_string(xml).unwrap();
/// assert_eq!(result, expected);
pub fn parse_site_native_string(xml_str: &str) -> Result<SiteNative, Error> {
    let native: SiteNative = serde_xml_rs::from_str(xml_str)?;

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

    let xml_file = read_to_string(xml_path)?;
    let native = parse_subject_native_string(&xml_file)?;

    Ok(native)
}

/// Parse a string of Preliude native subject XML into a `SubjectNative` struct.
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
///             when_created: DateTime::parse_from_rfc3339("2023-04-15T16:09:02Z")
///                 .unwrap()
///                 .with_timezone(&Utc),
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
///                     fields: vec![Field {
///                         name: "breed".to_string(),
///                         field_type: "combo-box".to_string(),
///                         data_type: Some("string".to_string()),
///                         error_code: "valid".to_string(),
///                         when_created: DateTime::parse_from_rfc3339("2023-04-15T16:08:26Z")
///                             .unwrap()
///                             .with_timezone(&Utc),
///                         keep_history: true,
///                         entries: Some(vec![Entry {
///                             entry_id: "1".to_string(),
///                             value: Some(Value {
///                                 by: "Paul Sanders".to_string(),
///                                 by_unique_id: Some("1681162687395".to_string()),
///                                 role: "Project Manager".to_string(),
///                                 when: DateTime::parse_from_rfc3339("2023-04-15T16:09:02Z")
///                                     .unwrap()
///                                     .with_timezone(&Utc),
///                                 value: "Labrador".to_string(),
///                             }),
///                             reason: None,
///                         }]),
///                     }],
///                 }]),
///             }]),
///         },
///         Patient {
///             patient_id: "DEF-002".to_string(),
///             unique_id: "1681574905820".to_string(),
///             when_created: DateTime::parse_from_rfc3339("2023-04-16T16:10:02Z")
///                 .unwrap()
///                 .with_timezone(&Utc),
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
///                     fields: vec![Field {
///                         name: "breed".to_string(),
///                         field_type: "combo-box".to_string(),
///                         data_type: Some("string".to_string()),
///                         error_code: "valid".to_string(),
///                         when_created: DateTime::parse_from_rfc3339("2023-04-15T16:08:26Z")
///                             .unwrap()
///                             .with_timezone(&Utc),
///                         keep_history: true,
///                         entries: Some(vec![Entry {
///                             entry_id: "1".to_string(),
///                             value: Some(Value {
///                                 by: "Paul Sanders".to_string(),
///                                 by_unique_id: Some("1681162687395".to_string()),
///                                 role: "Project Manager".to_string(),
///                                 when: DateTime::parse_from_rfc3339("2023-04-15T16:09:02Z")
///                                     .unwrap()
///                                     .with_timezone(&Utc),
///                                 value: "Labrador".to_string(),
///                             }),
///                             reason: None,
///                         }]),
///                     }],
///                 }]),
///             }]),
///         },
///     ],
/// };
/// let result = parse_subject_native_string(xml).unwrap();
/// assert_eq!(result, expected);
/// ```
pub fn parse_subject_native_string(xml_str: &str) -> Result<SubjectNative, Error> {
    let native: SubjectNative = serde_xml_rs::from_str(xml_str)?;

    Ok(native)
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

/// Parse a string of Preliude native user XML into a `UserNative` struct.
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
///                             fields: vec![
///                                 Field {
///                                     name: "address".to_string(),
///                                     field_type: "text".to_string(),
///                                     data_type: Some("string".to_string()),
///                                     error_code: "undefined".to_string(),
///                                     when_created: DateTime::parse_from_rfc3339("2024-01-12T20:14:09Z")
///                                         .unwrap()
///                                         .with_timezone(&Utc),
///                                     keep_history: true,
///                                     entries: None,
///                                 },
///                                 Field {
///                                     name: "email".to_string(),
///                                     field_type: "text".to_string(),
///                                     data_type: Some("string".to_string()),
///                                     error_code: "undefined".to_string(),
///                                     when_created: DateTime::parse_from_rfc3339("2023-08-07T15:15:41Z")
///                                         .unwrap()
///                                         .with_timezone(&Utc),
///                                     keep_history: true,
///                                     entries: Some(vec![Entry {
///                                         entry_id: "1".to_string(),
///                                         value: Some(Value {
///                                             by: "Paul Sanders".to_string(),
///                                             by_unique_id: Some("1681162687395".to_string()),
///                                             role: "Project Manager".to_string(),
///                                             when: DateTime::parse_from_rfc3339("2023-08-07T15:15:41Z")
///                                                 .unwrap()
///                                                 .with_timezone(&Utc),
///                                             value: "jazz@artemis.com".to_string(),
///                                         }),
///                                         reason: None,
///                                     }]),
///                                 },
///                             ],
///                         },
///                         Category {
///                             name: "Administrative".to_string(),
///                             category_type: "normal".to_string(),
///                             highest_index: 0,
///                             fields: vec![
///                                 Field {
///                                     name: "study_assignment".to_string(),
///                                     field_type: "text".to_string(),
///                                     data_type: None,
///                                     error_code: "undefined".to_string(),
///                                     when_created: DateTime::parse_from_rfc3339("2023-08-07T15:15:41Z")
///                                         .unwrap()
///                                         .with_timezone(&Utc),
///                                     keep_history: true,
///                                     entries: Some(vec![
///                                         Entry {
///                                             entry_id: "1".to_string(),
///                                             value: Some(Value {
///                                                 by: "set from calculation".to_string(),
///                                                 by_unique_id: None,
///                                                 role: "System".to_string(),
///                                                 when: DateTime::parse_from_rfc3339("2023-08-07T15:15:41Z")
///                                                     .unwrap()
///                                                     .with_timezone(&Utc),
///                                                 value: "On 07-Aug-2023 10:15 -0500, Paul Sanders assigned user from another study".to_string(),
///                                             }),
///                                             reason: Some(Reason {
///                                                 by: "set from calculation".to_string(),
///                                                 by_unique_id: None,
///                                                 role: "System".to_string(),
///                                                 when: DateTime::parse_from_rfc3339("2023-08-07T15:15:41Z")
///                                                     .unwrap()
///                                                     .with_timezone(&Utc),
///                                                 value: "calculated value".to_string(),
///                                             }),
///                                         },
///                                     ]),
///                                 },
///                             ],
///                         },
///             ]),
///         }]),
///     }],
/// };
///
/// let result = parse_user_native_string(xml).unwrap();
/// assert_eq!(result, expected);
/// ```
pub fn parse_user_native_string(xml_str: &str) -> Result<UserNative, Error> {
    let native: UserNative = serde_xml_rs::from_str(xml_str)?;

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
}
