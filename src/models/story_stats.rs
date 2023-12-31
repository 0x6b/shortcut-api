/*
 * Shortcut API
 *
 * Shortcut API
 *
 * The version of the OpenAPI document: 3.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// StoryStats : The stats object for Stories

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct StoryStats {
    /// The number of documents related to this Story.
    #[serde(rename = "num_related_documents")]
    pub num_related_documents: i64,
}

impl StoryStats {
    /// The stats object for Stories
    pub fn new(num_related_documents: i64) -> StoryStats {
        StoryStats { num_related_documents }
    }
}
