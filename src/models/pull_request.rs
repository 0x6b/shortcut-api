/*
 * Shortcut API
 *
 * Shortcut API
 *
 * The version of the OpenAPI document: 3.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PullRequest : Corresponds to a VCS Pull Request attached to a Shortcut story.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PullRequest {
    /// A string description of this resource.
    #[serde(rename = "entity_type")]
    pub entity_type: String,
    /// True/False boolean indicating whether the VCS pull request has been closed.
    #[serde(rename = "closed")]
    pub closed: bool,
    /// True/False boolean indicating whether the VCS pull request has been merged.
    #[serde(rename = "merged")]
    pub merged: bool,
    /// Number of lines added in the pull request, according to VCS.
    #[serde(rename = "num_added")]
    pub num_added: i64,
    /// The ID of the branch for the particular pull request.
    #[serde(rename = "branch_id")]
    pub branch_id: i64,
    /// An array of Story ids that have Pull Requests that change at least one of the same lines this Pull Request changes.
    #[serde(rename = "overlapping_stories", skip_serializing_if = "Option::is_none")]
    pub overlapping_stories: Option<Vec<i64>>,
    /// The pull request's unique number ID in VCS.
    #[serde(rename = "number")]
    pub number: i64,
    /// The name of the branch for the particular pull request.
    #[serde(rename = "branch_name")]
    pub branch_name: String,
    /// The name of the target branch for the particular pull request.
    #[serde(rename = "target_branch_name")]
    pub target_branch_name: String,
    /// The number of commits on the pull request.
    #[serde(rename = "num_commits", deserialize_with = "Option::deserialize")]
    pub num_commits: Option<i64>,
    /// The title of the pull request.
    #[serde(rename = "title")]
    pub title: String,
    /// The time/date the pull request was created.
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    /// True/False boolean indicating whether the VCS pull request is in the draft state.
    #[serde(rename = "draft")]
    pub draft: bool,
    /// The unique ID associated with the pull request in Shortcut.
    #[serde(rename = "id")]
    pub id: i64,
    /// An array of PullRequestLabels attached to the PullRequest.
    #[serde(rename = "vcs_labels", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub vcs_labels: Option<Option<Vec<crate::models::PullRequestLabel>>>,
    /// The URL for the pull request.
    #[serde(rename = "url")]
    pub url: String,
    /// Number of lines removed in the pull request, according to VCS.
    #[serde(rename = "num_removed")]
    pub num_removed: i64,
    /// The status of the review for the pull request.
    #[serde(rename = "review_status", skip_serializing_if = "Option::is_none")]
    pub review_status: Option<String>,
    /// Number of lines modified in the pull request, according to VCS.
    #[serde(rename = "num_modified", deserialize_with = "Option::deserialize")]
    pub num_modified: Option<i64>,
    /// The status of the Continuous Integration workflow for the pull request.
    #[serde(rename = "build_status", skip_serializing_if = "Option::is_none")]
    pub build_status: Option<String>,
    /// The ID of the target branch for the particular pull request.
    #[serde(rename = "target_branch_id")]
    pub target_branch_id: i64,
    /// The ID of the repository for the particular pull request.
    #[serde(rename = "repository_id")]
    pub repository_id: i64,
    /// The time/date the pull request was created.
    #[serde(rename = "created_at")]
    pub created_at: String,
}

impl PullRequest {
    /// Corresponds to a VCS Pull Request attached to a Shortcut story.
    pub fn new(entity_type: String, closed: bool, merged: bool, num_added: i64, branch_id: i64, number: i64, branch_name: String, target_branch_name: String, num_commits: Option<i64>, title: String, updated_at: String, draft: bool, id: i64, url: String, num_removed: i64, num_modified: Option<i64>, target_branch_id: i64, repository_id: i64, created_at: String) -> PullRequest {
        PullRequest {
            entity_type,
            closed,
            merged,
            num_added,
            branch_id,
            overlapping_stories: None,
            number,
            branch_name,
            target_branch_name,
            num_commits,
            title,
            updated_at,
            draft,
            id,
            vcs_labels: None,
            url,
            num_removed,
            review_status: None,
            num_modified,
            build_status: None,
            target_branch_id,
            repository_id,
            created_at,
        }
    }
}


