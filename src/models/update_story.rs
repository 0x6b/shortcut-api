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
pub struct UpdateStory {
    /// The description of the story.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// True if the story is archived, otherwise false.
    #[serde(rename = "archived", skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    /// An array of labels attached to the story.
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::models::CreateLabelParams>>,
    /// An array of IDs of Pull/Merge Requests attached to the story.
    #[serde(rename = "pull_request_ids", skip_serializing_if = "Option::is_none")]
    pub pull_request_ids: Option<Vec<i64>>,
    /// The type of story (feature, bug, chore).
    #[serde(rename = "story_type", skip_serializing_if = "Option::is_none")]
    pub story_type: Option<StoryType>,
    /// A map specifying a CustomField ID and CustomFieldEnumValue ID that represents an assertion of some value for a CustomField.
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<crate::models::CustomFieldValueParams>>,
    /// One of \"first\" or \"last\". This can be used to move the given story to the first or last position in the workflow state.
    #[serde(rename = "move_to", skip_serializing_if = "Option::is_none")]
    pub move_to: Option<MoveTo>,
    /// An array of IDs of files attached to the story.
    #[serde(rename = "file_ids", skip_serializing_if = "Option::is_none")]
    pub file_ids: Option<Vec<i64>>,
    /// A manual override for the time/date the Story was completed.
    #[serde(rename = "completed_at_override", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub completed_at_override: Option<Option<String>>,
    /// The title of the story.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The ID of the epic the story belongs to.
    #[serde(rename = "epic_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub epic_id: Option<Option<i64>>,
    /// An array of External Links associated with this story.
    #[serde(rename = "external_links", skip_serializing_if = "Option::is_none")]
    pub external_links: Option<Vec<String>>,
    /// An array of IDs of Branches attached to the story.
    #[serde(rename = "branch_ids", skip_serializing_if = "Option::is_none")]
    pub branch_ids: Option<Vec<i64>>,
    /// An array of IDs of Commits attached to the story.
    #[serde(rename = "commit_ids", skip_serializing_if = "Option::is_none")]
    pub commit_ids: Option<Vec<i64>>,
    /// The ID of the member that requested the story.
    #[serde(rename = "requested_by_id", skip_serializing_if = "Option::is_none")]
    pub requested_by_id: Option<uuid::Uuid>,
    /// The ID of the iteration the story belongs to.
    #[serde(rename = "iteration_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub iteration_id: Option<Option<i64>>,
    /// A manual override for the time/date the Story was started.
    #[serde(rename = "started_at_override", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub started_at_override: Option<Option<String>>,
    /// The ID of the group to associate with this story
    #[serde(rename = "group_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub group_id: Option<Option<uuid::Uuid>>,
    /// The ID of the workflow state to put the story in.
    #[serde(rename = "workflow_state_id", skip_serializing_if = "Option::is_none")]
    pub workflow_state_id: Option<i64>,
    /// An array of UUIDs of the followers of this story.
    #[serde(rename = "follower_ids", skip_serializing_if = "Option::is_none")]
    pub follower_ids: Option<Vec<uuid::Uuid>>,
    /// An array of UUIDs of the owners of this story.
    #[serde(rename = "owner_ids", skip_serializing_if = "Option::is_none")]
    pub owner_ids: Option<Vec<uuid::Uuid>>,
    /// The ID of the story we want to move this story before.
    #[serde(rename = "before_id", skip_serializing_if = "Option::is_none")]
    pub before_id: Option<i64>,
    /// The numeric point estimate of the story. Can also be null, which means unestimated.
    #[serde(rename = "estimate", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub estimate: Option<Option<i64>>,
    /// The ID of the story we want to move this story after.
    #[serde(rename = "after_id", skip_serializing_if = "Option::is_none")]
    pub after_id: Option<i64>,
    /// The ID of the project the story belongs to.
    #[serde(rename = "project_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<Option<i64>>,
    /// An array of IDs of linked files attached to the story.
    #[serde(rename = "linked_file_ids", skip_serializing_if = "Option::is_none")]
    pub linked_file_ids: Option<Vec<i64>>,
    /// The due date of the story.
    #[serde(rename = "deadline", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub deadline: Option<Option<String>>,
}

impl UpdateStory {
    pub fn new() -> UpdateStory {
        UpdateStory {
            description: None,
            archived: None,
            labels: None,
            pull_request_ids: None,
            story_type: None,
            custom_fields: None,
            move_to: None,
            file_ids: None,
            completed_at_override: None,
            name: None,
            epic_id: None,
            external_links: None,
            branch_ids: None,
            commit_ids: None,
            requested_by_id: None,
            iteration_id: None,
            started_at_override: None,
            group_id: None,
            workflow_state_id: None,
            follower_ids: None,
            owner_ids: None,
            before_id: None,
            estimate: None,
            after_id: None,
            project_id: None,
            linked_file_ids: None,
            deadline: None,
        }
    }
}

/// The type of story (feature, bug, chore).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum StoryType {
    #[serde(rename = "feature")]
    Feature,
    #[serde(rename = "chore")]
    Chore,
    #[serde(rename = "bug")]
    Bug,
}

impl Default for StoryType {
    fn default() -> StoryType {
        Self::Feature
    }
}
/// One of \"first\" or \"last\". This can be used to move the given story to the first or last position in the workflow state.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MoveTo {
    #[serde(rename = "last")]
    Last,
    #[serde(rename = "first")]
    First,
}

impl Default for MoveTo {
    fn default() -> MoveTo {
        Self::Last
    }
}

