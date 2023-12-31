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
pub struct UpdateStoryComment {
    /// The updated comment text.
    #[serde(rename = "text")]
    pub text: String,
}

impl UpdateStoryComment {
    pub fn new(text: String) -> UpdateStoryComment {
        UpdateStoryComment { text }
    }
}
