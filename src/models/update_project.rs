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
pub struct UpdateProject {
    /// The Project's description.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// A true/false boolean indicating whether the Story is in archived state.
    #[serde(rename = "archived", skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    /// The number of days before the thermometer appears in the Story summary.
    #[serde(rename = "days_to_thermometer", skip_serializing_if = "Option::is_none")]
    pub days_to_thermometer: Option<i64>,
    /// The color that represents the Project in the UI.
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    /// The Project's name.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// An array of UUIDs for any Members you want to add as Followers.
    #[serde(rename = "follower_ids", skip_serializing_if = "Option::is_none")]
    pub follower_ids: Option<Vec<uuid::Uuid>>,
    /// Configuration to enable or disable thermometers in the Story summary.
    #[serde(rename = "show_thermometer", skip_serializing_if = "Option::is_none")]
    pub show_thermometer: Option<bool>,
    /// The ID of the team the project belongs to.
    #[serde(rename = "team_id", skip_serializing_if = "Option::is_none")]
    pub team_id: Option<i64>,
    /// The Project abbreviation used in Story summaries. Should be kept to 3 characters at most.
    #[serde(rename = "abbreviation", skip_serializing_if = "Option::is_none")]
    pub abbreviation: Option<String>,
}

impl UpdateProject {
    pub fn new() -> UpdateProject {
        UpdateProject {
            description: None,
            archived: None,
            days_to_thermometer: None,
            color: None,
            name: None,
            follower_ids: None,
            show_thermometer: None,
            team_id: None,
            abbreviation: None,
        }
    }
}


