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
pub struct CreateTask {
    /// The Task description.
    #[serde(rename = "description")]
    pub description: String,
    /// True/false boolean indicating whether the Task is completed. Defaults to false.
    #[serde(rename = "complete", skip_serializing_if = "Option::is_none")]
    pub complete: Option<bool>,
    /// An array of UUIDs for any members you want to add as Owners on this new Task.
    #[serde(rename = "owner_ids", skip_serializing_if = "Option::is_none")]
    pub owner_ids: Option<Vec<uuid::Uuid>>,
    /// Defaults to the time/date the Task is created but can be set to reflect another creation
    /// time/date.
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Defaults to the time/date the Task is created in Shortcut but can be set to reflect another
    /// time/date.
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// This field can be set to another unique ID. In the case that the Task has been imported
    /// from another tool, the ID in the other tool can be indicated here.
    #[serde(rename = "external_id", skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
}

impl CreateTask {
    pub fn new(description: String) -> CreateTask {
        CreateTask {
            description,
            complete: None,
            owner_ids: None,
            created_at: None,
            updated_at: None,
            external_id: None,
        }
    }
}
