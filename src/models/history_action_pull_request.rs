/*
 * Shortcut API
 *
 * Shortcut API
 *
 * The version of the OpenAPI document: 3.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// HistoryActionPullRequest : An action representing various operations for a Pull Request.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct HistoryActionPullRequest {
    /// The ID of the entity referenced.
    #[serde(rename = "id")]
    pub id: i64,
    /// The type of entity referenced.
    #[serde(rename = "entity_type")]
    pub entity_type: String,
    /// The action of the entity referenced.
    #[serde(rename = "action")]
    pub action: Action,
    /// The VCS Repository-specific ID for the Pull Request.
    #[serde(rename = "number")]
    pub number: i64,
    /// The title of the Pull Request.
    #[serde(rename = "title")]
    pub title: String,
    /// The URL from the provider of the VCS Pull Request.
    #[serde(rename = "url")]
    pub url: String,
}

impl HistoryActionPullRequest {
    /// An action representing various operations for a Pull Request.
    pub fn new(
        id: i64,
        entity_type: String,
        action: Action,
        number: i64,
        title: String,
        url: String,
    ) -> HistoryActionPullRequest {
        HistoryActionPullRequest { id, entity_type, action, number, title, url }
    }
}

/// The action of the entity referenced.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Action {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "update")]
    Update,
    #[serde(rename = "reopen")]
    Reopen,
    #[serde(rename = "close")]
    Close,
    #[serde(rename = "sync")]
    Sync,
    #[serde(rename = "comment")]
    Comment,
}

impl Default for Action {
    fn default() -> Action {
        Self::Open
    }
}
