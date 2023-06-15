/*
 * Shortcut API
 *
 * Shortcut API
 *
 * The version of the OpenAPI document: 3.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// Branch : Branch refers to a VCS branch. Branches are feature branches associated with Shortcut
/// Stories.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Branch {
    /// A string description of this resource.
    #[serde(rename = "entity_type")]
    pub entity_type: String,
    /// A true/false boolean indicating if the Branch has been deleted.
    #[serde(rename = "deleted")]
    pub deleted: bool,
    /// The name of the Branch.
    #[serde(rename = "name")]
    pub name: String,
    /// A true/false boolean indicating if the Branch is persistent; e.g. master.
    #[serde(rename = "persistent")]
    pub persistent: bool,
    /// The time/date the Branch was updated.
    #[serde(rename = "updated_at", deserialize_with = "Option::deserialize")]
    pub updated_at: Option<String>,
    /// An array of PullRequests attached to the Branch (there is usually only one).
    #[serde(rename = "pull_requests")]
    pub pull_requests: Vec<crate::models::PullRequest>,
    /// The IDs of the Branches the Branch has been merged into.
    #[serde(rename = "merged_branch_ids")]
    pub merged_branch_ids: Vec<i64>,
    /// The unique ID of the Branch.
    #[serde(rename = "id", deserialize_with = "Option::deserialize")]
    pub id: Option<i64>,
    /// The URL of the Branch.
    #[serde(rename = "url")]
    pub url: String,
    /// The ID of the Repository that contains the Branch.
    #[serde(rename = "repository_id")]
    pub repository_id: i64,
    /// The time/date the Branch was created.
    #[serde(rename = "created_at", deserialize_with = "Option::deserialize")]
    pub created_at: Option<String>,
}

impl Branch {
    /// Branch refers to a VCS branch. Branches are feature branches associated with Shortcut
    /// Stories.
    pub fn new(
        entity_type: String,
        deleted: bool,
        name: String,
        persistent: bool,
        updated_at: Option<String>,
        pull_requests: Vec<crate::models::PullRequest>,
        merged_branch_ids: Vec<i64>,
        id: Option<i64>,
        url: String,
        repository_id: i64,
        created_at: Option<String>,
    ) -> Branch {
        Branch {
            entity_type,
            deleted,
            name,
            persistent,
            updated_at,
            pull_requests,
            merged_branch_ids,
            id,
            url,
            repository_id,
            created_at,
        }
    }
}
