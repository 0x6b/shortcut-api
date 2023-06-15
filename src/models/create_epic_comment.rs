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
pub struct CreateEpicComment {
    /// The comment text.
    #[serde(rename = "text")]
    pub text: String,
    /// The Member ID of the Comment's author. Defaults to the user identified by the API token.
    #[serde(rename = "author_id", skip_serializing_if = "Option::is_none")]
    pub author_id: Option<uuid::Uuid>,
    /// Defaults to the time/date the comment is created, but can be set to reflect another date.
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Defaults to the time/date the comment is last updated, but can be set to reflect another date.
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// This field can be set to another unique ID. In the case that the comment has been imported from another tool, the ID in the other tool can be indicated here.
    #[serde(rename = "external_id", skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
}

impl CreateEpicComment {
    pub fn new(text: String) -> CreateEpicComment {
        CreateEpicComment {
            text,
            author_id: None,
            created_at: None,
            updated_at: None,
            external_id: None,
        }
    }
}


