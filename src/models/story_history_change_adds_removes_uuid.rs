/*
 * Shortcut API
 *
 * Shortcut API
 *
 * The version of the OpenAPI document: 3.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// StoryHistoryChangeAddsRemovesUuid : Custom Field Enum Value IDs that have been added or removed
/// from the Story.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct StoryHistoryChangeAddsRemovesUuid {
    /// The values that have been added.
    #[serde(rename = "adds", skip_serializing_if = "Option::is_none")]
    pub adds: Option<Vec<uuid::Uuid>>,
    /// The values that have been removed
    #[serde(rename = "removes", skip_serializing_if = "Option::is_none")]
    pub removes: Option<Vec<uuid::Uuid>>,
}

impl StoryHistoryChangeAddsRemovesUuid {
    /// Custom Field Enum Value IDs that have been added or removed from the Story.
    pub fn new() -> StoryHistoryChangeAddsRemovesUuid {
        StoryHistoryChangeAddsRemovesUuid {
            adds: None,
            removes: None,
        }
    }
}
