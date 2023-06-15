/*
 * Shortcut API
 *
 * Shortcut API
 *
 * The version of the OpenAPI document: 3.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// SearchResults : The results of the multi-entity search query.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SearchResults {
    #[serde(rename = "epics", skip_serializing_if = "Option::is_none")]
    pub epics: Option<Box<crate::models::EpicSearchResults>>,
    #[serde(rename = "stories", skip_serializing_if = "Option::is_none")]
    pub stories: Option<Box<crate::models::StorySearchResults>>,
    #[serde(rename = "iterations", skip_serializing_if = "Option::is_none")]
    pub iterations: Option<Box<crate::models::IterationSearchResults>>,
    #[serde(rename = "milestones", skip_serializing_if = "Option::is_none")]
    pub milestones: Option<Box<crate::models::MilestoneSearchResults>>,
}

impl SearchResults {
    /// The results of the multi-entity search query.
    pub fn new() -> SearchResults {
        SearchResults {
            epics: None,
            stories: None,
            iterations: None,
            milestones: None,
        }
    }
}
