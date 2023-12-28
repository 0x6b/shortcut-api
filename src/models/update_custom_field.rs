/*
 * Shortcut API
 *
 * Shortcut API
 *
 * The version of the OpenAPI document: 3.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UpdateCustomField {
    /// Indicates whether the Field is enabled for the Workspace. Only enabled fields can be
    /// applied to Stories.
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// A collection of objects representing reporting periods for years.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// A collection of EnumValue objects representing the values in the domain of some Custom
    /// Field.
    #[serde(rename = "values", skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<crate::models::UpdateCustomFieldEnumValue>>,
    /// A frontend-controlled string that represents the icon for this custom field.
    #[serde(rename = "icon_set_identifier", skip_serializing_if = "Option::is_none")]
    pub icon_set_identifier: Option<String>,
    /// A description of the purpose of this field.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The ID of the CustomField we want to move this CustomField before.
    #[serde(rename = "before_id", skip_serializing_if = "Option::is_none")]
    pub before_id: Option<uuid::Uuid>,
    /// The ID of the CustomField we want to move this CustomField after.
    #[serde(rename = "after_id", skip_serializing_if = "Option::is_none")]
    pub after_id: Option<uuid::Uuid>,
}

impl UpdateCustomField {
    pub fn new() -> UpdateCustomField {
        UpdateCustomField {
            enabled: None,
            name: None,
            values: None,
            icon_set_identifier: None,
            description: None,
            before_id: None,
            after_id: None,
        }
    }
}
