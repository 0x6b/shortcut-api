/*
 * Shortcut API
 *
 * Shortcut API
 *
 * The version of the OpenAPI document: 3.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// HistoryReferenceLabel : A reference to an Label.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct HistoryReferenceLabel {
    /// The ID of the entity referenced.
    #[serde(rename = "id")]
    pub id: serde_json::Value,
    /// The type of entity referenced.
    #[serde(rename = "entity_type")]
    pub entity_type: String,
    /// The application URL of the Label.
    #[serde(rename = "app_url")]
    pub app_url: String,
    /// The name of the entity referenced.
    #[serde(rename = "name")]
    pub name: String,
}

impl HistoryReferenceLabel {
    /// A reference to an Label.
    pub fn new(
        id: serde_json::Value,
        entity_type: String,
        app_url: String,
        name: String,
    ) -> HistoryReferenceLabel {
        HistoryReferenceLabel { id, entity_type, app_url, name }
    }
}
