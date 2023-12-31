/*
 * Shortcut API
 *
 * Shortcut API
 *
 * The version of the OpenAPI document: 3.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// HistoryActionBranchMerge : An action representing a VCS Branch being merged.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct HistoryActionBranchMerge {
    /// The ID of the entity referenced.
    #[serde(rename = "id")]
    pub id: i64,
    /// The type of entity referenced.
    #[serde(rename = "entity_type")]
    pub entity_type: String,
    /// The name of the VCS Branch that was pushed
    #[serde(rename = "name")]
    pub name: String,
    /// The URL from the provider of the VCS Branch that was pushed
    #[serde(rename = "url")]
    pub url: String,
    /// The action of the entity referenced.
    #[serde(rename = "action")]
    pub action: Action,
}

impl HistoryActionBranchMerge {
    /// An action representing a VCS Branch being merged.
    pub fn new(
        id: i64,
        entity_type: String,
        name: String,
        url: String,
        action: Action,
    ) -> HistoryActionBranchMerge {
        HistoryActionBranchMerge { id, entity_type, name, url, action }
    }
}

/// The action of the entity referenced.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Action {
    #[serde(rename = "merge")]
    Merge,
}

impl Default for Action {
    fn default() -> Action {
        Self::Merge
    }
}
