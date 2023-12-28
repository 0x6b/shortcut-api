/// epic-metrics is a simple example that demonstrates how to use the Shortcut API to get the
/// average cycle time and average lead time for an epic. It also demonstrates how to use the
/// `humantime` crate to format the time in a human-readable format.
/// Equivalent to https://github.com/useshortcut/api-cookbook/blob/9f31045d6dbc88bf30ab123f778c0533c087bfb4/kanban-metrics/epic_metrics.py
use std::{env, error::Error, time::Duration};

use humantime::format_duration;
use shortcut_api::{
    apis::{
        configuration::{ApiKey, Configuration},
        default_api::get_epic,
    },
    models::EpicStats,
};

/// Stats is a wrapper around models::epic_stats::EpicStats that provides a more convenient
/// representation of the data.
#[derive(Debug)]
struct Stats {
    average_cycle_time: Time,
    average_lead_time: Time,
}

/// Convert an EpicStats into a Stats.
impl From<&Box<EpicStats>> for Stats {
    fn from(s: &Box<EpicStats>) -> Self {
        Self {
            average_cycle_time: Time::from(s.average_cycle_time),
            average_lead_time: Time::from(s.average_lead_time),
        }
    }
}

/// Time contains the raw seconds and a human-readable representation of the time.
#[derive(Debug)]
struct Time {
    seconds: i64,
    formatted: String,
}

/// Convert an Option<i64> into a Time.
impl From<Option<i64>> for Time {
    fn from(v: Option<i64>) -> Self {
        match v {
            None => Self { seconds: -1, formatted: "no data".to_string() },
            Some(seconds) => Self {
                seconds,
                formatted: format_duration(Duration::new(seconds as u64, 0)).to_string(),
            },
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Get the epic ID from the command line.
    let id = env::args()
        .nth(1)
        .expect("Usage: epic-metrics <epic-id>")
        .parse::<i64>()
        .expect("epic-id must be an integer");

    // Configure the API client with the API key from the SHORTCUT_API_TOKEN environment variable.
    let config = Configuration {
        api_key: Some(ApiKey { key: env::var("SHORTCUT_API_TOKEN")?, prefix: None }),
        ..Configuration::default()
    };

    // Get the epic
    let epic = get_epic(&config, id).await?;

    // Convert the EpicStats into a Stats.
    let stats = Stats::from(&epic.stats);

    // Print the results.
    println!("Epic {}: {}", epic.id, epic.name);
    println!(
        "- cycle time: {} seconds ({})",
        stats.average_cycle_time.seconds, stats.average_cycle_time.formatted
    );
    println!(
        "- lead time: {} seconds ({})",
        stats.average_lead_time.seconds, stats.average_lead_time.formatted
    );

    Ok(())
}
