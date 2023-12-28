/*
 * Shortcut API
 *
 * Shortcut API
 *
 * The version of the OpenAPI document: 3.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// Profile : A group of Member profile details.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Profile {
    /// A string description of this resource.
    #[serde(rename = "entity_type")]
    pub entity_type: String,
    /// A true/false boolean indicating whether the Member has been deactivated within Shortcut.
    #[serde(rename = "deactivated")]
    pub deactivated: bool,
    /// If Two Factor Authentication is activated for this User.
    #[serde(rename = "two_factor_auth_activated", skip_serializing_if = "Option::is_none")]
    pub two_factor_auth_activated: Option<bool>,
    /// The Member's username within the Organization.
    #[serde(rename = "mention_name")]
    pub mention_name: String,
    /// The Member's name within the Organization.
    #[serde(rename = "name", deserialize_with = "Option::deserialize")]
    pub name: Option<String>,
    /// This is the gravatar hash associated with email_address.
    #[serde(rename = "gravatar_hash", deserialize_with = "Option::deserialize")]
    pub gravatar_hash: Option<String>,
    /// The unique identifier of the profile.
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    #[serde(rename = "display_icon")]
    pub display_icon: Option<Box<crate::models::Icon>>,
    /// A boolean indicating whether this profile is an owner at their associated organization.
    #[serde(rename = "is_owner")]
    pub is_owner: bool,
    /// The primary email address of the Member with the Organization.
    #[serde(rename = "email_address", deserialize_with = "Option::deserialize")]
    pub email_address: Option<String>,
}

impl Profile {
    /// A group of Member profile details.
    pub fn new(
        entity_type: String,
        deactivated: bool,
        mention_name: String,
        name: Option<String>,
        gravatar_hash: Option<String>,
        id: uuid::Uuid,
        display_icon: crate::models::Icon,
        is_owner: bool,
        email_address: Option<String>,
    ) -> Profile {
        Profile {
            entity_type,
            deactivated,
            two_factor_auth_activated: None,
            mention_name,
            name,
            gravatar_hash,
            id,
            display_icon: Some(Box::new(display_icon)),
            is_owner,
            email_address,
        }
    }
}
