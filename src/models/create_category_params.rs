/*
 * Shortcut API
 *
 * Shortcut API
 *
 * The version of the OpenAPI document: 3.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// CreateCategoryParams : Request parameters for creating a Category with a Milestone.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateCategoryParams {
    /// The name of the new Category.
    #[serde(rename = "name")]
    pub name: String,
    /// The hex color to be displayed with the Category (for example, \"#ff0000\").
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    /// This field can be set to another unique ID. In the case that the Category has been imported
    /// from another tool, the ID in the other tool can be indicated here.
    #[serde(rename = "external_id", skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
}

impl CreateCategoryParams {
    /// Request parameters for creating a Category with a Milestone.
    pub fn new(name: String) -> CreateCategoryParams {
        CreateCategoryParams {
            name,
            color: None,
            external_id: None,
        }
    }
}
