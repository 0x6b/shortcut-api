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
pub struct CreateStoryComment {
    /// The Member ID of the Comment's author. Defaults to the user identified by the API token.
    #[serde(rename = "author_id", skip_serializing_if = "Option::is_none")]
    pub author_id: Option<uuid::Uuid>,
    /// Marks the comment as a blocker that can be surfaced to permissions or teams mentioned in the comment. Can only be used on a top-level comment.
    #[serde(rename = "blocker", skip_serializing_if = "Option::is_none")]
    pub blocker: Option<bool>,
    /// Defaults to the time/date the comment is last updated, but can be set to reflect another date.
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// This field can be set to another unique ID. In the case that the comment has been imported from another tool, the ID in the other tool can be indicated here.
    #[serde(rename = "external_id", skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// The ID of the Comment that this comment is threaded under.
    #[serde(rename = "parent_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<Option<i64>>,
    /// Marks the comment as an unblocker to its  blocker parent. Can only be set on a threaded comment who has a parent with `blocker` set.
    #[serde(rename = "unblocks_parent", skip_serializing_if = "Option::is_none")]
    pub unblocks_parent: Option<bool>,
    /// Defaults to the time/date the comment is created, but can be set to reflect another date.
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// The comment text.
    #[serde(rename = "text")]
    pub text: String,
}

impl CreateStoryComment {
    pub fn new(text: String) -> CreateStoryComment {
        CreateStoryComment {
            author_id: None,
            blocker: None,
            updated_at: None,
            external_id: None,
            parent_id: None,
            unblocks_parent: None,
            created_at: None,
            text,
        }
    }
}


