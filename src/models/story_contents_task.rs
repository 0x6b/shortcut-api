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
pub struct StoryContentsTask {
    /// Full text of the Task.
    #[serde(rename = "description")]
    pub description: String,
    /// The number corresponding to the Task's position within a list of Tasks on a Story.
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<i64>,
    /// True/false boolean indicating whether the Task has been completed.
    #[serde(rename = "complete", skip_serializing_if = "Option::is_none")]
    pub complete: Option<bool>,
    /// An array of UUIDs of the Owners of this Task.
    #[serde(rename = "owner_ids", skip_serializing_if = "Option::is_none")]
    pub owner_ids: Option<Vec<uuid::Uuid>>,
    /// This field can be set to another unique ID. In the case that the Task has been imported
    /// from another tool, the ID in the other tool can be indicated here.
    #[serde(
        rename = "external_id",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub external_id: Option<Option<String>>,
}

impl StoryContentsTask {
    pub fn new(description: String) -> StoryContentsTask {
        StoryContentsTask {
            description,
            position: None,
            complete: None,
            owner_ids: None,
            external_id: None,
        }
    }
}
