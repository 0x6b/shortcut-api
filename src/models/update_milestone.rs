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
pub struct UpdateMilestone {
    /// The Milestone's description.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// A boolean indicating whether the Milestone is archived or not
    #[serde(rename = "archived", skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    /// A manual override for the time/date the Milestone was completed.
    #[serde(
        rename = "completed_at_override",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub completed_at_override: Option<Option<String>>,
    /// The name of the Milestone.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The workflow state that the Milestone is in.
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
    /// A manual override for the time/date the Milestone was started.
    #[serde(
        rename = "started_at_override",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub started_at_override: Option<Option<String>>,
    /// An array of IDs of Categories attached to the Milestone.
    #[serde(rename = "categories", skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<crate::models::CreateCategoryParams>>,
    /// The ID of the Milestone we want to move this Milestone before.
    #[serde(rename = "before_id", skip_serializing_if = "Option::is_none")]
    pub before_id: Option<i64>,
    /// The ID of the Milestone we want to move this Milestone after.
    #[serde(rename = "after_id", skip_serializing_if = "Option::is_none")]
    pub after_id: Option<i64>,
}

impl UpdateMilestone {
    pub fn new() -> UpdateMilestone {
        UpdateMilestone {
            description: None,
            archived: None,
            completed_at_override: None,
            name: None,
            state: None,
            started_at_override: None,
            categories: None,
            before_id: None,
            after_id: None,
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
