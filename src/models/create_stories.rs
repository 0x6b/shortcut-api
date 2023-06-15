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
pub struct CreateStories {
    /// An array of stories to be created.
    #[serde(rename = "stories")]
    pub stories: Vec<crate::models::CreateStoryParams>,
}

impl CreateStories {
    pub fn new(stories: Vec<crate::models::CreateStoryParams>) -> CreateStories {
        CreateStories { stories }
    }
}
