/*
 * Shortcut API
 *
 * Shortcut API
 *
 * The version of the OpenAPI document: 3.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// HistoryReferenceGeneral : A default reference for entity types that don't have extra fields.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct HistoryReferenceGeneral {
    /// The ID of the entity referenced.
    #[serde(rename = "id")]
    pub id: serde_json::Value,
    /// The type of entity referenced.
    #[serde(rename = "entity_type")]
    pub entity_type: String,
    /// The name of the entity referenced.
    #[serde(rename = "name")]
    pub name: String,
}

impl HistoryReferenceGeneral {
    /// A default reference for entity types that don't have extra fields.
    pub fn new(
        id: serde_json::Value,
        entity_type: String,
        name: String,
    ) -> HistoryReferenceGeneral {
        HistoryReferenceGeneral { id, entity_type, name }
    }
}
