/*
 * Shortcut API
 *
 * Shortcut API
 *
 * The version of the OpenAPI document: 3.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// LabelSlim : A Label can be used to associate and filter Stories and Epics, and also create new Workspaces. A slim Label does not include aggregate stats. Fetch the Label using the labels endpoint to retrieve them.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LabelSlim {
    /// The Shortcut application url for the Label.
    #[serde(rename = "app_url")]
    pub app_url: String,
    /// The description of the Label.
    #[serde(rename = "description", deserialize_with = "Option::deserialize")]
    pub description: Option<String>,
    /// A true/false boolean indicating if the Label has been archived.
    #[serde(rename = "archived")]
    pub archived: bool,
    /// A string description of this resource.
    #[serde(rename = "entity_type")]
    pub entity_type: String,
    /// The hex color to be displayed with the Label (for example, \"#ff0000\").
    #[serde(rename = "color", deserialize_with = "Option::deserialize")]
    pub color: Option<String>,
    /// The name of the Label.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "global_id")]
    pub global_id: String,
    /// The time/date that the Label was updated.
    #[serde(rename = "updated_at", deserialize_with = "Option::deserialize")]
    pub updated_at: Option<String>,
    /// This field can be set to another unique ID. In the case that the Label has been imported from another tool, the ID in the other tool can be indicated here.
    #[serde(rename = "external_id", deserialize_with = "Option::deserialize")]
    pub external_id: Option<String>,
    /// The unique ID of the Label.
    #[serde(rename = "id")]
    pub id: i64,
    /// The time/date that the Label was created.
    #[serde(rename = "created_at", deserialize_with = "Option::deserialize")]
    pub created_at: Option<String>,
}

impl LabelSlim {
    /// A Label can be used to associate and filter Stories and Epics, and also create new Workspaces. A slim Label does not include aggregate stats. Fetch the Label using the labels endpoint to retrieve them.
    pub fn new(app_url: String, description: Option<String>, archived: bool, entity_type: String, color: Option<String>, name: String, global_id: String, updated_at: Option<String>, external_id: Option<String>, id: i64, created_at: Option<String>) -> LabelSlim {
        LabelSlim {
            app_url,
            description,
            archived,
            entity_type,
            color,
            name,
            global_id,
            updated_at,
            external_id,
            id,
            created_at,
        }
    }
}


