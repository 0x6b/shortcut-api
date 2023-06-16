use std::{env, error::Error};

use shortcut_api::apis::default_api::get_epic;
use shortcut_api::{
    apis::{
        configuration::{ApiKey, Configuration},
        default_api::{list_milestone_epics, search_epics, search_milestones, search_stories},
    },
    models::search::{Detail, EntityTypes},
    models::Search,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = Configuration {
        api_key: Some(ApiKey {
            key: env::var("SHORTCUT_API_TOKEN")?,
            prefix: None,
        }),
        ..Configuration::default()
    };

    // Search for epics with the word "documentation" in the name
    let epics = search_epics(
        &config,
        Search {
            query: "documentation".to_string(),
            page_size: Some(25),
            detail: Some(Detail::Full),
            entity_types: Some(vec![EntityTypes::Epic]),
            ..Search::default()
        },
    )
    .await?;
    epics
        .data
        .iter()
        .for_each(|e| println!("{}: {}", e.id, e.name));

    // Search for stories with the word "documentation" in the name
    let stories = search_stories(
        &config,
        Search {
            query: "documentation".to_string(),
            page_size: Some(25),
            detail: Some(Detail::Full),
            entity_types: Some(vec![EntityTypes::Story]),
            ..Search::default()
        },
    )
    .await?;
    stories
        .data
        .iter()
        .for_each(|s| println!("{}: {}", s.id, s.name));

    // Search for milestones with the word "Jun26-Jun09" in the name
    let milestones = search_milestones(
        &config,
        Search {
            query: "Jun26-Jul09".to_string(),
            ..Search::default()
        },
    )
    .await?;

    // List the epics in each milestone
    for milestone in milestones.data {
        let epics = list_milestone_epics(&config, milestone.id).await?;
        println!("{}: {}", milestone.id, milestone.name);
        epics
            .iter()
            .for_each(|e| println!("  {}: {} ({})", e.id, e.name, e.state));
    }

    Ok(())
}
