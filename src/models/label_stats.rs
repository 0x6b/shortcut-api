/*
 * Shortcut API
 *
 * Shortcut API
 *
 * The version of the OpenAPI document: 3.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// LabelStats : A group of calculated values for this Label. This is not included if the slim? flag is set to true for the List Labels endpoint.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LabelStats {
    /// The total number of Documents associated this Label.
    #[serde(rename = "num_related_documents")]
    pub num_related_documents: i64,
    /// The total number of Epics with this Label.
    #[serde(rename = "num_epics")]
    pub num_epics: i64,
    /// The total number of stories unstarted Stories with this Label.
    #[serde(rename = "num_stories_unstarted")]
    pub num_stories_unstarted: i64,
    /// The total number of Stories with this Label.
    #[serde(rename = "num_stories_total")]
    pub num_stories_total: i64,
    /// The number of unstarted epics associated with this label.
    #[serde(rename = "num_epics_unstarted")]
    pub num_epics_unstarted: i64,
    /// The number of in progress epics associated with this label.
    #[serde(rename = "num_epics_in_progress")]
    pub num_epics_in_progress: i64,
    /// The total number of unstarted points with this Label.
    #[serde(rename = "num_points_unstarted")]
    pub num_points_unstarted: i64,
    /// The total number of Stories with no point estimate with this Label.
    #[serde(rename = "num_stories_unestimated")]
    pub num_stories_unestimated: i64,
    /// The total number of in-progress points with this Label.
    #[serde(rename = "num_points_in_progress")]
    pub num_points_in_progress: i64,
    /// The total number of Epics associated with this Label.
    #[serde(rename = "num_epics_total")]
    pub num_epics_total: i64,
    /// The total number of completed Stories with this Label.
    #[serde(rename = "num_stories_completed")]
    pub num_stories_completed: i64,
    /// The total number of completed points with this Label.
    #[serde(rename = "num_points_completed")]
    pub num_points_completed: i64,
    /// The total number of stories backlog Stories with this Label.
    #[serde(rename = "num_stories_backlog")]
    pub num_stories_backlog: i64,
    /// The total number of points with this Label.
    #[serde(rename = "num_points_total")]
    pub num_points_total: i64,
    /// The total number of in-progress Stories with this Label.
    #[serde(rename = "num_stories_in_progress")]
    pub num_stories_in_progress: i64,
    /// The total number of backlog points with this Label.
    #[serde(rename = "num_points_backlog")]
    pub num_points_backlog: i64,
    /// The number of completed Epics associated with this Label.
    #[serde(rename = "num_epics_completed")]
    pub num_epics_completed: i64,
}

impl LabelStats {
    /// A group of calculated values for this Label. This is not included if the slim? flag is set to true for the List Labels endpoint.
    pub fn new(num_related_documents: i64, num_epics: i64, num_stories_unstarted: i64, num_stories_total: i64, num_epics_unstarted: i64, num_epics_in_progress: i64, num_points_unstarted: i64, num_stories_unestimated: i64, num_points_in_progress: i64, num_epics_total: i64, num_stories_completed: i64, num_points_completed: i64, num_stories_backlog: i64, num_points_total: i64, num_stories_in_progress: i64, num_points_backlog: i64, num_epics_completed: i64) -> LabelStats {
        LabelStats {
            num_related_documents,
            num_epics,
            num_stories_unstarted,
            num_stories_total,
            num_epics_unstarted,
            num_epics_in_progress,
            num_points_unstarted,
            num_stories_unestimated,
            num_points_in_progress,
            num_epics_total,
            num_stories_completed,
            num_points_completed,
            num_stories_backlog,
            num_points_total,
            num_stories_in_progress,
            num_points_backlog,
            num_epics_completed,
        }
    }
}


