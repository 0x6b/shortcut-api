/*
 * Shortcut API
 *
 * Shortcut API
 *
 * The version of the OpenAPI document: 3.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// Epic : An Epic is a collection of stories that together might make up a release, a milestone, or
/// some other large initiative that you are working on.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Epic {
    /// The Shortcut application url for the Epic.
    #[serde(rename = "app_url")]
    pub app_url: String,
    /// The Epic's description.
    #[serde(rename = "description")]
    pub description: String,
    /// True/false boolean that indicates whether the Epic is archived or not.
    #[serde(rename = "archived")]
    pub archived: bool,
    /// A true/false boolean indicating if the Epic has been started.
    #[serde(rename = "started")]
    pub started: bool,
    /// A string description of this resource.
    #[serde(rename = "entity_type")]
    pub entity_type: String,
    /// An array of Labels attached to the Epic.
    #[serde(rename = "labels")]
    pub labels: Vec<crate::models::LabelSlim>,
    /// Deprecated: use member_mention_ids.
    #[serde(rename = "mention_ids")]
    pub mention_ids: Vec<uuid::Uuid>,
    /// An array of Member IDs that have been mentioned in the Epic description.
    #[serde(rename = "member_mention_ids")]
    pub member_mention_ids: Vec<uuid::Uuid>,
    /// An array containing Group IDs and Group-owned story counts for the Epic's associated
    /// groups.
    #[serde(rename = "associated_groups")]
    pub associated_groups: Vec<crate::models::EpicAssociatedGroup>,
    /// The IDs of Projects related to this Epic.
    #[serde(rename = "project_ids")]
    pub project_ids: Vec<i64>,
    /// The number of stories in this epic which are not associated with a project.
    #[serde(rename = "stories_without_projects")]
    pub stories_without_projects: i64,
    /// A manual override for the time/date the Epic was completed.
    #[serde(
        rename = "completed_at_override",
        deserialize_with = "Option::deserialize"
    )]
    pub completed_at_override: Option<String>,
    /// The ID of the associated productboard integration.
    #[serde(
        rename = "productboard_plugin_id",
        deserialize_with = "Option::deserialize"
    )]
    pub productboard_plugin_id: Option<uuid::Uuid>,
    /// The time/date the Epic was started.
    #[serde(rename = "started_at", deserialize_with = "Option::deserialize")]
    pub started_at: Option<String>,
    /// The time/date the Epic was completed.
    #[serde(rename = "completed_at", deserialize_with = "Option::deserialize")]
    pub completed_at: Option<String>,
    /// The name of the Epic.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "global_id")]
    pub global_id: String,
    /// A true/false boolean indicating if the Epic has been completed.
    #[serde(rename = "completed")]
    pub completed: bool,
    /// A nested array of threaded comments.
    #[serde(rename = "comments")]
    pub comments: Vec<crate::models::ThreadedComment>,
    /// The URL of the associated productboard feature.
    #[serde(rename = "productboard_url", deserialize_with = "Option::deserialize")]
    pub productboard_url: Option<String>,
    /// The Epic's planned start date.
    #[serde(
        rename = "planned_start_date",
        deserialize_with = "Option::deserialize"
    )]
    pub planned_start_date: Option<String>,
    /// `Deprecated` The workflow state that the Epic is in.
    #[serde(rename = "state")]
    pub state: String,
    /// The ID of the Milestone this Epic is related to.
    #[serde(rename = "milestone_id", deserialize_with = "Option::deserialize")]
    pub milestone_id: Option<i64>,
    /// The ID of the Member that requested the epic.
    #[serde(rename = "requested_by_id")]
    pub requested_by_id: uuid::Uuid,
    /// The ID of the Epic State.
    #[serde(rename = "epic_state_id")]
    pub epic_state_id: i64,
    /// An array of Label ids attached to the Epic.
    #[serde(rename = "label_ids")]
    pub label_ids: Vec<i64>,
    /// A manual override for the time/date the Epic was started.
    #[serde(
        rename = "started_at_override",
        deserialize_with = "Option::deserialize"
    )]
    pub started_at_override: Option<String>,
    #[serde(rename = "group_id", deserialize_with = "Option::deserialize")]
    pub group_id: Option<uuid::Uuid>,
    /// The time/date the Epic was updated.
    #[serde(rename = "updated_at", deserialize_with = "Option::deserialize")]
    pub updated_at: Option<String>,
    /// An array of Group IDs that have been mentioned in the Epic description.
    #[serde(rename = "group_mention_ids")]
    pub group_mention_ids: Vec<uuid::Uuid>,
    /// The ID of the associated productboard feature.
    #[serde(rename = "productboard_id", deserialize_with = "Option::deserialize")]
    pub productboard_id: Option<uuid::Uuid>,
    /// An array of UUIDs for any Members you want to add as Followers on this Epic.
    #[serde(rename = "follower_ids")]
    pub follower_ids: Vec<uuid::Uuid>,
    /// An array of UUIDs for any members you want to add as Owners on this new Epic.
    #[serde(rename = "owner_ids")]
    pub owner_ids: Vec<uuid::Uuid>,
    /// This field can be set to another unique ID. In the case that the Epic has been imported
    /// from another tool, the ID in the other tool can be indicated here.
    #[serde(rename = "external_id", deserialize_with = "Option::deserialize")]
    pub external_id: Option<String>,
    /// The unique ID of the Epic.
    #[serde(rename = "id")]
    pub id: i64,
    /// The Epic's relative position in the Epic workflow state.
    #[serde(rename = "position")]
    pub position: i64,
    /// The name of the associated productboard feature.
    #[serde(rename = "productboard_name", deserialize_with = "Option::deserialize")]
    pub productboard_name: Option<String>,
    /// The Epic's deadline.
    #[serde(rename = "deadline", deserialize_with = "Option::deserialize")]
    pub deadline: Option<String>,
    #[serde(rename = "stats")]
    pub stats: Box<crate::models::EpicStats>,
    /// The time/date the Epic was created.
    #[serde(rename = "created_at", deserialize_with = "Option::deserialize")]
    pub created_at: Option<String>,
}

impl Epic {
    /// An Epic is a collection of stories that together might make up a release, a milestone, or
    /// some other large initiative that you are working on.
    pub fn new(
        app_url: String,
        description: String,
        archived: bool,
        started: bool,
        entity_type: String,
        labels: Vec<crate::models::LabelSlim>,
        mention_ids: Vec<uuid::Uuid>,
        member_mention_ids: Vec<uuid::Uuid>,
        associated_groups: Vec<crate::models::EpicAssociatedGroup>,
        project_ids: Vec<i64>,
        stories_without_projects: i64,
        completed_at_override: Option<String>,
        productboard_plugin_id: Option<uuid::Uuid>,
        started_at: Option<String>,
        completed_at: Option<String>,
        name: String,
        global_id: String,
        completed: bool,
        comments: Vec<crate::models::ThreadedComment>,
        productboard_url: Option<String>,
        planned_start_date: Option<String>,
        state: String,
        milestone_id: Option<i64>,
        requested_by_id: uuid::Uuid,
        epic_state_id: i64,
        label_ids: Vec<i64>,
        started_at_override: Option<String>,
        group_id: Option<uuid::Uuid>,
        updated_at: Option<String>,
        group_mention_ids: Vec<uuid::Uuid>,
        productboard_id: Option<uuid::Uuid>,
        follower_ids: Vec<uuid::Uuid>,
        owner_ids: Vec<uuid::Uuid>,
        external_id: Option<String>,
        id: i64,
        position: i64,
        productboard_name: Option<String>,
        deadline: Option<String>,
        stats: crate::models::EpicStats,
        created_at: Option<String>,
    ) -> Epic {
        Epic {
            app_url,
            description,
            archived,
            started,
            entity_type,
            labels,
            mention_ids,
            member_mention_ids,
            associated_groups,
            project_ids,
            stories_without_projects,
            completed_at_override,
            productboard_plugin_id,
            started_at,
            completed_at,
            name,
            global_id,
            completed,
            comments,
            productboard_url,
            planned_start_date,
            state,
            milestone_id,
            requested_by_id,
            epic_state_id,
            label_ids,
            started_at_override,
            group_id,
            updated_at,
            group_mention_ids,
            productboard_id,
            follower_ids,
            owner_ids,
            external_id,
            id,
            position,
            productboard_name,
            deadline,
            stats: Box::new(stats),
            created_at,
        }
    }
}
