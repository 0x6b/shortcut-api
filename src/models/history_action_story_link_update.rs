/*
 * Shortcut API
 *
 * Shortcut API
 *
 * The version of the OpenAPI document: 3.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// HistoryActionStoryLinkUpdate : An action representing a Story Link being updated.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct HistoryActionStoryLinkUpdate {
    /// The ID of the entity referenced.
    #[serde(rename = "id")]
    pub id: i64,
    /// The type of entity referenced.
    #[serde(rename = "entity_type")]
    pub entity_type: String,
    /// The action of the entity referenced.
    #[serde(rename = "action")]
    pub action: Action,
    /// The verb describing the link's relationship.
    #[serde(rename = "verb")]
    pub verb: Verb,
    /// The Story ID of the subject Story.
    #[serde(rename = "subject_id")]
    pub subject_id: i64,
    /// The Story ID of the object Story.
    #[serde(rename = "object_id")]
    pub object_id: i64,
    #[serde(rename = "changes")]
    pub changes: Box<crate::models::HistoryChangesStoryLink>,
}

impl HistoryActionStoryLinkUpdate {
    /// An action representing a Story Link being updated.
    pub fn new(
        id: i64,
        entity_type: String,
        action: Action,
        verb: Verb,
        subject_id: i64,
        object_id: i64,
        changes: crate::models::HistoryChangesStoryLink,
    ) -> HistoryActionStoryLinkUpdate {
        HistoryActionStoryLinkUpdate {
            id,
            entity_type,
            action,
            verb,
            subject_id,
            object_id,
            changes: Box::new(changes),
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
/// The verb describing the link's relationship.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Verb {
    #[serde(rename = "blocks")]
    Blocks,
    #[serde(rename = "duplicates")]
    Duplicates,
    #[serde(rename = "relates to")]
    RelatesTo,
}

impl Default for Verb {
    fn default() -> Verb {
        Self::Blocks
    }
}
