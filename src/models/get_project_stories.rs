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
pub struct GetProjectStories {
    /// A true/false boolean indicating whether to return Stories with their descriptions.
    #[serde(rename = "includes_description", skip_serializing_if = "Option::is_none")]
    pub includes_description: Option<bool>,
}

impl GetProjectStories {
    pub fn new() -> GetProjectStories {
        GetProjectStories { includes_description: None }
    }
}
