/*
 * Shortcut API
 *
 * Shortcut API
 *
 * The version of the OpenAPI document: 3.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// CreateStoryLinkParams : Request parameters for creating a Story Link within a Story.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateStoryLinkParams {
    /// The unique ID of the Story defined as subject.
    #[serde(rename = "subject_id", skip_serializing_if = "Option::is_none")]
    pub subject_id: Option<i64>,
    /// How the subject Story acts on the object Story. This can be \"blocks\", \"duplicates\", or
    /// \"relates to\".
    #[serde(rename = "verb")]
    pub verb: Verb,
    /// The unique ID of the Story defined as object.
    #[serde(rename = "object_id", skip_serializing_if = "Option::is_none")]
    pub object_id: Option<i64>,
}

impl CreateStoryLinkParams {
    /// Request parameters for creating a Story Link within a Story.
    pub fn new(verb: Verb) -> CreateStoryLinkParams {
        CreateStoryLinkParams { subject_id: None, verb, object_id: None }
    }
}

/// How the subject Story acts on the object Story. This can be \"blocks\", \"duplicates\", or
/// \"relates to\".
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Verb {
    #[serde(rename = "blocks")]
    Blocks,
    #[serde(rename = "duplicates")]
    Duplicates,
    #[serde(rename = "relates to")]
    RelatesTo,
}

impl Default for Verb {
    fn default() -> Verb {
        Self::Blocks
    }
}
