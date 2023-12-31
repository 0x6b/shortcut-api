/*
 * Shortcut API
 *
 * Shortcut API
 *
 * The version of the OpenAPI document: 3.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// HistoryReferenceCustomFieldEnumValue : A reference to a CustomField value asserted on a Story.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct HistoryReferenceCustomFieldEnumValue {
    /// The type of entity referenced.
    #[serde(rename = "entity_type")]
    pub entity_type: String,
    /// The name as it is displayed to the user of the parent custom-field of this enum value.
    #[serde(rename = "field_name")]
    pub field_name: String,
    /// The custom-field enum value as a string.
    #[serde(rename = "integer_value", deserialize_with = "Option::deserialize")]
    pub integer_value: Option<i64>,
    /// Whether or not the custom-field is enabled.
    #[serde(rename = "field_enabled")]
    pub field_enabled: bool,
    /// The ID of the entity referenced.
    #[serde(rename = "id")]
    pub id: serde_json::Value,
    /// The type variety of the parent custom-field of this enum value.
    #[serde(rename = "field_type")]
    pub field_type: String,
    /// The public-id of the parent custom-field of this enum value.
    #[serde(rename = "field_id")]
    pub field_id: uuid::Uuid,
    /// The custom-field enum value as a string.
    #[serde(rename = "string_value", deserialize_with = "Option::deserialize")]
    pub string_value: Option<String>,
    /// Whether or not the custom-field enum value is enabled.
    #[serde(rename = "enum_value_enabled", deserialize_with = "Option::deserialize")]
    pub enum_value_enabled: Option<bool>,
}

impl HistoryReferenceCustomFieldEnumValue {
    /// A reference to a CustomField value asserted on a Story.
    pub fn new(
        entity_type: String,
        field_name: String,
        integer_value: Option<i64>,
        field_enabled: bool,
        id: serde_json::Value,
        field_type: String,
        field_id: uuid::Uuid,
        string_value: Option<String>,
        enum_value_enabled: Option<bool>,
    ) -> HistoryReferenceCustomFieldEnumValue {
        HistoryReferenceCustomFieldEnumValue {
            entity_type,
            field_name,
            integer_value,
            field_enabled,
            id,
            field_type,
            field_id,
            string_value,
            enum_value_enabled,
        }
    }
}
