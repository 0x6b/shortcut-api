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
pub struct SearchStories {
    /// A true/false boolean indicating whether the Story is in archived state.
    #[serde(rename = "archived", skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    /// An array of UUIDs for any Users who may be Owners of the Stories.
    #[serde(
        rename = "owner_id",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub owner_id: Option<Option<uuid::Uuid>>,
    /// The type of Stories that you want returned.
    #[serde(rename = "story_type", skip_serializing_if = "Option::is_none")]
    pub story_type: Option<StoryType>,
    /// The Epic IDs that may be associated with the Stories.
    #[serde(rename = "epic_ids", skip_serializing_if = "Option::is_none")]
    pub epic_ids: Option<Vec<i64>>,
    /// The IDs for the Projects the Stories may be assigned to.
    #[serde(rename = "project_ids", skip_serializing_if = "Option::is_none")]
    pub project_ids: Option<Vec<i64>>,
    /// Stories should have been updated before this date.
    #[serde(rename = "updated_at_end", skip_serializing_if = "Option::is_none")]
    pub updated_at_end: Option<String>,
    /// Stories should have been completed before this date.
    #[serde(rename = "completed_at_end", skip_serializing_if = "Option::is_none")]
    pub completed_at_end: Option<String>,
    /// The type of Workflow State the Stories may be in.
    #[serde(rename = "workflow_state_types", skip_serializing_if = "Option::is_none")]
    pub workflow_state_types: Option<Vec<WorkflowStateTypes>>,
    /// Stories should have a deadline before this date.
    #[serde(rename = "deadline_end", skip_serializing_if = "Option::is_none")]
    pub deadline_end: Option<String>,
    /// Stories should have been created after this date.
    #[serde(rename = "created_at_start", skip_serializing_if = "Option::is_none")]
    pub created_at_start: Option<String>,
    /// The Epic IDs that may be associated with the Stories.
    #[serde(
        rename = "epic_id",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub epic_id: Option<Option<i64>>,
    /// The name of any associated Labels.
    #[serde(rename = "label_name", skip_serializing_if = "Option::is_none")]
    pub label_name: Option<String>,
    /// The UUID of any Users who may have requested the Stories.
    #[serde(rename = "requested_by_id", skip_serializing_if = "Option::is_none")]
    pub requested_by_id: Option<uuid::Uuid>,
    /// The Iteration ID that may be associated with the Stories.
    #[serde(
        rename = "iteration_id",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub iteration_id: Option<Option<i64>>,
    /// The Label IDs that may be associated with the Stories.
    #[serde(rename = "label_ids", skip_serializing_if = "Option::is_none")]
    pub label_ids: Option<Vec<i64>>,
    /// The Group ID that is associated with the Stories
    #[serde(
        rename = "group_id",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub group_id: Option<Option<uuid::Uuid>>,
    /// The unique IDs of the specific Workflow States that the Stories should be in.
    #[serde(rename = "workflow_state_id", skip_serializing_if = "Option::is_none")]
    pub workflow_state_id: Option<i64>,
    /// The Iteration IDs that may be associated with the Stories.
    #[serde(rename = "iteration_ids", skip_serializing_if = "Option::is_none")]
    pub iteration_ids: Option<Vec<i64>>,
    /// Stories should have been created before this date.
    #[serde(rename = "created_at_end", skip_serializing_if = "Option::is_none")]
    pub created_at_end: Option<String>,
    /// Stories should have a deadline after this date.
    #[serde(rename = "deadline_start", skip_serializing_if = "Option::is_none")]
    pub deadline_start: Option<String>,
    /// The Group IDs that are associated with the Stories
    #[serde(rename = "group_ids", skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<Vec<uuid::Uuid>>,
    /// An array of UUIDs for any Users who may be Owners of the Stories.
    #[serde(rename = "owner_ids", skip_serializing_if = "Option::is_none")]
    pub owner_ids: Option<Vec<uuid::Uuid>>,
    /// An ID or URL that references an external resource. Useful during imports.
    #[serde(rename = "external_id", skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// Whether to include the story description in the response.
    #[serde(rename = "includes_description", skip_serializing_if = "Option::is_none")]
    pub includes_description: Option<bool>,
    /// The number of estimate points associate with the Stories.
    #[serde(rename = "estimate", skip_serializing_if = "Option::is_none")]
    pub estimate: Option<i64>,
    /// The IDs for the Projects the Stories may be assigned to.
    #[serde(
        rename = "project_id",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub project_id: Option<Option<i64>>,
    /// Stories should have been completed after this date.
    #[serde(rename = "completed_at_start", skip_serializing_if = "Option::is_none")]
    pub completed_at_start: Option<String>,
    /// Stories should have been updated after this date.
    #[serde(rename = "updated_at_start", skip_serializing_if = "Option::is_none")]
    pub updated_at_start: Option<String>,
}

impl SearchStories {
    pub fn new() -> SearchStories {
        SearchStories {
            archived: None,
            owner_id: None,
            story_type: None,
            epic_ids: None,
            project_ids: None,
            updated_at_end: None,
            completed_at_end: None,
            workflow_state_types: None,
            deadline_end: None,
            created_at_start: None,
            epic_id: None,
            label_name: None,
            requested_by_id: None,
            iteration_id: None,
            label_ids: None,
            group_id: None,
            workflow_state_id: None,
            iteration_ids: None,
            created_at_end: None,
            deadline_start: None,
            group_ids: None,
            owner_ids: None,
            external_id: None,
            includes_description: None,
            estimate: None,
            project_id: None,
            completed_at_start: None,
            updated_at_start: None,
        }
    }
}

/// The type of Stories that you want returned.
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
/// The type of Workflow State the Stories may be in.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum WorkflowStateTypes {
    #[serde(rename = "started")]
    Started,
    #[serde(rename = "backlog")]
    Backlog,
    #[serde(rename = "unstarted")]
    Unstarted,
    #[serde(rename = "done")]
    Done,
}

impl Default for WorkflowStateTypes {
    fn default() -> WorkflowStateTypes {
        Self::Started
    }
}
