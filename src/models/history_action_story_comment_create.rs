/*
 * Shortcut API
 *
 * Shortcut API
 *
 * The version of the OpenAPI document: 3.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// HistoryActionStoryCommentCreate : An action representing a Story Comment being created.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct HistoryActionStoryCommentCreate {
    /// The ID of the entity referenced.
    #[serde(rename = "id")]
    pub id: i64,
    /// The type of entity referenced.
    #[serde(rename = "entity_type")]
    pub entity_type: String,
    /// The action of the entity referenced.
    #[serde(rename = "action")]
    pub action: Action,
    /// The application URL of the Story Comment.
    #[serde(rename = "app_url")]
    pub app_url: String,
    /// The text of the Story Comment.
    #[serde(rename = "text")]
    pub text: String,
    /// The Member ID of who created the Story Comment.
    #[serde(rename = "author_id")]
    pub author_id: uuid::Uuid,
}

impl HistoryActionStoryCommentCreate {
    /// An action representing a Story Comment being created.
    pub fn new(
        id: i64,
        entity_type: String,
        action: Action,
        app_url: String,
        text: String,
        author_id: uuid::Uuid,
    ) -> HistoryActionStoryCommentCreate {
        HistoryActionStoryCommentCreate {
            id,
            entity_type,
            action,
            app_url,
            text,
            author_id,
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
