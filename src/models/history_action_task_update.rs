/*
 * Shortcut API
 *
 * Shortcut API
 *
 * The version of the OpenAPI document: 3.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// HistoryActionTaskUpdate : An action representing a Task being updated.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct HistoryActionTaskUpdate {
    /// The ID of the entity referenced.
    #[serde(rename = "id")]
    pub id: i64,
    /// The type of entity referenced.
    #[serde(rename = "entity_type")]
    pub entity_type: String,
    /// The action of the entity referenced.
    #[serde(rename = "action")]
    pub action: Action,
    #[serde(rename = "changes")]
    pub changes: Box<crate::models::HistoryChangesTask>,
    /// Whether or not the Task is complete.
    #[serde(rename = "complete", skip_serializing_if = "Option::is_none")]
    pub complete: Option<bool>,
    /// The description of the Task.
    #[serde(rename = "description")]
    pub description: String,
    /// The Story ID that contains the Task.
    #[serde(rename = "story_id")]
    pub story_id: i64,
}

impl HistoryActionTaskUpdate {
    /// An action representing a Task being updated.
    pub fn new(id: i64, entity_type: String, action: Action, changes: crate::models::HistoryChangesTask, description: String, story_id: i64) -> HistoryActionTaskUpdate {
        HistoryActionTaskUpdate {
            id,
            entity_type,
            action,
            changes: Box::new(changes),
            complete: None,
            description,
            story_id,
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

