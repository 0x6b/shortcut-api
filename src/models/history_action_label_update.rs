/*
 * Shortcut API
 *
 * Shortcut API
 *
 * The version of the OpenAPI document: 3.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// HistoryActionLabelUpdate : An action representing a Label being updated.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct HistoryActionLabelUpdate {
    /// The ID of the entity referenced.
    #[serde(rename = "id")]
    pub id: i64,
    /// The type of entity referenced.
    #[serde(rename = "entity_type")]
    pub entity_type: String,
    /// The action of the entity referenced.
    #[serde(rename = "action")]
    pub action: Action,
}

impl HistoryActionLabelUpdate {
    /// An action representing a Label being updated.
    pub fn new(id: i64, entity_type: String, action: Action) -> HistoryActionLabelUpdate {
        HistoryActionLabelUpdate {
            id,
            entity_type,
            action,
        }
    }
}

/// The action of the entity referenced.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Action {
    #[serde(rename = "update")]
    Update,
}

impl Default for Action {
    fn default() -> Action {
        Self::Update
    }
}
