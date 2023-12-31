/*
 * Shortcut API
 *
 * Shortcut API
 *
 * The version of the OpenAPI document: 3.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// Task : A Task on a Story.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Task {
    /// Full text of the Task.
    #[serde(rename = "description")]
    pub description: String,
    /// A string description of this resource.
    #[serde(rename = "entity_type")]
    pub entity_type: String,
    /// The unique identifier of the parent Story.
    #[serde(rename = "story_id")]
    pub story_id: i64,
    /// Deprecated: use member_mention_ids.
    #[serde(rename = "mention_ids")]
    pub mention_ids: Vec<uuid::Uuid>,
    /// An array of UUIDs of Members mentioned in this Task.
    #[serde(rename = "member_mention_ids")]
    pub member_mention_ids: Vec<uuid::Uuid>,
    /// The time/date the Task was completed.
    #[serde(rename = "completed_at", deserialize_with = "Option::deserialize")]
    pub completed_at: Option<String>,
    /// The time/date the Task was updated.
    #[serde(rename = "updated_at", deserialize_with = "Option::deserialize")]
    pub updated_at: Option<String>,
    /// An array of UUIDs of Groups mentioned in this Task.
    #[serde(rename = "group_mention_ids")]
    pub group_mention_ids: Vec<uuid::Uuid>,
    /// An array of UUIDs of the Owners of this Task.
    #[serde(rename = "owner_ids")]
    pub owner_ids: Vec<uuid::Uuid>,
    /// This field can be set to another unique ID. In the case that the Task has been imported
    /// from another tool, the ID in the other tool can be indicated here.
    #[serde(rename = "external_id", deserialize_with = "Option::deserialize")]
    pub external_id: Option<String>,
    /// The unique ID of the Task.
    #[serde(rename = "id")]
    pub id: i64,
    /// The number corresponding to the Task's position within a list of Tasks on a Story.
    #[serde(rename = "position")]
    pub position: i64,
    /// True/false boolean indicating whether the Task has been completed.
    #[serde(rename = "complete")]
    pub complete: bool,
    /// The time/date the Task was created.
    #[serde(rename = "created_at")]
    pub created_at: String,
}

impl Task {
    /// A Task on a Story.
    pub fn new(
        description: String,
        entity_type: String,
        story_id: i64,
        mention_ids: Vec<uuid::Uuid>,
        member_mention_ids: Vec<uuid::Uuid>,
        completed_at: Option<String>,
        updated_at: Option<String>,
        group_mention_ids: Vec<uuid::Uuid>,
        owner_ids: Vec<uuid::Uuid>,
        external_id: Option<String>,
        id: i64,
        position: i64,
        complete: bool,
        created_at: String,
    ) -> Task {
        Task {
            description,
            entity_type,
            story_id,
            mention_ids,
            member_mention_ids,
            completed_at,
            updated_at,
            group_mention_ids,
            owner_ids,
            external_id,
            id,
            position,
            complete,
            created_at,
        }
    }
}
