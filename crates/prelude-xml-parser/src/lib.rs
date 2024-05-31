/// Module containing the [errors].
pub mod errors;
/// Module containing the [native] structs.
pub mod native;

use std::{fs::read_to_string, path::Path};

use crate::errors::Error;
use crate::native::Native;

/// Parses a Prelude native XML file into a `Native` stuct.
///
/// # Example
///
/// ```
/// use std::path::Path;
///
/// use prelude_xml_parser::parse_native_file;
///
/// let file_path = Path::new("tests/assets/native.xml");
/// // let native = parse_native_file(&file_path).unwrap();
/// ```
pub fn parse_native_file(xml_path: &Path) -> Result<Native, Error> {
    if !xml_path.exists() {
        return Err(Error::FileNotFound(xml_path.to_path_buf()));
    }

    let xml_file = read_to_string(xml_path)?;
    let native = parse_native_string(&xml_file)?;

    Ok(native)
}

/// Parse a string of Preliude native XML into a `Native` struct.
///
/// # Example
///
/// ```
/// use chrono::{DateTime, Utc};
/// use prelude_xml_parser::parse_native_string;
/// use prelude_xml_parser::native::*;
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
/// let expected = Native {
///     patients: vec![
///         Patient {
///             patient_id: "ABC-001".to_string(),
///             unique_id: 1681574905819,
///             when_created: DateTime::parse_from_rfc3339("2023-04-15T16:09:02Z")
///                 .unwrap()
///                 .with_timezone(&Utc),
///             creator: "Paul Sanders".to_string(),
///             site_name: "Some Site".to_string(),
///             site_unique_id: 1681574834910,
///             last_language: Some("English".to_string()),
///             number_of_forms: 6,
///             forms: vec![Form {
///                 name: "day.0.form.name.demographics".to_string(),
///                 last_modified: DateTime::parse_from_rfc3339("2023-04-15T16:09:15Z")
///                     .unwrap()
///                     .with_timezone(&Utc),
///                 who_last_modified_name: "Paul Sanders".to_string(),
///                 who_last_modified_role: "Project Manager".to_string(),
///                 when_created: 1681574905839,
///                 has_errors: false,
///                 has_warnings: false,
///                 locked: false,
///                 user: None,
///                 date_time_changed: None,
///                 form_title: "Demographics".to_string(),
///                 form_index: 1,
///                 form_group: "Day 0".to_string(),
///                 form_state: "In-Work".to_string(),
///                 state: Some(State {
///                     value: "form.state.in.work".to_string(),
///                     signer: "Paul Sanders - Project Manager".to_string(),
///                     signer_unique_id: 1681162687395,
///                     date_signed: Some(
///                         DateTime::parse_from_rfc3339("2023-04-15T16:09:02Z")
///                             .unwrap()
///                             .with_timezone(&Utc),
///                     ),
///                 }),
///                 category: Category {
///                     name: "Demographics".to_string(),
///                     category_type: "normal".to_string(),
///                     highest_index: 0,
///                     fields: vec![Field {
///                         name: "breed".to_string(),
///                         field_type: "combo-box".to_string(),
///                         data_type: "string".to_string(),
///                         error_code: "valid".to_string(),
///                         when_created: DateTime::parse_from_rfc3339("2023-04-15T16:08:26Z")
///                             .unwrap()
///                             .with_timezone(&Utc),
///                         keep_history: true,
///                         entries: vec![Entry {
///                             id: 1,
///                             value: Value {
///                                 by: "Paul Sanders".to_string(),
///                                 by_unique_id: 1681162687395,
///                                 role: "Project Manager".to_string(),
///                                 when: DateTime::parse_from_rfc3339("2023-04-15T16:09:02Z")
///                                     .unwrap()
///                                     .with_timezone(&Utc),
///                                 value: "Labrador".to_string(),
///                             },
///                         }],
///                     }],
///                 },
///             }],
///         },
///         Patient {
///             patient_id: "DEF-002".to_string(),
///             unique_id: 1681574905820,
///             when_created: DateTime::parse_from_rfc3339("2023-04-16T16:10:02Z")
///                 .unwrap()
///                 .with_timezone(&Utc),
///             creator: "Wade Watts".to_string(),
///             site_name: "Another Site".to_string(),
///             site_unique_id: 1681574834911,
///             last_language: None,
///             number_of_forms: 8,
///             forms: vec![Form {
///                 name: "day.0.form.name.demographics".to_string(),
///                 last_modified: DateTime::parse_from_rfc3339("2023-04-16T16:10:15Z")
///                     .unwrap()
///                     .with_timezone(&Utc),
///                 who_last_modified_name: "Barney Rubble".to_string(),
///                 who_last_modified_role: "Technician".to_string(),
///                 when_created: 1681574905838,
///                 has_errors: false,
///                 has_warnings: false,
///                 locked: false,
///                 user: None,
///                 date_time_changed: None,
///                 form_title: "Demographics".to_string(),
///                 form_index: 1,
///                 form_group: "Day 0".to_string(),
///                 form_state: "In-Work".to_string(),
///                 state: Some(State {
///                     value: "form.state.in.work".to_string(),
///                     signer: "Paul Sanders - Project Manager".to_string(),
///                     signer_unique_id: 1681162687395,
///                     date_signed: Some(
///                         DateTime::parse_from_rfc3339("2023-04-16T16:10:02Z")
///                             .unwrap()
///                             .with_timezone(&Utc),
///                     ),
///                 }),
///                 category: Category {
///                     name: "Demographics".to_string(),
///                     category_type: "normal".to_string(),
///                     highest_index: 0,
///                     fields: vec![Field {
///                         name: "breed".to_string(),
///                         field_type: "combo-box".to_string(),
///                         data_type: "string".to_string(),
///                         error_code: "valid".to_string(),
///                         when_created: DateTime::parse_from_rfc3339("2023-04-15T16:08:26Z")
///                             .unwrap()
///                             .with_timezone(&Utc),
///                         keep_history: true,
///                         entries: vec![Entry {
///                             id: 1,
///                             value: Value {
///                                 by: "Paul Sanders".to_string(),
///                                 by_unique_id: 1681162687395,
///                                 role: "Project Manager".to_string(),
///                                 when: DateTime::parse_from_rfc3339("2023-04-15T16:09:02Z")
///                                     .unwrap()
///                                     .with_timezone(&Utc),
///                                 value: "Labrador".to_string(),
///                             },
///                         }],
///                     }],
///                 },
///             }],
///         },
///     ],
/// };
/// let result = parse_native_string(xml).unwrap();
/// assert_eq!(result, expected);
/// ```
pub fn parse_native_string(xml_str: &str) -> Result<Native, Error> {
    let native: Native = serde_xml_rs::from_str(xml_str)?;

    Ok(native)
}
