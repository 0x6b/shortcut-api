/*
 * Shortcut API
 *
 * Shortcut API
 *
 * The version of the OpenAPI document: 3.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// HistoryReferenceStoryTask : A reference to a Story Task.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct HistoryReferenceStoryTask {
    /// The ID of the entity referenced.
    #[serde(rename = "id")]
    pub id: serde_json::Value,
    /// The type of entity referenced.
    #[serde(rename = "entity_type")]
    pub entity_type: String,
    /// The description of the Story Task.
    #[serde(rename = "description")]
    pub description: String,
}

impl HistoryReferenceStoryTask {
    /// A reference to a Story Task.
    pub fn new(
        id: serde_json::Value,
        entity_type: String,
        description: String,
    ) -> HistoryReferenceStoryTask {
        HistoryReferenceStoryTask {
            id,
            entity_type,
            description,
        }
    }
}
