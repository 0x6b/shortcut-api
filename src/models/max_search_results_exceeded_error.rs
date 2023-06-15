/*
 * Shortcut API
 *
 * Shortcut API
 *
 * The version of the OpenAPI document: 3.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// MaxSearchResultsExceededError : Error returned when total maximum supported results have been reached.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MaxSearchResultsExceededError {
    /// The name for this type of error, `maximum-results-exceeded`
    #[serde(rename = "error")]
    pub error: Error,
    /// An explanatory message: \"A maximum of 1000 search results are supported.\"
    #[serde(rename = "message")]
    pub message: String,
    /// The maximum number of search results supported, `1000`
    #[serde(rename = "maximum-results")]
    pub maximum_results: MaximumResults,
}

impl MaxSearchResultsExceededError {
    /// Error returned when total maximum supported results have been reached.
    pub fn new(error: Error, message: String, maximum_results: MaximumResults) -> MaxSearchResultsExceededError {
        MaxSearchResultsExceededError {
            error,
            message,
            maximum_results,
        }
    }
}

/// The name for this type of error, `maximum-results-exceeded`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Error {
    #[serde(rename = "maximum-results-exceeded")]
    MaximumResultsExceeded,
}

impl Default for Error {
    fn default() -> Error {
        Self::MaximumResultsExceeded
    }
}
/// The maximum number of search results supported, `1000`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MaximumResults {
    #[serde(rename = "1000")]
    Variant1000,
}

impl Default for MaximumResults {
    fn default() -> MaximumResults {
        Self::Variant1000
    }
}

