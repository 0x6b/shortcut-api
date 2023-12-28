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
pub struct CreateStoryLink {
    /// The type of link.
    #[serde(rename = "verb")]
    pub verb: Verb,
    /// The ID of the subject Story.
    #[serde(rename = "subject_id")]
    pub subject_id: i64,
    /// The ID of the object Story.
    #[serde(rename = "object_id")]
    pub object_id: i64,
}

impl CreateStoryLink {
    pub fn new(verb: Verb, subject_id: i64, object_id: i64) -> CreateStoryLink {
        CreateStoryLink { verb, subject_id, object_id }
    }
}

/// The type of link.
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
