/*
 * Shortcut API
 *
 * Shortcut API
 *
 * The version of the OpenAPI document: 3.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// UpdateEntityTemplate : Request parameters for changing either a template's name or any of   the
/// attributes it is designed to pre-populate.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UpdateEntityTemplate {
    /// The updated template name.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "story_contents", skip_serializing_if = "Option::is_none")]
    pub story_contents: Option<Box<crate::models::UpdateStoryContents>>,
}

impl UpdateEntityTemplate {
    /// Request parameters for changing either a template's name or any of   the attributes it is
    /// designed to pre-populate.
    pub fn new() -> UpdateEntityTemplate {
        UpdateEntityTemplate {
            name: None,
            story_contents: None,
        }
    }
}
