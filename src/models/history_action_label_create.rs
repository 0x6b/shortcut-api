/*
 * Shortcut API
 *
 * Shortcut API
 *
 * The version of the OpenAPI document: 3.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// HistoryActionLabelCreate : An action representing a Label being created.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct HistoryActionLabelCreate {
    /// The ID of the entity referenced.
    #[serde(rename = "id")]
    pub id: i64,
    /// The type of entity referenced.
    #[serde(rename = "entity_type")]
    pub entity_type: String,
    /// The action of the entity referenced.
    #[serde(rename = "action")]
    pub action: Action,
    /// The application URL of the Label.
    #[serde(rename = "app_url")]
    pub app_url: String,
    /// The name of the Label.
    #[serde(rename = "name")]
    pub name: String,
}

impl HistoryActionLabelCreate {
    /// An action representing a Label being created.
    pub fn new(
        id: i64,
        entity_type: String,
        action: Action,
        app_url: String,
        name: String,
    ) -> HistoryActionLabelCreate {
        HistoryActionLabelCreate { id, entity_type, action, app_url, name }
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
