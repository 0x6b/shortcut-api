/*
 * Shortcut API
 *
 * Shortcut API
 *
 * The version of the OpenAPI document: 3.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// UploadedFile : An UploadedFile is any document uploaded to your Shortcut Workspace. Files
/// attached from a third-party service are different: see the Linked Files endpoint.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UploadedFile {
    /// The description of the file.
    #[serde(rename = "description", deserialize_with = "Option::deserialize")]
    pub description: Option<String>,
    /// A string description of this resource.
    #[serde(rename = "entity_type")]
    pub entity_type: String,
    /// The unique IDs of the Stories associated with this file.
    #[serde(rename = "story_ids")]
    pub story_ids: Vec<i64>,
    /// Deprecated: use member_mention_ids.
    #[serde(rename = "mention_ids")]
    pub mention_ids: Vec<uuid::Uuid>,
    /// The unique IDs of the Members who are mentioned in the file description.
    #[serde(rename = "member_mention_ids")]
    pub member_mention_ids: Vec<uuid::Uuid>,
    /// The optional User-specified name of the file.
    #[serde(rename = "name")]
    pub name: String,
    /// The url where the thumbnail of the file can be found in Shortcut.
    #[serde(rename = "thumbnail_url", deserialize_with = "Option::deserialize")]
    pub thumbnail_url: Option<String>,
    /// The size of the file.
    #[serde(rename = "size")]
    pub size: i64,
    /// The unique ID of the Member who uploaded the file.
    #[serde(rename = "uploader_id")]
    pub uploader_id: uuid::Uuid,
    /// Free form string corresponding to a text or image file.
    #[serde(rename = "content_type")]
    pub content_type: String,
    /// The time/date that the file was updated.
    #[serde(rename = "updated_at", deserialize_with = "Option::deserialize")]
    pub updated_at: Option<String>,
    /// The name assigned to the file in Shortcut upon upload.
    #[serde(rename = "filename")]
    pub filename: String,
    /// The unique IDs of the Groups who are mentioned in the file description.
    #[serde(rename = "group_mention_ids")]
    pub group_mention_ids: Vec<uuid::Uuid>,
    /// This field can be set to another unique ID. In the case that the File has been imported
    /// from another tool, the ID in the other tool can be indicated here.
    #[serde(rename = "external_id", deserialize_with = "Option::deserialize")]
    pub external_id: Option<String>,
    /// The unique ID for the file.
    #[serde(rename = "id")]
    pub id: i64,
    /// The URL for the file.
    #[serde(rename = "url", deserialize_with = "Option::deserialize")]
    pub url: Option<String>,
    /// The time/date that the file was created.
    #[serde(rename = "created_at")]
    pub created_at: String,
}

impl UploadedFile {
    /// An UploadedFile is any document uploaded to your Shortcut Workspace. Files attached from a
    /// third-party service are different: see the Linked Files endpoint.
    pub fn new(
        description: Option<String>,
        entity_type: String,
        story_ids: Vec<i64>,
        mention_ids: Vec<uuid::Uuid>,
        member_mention_ids: Vec<uuid::Uuid>,
        name: String,
        thumbnail_url: Option<String>,
        size: i64,
        uploader_id: uuid::Uuid,
        content_type: String,
        updated_at: Option<String>,
        filename: String,
        group_mention_ids: Vec<uuid::Uuid>,
        external_id: Option<String>,
        id: i64,
        url: Option<String>,
        created_at: String,
    ) -> UploadedFile {
        UploadedFile {
            description,
            entity_type,
            story_ids,
            mention_ids,
            member_mention_ids,
            name,
            thumbnail_url,
            size,
            uploader_id,
            content_type,
            updated_at,
            filename,
            group_mention_ids,
            external_id,
            id,
            url,
            created_at,
        }
    }
}
