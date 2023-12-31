/*
 * Shortcut API
 *
 * Shortcut API
 *
 * The version of the OpenAPI document: 3.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateMilestone {
    /// The name of the Milestone.
    #[serde(rename = "name")]
    pub name: String,
    /// The Milestone's description.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The workflow state that the Milestone is in.
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
    /// A manual override for the time/date the Milestone was started.
    #[serde(rename = "started_at_override", skip_serializing_if = "Option::is_none")]
    pub started_at_override: Option<String>,
    /// A manual override for the time/date the Milestone was completed.
    #[serde(rename = "completed_at_override", skip_serializing_if = "Option::is_none")]
    pub completed_at_override: Option<String>,
    /// An array of IDs of Categories attached to the Milestone.
    #[serde(rename = "categories", skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<crate::models::CreateCategoryParams>>,
}

impl CreateMilestone {
    pub fn new(name: String) -> CreateMilestone {
        CreateMilestone {
            name,
            description: None,
            state: None,
            started_at_override: None,
            completed_at_override: None,
            categories: None,
        }
    }
}

/// The workflow state that the Milestone is in.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "in progress")]
    InProgress,
    #[serde(rename = "to do")]
    ToDo,
    #[serde(rename = "done")]
    Done,
}

impl Default for State {
    fn default() -> State {
        Self::InProgress
    }
}
