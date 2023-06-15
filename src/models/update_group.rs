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
pub struct UpdateGroup {
    /// The description of this Group.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Whether or not this Group is archived.
    #[serde(rename = "archived", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub archived: Option<Option<bool>>,
    /// The color you wish to use for the Group in the system.
    #[serde(rename = "color", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub color: Option<Option<String>>,
    /// The Icon id for the avatar of this Group.
    #[serde(rename = "display_icon_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub display_icon_id: Option<Option<uuid::Uuid>>,
    /// The mention name of this Group.
    #[serde(rename = "mention_name", skip_serializing_if = "Option::is_none")]
    pub mention_name: Option<String>,
    /// The name of this Group.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The color key you wish to use for the Group in the system.
    #[serde(rename = "color_key", skip_serializing_if = "Option::is_none")]
    pub color_key: Option<ColorKey>,
    /// The Member ids to add to this Group.
    #[serde(rename = "member_ids", skip_serializing_if = "Option::is_none")]
    pub member_ids: Option<Vec<uuid::Uuid>>,
    /// The Workflow ids to add to the Group.
    #[serde(rename = "workflow_ids", skip_serializing_if = "Option::is_none")]
    pub workflow_ids: Option<Vec<i64>>,
}

impl UpdateGroup {
    pub fn new() -> UpdateGroup {
        UpdateGroup {
            description: None,
            archived: None,
            color: None,
            display_icon_id: None,
            mention_name: None,
            name: None,
            color_key: None,
            member_ids: None,
            workflow_ids: None,
        }
    }
}

/// The color key you wish to use for the Group in the system.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ColorKey {
    #[serde(rename = "blue")]
    Blue,
    #[serde(rename = "purple")]
    Purple,
    #[serde(rename = "midnight-blue")]
    MidnightBlue,
    #[serde(rename = "orange")]
    Orange,
    #[serde(rename = "yellow-green")]
    YellowGreen,
    #[serde(rename = "brass")]
    Brass,
    #[serde(rename = "gray")]
    Gray,
    #[serde(rename = "fuchsia")]
    Fuchsia,
    #[serde(rename = "yellow")]
    Yellow,
    #[serde(rename = "pink")]
    Pink,
    #[serde(rename = "sky-blue")]
    SkyBlue,
    #[serde(rename = "green")]
    Green,
    #[serde(rename = "red")]
    Red,
    #[serde(rename = "black")]
    Black,
    #[serde(rename = "slate")]
    Slate,
    #[serde(rename = "turquoise")]
    Turquoise,
}

impl Default for ColorKey {
    fn default() -> ColorKey {
        Self::Blue
    }
}

