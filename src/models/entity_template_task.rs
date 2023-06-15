/*
 * Shortcut API
 *
 * Shortcut API
 *
 * The version of the OpenAPI document: 3.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// EntityTemplateTask : Request parameters for specifying how to pre-populate a task through a template.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct EntityTemplateTask {
    /// The Task description.
    #[serde(rename = "description")]
    pub description: String,
    /// True/false boolean indicating whether the Task is completed. Defaults to false.
    #[serde(rename = "complete", skip_serializing_if = "Option::is_none")]
    pub complete: Option<bool>,
    /// An array of UUIDs for any members you want to add as Owners on this new Task.
    #[serde(rename = "owner_ids", skip_serializing_if = "Option::is_none")]
    pub owner_ids: Option<Vec<uuid::Uuid>>,
    /// This field can be set to another unique ID. In the case that the Task has been imported from another tool, the ID in the other tool can be indicated here.
    #[serde(rename = "external_id", skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
}

impl EntityTemplateTask {
    /// Request parameters for specifying how to pre-populate a task through a template.
    pub fn new(description: String) -> EntityTemplateTask {
        EntityTemplateTask {
            description,
            complete: None,
            owner_ids: None,
            external_id: None,
        }
    }
}


