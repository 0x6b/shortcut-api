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
pub struct CustomField {
    /// A string description of the CustomField
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// A string that represents the icon that corresponds to this custom field.
    #[serde(rename = "icon_set_identifier", skip_serializing_if = "Option::is_none")]
    pub icon_set_identifier: Option<String>,
    /// A string description of this resource.
    #[serde(rename = "entity_type")]
    pub entity_type: EntityType,
    /// The types of stories this CustomField is scoped to.
    #[serde(rename = "story_types", skip_serializing_if = "Option::is_none")]
    pub story_types: Option<Vec<String>>,
    /// The name of the Custom Field.
    #[serde(rename = "name")]
    pub name: String,
    /// When true, the CustomFieldEnumValues may not be reordered.
    #[serde(rename = "fixed_position", skip_serializing_if = "Option::is_none")]
    pub fixed_position: Option<bool>,
    /// The instant when this CustomField was last updated.
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    /// The unique public ID for the CustomField.
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    /// A collection of legal values for a CustomField.
    #[serde(rename = "values", skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<crate::models::CustomFieldEnumValue>>,
    /// The type of Custom Field, eg. 'enum'.
    #[serde(rename = "field_type")]
    pub field_type: FieldType,
    /// An integer indicating the position of this Custom Field with respect to the other CustomField
    #[serde(rename = "position")]
    pub position: i64,
    /// The canonical name for a Shortcut-defined field.
    #[serde(rename = "canonical_name", skip_serializing_if = "Option::is_none")]
    pub canonical_name: Option<String>,
    /// When true, the CustomField can be applied to entities in the Workspace.
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// The instant when this CustomField was created.
    #[serde(rename = "created_at")]
    pub created_at: String,
}

impl CustomField {
    pub fn new(entity_type: EntityType, name: String, updated_at: String, id: uuid::Uuid, field_type: FieldType, position: i64, enabled: bool, created_at: String) -> CustomField {
        CustomField {
            description: None,
            icon_set_identifier: None,
            entity_type,
            story_types: None,
            name,
            fixed_position: None,
            updated_at,
            id,
            values: None,
            field_type,
            position,
            canonical_name: None,
            enabled,
            created_at,
        }
    }
}

/// A string description of this resource.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EntityType {
    #[serde(rename = "custom-field")]
    CustomField,
}

impl Default for EntityType {
    fn default() -> EntityType {
        Self::CustomField
    }
}
/// The type of Custom Field, eg. 'enum'.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FieldType {
    #[serde(rename = "enum")]
    Enum,
}

impl Default for FieldType {
    fn default() -> FieldType {
        Self::Enum
    }
}

