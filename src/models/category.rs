/*
 * Shortcut API
 *
 * Shortcut API
 *
 * The version of the OpenAPI document: 3.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// Category : A Category can be used to associate Milestones.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Category {
    /// A true/false boolean indicating if the Category has been archived.
    #[serde(rename = "archived")]
    pub archived: bool,
    /// A string description of this resource.
    #[serde(rename = "entity_type")]
    pub entity_type: String,
    /// The hex color to be displayed with the Category (for example, \"#ff0000\").
    #[serde(rename = "color", deserialize_with = "Option::deserialize")]
    pub color: Option<String>,
    /// The name of the Category.
    #[serde(rename = "name")]
    pub name: String,
    /// The type of entity this Category is associated with; currently Milestone is the only type
    /// of Category.
    #[serde(rename = "type")]
    pub r#type: String,
    /// The time/date that the Category was updated.
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    /// This field can be set to another unique ID. In the case that the Category has been imported
    /// from another tool, the ID in the other tool can be indicated here.
    #[serde(rename = "external_id", deserialize_with = "Option::deserialize")]
    pub external_id: Option<String>,
    /// The unique ID of the Category.
    #[serde(rename = "id")]
    pub id: i64,
    /// The time/date that the Category was created.
    #[serde(rename = "created_at")]
    pub created_at: String,
}

impl Category {
    /// A Category can be used to associate Milestones.
    pub fn new(
        archived: bool,
        entity_type: String,
        color: Option<String>,
        name: String,
        r#type: String,
        updated_at: String,
        external_id: Option<String>,
        id: i64,
        created_at: String,
    ) -> Category {
        Category {
            archived,
            entity_type,
            color,
            name,
            r#type,
            updated_at,
            external_id,
            id,
            created_at,
        }
    }
}
