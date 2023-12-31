/*
 * Shortcut API
 *
 * Shortcut API
 *
 * The version of the OpenAPI document: 3.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// MilestoneSearchResults : The results of the Milestone search query.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MilestoneSearchResults {
    /// The total number of matches for the search query. The first 1000 matches can be paged
    /// through via the API.
    #[serde(rename = "total")]
    pub total: i64,
    /// A list of search results.
    #[serde(rename = "data")]
    pub data: Vec<crate::models::MilestoneSearchResult>,
    /// The URL path and query string for the next page of search results.
    #[serde(rename = "next", deserialize_with = "Option::deserialize")]
    pub next: Option<String>,
    #[serde(rename = "cursors", skip_serializing_if = "Option::is_none")]
    pub cursors: Option<Vec<String>>,
}

impl MilestoneSearchResults {
    /// The results of the Milestone search query.
    pub fn new(
        total: i64,
        data: Vec<crate::models::MilestoneSearchResult>,
        next: Option<String>,
    ) -> MilestoneSearchResults {
        MilestoneSearchResults { total, data, next, cursors: None }
    }
}
