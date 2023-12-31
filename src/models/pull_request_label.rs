/*
 * Shortcut API
 *
 * Shortcut API
 *
 * The version of the OpenAPI document: 3.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// PullRequestLabel : Corresponds to a VCS Label associated with a Pull Request.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PullRequestLabel {
    /// A string description of this resource.
    #[serde(rename = "entity_type")]
    pub entity_type: String,
    /// The unique ID of the VCS Label.
    #[serde(rename = "id")]
    pub id: String,
    /// The color of the VCS label.
    #[serde(rename = "color")]
    pub color: String,
    /// The description of the VCS label.
    #[serde(
        rename = "description",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<Option<String>>,
    /// The name of the VCS label.
    #[serde(rename = "name")]
    pub name: String,
}

impl PullRequestLabel {
    /// Corresponds to a VCS Label associated with a Pull Request.
    pub fn new(entity_type: String, id: String, color: String, name: String) -> PullRequestLabel {
        PullRequestLabel { entity_type, id, color, description: None, name }
    }
}
