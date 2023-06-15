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
pub struct CustomFieldEnumValue {
    /// The unique public ID for the Custom Field.
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    /// A string value within the domain of this Custom Field.
    #[serde(rename = "value")]
    pub value: String,
    /// An integer indicating the position of this Value with respect to the other
    /// CustomFieldEnumValues in the enumeration.
    #[serde(rename = "position")]
    pub position: i64,
    /// A color key associated with this CustomFieldEnumValue.
    #[serde(rename = "color_key", deserialize_with = "Option::deserialize")]
    pub color_key: Option<String>,
    /// A string description of this resource.
    #[serde(rename = "entity_type")]
    pub entity_type: EntityType,
    /// When true, the CustomFieldEnumValue can be selected for the CustomField.
    #[serde(rename = "enabled")]
    pub enabled: bool,
}

impl CustomFieldEnumValue {
    pub fn new(
        id: uuid::Uuid,
        value: String,
        position: i64,
        color_key: Option<String>,
        entity_type: EntityType,
        enabled: bool,
    ) -> CustomFieldEnumValue {
        CustomFieldEnumValue {
            id,
            value,
            position,
            color_key,
            entity_type,
            enabled,
        }
    }
}

/// A string description of this resource.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EntityType {
    #[serde(rename = "custom-field-enum-value")]
    CustomFieldEnumValue,
}

impl Default for EntityType {
    fn default() -> EntityType {
        Self::CustomFieldEnumValue
    }
}
