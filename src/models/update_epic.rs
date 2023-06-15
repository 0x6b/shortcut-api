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
pub struct UpdateEpic {
    /// The Epic's description.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// A true/false boolean indicating whether the Epic is in archived state.
    #[serde(rename = "archived", skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    /// An array of Labels attached to the Epic.
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::models::CreateLabelParams>>,
    /// A manual override for the time/date the Epic was completed.
    #[serde(
        rename = "completed_at_override",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub completed_at_override: Option<Option<String>>,
    /// The Epic's name.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The Epic's planned start date.
    #[serde(
        rename = "planned_start_date",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub planned_start_date: Option<Option<String>>,
    /// `Deprecated` The Epic's state (to do, in progress, or done); will be ignored when
    /// `epic_state_id` is set.
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
    /// The ID of the Milestone this Epic is related to.
    #[serde(
        rename = "milestone_id",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub milestone_id: Option<Option<i64>>,
    /// The ID of the member that requested the epic.
    #[serde(rename = "requested_by_id", skip_serializing_if = "Option::is_none")]
    pub requested_by_id: Option<uuid::Uuid>,
    /// The ID of the Epic State.
    #[serde(rename = "epic_state_id", skip_serializing_if = "Option::is_none")]
    pub epic_state_id: Option<i64>,
    /// A manual override for the time/date the Epic was started.
    #[serde(
        rename = "started_at_override",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub started_at_override: Option<Option<String>>,
    /// The ID of the group to associate with the epic.
    #[serde(
        rename = "group_id",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub group_id: Option<Option<uuid::Uuid>>,
    /// An array of UUIDs for any Members you want to add as Followers on this Epic.
    #[serde(rename = "follower_ids", skip_serializing_if = "Option::is_none")]
    pub follower_ids: Option<Vec<uuid::Uuid>>,
    /// An array of UUIDs for any members you want to add as Owners on this Epic.
    #[serde(rename = "owner_ids", skip_serializing_if = "Option::is_none")]
    pub owner_ids: Option<Vec<uuid::Uuid>>,
    /// The ID of the Epic we want to move this Epic before.
    #[serde(rename = "before_id", skip_serializing_if = "Option::is_none")]
    pub before_id: Option<i64>,
    /// The ID of the Epic we want to move this Epic after.
    #[serde(rename = "after_id", skip_serializing_if = "Option::is_none")]
    pub after_id: Option<i64>,
    /// The Epic's deadline.
    #[serde(
        rename = "deadline",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub deadline: Option<Option<String>>,
}

impl UpdateEpic {
    pub fn new() -> UpdateEpic {
        UpdateEpic {
            description: None,
            archived: None,
            labels: None,
            completed_at_override: None,
            name: None,
            planned_start_date: None,
            state: None,
            milestone_id: None,
            requested_by_id: None,
            epic_state_id: None,
            started_at_override: None,
            group_id: None,
            follower_ids: None,
            owner_ids: None,
            before_id: None,
            after_id: None,
            deadline: None,
        }
    }
}

/// `Deprecated` The Epic's state (to do, in progress, or done); will be ignored when
/// `epic_state_id` is set.
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
