/*
 * Shortcut API
 *
 * Shortcut API
 *
 * The version of the OpenAPI document: 3.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// HistoryReferenceBranch : A reference to a VCS Branch.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct HistoryReferenceBranch {
    /// The ID of the entity referenced.
    #[serde(rename = "id")]
    pub id: serde_json::Value,
    /// The type of entity referenced.
    #[serde(rename = "entity_type")]
    pub entity_type: String,
    /// The name of the entity referenced.
    #[serde(rename = "name")]
    pub name: String,
    /// The external URL for the Branch.
    #[serde(rename = "url")]
    pub url: String,
}

impl HistoryReferenceBranch {
    /// A reference to a VCS Branch.
    pub fn new(
        id: serde_json::Value,
        entity_type: String,
        name: String,
        url: String,
    ) -> HistoryReferenceBranch {
        HistoryReferenceBranch {
            id,
            entity_type,
            name,
            url,
        }
    }
}
