/*
 * Shortcut API
 *
 * Shortcut API
 *
 * The version of the OpenAPI document: 3.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CreateStoryParams : Request parameters for creating a story.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateStoryParams {
    /// The description of the story.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Controls the story's archived state.
    #[serde(rename = "archived", skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    /// An array of story links attached to the story.
    #[serde(rename = "story_links", skip_serializing_if = "Option::is_none")]
    pub story_links: Option<Vec<crate::models::CreateStoryLinkParams>>,
    /// An array of labels attached to the story.
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::models::CreateLabelParams>>,
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
    #[serde(rename = "completed_at_override", skip_serializing_if = "Option::is_none")]
    pub completed_at_override: Option<String>,
    /// The name of the story.
    #[serde(rename = "name")]
    pub name: String,
    /// An array of comments to add to the story.
    #[serde(rename = "comments", skip_serializing_if = "Option::is_none")]
    pub comments: Option<Vec<crate::models::CreateStoryCommentParams>>,
    /// The ID of the epic the story belongs to.
    #[serde(rename = "epic_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub epic_id: Option<Option<i64>>,
    /// The id of the story template used to create this story, if applicable. This is just an association; no content from the story template is inherited by the story simply by setting this field.
    #[serde(rename = "story_template_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub story_template_id: Option<Option<uuid::Uuid>>,
    /// An array of External Links associated with this story.
    #[serde(rename = "external_links", skip_serializing_if = "Option::is_none")]
    pub external_links: Option<Vec<String>>,
    /// The ID of the member that requested the story.
    #[serde(rename = "requested_by_id", skip_serializing_if = "Option::is_none")]
    pub requested_by_id: Option<uuid::Uuid>,
    /// The ID of the iteration the story belongs to.
    #[serde(rename = "iteration_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub iteration_id: Option<Option<i64>>,
    /// An array of tasks connected to the story.
    #[serde(rename = "tasks", skip_serializing_if = "Option::is_none")]
    pub tasks: Option<Vec<crate::models::CreateTaskParams>>,
    /// A manual override for the time/date the Story was started.
    #[serde(rename = "started_at_override", skip_serializing_if = "Option::is_none")]
    pub started_at_override: Option<String>,
    /// The id of the group to associate with this story.
    #[serde(rename = "group_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub group_id: Option<Option<uuid::Uuid>>,
    /// The ID of the workflow state the story will be in.
    #[serde(rename = "workflow_state_id", skip_serializing_if = "Option::is_none")]
    pub workflow_state_id: Option<i64>,
    /// The time/date the Story was updated.
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// An array of UUIDs of the followers of this story.
    #[serde(rename = "follower_ids", skip_serializing_if = "Option::is_none")]
    pub follower_ids: Option<Vec<uuid::Uuid>>,
    /// An array of UUIDs of the owners of this story.
    #[serde(rename = "owner_ids", skip_serializing_if = "Option::is_none")]
    pub owner_ids: Option<Vec<uuid::Uuid>>,
    /// This field can be set to another unique ID. In the case that the Story has been imported from another tool, the ID in the other tool can be indicated here.
    #[serde(rename = "external_id", skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// The numeric point estimate of the story. Can also be null, which means unestimated.
    #[serde(rename = "estimate", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub estimate: Option<Option<i64>>,
    /// The ID of the project the story belongs to.
    #[serde(rename = "project_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<Option<i64>>,
    /// An array of IDs of linked files attached to the story.
    #[serde(rename = "linked_file_ids", skip_serializing_if = "Option::is_none")]
    pub linked_file_ids: Option<Vec<i64>>,
    /// The due date of the story.
    #[serde(rename = "deadline", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub deadline: Option<Option<String>>,
    /// The time/date the Story was created.
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

impl CreateStoryParams {
    /// Request parameters for creating a story.
    pub fn new(name: String) -> CreateStoryParams {
        CreateStoryParams {
            description: None,
            archived: None,
            story_links: None,
            labels: None,
            story_type: None,
            custom_fields: None,
            move_to: None,
            file_ids: None,
            completed_at_override: None,
            name,
            comments: None,
            epic_id: None,
            story_template_id: None,
            external_links: None,
            requested_by_id: None,
            iteration_id: None,
            tasks: None,
            started_at_override: None,
            group_id: None,
            workflow_state_id: None,
            updated_at: None,
            follower_ids: None,
            owner_ids: None,
            external_id: None,
            estimate: None,
            project_id: None,
            linked_file_ids: None,
            deadline: None,
            created_at: None,
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

