/*
 * Shortcut API
 *
 * Shortcut API
 *
 * The version of the OpenAPI document: 3.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// HistoryActionTaskCreate : An action representing a Task being created.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct HistoryActionTaskCreate {
    /// The description of the Task.
    #[serde(rename = "description")]
    pub description: String,
    /// The type of entity referenced.
    #[serde(rename = "entity_type")]
    pub entity_type: String,
    /// An array of Member IDs that represent who has been mentioned in the Task.
    #[serde(rename = "mention_ids", skip_serializing_if = "Option::is_none")]
    pub mention_ids: Option<Vec<uuid::Uuid>>,
    /// An array of Groups IDs that represent which have been mentioned in the Task.
    #[serde(rename = "group_mention_ids", skip_serializing_if = "Option::is_none")]
    pub group_mention_ids: Option<Vec<uuid::Uuid>>,
    /// An array of Member IDs that represent the Task's owners.
    #[serde(rename = "owner_ids", skip_serializing_if = "Option::is_none")]
    pub owner_ids: Option<Vec<uuid::Uuid>>,
    /// The ID of the entity referenced.
    #[serde(rename = "id")]
    pub id: i64,
    /// The action of the entity referenced.
    #[serde(rename = "action")]
    pub action: Action,
    /// Whether or not the Task is complete.
    #[serde(rename = "complete")]
    pub complete: bool,
    /// A timestamp that represent's the Task's deadline.
    #[serde(rename = "deadline", skip_serializing_if = "Option::is_none")]
    pub deadline: Option<String>,
}

impl HistoryActionTaskCreate {
    /// An action representing a Task being created.
    pub fn new(description: String, entity_type: String, id: i64, action: Action, complete: bool) -> HistoryActionTaskCreate {
        HistoryActionTaskCreate {
            description,
            entity_type,
            mention_ids: None,
            group_mention_ids: None,
            owner_ids: None,
            id,
            action,
            complete,
            deadline: None,
        }
    }
}

/// The action of the entity referenced.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Action {
    #[serde(rename = "create")]
    Create,
}

impl Default for Action {
    fn default() -> Action {
        Self::Create
    }
}

