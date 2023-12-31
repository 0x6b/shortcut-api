/*
 * Shortcut API
 *
 * Shortcut API
 *
 * The version of the OpenAPI document: 3.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// HistoryChangesStoryLink : The changes that have occurred as a result of the action.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct HistoryChangesStoryLink {
    #[serde(rename = "verb", skip_serializing_if = "Option::is_none")]
    pub verb: Option<Box<crate::models::StoryHistoryChangeOldNewStr>>,
    #[serde(rename = "object_id", skip_serializing_if = "Option::is_none")]
    pub object_id: Option<Box<crate::models::StoryHistoryChangeOldNewInt>>,
    #[serde(rename = "subject_id", skip_serializing_if = "Option::is_none")]
    pub subject_id: Option<Box<crate::models::StoryHistoryChangeOldNewInt>>,
}

impl HistoryChangesStoryLink {
    /// The changes that have occurred as a result of the action.
    pub fn new() -> HistoryChangesStoryLink {
        HistoryChangesStoryLink { verb: None, object_id: None, subject_id: None }
    }
}
