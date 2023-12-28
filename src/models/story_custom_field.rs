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
pub struct StoryCustomField {
    /// The unique public ID for a CustomField.
    #[serde(rename = "field_id")]
    pub field_id: uuid::Uuid,
    /// The unique public ID for a CustomFieldEnumValue.
    #[serde(rename = "value_id")]
    pub value_id: uuid::Uuid,
    /// A string representation of the value, if applicable.
    #[serde(rename = "value")]
    pub value: String,
}

impl StoryCustomField {
    pub fn new(field_id: uuid::Uuid, value_id: uuid::Uuid, value: String) -> StoryCustomField {
        StoryCustomField { field_id, value_id, value }
    }
}
