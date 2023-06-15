/*
 * Shortcut API
 *
 * Shortcut API
 *
 * The version of the OpenAPI document: 3.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ProjectStats : A group of calculated values for this Project.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ProjectStats {
    /// The total number of stories in this Project.
    #[serde(rename = "num_stories")]
    pub num_stories: i64,
    /// The total number of points in this Project.
    #[serde(rename = "num_points")]
    pub num_points: i64,
    /// The total number of documents related to this Project
    #[serde(rename = "num_related_documents")]
    pub num_related_documents: i64,
}

impl ProjectStats {
    /// A group of calculated values for this Project.
    pub fn new(num_stories: i64, num_points: i64, num_related_documents: i64) -> ProjectStats {
        ProjectStats {
            num_stories,
            num_points,
            num_related_documents,
        }
    }
}


