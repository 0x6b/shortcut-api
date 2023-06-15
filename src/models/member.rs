/*
 * Shortcut API
 *
 * Shortcut API
 *
 * The version of the OpenAPI document: 3.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Member : Details about an individual user within the Workspace.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Member {
    /// The Member's role in the Workspace.
    #[serde(rename = "role")]
    pub role: String,
    /// A string description of this resource.
    #[serde(rename = "entity_type")]
    pub entity_type: String,
    /// True/false boolean indicating whether the Member has been disabled within the Workspace.
    #[serde(rename = "disabled")]
    pub disabled: bool,
    #[serde(rename = "global_id")]
    pub global_id: String,
    /// The user state, one of partial, full, disabled, or imported.  A partial user is disabled, has no means to log in, and is not an import user.  A full user is enabled and has a means to log in.  A disabled user is disabled and has a means to log in.  An import user is disabled, has no means to log in, and is marked as an import user.
    #[serde(rename = "state")]
    pub state: State,
    /// The time/date the Member was last updated.
    #[serde(rename = "updated_at", deserialize_with = "Option::deserialize")]
    pub updated_at: Option<String>,
    /// Whether this member was created as a placeholder entity.
    #[serde(rename = "created_without_invite")]
    pub created_without_invite: bool,
    /// The Member's group ids
    #[serde(rename = "group_ids")]
    pub group_ids: Vec<uuid::Uuid>,
    /// The Member's ID in Shortcut.
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    #[serde(rename = "profile")]
    pub profile: Box<crate::models::Profile>,
    /// The time/date the Member was created.
    #[serde(rename = "created_at", deserialize_with = "Option::deserialize")]
    pub created_at: Option<String>,
    /// The id of the member that replaces this one when merged.
    #[serde(rename = "replaced_by", skip_serializing_if = "Option::is_none")]
    pub replaced_by: Option<uuid::Uuid>,
}

impl Member {
    /// Details about an individual user within the Workspace.
    pub fn new(role: String, entity_type: String, disabled: bool, global_id: String, state: State, updated_at: Option<String>, created_without_invite: bool, group_ids: Vec<uuid::Uuid>, id: uuid::Uuid, profile: crate::models::Profile, created_at: Option<String>) -> Member {
        Member {
            role,
            entity_type,
            disabled,
            global_id,
            state,
            updated_at,
            created_without_invite,
            group_ids,
            id,
            profile: Box::new(profile),
            created_at,
            replaced_by: None,
        }
    }
}

/// The user state, one of partial, full, disabled, or imported.  A partial user is disabled, has no means to log in, and is not an import user.  A full user is enabled and has a means to log in.  A disabled user is disabled and has a means to log in.  An import user is disabled, has no means to log in, and is marked as an import user.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "partial")]
    Partial,
    #[serde(rename = "full")]
    Full,
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "imported")]
    Imported,
}

impl Default for State {
    fn default() -> State {
        Self::Partial
    }
}

