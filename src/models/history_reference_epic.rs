/*
 * Shortcut API
 *
 * Shortcut API
 *
 * The version of the OpenAPI document: 3.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// HistoryReferenceEpic : A reference to an Epic.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct HistoryReferenceEpic {
    /// The ID of the entity referenced.
    #[serde(rename = "id")]
    pub id: serde_json::Value,
    /// The type of entity referenced.
    #[serde(rename = "entity_type")]
    pub entity_type: String,
    /// The application URL of the Epic.
    #[serde(rename = "app_url")]
    pub app_url: String,
    /// The name of the entity referenced.
    #[serde(rename = "name")]
    pub name: String,
}

impl HistoryReferenceEpic {
    /// A reference to an Epic.
    pub fn new(
        id: serde_json::Value,
        entity_type: String,
        app_url: String,
        name: String,
    ) -> HistoryReferenceEpic {
        HistoryReferenceEpic { id, entity_type, app_url, name }
    }
}
