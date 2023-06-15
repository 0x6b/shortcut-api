/*
 * Shortcut API
 *
 * Shortcut API
 *
 * The version of the OpenAPI document: 3.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// HistoryActionStoryCreate : An action representing a Story being created.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct HistoryActionStoryCreate {
    /// The application URL of the Story.
    #[serde(rename = "app_url")]
    pub app_url: String,
    /// The description of the Story.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Whether or not the Story has been started.
    #[serde(rename = "started", skip_serializing_if = "Option::is_none")]
    pub started: Option<bool>,
    /// The type of entity referenced.
    #[serde(rename = "entity_type")]
    pub entity_type: String,
    /// An array of Task IDs on this Story.
    #[serde(rename = "task_ids", skip_serializing_if = "Option::is_none")]
    pub task_ids: Option<Vec<i64>>,
    /// The type of Story; either feature, bug, or chore.
    #[serde(rename = "story_type")]
    pub story_type: StoryType,
    /// The name of the Story.
    #[serde(rename = "name")]
    pub name: String,
    /// Whether or not the Story is completed.
    #[serde(rename = "completed", skip_serializing_if = "Option::is_none")]
    pub completed: Option<bool>,
    /// Whether or not the Story is blocking another Story.
    #[serde(rename = "blocker", skip_serializing_if = "Option::is_none")]
    pub blocker: Option<bool>,
    /// The Epic ID for this Story.
    #[serde(rename = "epic_id", skip_serializing_if = "Option::is_none")]
    pub epic_id: Option<i64>,
    /// The ID of the Member that requested the Story.
    #[serde(rename = "requested_by_id", skip_serializing_if = "Option::is_none")]
    pub requested_by_id: Option<uuid::Uuid>,
    /// The Iteration ID the Story is in.
    #[serde(
        rename = "iteration_id",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub iteration_id: Option<Option<i64>>,
    /// An array of Labels IDs attached to the Story.
    #[serde(rename = "label_ids", skip_serializing_if = "Option::is_none")]
    pub label_ids: Option<Vec<i64>>,
    /// The Team IDs for the followers of the Story.
    #[serde(rename = "group_id", skip_serializing_if = "Option::is_none")]
    pub group_id: Option<uuid::Uuid>,
    /// An array of Workflow State IDs attached to the Story.
    #[serde(rename = "workflow_state_id", skip_serializing_if = "Option::is_none")]
    pub workflow_state_id: Option<i64>,
    /// An array of Story IDs that are the object of a Story Link relationship.
    #[serde(
        rename = "object_story_link_ids",
        skip_serializing_if = "Option::is_none"
    )]
    pub object_story_link_ids: Option<Vec<i64>>,
    /// An array of Member IDs for the followers of the Story.
    #[serde(rename = "follower_ids", skip_serializing_if = "Option::is_none")]
    pub follower_ids: Option<Vec<uuid::Uuid>>,
    /// An array of Member IDs that are the owners of the Story.
    #[serde(rename = "owner_ids", skip_serializing_if = "Option::is_none")]
    pub owner_ids: Option<Vec<uuid::Uuid>>,
    /// An array of Custom Field Enum Value ids on this Story.
    #[serde(
        rename = "custom_field_value_ids",
        skip_serializing_if = "Option::is_none"
    )]
    pub custom_field_value_ids: Option<Vec<uuid::Uuid>>,
    /// The ID of the entity referenced.
    #[serde(rename = "id")]
    pub id: i64,
    /// The estimate (or point value) for the Story.
    #[serde(rename = "estimate", skip_serializing_if = "Option::is_none")]
    pub estimate: Option<i64>,
    /// An array of Story IDs that are the subject of a Story Link relationship.
    #[serde(
        rename = "subject_story_link_ids",
        skip_serializing_if = "Option::is_none"
    )]
    pub subject_story_link_ids: Option<Vec<i64>>,
    /// The action of the entity referenced.
    #[serde(rename = "action")]
    pub action: Action,
    /// Whether or not the Story is blocked by another Story.
    #[serde(rename = "blocked", skip_serializing_if = "Option::is_none")]
    pub blocked: Option<bool>,
    /// The Project ID of the Story is in.
    #[serde(rename = "project_id", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<i64>,
    /// The timestamp representing the Story's deadline.
    #[serde(rename = "deadline", skip_serializing_if = "Option::is_none")]
    pub deadline: Option<String>,
}

impl HistoryActionStoryCreate {
    /// An action representing a Story being created.
    pub fn new(
        app_url: String,
        entity_type: String,
        story_type: StoryType,
        name: String,
        id: i64,
        action: Action,
    ) -> HistoryActionStoryCreate {
        HistoryActionStoryCreate {
            app_url,
            description: None,
            started: None,
            entity_type,
            task_ids: None,
            story_type,
            name,
            completed: None,
            blocker: None,
            epic_id: None,
            requested_by_id: None,
            iteration_id: None,
            label_ids: None,
            group_id: None,
            workflow_state_id: None,
            object_story_link_ids: None,
            follower_ids: None,
            owner_ids: None,
            custom_field_value_ids: None,
            id,
            estimate: None,
            subject_story_link_ids: None,
            action,
            blocked: None,
            project_id: None,
            deadline: None,
        }
    }
}

/// The type of Story; either feature, bug, or chore.
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
