/*
 * Shortcut API
 *
 * Shortcut API
 *
 * The version of the OpenAPI document: 3.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Workflow : Workflow is the array of defined Workflow States. Workflow can be queried using the API but must be updated in the Shortcut UI. 



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Workflow {
    /// A description of the workflow.
    #[serde(rename = "description")]
    pub description: String,
    /// A string description of this resource.
    #[serde(rename = "entity_type")]
    pub entity_type: String,
    /// An array of IDs of projects within the Workflow.
    #[serde(rename = "project_ids")]
    pub project_ids: Vec<f64>,
    /// A map of the states in this Workflow.
    #[serde(rename = "states")]
    pub states: Vec<crate::models::WorkflowState>,
    /// The name of the workflow.
    #[serde(rename = "name")]
    pub name: String,
    /// The date the Workflow was updated.
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    /// Indicates if an owner is automatically assigned when an unowned story is started.
    #[serde(rename = "auto_assign_owner")]
    pub auto_assign_owner: bool,
    /// The unique ID of the Workflow.
    #[serde(rename = "id")]
    pub id: i64,
    /// The ID of the team the workflow belongs to.
    #[serde(rename = "team_id")]
    pub team_id: i64,
    /// The date the Workflow was created.
    #[serde(rename = "created_at")]
    pub created_at: String,
    /// The unique ID of the default state that new Stories are entered into.
    #[serde(rename = "default_state_id")]
    pub default_state_id: i64,
}

impl Workflow {
    /// Workflow is the array of defined Workflow States. Workflow can be queried using the API but must be updated in the Shortcut UI. 
    pub fn new(description: String, entity_type: String, project_ids: Vec<f64>, states: Vec<crate::models::WorkflowState>, name: String, updated_at: String, auto_assign_owner: bool, id: i64, team_id: i64, created_at: String, default_state_id: i64) -> Workflow {
        Workflow {
            description,
            entity_type,
            project_ids,
            states,
            name,
            updated_at,
            auto_assign_owner,
            id,
            team_id,
            created_at,
            default_state_id,
        }
    }
}


