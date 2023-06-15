/*
 * Shortcut API
 *
 * Shortcut API
 *
 * The version of the OpenAPI document: 3.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// WorkflowState : Workflow State is any of the at least 3 columns. Workflow States correspond to
/// one of 3 types: Unstarted, Started, or Done.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct WorkflowState {
    /// The description of what sort of Stories belong in that Workflow state.
    #[serde(rename = "description")]
    pub description: String,
    /// A string description of this resource.
    #[serde(rename = "entity_type")]
    pub entity_type: String,
    /// The hex color for this Workflow State.
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    /// The verb that triggers a move to that Workflow State when making VCS commits.
    #[serde(rename = "verb", deserialize_with = "Option::deserialize")]
    pub verb: Option<String>,
    /// The Workflow State's name.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "global_id")]
    pub global_id: String,
    /// The number of Stories currently in that Workflow State.
    #[serde(rename = "num_stories")]
    pub num_stories: i64,
    /// The type of Workflow State (Unstarted, Started, or Finished)
    #[serde(rename = "type")]
    pub r#type: String,
    /// When the Workflow State was last updated.
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    /// The unique ID of the Workflow State.
    #[serde(rename = "id")]
    pub id: i64,
    /// The number of Story Templates associated with that Workflow State.
    #[serde(rename = "num_story_templates")]
    pub num_story_templates: i64,
    /// The position that the Workflow State is in, starting with 0 at the left.
    #[serde(rename = "position")]
    pub position: i64,
    /// The time/date the Workflow State was created.
    #[serde(rename = "created_at")]
    pub created_at: String,
}

impl WorkflowState {
    /// Workflow State is any of the at least 3 columns. Workflow States correspond to one of 3
    /// types: Unstarted, Started, or Done.
    pub fn new(
        description: String,
        entity_type: String,
        verb: Option<String>,
        name: String,
        global_id: String,
        num_stories: i64,
        r#type: String,
        updated_at: String,
        id: i64,
        num_story_templates: i64,
        position: i64,
        created_at: String,
    ) -> WorkflowState {
        WorkflowState {
            description,
            entity_type,
            color: None,
            verb,
            name,
            global_id,
            num_stories,
            r#type,
            updated_at,
            id,
            num_story_templates,
            position,
            created_at,
        }
    }
}
