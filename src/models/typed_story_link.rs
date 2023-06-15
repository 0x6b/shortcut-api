/*
 * Shortcut API
 *
 * Shortcut API
 *
 * The version of the OpenAPI document: 3.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TypedStoryLink : The type of Story Link. The string can be subject or object. 



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TypedStoryLink {
    /// A string description of this resource.
    #[serde(rename = "entity_type")]
    pub entity_type: String,
    /// The ID of the object Story.
    #[serde(rename = "object_id")]
    pub object_id: i64,
    /// How the subject Story acts on the object Story. This can be \"blocks\", \"duplicates\", or \"relates to\".
    #[serde(rename = "verb")]
    pub verb: String,
    /// This indicates whether the Story is the subject or object in the Story Link.
    #[serde(rename = "type")]
    pub r#type: String,
    /// The time/date when the Story Link was last updated.
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    /// The unique identifier of the Story Link.
    #[serde(rename = "id")]
    pub id: i64,
    /// The ID of the subject Story.
    #[serde(rename = "subject_id")]
    pub subject_id: i64,
    /// The workflow state of the \"subject\" story.
    #[serde(rename = "subject_workflow_state_id")]
    pub subject_workflow_state_id: i64,
    /// The time/date when the Story Link was created.
    #[serde(rename = "created_at")]
    pub created_at: String,
}

impl TypedStoryLink {
    /// The type of Story Link. The string can be subject or object. 
    pub fn new(entity_type: String, object_id: i64, verb: String, r#type: String, updated_at: String, id: i64, subject_id: i64, subject_workflow_state_id: i64, created_at: String) -> TypedStoryLink {
        TypedStoryLink {
            entity_type,
            object_id,
            verb,
            r#type,
            updated_at,
            id,
            subject_id,
            subject_workflow_state_id,
            created_at,
        }
    }
}


